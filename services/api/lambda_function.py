"""PRANA-VAYU API Lambda: incident intake, DynamoDB persistence and dual Bedrock/Gemini analysis."""
from __future__ import annotations

import json
import logging
import os
import re
import time
import urllib.error
import urllib.request
import uuid
from datetime import datetime, timezone
from typing import Any, Mapping

import boto3
from botocore.config import Config

LOGGER = logging.getLogger()
LOGGER.setLevel(logging.INFO)

TABLE_NAME: str = os.environ.get("TABLE_NAME", "PRANA_VAYU_Incidents")
AWS_REGION: str = os.environ.get("AWS_REGION") or os.environ.get("AWS_REGION_NAME") or "us-east-1"
AI_PROVIDER: str = os.environ.get("AI_PROVIDER", "AUTO").upper()  # AUTO | BEDROCK | GEMINI | FALLBACK
BEDROCK_MODEL_ID: str = os.environ.get("BEDROCK_MODEL_ID", "anthropic.claude-3-5-sonnet-20240620-v1:0")
GEMINI_API_KEY: str = os.environ.get("GEMINI_API_KEY", "")
GEMINI_MODEL: str = os.environ.get("GEMINI_MODEL", "gemini-3.8-flash")

CORS_HEADERS: dict[str, str] = {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type, Authorization",
    "Content-Type": "application/json; charset=utf-8",
    "Cache-Control": "no-store",
}
JSON_ENCODER = json.JSONEncoder(default=str)
AWS_CONFIG = Config(connect_timeout=3, read_timeout=12, retries={"max_attempts": 2, "mode": "standard"})
_DDB_RESOURCE: Any = None
_BEDROCK_CLIENT: Any = None
_TABLE: Any = None

ALLOWED_JURISDICTIONS = {"India", "US", "UK", "EU", "Global"}
ALLOWED_SOURCES = {"Biomass burning", "Vehicle idling", "Silica dust", "Kitchen exhaust", "Perimeter waste combustion"}
ALLOWED_SEVERITIES = {"Moderate", "High", "Severe"}


def response(status_code: int, payload: Any) -> dict[str, Any]:
    """Return a Lambda proxy response with consistent CORS headers."""
    try:
        body = JSON_ENCODER.encode(payload)
    except (TypeError, ValueError):
        status_code, body = 500, '{"error":"Response serialization failed"}'
    return {"statusCode": status_code, "headers": dict(CORS_HEADERS), "body": body}


def _clients() -> tuple[Any, Any]:
    global _DDB_RESOURCE, _BEDROCK_CLIENT, _TABLE
    if _DDB_RESOURCE is None:
        _DDB_RESOURCE = boto3.resource("dynamodb", region_name=AWS_REGION, config=AWS_CONFIG)
        _TABLE = _DDB_RESOURCE.Table(TABLE_NAME)
    if _BEDROCK_CLIENT is None:
        _BEDROCK_CLIENT = boto3.client("bedrock-runtime", region_name=AWS_REGION, config=AWS_CONFIG)
    return _TABLE, _BEDROCK_CLIENT


def _fallback_analysis(record: Mapping[str, Any]) -> dict[str, Any]:
    """Deterministic, qualitative fallback; never invent measured pollutant values."""
    source = str(record.get("source", "Biomass burning"))
    severity = str(record.get("severity", "Moderate"))
    jurisdiction = str(record.get("jurisdiction", "Global"))
    profiles = {
        "Biomass burning": "Combustion may emit fine and coarse particles, carbon monoxide, nitrogen oxides and variable organic vapours. Fuel and burn conditions affect composition. No concentrations were measured by this application.",
        "Vehicle idling": "Vehicle exhaust may contain nitrogen oxides, fine particles, carbon monoxide and volatile organic compounds. Actual concentrations require suitable monitoring; none is attached to this report.",
        "Silica dust": "Dust-generating work may produce inhalable and respirable particles. Crystalline silica exposure cannot be confirmed visually and requires material/process assessment and suitable occupational monitoring.",
        "Kitchen exhaust": "Cooking emissions may include fine particles and vapours. Fuel type and ventilation influence exposure; no concentration was measured.",
        "Perimeter waste combustion": "Mixed-waste burning can emit fine particles and a variable mixture of gases and organic compounds. Visual observation cannot establish chemical composition.",
    }
    if severity == "Severe":
        priority = "Prioritize separation from the source and contact emergency services or the competent environmental authority if there is active fire, acute symptoms or uncontrolled danger."
    elif severity == "High":
        priority = "Arrange prompt review by the responsible site or local authority and reduce avoidable exposure while the source is assessed."
    else:
        priority = "Record observations safely and request review by the responsible site or local authority."
    legal = {
        "India": "For India, refer the report to the relevant State Pollution Control Board / Pollution Control Committee and local authority. Section 19(5) of the Air (Prevention and Control of Pollution) Act, 1981 concerns restrictions on burning certain materials in an air-pollution-control area when duly notified. Applicability depends on current notifications and facts. This is not a statutory notice or legal opinion.",
        "US": "Refer to the relevant state or local air agency. Applicable federal, state, tribal and local requirements depend on source and location. This draft does not determine a violation or issue an enforcement order.",
        "UK": "Refer to the relevant local authority or environmental regulator. Applicable requirements depend on activity, location and devolved jurisdiction. This is not a statutory notice.",
        "EU": "Refer to the competent national or local environmental authority. EU instruments operate alongside national and local law; the applicable rule must be verified for the activity and location. This is not a statutory notice.",
        "Global": "The jurisdiction is not specific enough to identify a governing statute. Refer to the competent local environmental authority and have qualified counsel verify applicable law before issuing a formal notice.",
    }
    engineering = (
        "For dust-generating work, assess source enclosure, suitable wet suppression, local exhaust ventilation, safe housekeeping and task-appropriate protective equipment selected by a qualified safety professional. Verify exposure with appropriate occupational monitoring."
        if source == "Silica dust" else
        "Prioritize source control, containment, suitable ventilation and prevention of repeat emissions. Vegetation is supplementary, not a substitute for emission control. Native Ficus or Polyalthia may be assessed only by qualified local specialists for ecology, root space, utilities, water, allergens and maintenance constraints."
    )
    return {
        "particulate_profile": profiles.get(source, profiles["Biomass burning"]),
        "immediate_protocol": [
            "Do not approach the source, enter smoke or dust, confront operators, or attempt to extinguish an industrial or waste fire. Move away if safe and contact emergency services for immediate danger.",
            f"{priority} Record time, general location and visible conditions from a safe place; follow competent responders' instructions.",
        ],
        "long_term_engineering": engineering,
        "statutory_memo": legal.get(jurisdiction, legal["Global"]),
        "analysis_origin": "deterministic_fallback",
        "measurements": {"PM2_5": None, "PM10": None, "NOx": None, "VOCs": None},
    }


def _strip_markdown_fences(text: str) -> str:
    cleaned = text.strip()
    cleaned = re.sub(r"^```(?:json)?\s*", "", cleaned, flags=re.IGNORECASE)
    cleaned = re.sub(r"\s*```$", "", cleaned)
    return cleaned.strip()


def _parse_model_json(raw_text: str, provider_tag: str = "amazon_bedrock") -> dict[str, Any]:
    cleaned = _strip_markdown_fences(raw_text)
    start, end = cleaned.find("{"), cleaned.rfind("}")
    if start < 0 or end <= start:
        raise ValueError("Model response did not contain a JSON object")
    parsed = json.loads(cleaned[start : end + 1])
    if not isinstance(parsed, dict):
        raise ValueError("Analysis must be a JSON object")
    required = {"particulate_profile", "immediate_protocol", "long_term_engineering", "statutory_memo"}
    if not required.issubset(parsed):
        raise ValueError("Analysis is missing required keys")
    for key in ("particulate_profile", "long_term_engineering", "statutory_memo"):
        if not isinstance(parsed[key], str):
            raise ValueError(f"Field {key} must be a string")
        parsed[key] = parsed[key][:6000]
    if not isinstance(parsed["immediate_protocol"], list):
        raise ValueError("immediate_protocol must be a list")
    parsed["immediate_protocol"] = [str(value)[:1500] for value in parsed["immediate_protocol"][:2]]
    while len(parsed["immediate_protocol"]) < 2:
        parsed["immediate_protocol"].append("Do not approach the source. Follow local emergency instructions and refer the incident to the competent authority.")
    parsed["analysis_origin"] = provider_tag
    parsed["measurements"] = {"PM2_5": None, "PM10": None, "NOx": None, "VOCs": None}
    return parsed


def _build_ai_prompt(record: Mapping[str, Any]) -> str:
    prompt = {
        "incident": {key: record.get(key) for key in ("jurisdiction", "zone", "source", "severity", "notes")},
        "task": (
            "Return only a JSON object with exactly these keys: particulate_profile (string), "
            "immediate_protocol (array of exactly 2 strings), long_term_engineering (string), statutory_memo (string). "
            "Give qualitative source-related context, safe actions, practical source-control engineering, and cautious "
            "jurisdiction-aware referral language. Never invent pollutant concentrations, AQI, monitoring results, "
            "coordinates, statutory findings, or official powers. State that legal applicability needs competent review. "
            "Do not tell volunteers to confront people or approach hazards. For India, mention Air Act Section 19(5) only "
            "with its conditional scope concerning notified air-pollution-control areas and restrictions on burning certain "
            "materials; never assert a violation. For other jurisdictions, do not invent a specific legal section. "
            "Do not output Markdown fences."
        ),
    }
    return json.dumps(prompt, ensure_ascii=False)


def _bedrock_analysis(record: Mapping[str, Any], client: Any) -> dict[str, Any]:
    prompt_text = _build_ai_prompt(record)
    body = {
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 900,
        "temperature": 0,
        "system": "You provide cautious environmental incident triage support. Output valid JSON only. This is not legal advice or a measurement system.",
        "messages": [{"role": "user", "content": [{"type": "text", "text": prompt_text}]}],
    }
    result = client.invoke_model(
        modelId=BEDROCK_MODEL_ID,
        body=json.dumps(body).encode("utf-8"),
        contentType="application/json",
        accept="application/json",
    )
    response_body = json.loads(result["body"].read())
    content = response_body.get("content", [])
    text_chunks = [part.get("text", "") for part in content if isinstance(part, dict) and isinstance(part.get("text"), str)]
    if not text_chunks:
        raise ValueError("Bedrock returned no text content")
    return _parse_model_json("\n".join(text_chunks), "amazon_bedrock")


def _gemini_analysis(record: Mapping[str, Any]) -> dict[str, Any]:
    """Invoke Google Gemini Interactions API securely using standard library urllib."""
    if not GEMINI_API_KEY:
        raise ValueError("GEMINI_API_KEY is not configured")
    url = "https://generativelanguage.googleapis.com/v1beta/interactions"
    prompt_text = _build_ai_prompt(record)
    payload = {
        "model": GEMINI_MODEL,
        "input": prompt_text,
    }
    req = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers={
            "Content-Type": "application/json",
            "x-goog-api-key": GEMINI_API_KEY,
            "Api-Revision": "2026-05-20",
        },
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=12) as resp:
        data = json.loads(resp.read().decode("utf-8"))

    raw_text = data.get("output_text") or ""
    if not raw_text:
        for candidate in data.get("candidates", []):
            parts = candidate.get("content", {}).get("parts", [])
            chunks = [p.get("text", "") for p in parts if isinstance(p, dict) and "text" in p]
            if chunks:
                raw_text = "\n".join(chunks)
                break
        if not raw_text:
            for step in data.get("steps", []):
                if step.get("type") == "model_output" and "text" in step:
                    raw_text = step["text"]
                    break
    if not raw_text:
        raise ValueError("Gemini API returned no output text")
    return _parse_model_json(raw_text, "google_gemini")


def _perform_ai_analysis(record: Mapping[str, Any], bedrock: Any) -> tuple[dict[str, Any], str, str]:
    """Route AI analysis to configured provider with deterministic fallback."""
    if AI_PROVIDER == "GEMINI":
        try:
            return _gemini_analysis(record), "google_gemini", "completed"
        except Exception as exc:
            LOGGER.warning("Gemini analysis failed: %s; falling back", type(exc).__name__)
            return _fallback_analysis(record), "deterministic_fallback", "fallback"

    if AI_PROVIDER == "BEDROCK":
        try:
            return _bedrock_analysis(record, bedrock), "amazon_bedrock", "completed"
        except Exception as exc:
            LOGGER.warning("Bedrock analysis failed: %s; falling back", type(exc).__name__)
            return _fallback_analysis(record), "deterministic_fallback", "fallback"

    if AI_PROVIDER == "FALLBACK":
        return _fallback_analysis(record), "deterministic_fallback", "fallback"

    if GEMINI_API_KEY:
        try:
            return _gemini_analysis(record), "google_gemini", "completed"
        except Exception as exc:
            LOGGER.info("Gemini attempt failed (%s); trying Bedrock", type(exc).__name__)

    try:
        return _bedrock_analysis(record, bedrock), "amazon_bedrock", "completed"
    except Exception as exc:
        LOGGER.info("Bedrock attempt failed (%s); using deterministic fallback", type(exc).__name__)
        return _fallback_analysis(record), "deterministic_fallback", "fallback"


def _validate_payload(event: Mapping[str, Any]) -> dict[str, Any]:
    body_raw = event.get("body")
    if event.get("isBase64Encoded"):
        raise ValueError("Base64 request bodies are not supported")
    if not isinstance(body_raw, str) or len(body_raw.encode("utf-8")) > 20_000:
        raise ValueError("Request body must be JSON under 20 KB")
    try:
        payload = json.loads(body_raw)
    except (json.JSONDecodeError, TypeError) as exc:
        raise ValueError("Request body must contain valid JSON") from exc
    if not isinstance(payload, dict):
        raise ValueError("JSON request body must be an object")
    jurisdiction = payload.get("jurisdiction")
    zone = payload.get("zone")
    source = payload.get("source")
    severity = payload.get("severity")
    notes = payload.get("notes", "")
    if jurisdiction not in ALLOWED_JURISDICTIONS:
        raise ValueError("jurisdiction must be one of: " + ", ".join(sorted(ALLOWED_JURISDICTIONS)))
    if not isinstance(zone, str) or not zone.strip() or len(zone.strip()) > 180:
        raise ValueError("zone must be a non-empty string of at most 180 characters")
    if source not in ALLOWED_SOURCES:
        raise ValueError("source is not an allowed contaminant source")
    if severity not in ALLOWED_SEVERITIES:
        raise ValueError("severity must be Moderate, High, or Severe")
    if not isinstance(notes, str) or len(notes) > 3000:
        raise ValueError("notes must be a string of at most 3000 characters")
    record: dict[str, Any] = {"jurisdiction": jurisdiction, "zone": zone.strip(), "source": source, "severity": severity, "notes": notes.strip()}
    for input_key, output_key, low, high in (("latitude", "latitude", -90.0, 90.0), ("longitude", "longitude", -180.0, 180.0)):
        value = payload.get(input_key)
        if value is not None:
            if isinstance(value, bool):
                raise ValueError(f"{input_key} must be a number")
            try:
                numeric = float(value)
            except (TypeError, ValueError) as exc:
                raise ValueError(f"{input_key} must be a number") from exc
            if numeric < low or numeric > high:
                raise ValueError(f"{input_key} is outside its valid range")
            record[output_key] = numeric
    if ("latitude" in record) != ("longitude" in record):
        raise ValueError("latitude and longitude must be provided together")
    return record


def _handle_get(table: Any) -> dict[str, Any]:
    try:
        result = table.scan(Limit=50)
        items = result.get("Items", [])
        items.sort(key=lambda item: int(item.get("timestamp", 0)), reverse=True)
        return response(200, {"incidents": items[:50], "count": min(len(items), 50), "truncated": bool(result.get("LastEvaluatedKey"))})
    except Exception as exc:
        LOGGER.exception("DynamoDB scan failed (%s)", type(exc).__name__)
        return response(503, {"error": "Incident registry is temporarily unavailable", "request_id": str(uuid.uuid4())})


def _handle_post(event: Mapping[str, Any], table: Any, bedrock: Any) -> dict[str, Any]:
    try:
        payload = _validate_payload(event)
    except ValueError as exc:
        return response(400, {"error": str(exc)})

    analysis, provider, status = _perform_ai_analysis(payload, bedrock)
    timestamp = int(time.time())
    record: dict[str, Any] = {
        "incident_id": str(uuid.uuid4()),
        "timestamp": timestamp,
        "iso_date": datetime.fromtimestamp(timestamp, tz=timezone.utc).isoformat(),
        "lifecycle_state": "Reported",
        **payload,
        "analysis": analysis,
        "analysis_provider": provider,
        "analysis_status": status,
    }
    try:
        table.put_item(Item=record, ConditionExpression="attribute_not_exists(incident_id)")
    except Exception as exc:
        LOGGER.exception("DynamoDB put_item failed (%s)", type(exc).__name__)
        return response(503, {"error": "Incident could not be persisted. Please retry.", "request_id": str(uuid.uuid4())})
    return response(201, {"record": record})


def lambda_handler(event: dict[str, Any], context: Any) -> dict[str, Any]:
    """Handle API Gateway HTTP API v2 events and fail closed with structured responses."""
    method = str((event.get("requestContext", {}).get("http", {}).get("method") or event.get("httpMethod") or "GET")).upper()
    if method == "OPTIONS":
        return {"statusCode": 200, "headers": dict(CORS_HEADERS), "body": ""}
    if method not in {"GET", "POST"}:
        return response(405, {"error": "Method not allowed"})
    try:
        table, bedrock = _clients()
    except Exception as exc:
        LOGGER.exception("AWS client initialization failed (%s)", type(exc).__name__)
        return response(503, {"error": "Backend services are temporarily unavailable", "request_id": str(uuid.uuid4())})
    if method == "GET":
        return _handle_get(table)
    return _handle_post(event, table, bedrock)

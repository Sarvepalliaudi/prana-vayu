"""Comprehensive unit and event tests for PRANA-VAYU lambda_function.py in services/api."""
from __future__ import annotations

import json
import os
import sys
import unittest
from unittest.mock import MagicMock, patch

# Ensure services/api is in sys.path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

# Gracefully mock boto3 / botocore if testing in an environment without boto3 pre-installed
if "boto3" not in sys.modules:
    try:
        import boto3  # noqa: F401
    except ImportError:
        mock_boto3 = MagicMock()
        mock_botocore = MagicMock()
        mock_botocore_config = MagicMock()
        mock_botocore_exceptions = MagicMock()

        class MockBotoCoreError(Exception):
            pass

        class MockClientError(Exception):
            pass

        mock_botocore_exceptions.BotoCoreError = MockBotoCoreError
        mock_botocore_exceptions.ClientError = MockClientError

        sys.modules["boto3"] = mock_boto3
        sys.modules["botocore"] = mock_botocore
        sys.modules["botocore.config"] = mock_botocore_config
        sys.modules["botocore.exceptions"] = mock_botocore_exceptions

import lambda_function


class TestPranaVayuLambda(unittest.TestCase):
    def setUp(self) -> None:
        self.mock_table = MagicMock()
        self.mock_bedrock = MagicMock()

    def test_options_request(self) -> None:
        """OPTIONS request must return 200 with explicit CORS headers."""
        event = {
            "httpMethod": "OPTIONS",
            "requestContext": {"http": {"method": "OPTIONS"}},
        }
        res = lambda_function.lambda_handler(event, None)
        self.assertEqual(res["statusCode"], 200)
        self.assertEqual(res["headers"]["Access-Control-Allow-Origin"], "*")
        self.assertIn("POST", res["headers"]["Access-Control-Allow-Methods"])
        self.assertEqual(res["body"], "")

    def test_method_not_allowed(self) -> None:
        """Unsupported HTTP methods must return 405 Method Not Allowed."""
        event = {
            "httpMethod": "DELETE",
            "requestContext": {"http": {"method": "DELETE"}},
        }
        res = lambda_function.lambda_handler(event, None)
        self.assertEqual(res["statusCode"], 405)
        body = json.loads(res["body"])
        self.assertIn("error", body)

    def test_validation_malformed_json(self) -> None:
        """Malformed JSON payloads must return 400 with a clear error."""
        event = {
            "httpMethod": "POST",
            "requestContext": {"http": {"method": "POST"}},
            "body": "{invalid-json",
        }
        with patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 400)
            body = json.loads(res["body"])
            self.assertIn("valid JSON", body["error"])

    def test_validation_invalid_jurisdiction(self) -> None:
        """Disallowed jurisdiction values must be rejected with 400."""
        payload = {
            "jurisdiction": "Mars",
            "zone": "Sector 4",
            "source": "Biomass burning",
            "severity": "Moderate",
        }
        event = {
            "httpMethod": "POST",
            "requestContext": {"http": {"method": "POST"}},
            "body": json.dumps(payload),
        }
        with patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 400)
            body = json.loads(res["body"])
            self.assertIn("jurisdiction must be one of", body["error"])

    def test_validation_coordinates_asymmetry(self) -> None:
        """Providing latitude without longitude must fail closed with 400."""
        payload = {
            "jurisdiction": "India",
            "zone": "North Campus, Delhi",
            "source": "Biomass burning",
            "severity": "Moderate",
            "latitude": 28.6924,
        }
        event = {
            "httpMethod": "POST",
            "requestContext": {"http": {"method": "POST"}},
            "body": json.dumps(payload),
        }
        with patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 400)
            body = json.loads(res["body"])
            self.assertIn("latitude and longitude must be provided together", body["error"])

    def test_validation_coordinates_out_of_range(self) -> None:
        """Out of bounds latitude values must fail closed with 400."""
        payload = {
            "jurisdiction": "India",
            "zone": "North Campus, Delhi",
            "source": "Biomass burning",
            "severity": "Moderate",
            "latitude": 125.0,
            "longitude": 77.21,
        }
        event = {
            "httpMethod": "POST",
            "requestContext": {"http": {"method": "POST"}},
            "body": json.dumps(payload),
        }
        with patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 400)
            body = json.loads(res["body"])
            self.assertIn("outside its valid range", body["error"])

    def test_post_success_with_deterministic_fallback(self) -> None:
        """POST must succeed and write to DynamoDB even when Bedrock/Gemini fail or are unconfigured."""
        payload = {
            "jurisdiction": "India",
            "zone": "Samayapuram, Trichy",
            "source": "Biomass burning",
            "severity": "Severe",
            "notes": "Grounds perimeter smoke observed near hostel gate.",
            "latitude": 10.92,
            "longitude": 78.74,
        }
        event = {
            "httpMethod": "POST",
            "requestContext": {"http": {"method": "POST"}},
            "body": json.dumps(payload),
        }

        self.mock_bedrock.invoke_model.side_effect = Exception("Model access not authorized")

        with patch.object(lambda_function, "AI_PROVIDER", "FALLBACK"), \
             patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 201)
            body = json.loads(res["body"])
            self.assertIn("record", body)
            record = body["record"]
            self.assertEqual(record["zone"], "Samayapuram, Trichy")
            self.assertEqual(record["jurisdiction"], "India")
            self.assertEqual(record["severity"], "Severe")
            self.assertIn("incident_id", record)
            self.assertIn("iso_date", record)
            self.assertEqual(record["analysis_provider"], "deterministic_fallback")
            self.assertEqual(record["analysis_status"], "fallback")

            analysis = record["analysis"]
            self.assertIn("particulate_profile", analysis)
            self.assertEqual(len(analysis["immediate_protocol"]), 2)
            self.assertIn("long_term_engineering", analysis)
            self.assertIn("statutory_memo", analysis)
            self.assertEqual(analysis["analysis_origin"], "deterministic_fallback")
            self.assertIsNone(analysis["measurements"]["PM2_5"])
            self.assertIsNone(analysis["measurements"]["PM10"])

            self.mock_table.put_item.assert_called_once()

    def test_gemini_interactions_api_success(self) -> None:
        """Gemini Interactions API returning output_text must parse correctly into schema."""
        payload = {
            "jurisdiction": "US",
            "zone": "San Jose, CA",
            "source": "Vehicle idling",
            "severity": "High",
        }
        event = {
            "httpMethod": "POST",
            "requestContext": {"http": {"method": "POST"}},
            "body": json.dumps(payload),
        }
        gemini_response = {
            "output_text": json.dumps({
                "particulate_profile": "Fleet diesel exhaust contains NO2 and elemental carbon.",
                "immediate_protocol": [
                    "Do not approach idling vehicles directly.",
                    "Notify campus fleet manager to enforce anti-idling policies.",
                ],
                "long_term_engineering": "Install electric hookups and designated staging zones away from air intakes.",
                "statutory_memo": "Consult California Air Resources Board (CARB) commercial vehicle idling regulations.",
            })
        }
        mock_http_resp = MagicMock()
        mock_http_resp.read.return_value = json.dumps(gemini_response).encode("utf-8")
        mock_http_resp.__enter__.return_value = mock_http_resp

        with patch.object(lambda_function, "AI_PROVIDER", "GEMINI"), \
             patch.object(lambda_function, "GEMINI_API_KEY", "test-key-mock"), \
             patch("urllib.request.urlopen", return_value=mock_http_resp), \
             patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 201)
            record = json.loads(res["body"])["record"]
            self.assertEqual(record["analysis_provider"], "google_gemini")
            self.assertEqual(record["analysis_status"], "completed")
            self.assertEqual(record["analysis"]["analysis_origin"], "google_gemini")

    def test_get_incidents(self) -> None:
        """GET request must scan table and return up to 50 incidents sorted by timestamp descending."""
        sample_items = [
            {"incident_id": "id-1", "timestamp": 1700000100, "zone": "Zone A"},
            {"incident_id": "id-2", "timestamp": 1700000200, "zone": "Zone B"},
        ]
        self.mock_table.scan.return_value = {"Items": sample_items}

        event = {
            "httpMethod": "GET",
            "requestContext": {"http": {"method": "GET"}},
        }
        with patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 200)
            body = json.loads(res["body"])
            self.assertEqual(body["count"], 2)
            self.assertEqual(body["incidents"][0]["incident_id"], "id-2")
            self.assertEqual(body["incidents"][1]["incident_id"], "id-1")

    def test_dynamodb_error_fails_closed(self) -> None:
        """DynamoDB scan error must return 503 without leaking stack traces or credentials."""
        self.mock_table.scan.side_effect = Exception("Internal DynamoDB connection drop")

        event = {
            "httpMethod": "GET",
            "requestContext": {"http": {"method": "GET"}},
        }
        with patch.object(lambda_function, "_clients", return_value=(self.mock_table, self.mock_bedrock)):
            res = lambda_function.lambda_handler(event, None)
            self.assertEqual(res["statusCode"], 503)
            body = json.loads(res["body"])
            self.assertIn("temporarily unavailable", body["error"])
            self.assertNotIn("Traceback", body["error"])


if __name__ == "__main__":
    unittest.main()

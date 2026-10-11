"""PRANA-VAYU HTTP Server Bridge for lambda_function.py.

Translates standard HTTP requests to AWS API Gateway v2 events,
allowing lambda_function.py to run seamlessly on Render, local development,
or containerized runtimes without requiring AWS deployment.

Developer: SARVEPALLI AUDI SIVA BHANUVARDHAN | Brand: AS PHENIX
"""
from __future__ import annotations

import json
import logging
import os
import sys
import urllib.parse
from http.server import BaseHTTPRequestHandler, HTTPServer
from typing import Any

import lambda_function

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")
LOGGER = logging.getLogger("prana-vayu-server")


class LambdaHttpProxyHandler(BaseHTTPRequestHandler):
    def do_OPTIONS(self) -> None:
        self._proxy_to_lambda("OPTIONS")

    def do_GET(self) -> None:
        self._proxy_to_lambda("GET")

    def do_POST(self) -> None:
        self._proxy_to_lambda("POST")

    def _proxy_to_lambda(self, method: str) -> None:
        parsed = urllib.parse.urlparse(self.path)
        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length).decode("utf-8") if content_length > 0 else None

        # Format as API Gateway HTTP API v2 event
        event: dict[str, Any] = {
            "version": "2.0",
            "routeKey": f"{method} {parsed.path}",
            "rawPath": parsed.path,
            "rawQueryString": parsed.query,
            "headers": dict(self.headers),
            "requestContext": {
                "http": {
                    "method": method,
                    "path": parsed.path,
                    "protocol": "HTTP/1.1",
                    "sourceIp": self.client_address[0] if self.client_address else "127.0.0.1",
                    "userAgent": self.headers.get("User-Agent", "PRANA-VAYU-HTTP"),
                }
            },
            "body": body,
            "isBase64Encoded": False,
        }

        try:
            res = lambda_function.lambda_handler(event, None)
        except Exception as exc:
            LOGGER.exception("Unhandled error in lambda_handler: %s", exc)
            res = {
                "statusCode": 500,
                "headers": {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"},
                "body": json.dumps({"error": "Internal server error"}),
            }

        status_code = int(res.get("statusCode", 200))
        headers = res.get("headers", {})
        body_bytes = str(res.get("body", "")).encode("utf-8")

        self.send_response(status_code)
        for key, val in headers.items():
            self.send_header(key, val)
        if "Content-Length" not in headers:
            self.send_header("Content-Length", str(len(body_bytes)))
        self.end_headers()
        self.wfile.write(body_bytes)

    def log_message(self, format: str, *args: Any) -> None:
        LOGGER.info("%s - - [%s] %s", self.client_address[0] if self.client_address else "localhost", self.log_date_time_string(), format % args)


def run_server() -> None:
    port = int(os.environ.get("PORT", 8080))
    host = os.environ.get("HOST", "0.0.0.0")
    server_address = (host, port)
    httpd = HTTPServer(server_address, LambdaHttpProxyHandler)
    LOGGER.info("==================================================================")
    LOGGER.info(" PRANA-VAYU (प्राण-वायु) — HTTP Server Bridge for Python Lambda")
    LOGGER.info(" Developer: SARVEPALLI AUDI SIVA BHANUVARDHAN | Brand: AS PHENIX")
    LOGGER.info(" Bound to http://%s:%d (Environment: Render / Local)", host, port)
    LOGGER.info(" Endpoints: GET /health, GET /incidents, POST /incidents, POST /incidents/transition")
    LOGGER.info("==================================================================")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        LOGGER.info("Server terminated by user.")
        httpd.server_close()


if __name__ == "__main__":
    run_server()

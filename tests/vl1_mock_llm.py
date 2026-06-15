#!/usr/bin/env python3
"""Minimal OpenAI-compatible mock server for OpenWand VL-1 workflow test.

Returns a simple, deterministic response. This exercises the full agent loop
(session creation, policy evaluation, tool execution, trace writing) without
requiring an external LLM provider.

The mock is intentionally simple — it returns one assistant turn with no tool
calls. This proves the agent loop runs end-to-end: HTTP request → SSE parsing →
response → trace write → session store update.
"""

import json
import http.server
import sys
import time
import uuid

class MockHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path == "/v1/chat/completions":
            content_length = int(self.headers["Content-Length"])
            body = json.loads(self.rfile.read(content_length))

            model = body.get("model", "mock-model")
            messages = body.get("messages", [])
            user_msg = ""
            for m in reversed(messages):
                if m.get("role") == "user":
                    user_msg = m.get("content", "")
                    break

            response = {
                "id": f"chatcmpl-{uuid.uuid4().hex[:12]}",
                "object": "chat.completion",
                "created": int(time.time()),
                "model": model,
                "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": f"Hello from OpenWand v1.0.0! I received your message: '{user_msg[:50]}'. This is a mock response from the VL-1 workflow test.",
                    },
                    "finish_reason": "stop",
                }],
                "usage": {
                    "prompt_tokens": len(str(user_msg)) // 4 + 10,
                    "completion_tokens": 30,
                    "total_tokens": len(str(user_msg)) // 4 + 40,
                },
            }

            # Check if streaming is requested
            stream = body.get("stream", False)
            if stream:
                self.send_response(200)
                self.send_header("Content-Type", "text/event-stream")
                self.send_header("Cache-Control", "no-cache")
                self.end_headers()

                # Send chunk
                chunk = {
                    "id": response["id"],
                    "object": "chat.completion.chunk",
                    "created": response["created"],
                    "model": model,
                    "choices": [{
                        "index": 0,
                        "delta": {
                            "role": "assistant",
                            "content": response["choices"][0]["message"]["content"],
                        },
                        "finish_reason": None,
                    }],
                }
                self.wfile.write(f"data: {json.dumps(chunk)}\n\n".encode())
                self.wfile.flush()

                # Final chunk
                final_chunk = {
                    "id": response["id"],
                    "object": "chat.completion.chunk",
                    "created": response["created"],
                    "model": model,
                    "choices": [{
                        "index": 0,
                        "delta": {},
                        "finish_reason": "stop",
                    }],
                }
                self.wfile.write(f"data: {json.dumps(final_chunk)}\n\n".encode())
                self.wfile.write(b"data: [DONE]\n\n")
                self.wfile.flush()
            else:
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(json.dumps(response).encode())
        else:
            self.send_response(404)
            self.end_headers()

    def do_GET(self):
        if self.path == "/v1/models":
            response = {
                "object": "list",
                "data": [{
                    "id": "mock-vl1",
                    "object": "model",
                    "created": int(time.time()),
                    "owned_by": "vl1-test",
                }],
            }
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(json.dumps(response).encode())
        else:
            self.send_response(404)
            self.end_headers()

    def log_message(self, format, *args):
        # Log to stderr so we can capture request flow
        print(f"[mock-llm] {args[0]}", file=sys.stderr, flush=True)


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 18888
    server = http.server.HTTPServer(("127.0.0.1", port), MockHandler)
    print(f"[mock-llm] Mock OpenAI-compatible server on port {port}", flush=True)
    server.serve_forever()

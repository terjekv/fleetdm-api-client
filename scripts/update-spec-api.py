#!/usr/bin/env python3
"""Refresh Fleet REST API markdown and generate route wrappers.

Usage:
  scripts/update-spec-api.py --check
  scripts/update-spec-api.py --fetch
  scripts/update-spec-api.py
"""

from __future__ import annotations

import argparse
import difflib
import re
import subprocess
import sys
import tempfile
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC_URL = "https://raw.githubusercontent.com/fleetdm/fleet/main/docs/REST%20API/rest-api.md"
SPEC_PATH = ROOT / "docs" / "fleet-rest-api-main.md"
RAW_API_PATH = ROOT / "src" / "endpoints" / "raw_api.rs"
ROUTE_RE = re.compile(r"^`(GET|POST|PATCH|PUT|DELETE) (/api[^` ?&]+)")


def normalize_markdown(markdown: str) -> str:
    """Keep the vendored spec stable and free of trailing whitespace."""
    return "\n".join(line.rstrip() for line in markdown.splitlines()) + "\n"


def route_shapes(markdown: str) -> list[tuple[str, str]]:
    routes: dict[tuple[str, str], None] = {}
    heading = ""
    for line in markdown.splitlines():
        if line.startswith("#"):
            heading = line.lstrip("#").strip().lower()
        match = ROUTE_RE.match(line)
        if not match:
            continue
        if heading.startswith("example"):
            continue
        method, path = match.groups()
        path = path.rstrip("/") or path
        routes[(method, path)] = None
    return sorted(routes)


def function_name(method: str, path: str, used: set[str]) -> str:
    parts = ["ep", method.lower()]
    for segment in path.strip("/").split("/"):
        if segment.startswith(":"):
            parts.extend(["by", segment[1:]])
        else:
            parts.append(segment)
    raw = re.sub(r"[^A-Za-z0-9_]", "_", "_".join(parts)).lower()
    raw = re.sub(r"_+", "_", raw).strip("_")
    name = raw
    suffix = 2
    while name in used:
        name = f"{raw}_{suffix}"
        suffix += 1
    used.add(name)
    return name


def route_params(path: str) -> list[str]:
    return re.findall(r":([A-Za-z0-9_]+)", path)


def render_raw_api(routes: list[tuple[str, str]]) -> str:
    lines: list[str] = [
        "use crate::client::FleetClient;",
        "use crate::error::{FleetError, Result};",
        "use crate::http::{",
        "    append_query, encode_path_segment, handle_response, read_limited_body, send_with_retry,",
        "    MAX_RAW_RESPONSE_SIZE,",
        "};",
        "use crate::models::{ApiQuery, ApiRequestBody, ApiResponse};",
        "",
        "/// Generated authenticated transport wrappers for routes in docs/fleet-rest-api-main.md.",
        "///",
        "/// Successful responses preserve status, headers, and bytes. Public authentication",
        "/// routes are exposed through `FleetClient::builder(...).auth()` instead. Curated",
        "/// endpoint modules remain the preferred API for supported workflows.",
        "pub struct RawApiEndpoint<'a> {",
        "    client: &'a FleetClient,",
        "}",
        "",
        "impl<'a> RawApiEndpoint<'a> {",
        "    pub(crate) fn new(client: &'a FleetClient) -> Self {",
        "        Self { client }",
        "    }",
        "",
        "    async fn send(",
        "        &self,",
        "        method: reqwest::Method,",
        "        path: &str,",
        "        query: Option<ApiQuery<'_>>,",
        "        body: Option<&ApiRequestBody>,",
        "    ) -> Result<ApiResponse> {",
        "        let path = append_query(path, query);",
        "        let send_once = || async {",
        "            let mut request = self.client.request(method.clone(), &path)?;",
        "            if let Some(b) = body {",
        "                request = b.apply(request)?;",
        "            }",
        "            request.send().await.map_err(Into::into)",
        "        };",
        "        let response = if let Some(policy) = self.client.retry_policy() {",
        "            send_with_retry(policy, send_once).await?",
        "        } else {",
        "            send_once().await?",
        "        };",
        "",
        "        if !response.status().is_success() {",
        "            let result: Result<serde_json::Value> = handle_response(response).await;",
        "            return match result {",
        "                Err(error) => Err(error),",
        "                Ok(_) => Err(FleetError::Http(",
        "                    \"error response unexpectedly decoded as success\".into(),",
        "                )),",
        "            };",
        "        }",
        "",
        "        let status = response.status();",
        "        let headers = response.headers().clone();",
        "        let bytes = read_limited_body(response, MAX_RAW_RESPONSE_SIZE).await?;",
        "        Ok(ApiResponse::from_parts(status, headers, bytes))",
        "    }",
    ]
    used: set[str] = set()
    for method, path in routes:
        render_route(lines, method, path, function_name(method, path, used))
    lines.append("}")
    return "\n".join(lines) + "\n"


def render_route(lines: list[str], method: str, path: str, name: str) -> None:
    params = route_params(path)
    has_body = method in {"POST", "PATCH", "PUT", "DELETE"}
    lines.append("")
    lines.append(f"    /// `{method} {path}`")
    lines.append(f"    pub async fn {name}(")
    lines.append("        &self,")
    for param in params:
        lines.append(f"        {param}: impl AsRef<str>,")
    lines.append("        query: Option<ApiQuery<'_>>,")
    if has_body:
        lines.append("        body: Option<&ApiRequestBody>,")
    lines.append("    ) -> Result<ApiResponse> {")
    send_body = "body" if has_body else "None"
    if params:
        lines.append(f"        let mut path = \"{path}\".to_string();")
        for param in params:
            lines.append(f"        let {param}_encoded = encode_path_segment({param}.as_ref());")
            lines.append(f'        path = path.replace(":{param}", &{param}_encoded);')
        lines.append(f"        self.send(reqwest::Method::{method}, &path, query, {send_body}).await")
    else:
        lines.append(f"        self.send(reqwest::Method::{method}, \"{path}\", query, {send_body})")
        lines.append("            .await")
    lines.append("    }")


def format_rust(content: str) -> str:
    with tempfile.NamedTemporaryFile("w+", suffix=".rs") as tmp:
        tmp.write(content)
        tmp.flush()
        subprocess.run(["rustfmt", "--edition", "2024", tmp.name], check=True)
        tmp.seek(0)
        return tmp.read()


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if path.suffix == ".rs":
        content = format_rust(content)
    if check:
        old = path.read_text() if path.exists() else ""
        if old != content:
            print(f"{path.relative_to(ROOT)} is out of date", file=sys.stderr)
            for line in difflib.unified_diff(old.splitlines(), content.splitlines(), fromfile=str(path), tofile=f"{path} (generated)", lineterm=""):
                print(line, file=sys.stderr)
            return False
        return True
    path.write_text(content)
    return True


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fetch", action="store_true", help="Fetch upstream Fleet REST API markdown before generating")
    parser.add_argument("--check", action="store_true", help="Fail if generated files are stale")
    args = parser.parse_args()

    if args.fetch:
        with urllib.request.urlopen(SPEC_URL) as response:
            data = normalize_markdown(response.read().decode("utf-8"))
        if args.check:
            current = SPEC_PATH.read_text()
            if current != data:
                print("docs/fleet-rest-api-main.md is out of date", file=sys.stderr)
                return 1
        else:
            SPEC_PATH.write_text(data)

    spec = normalize_markdown(SPEC_PATH.read_text())
    routes = route_shapes(spec)
    ok = True
    ok &= write_or_check(RAW_API_PATH, render_raw_api(routes), args.check)
    if args.check and ok:
        print(f"spec route wrappers are current ({len(routes)} routes)")
    elif not args.check:
        print(f"generated {len(routes)} routes")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())

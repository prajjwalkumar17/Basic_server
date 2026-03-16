#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASE_URL="http://127.0.0.1:8080"
SERVER_LOG="${ROOT_DIR}/.tmp_test_server.log"
SERVER_PID=""

cleanup() {
  if [[ -n "$SERVER_PID" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

run_case() {
  local path="$1"
  local expected_status="$2"
  local expected_body="$3"

  local body_file
  body_file="$(mktemp)"

  local status
  if ! status=$(curl -sS -o "$body_file" -w "%{http_code}" "$BASE_URL$path" 2>/dev/null); then
    echo "FAILED: $path (could not connect)"
    rm -f "$body_file"
    return 1
  fi

  if [[ "$status" != "$expected_status" ]]; then
    echo "FAILED: $path expected status $expected_status, got $status"
    rm -f "$body_file"
    return 1
  fi

  if [[ -n "$expected_body" ]] && ! grep -qF "$expected_body" "$body_file"; then
    echo "FAILED: $path expected response body to include '$expected_body'"
    rm -f "$body_file"
    return 1
  fi

  echo "OK: $path -> HTTP $status"
  rm -f "$body_file"
}

echo "Starting server for curl checks on $BASE_URL"
(
  cd "$ROOT_DIR"
  cargo run >"$SERVER_LOG" 2>&1 &
  SERVER_PID=$!
)

for _ in $(seq 1 20); do
  if curl -sS "$BASE_URL/" >/dev/null 2>&1; then
    break
  fi
  sleep 0.5
 done

run_case "/" 200 "this is index"
run_case "/hello" 200 "hellow there!!!"
run_case "/style.css" 200 "h1"
run_case "/does-not-exist" 404 ""

echo "All curl checks passed"

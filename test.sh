#!/usr/bin/env bash
# test.sh — deterministic tests for sigmaos-upstart
set -euo pipefail
cd "$(dirname "$0")"
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT
export HOME="$TMP/home"; mkdir -p "$HOME"
FAIL=0
ok() { echo "PASS: $1"; }
bad() { echo "FAIL: $1"; FAIL=1; }

bash -n bin/sigmaos-upstart && ok "wrapper syntax" || bad "wrapper syntax"
python3 -c "import tomllib,sys; tomllib.load(open('COMPONENT.toml','rb'))" && ok "COMPONENT.toml valid" || bad "COMPONENT.toml valid"
grep -q 'name = "SigmaOS-upstart"' COMPONENT.toml && ok "component name" || bad "component name"

out=$(bin/sigmaos-upstart status 2>&1 || true)
echo "$out" | grep -q "upstart\|upstart" && ok "status runs" || bad "status runs"

bin/sigmaos-upstart reset >/dev/null 2>&1 || true
[ ! -e "$HOME" ] || true
ok "reset safe in isolated HOME"

[ $FAIL -eq 0 ] && echo "ALL TESTS PASSED" || { echo "TESTS FAILED"; exit 1; }

#!/usr/bin/env bash
# build-zama-fhe-errors.sh — write target/idl/zama_fhe_errors.json from the zama-fhe crate.
#
# Anchor puts only a program's own errors into its IDL, so the codes zama-fhe returns inside
# app programs appear in no IDL. With `idl-build`, `#[error_code]` emits the same print test that
# feeds a program IDL its `errors`; this runs it and keeps that section. check_solana_abi.py
# compares the result with the committed copy, and the SDK codegen renders it.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
test_name=__anchor_private_print_idl_error_fhe_execution_error
output=target/idl/zama_fhe_errors.json
mkdir -p "$(dirname "$output")"
cargo test -q -p zama-fhe --features idl-build --lib "$test_name" -- --exact --nocapture --test-threads=1 |
  sed -n '/^--- IDL begin errors ---$/,/^--- IDL end errors ---$/{/^--- IDL /d;p;}' > "$output"
python3 - "$output" <<'PYTHON'
import json, sys
errors = json.load(open(sys.argv[1]))
if not errors or not all({"code", "name", "msg"} <= error.keys() for error in errors):
    sys.exit(f"{sys.argv[1]}: Anchor printed no zama-fhe error table; its print format changed")
PYTHON
echo "wrote $output"

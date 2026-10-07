#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Demo clients describe the deployed feature set; public host/token clients keep the default IDLs.
for program in confidential_batcher demo_vault; do
  features=$(python3 -c 'import json, sys; print(",".join(json.load(open(sys.argv[1]))["features"][sys.argv[2]]))' environments/preview-env.json "$program")
  args=(idl build -p "$program" -o "target/idl/$program.json")
  if [[ -n "$features" ]]; then args+=(-- --features "$features"); fi
  NO_DNA=1 anchor "${args[@]}"
done

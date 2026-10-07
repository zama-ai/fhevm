#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Demo clients describe the deployed feature set; public host/token clients keep the default IDLs.
programs=(confidential_batcher demo_vault)
if [[ "${1:-}" == '--preview-cleanup' ]]; then programs+=(confidential_token); fi
for program in "${programs[@]}"; do
  features=$(python3 -c 'import json, sys; print(",".join(json.load(open(sys.argv[1]))["features"][sys.argv[2]]))' environments/preview-env.json "$program")
  output="target/idl/$program.json"
  if [[ "$program" == confidential_token ]]; then output="target/idl/confidential_token_admin_sweep.json"; fi
  args=(idl build -p "$program" -o "$output")
  if [[ -n "$features" ]]; then args+=(-- --features "$features"); fi
  NO_DNA=1 anchor "${args[@]}"
done

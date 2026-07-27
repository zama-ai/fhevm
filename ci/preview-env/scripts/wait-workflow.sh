#!/usr/bin/env bash
# Poll an Argo Workflow until terminal (Succeeded/Failed/Error) or a 2h timeout.
# Progress -> stderr; final phase (empty on timeout) -> stdout so callers capture it.
# Usage: wait-workflow.sh <workflow-name>   (NAMESPACE from env)
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"
wf="${1:?workflow name required}"

max_wait=7200 elapsed=0 interval=30 phase=""
while [[ $elapsed -lt $max_wait ]]; do
  phase=$(kubectl get workflow "${wf}" -n "${NAMESPACE}" -o jsonpath='{.status.phase}' 2>/dev/null || echo "")
  case "${phase}" in
    Succeeded|Failed|Error) break ;;
    *) echo "[${wf}] phase='${phase:-<none>}' (${elapsed}s/${max_wait}s)" >&2; sleep "${interval}"; elapsed=$((elapsed + interval)) ;;
  esac
done
echo "[${wf}] final phase: ${phase:-<timed out>}" >&2
printf '%s' "${phase}"

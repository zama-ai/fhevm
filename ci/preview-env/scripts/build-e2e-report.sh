#!/usr/bin/env bash
# Read the terminal status of the two per-SDK e2e Argo Workflows and emit a
# combined markdown report (per-test pass/fail + resolved versions) to
# $GITHUB_OUTPUT (report_path) and $GITHUB_STEP_SUMMARY.
# Env: NAMESPACE, TEST_SUITE_TAG, HOST_CONTRACTS_TAG, GATEWAY_CONTRACTS_TAG,
# COPROCESSOR_TAG, RELAYER_TAG, KMS_CONNECTOR_TAG, KMS_CORE_TAG, NB_KMS_CORE.
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

# The relayer-sdk suite is only deployed when RELAYER_SDK_VERSION is set; skip
# it here too, otherwise wait-workflow.sh would block for the full timeout on a
# workflow that never existed.
wfs=(e2e-test-suite-fhevm-sdk)
if [[ -n "${RELAYER_SDK_VERSION:-}" ]]; then
  wfs+=(e2e-test-suite-relayer-sdk)
fi
# The Polygon suite is only deployed when deploy_polygon is on; skip it here too
# so wait-workflow.sh doesn't block on a workflow that never existed.
if [[ "${DEPLOY_POLYGON:-false}" == "true" ]]; then
  wfs+=(e2e-test-suite-polygon)
fi
max_wait=7200
sdk_label() {
  case "$1" in
    e2e-test-suite-fhevm-sdk) echo "@fhevm/sdk (RELAYER_SDK_VERSION=\"\")" ;;
    e2e-test-suite-relayer-sdk) echo "@zama-fhe/relayer-sdk" ;;
    e2e-test-suite-polygon) echo "@fhevm/sdk (Polygon host chain)" ;;
    *) echo "$1" ;;
  esac
}

sections=$(mktemp --suffix=.md)
overall_failed=0
for wf in "${wfs[@]}"; do
  # continueOn.failed keeps failed tests from failing the DAG, so the phase is
  # ~always Succeeded; per-test pass/fail comes from the node statuses below.
  phase=$(bash "${script_dir}/wait-workflow.sh" "${wf}")

  # A failed fetch (RBAC, missing workflow, apiserver hiccup) must NOT be
  # silently reported as "all 0 passed" - track it so it maps to a warning below.
  if wf_json=$(kubectl get workflow "${wf}" -n "${NAMESPACE}" -o json 2>/dev/null); then
    fetch_ok=1
  else
    wf_json='{}'
    fetch_ok=0
  fi

  # Per-test phase table from the run-test pod nodes.
  rows=$(jq -r '
    (.status.nodes // {}) | to_entries
    | map(.value | select(.templateName == "run-test"))
    | sort_by(.displayName)
    | .[] | "| \(.displayName) | \(.phase) |"' <<<"${wf_json}")
  failed=$(jq -r '
    [(.status.nodes // {}) | to_entries[] | .value
     | select(.templateName == "run-test")
     | select(.phase != "Succeeded")] | length' <<<"${wf_json}")
  total=$(jq -r '
    [(.status.nodes // {}) | to_entries[] | .value
     | select(.templateName == "run-test")] | length' <<<"${wf_json}")
  if [[ -z "${rows}" ]]; then rows="| _(no test nodes found)_ | ${phase:-Unknown} |"; fi

  if [[ "${phase}" != "Succeeded" && "${phase}" != "Failed" && "${phase}" != "Error" ]]; then
    sub="### :warning: $(sdk_label "${wf}") - timed out after ${max_wait}s"
    overall_failed=$((overall_failed + 1))
  elif [[ "${fetch_ok}" -eq 0 || "${total}" -eq 0 ]]; then
    # Terminal phase but no run-test nodes: either the status fetch failed or the
    # workflow never produced test pods. Either way we can't claim a pass.
    sub="### :warning: $(sdk_label "${wf}") - could not retrieve results (0 test nodes)"
    overall_failed=$((overall_failed + 1))
  elif [[ "${failed}" -gt 0 ]]; then
    sub="### :x: $(sdk_label "${wf}") - ${failed}/${total} failed"
    overall_failed=$((overall_failed + failed))
  else
    sub="### :white_check_mark: $(sdk_label "${wf}") - all ${total} passed"
  fi

  {
    echo "${sub}"
    echo
    echo "Workflow \`${wf}\`."
    echo
    echo "| Test | Result |"
    echo "| --- | --- |"
    echo "${rows}"
    echo
  } >> "${sections}"
done

if [[ "${overall_failed}" -gt 0 ]]; then
  header="## :x: Automated e2e tests (SDK matrix): ${overall_failed} failure(s)"
else
  header="## :white_check_mark: Automated e2e tests (SDK matrix): all passed"
fi

report=$(mktemp --suffix=.md)
{
  echo "${header}"
  echo
  echo "Namespace \`${NAMESPACE}\` on zws-dev."
  echo
  cat "${sections}"
  echo "<details><summary>Resolved component versions</summary>"
  echo
  echo "| Component | Version |"
  echo "| --- | --- |"
  echo "| test-suite | \`${TEST_SUITE_TAG}\` |"
  echo "| host-contracts | \`${HOST_CONTRACTS_TAG}\` |"
  echo "| gateway-contracts | \`${GATEWAY_CONTRACTS_TAG}\` |"
  echo "| coprocessor | \`${COPROCESSOR_TAG}\` |"
  echo "| relayer | \`${RELAYER_TAG}\` |"
  echo "| kms-connector | \`${KMS_CONNECTOR_TAG}\` |"
  echo "| kms-core | \`${KMS_CORE_TAG}\` (parties: ${NB_KMS_CORE}) |"
  echo "</details>"
} > "${report}"

echo "report_path=${report}" >> "$GITHUB_OUTPUT"
cat "${report}" >> "$GITHUB_STEP_SUMMARY"

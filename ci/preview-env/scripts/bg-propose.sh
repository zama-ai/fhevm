#!/usr/bin/env bash
# Blue/Green QA step 6: propose the coprocessor upgrade from a laptop.
#
#   bg-propose.sh calldata       read-only: run task:buildProposeCoprocessorUpgradeCalldata and print
#                                the per-chain windows, skew, drift and buffer report (no broadcast)
#   bg-propose.sh send           broadcast task:proposeCoprocessorUpgrade with the ACL owner key and
#                                wait for DryRunStarted on every host chain
#   bg-propose.sh wait-cutover   no proposal: wait until versioning reaches the Green release
#
# Thin wrapper around propose-coprocessor-upgrade.sh, which CI feeds from $GITHUB_ENV. Everything it
# needs is derived from the namespace: RPC URLs from the `rpc` Secret, the gateway URL from a
# coprocessor deployment, chain ids from host_chains, the ACL owner key (index 9) from
# preview-wallets-mnemonic, and the hardhat image from the deployed host-contracts release.
#
# Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg-propose.sh calldata|send|wait-cutover
# Env: NAMESPACE (required); PROPOSAL_ID (default: unix seconds - unique and above any previous id);
#      GCS_VERSION (default: v + the gcs overlay's stackVersion); START_LEAD_SECS (300);
#      WINDOW_DURATION (5h); BUFFER (0); TIMEOUT_SECS (900, DryRunStarted / cutover wait).
set -euo pipefail

verb="${1:?usage: bg-propose.sh calldata|send|wait-cutover}"
: "${NAMESPACE:?}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
fail() { echo "::error::$*" >&2; exit 1; }

secret_val() { kubectl get secret -n "${NAMESPACE}" "$1" -o jsonpath="{.data.$2}" 2>/dev/null | base64 -d; }
psql1() { kubectl exec -n "${NAMESPACE}" postgres-coprocessor-1-0 -- env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "$1"; }

host_http=$(secret_val rpc ethereum-rpc-url);  [[ -n "${host_http}" ]] || fail "the rpc Secret has no ethereum-rpc-url"
polygon_http=$(secret_val rpc polygon-rpc-url)
# The coprocessor talks to the gateway over ws://<host>:8548; the tool needs the http port on the same host.
gateway_ws=$(kubectl get deploy -n "${NAMESPACE}" coprocessor-1-gw-listener \
  -o jsonpath='{.spec.template.spec.containers[0].env[?(@.name=="GATEWAY_URL")].value}')
[[ -n "${gateway_ws}" ]] || fail "could not read GATEWAY_URL from coprocessor-1-gw-listener"
gateway_http="${GATEWAY_HTTP:-$(sed -E 's#^ws://#http://#; s#:8548$#:8547#' <<<"${gateway_ws}")}"

chains=$(psql1 "SELECT chain_id FROM host_chains ORDER BY chain_id;" | tr '\n' ' ')
grep -qE '(^| )11155111( |$)' <<<"${chains}" || fail "host_chains has no 11155111 (chains: ${chains})"
deploy_polygon=false
grep -qE '(^| )80002( |$)' <<<"${chains}" && deploy_polygon=true
[[ "${deploy_polygon}" != "true" || -n "${polygon_http}" ]] || fail "host_chains has 80002 but the rpc Secret has no polygon-rpc-url"

mnemonic=$(secret_val preview-wallets-mnemonic mnemonic); [[ -n "${mnemonic}" ]] || fail "preview-wallets-mnemonic not found"
owner_key=$(cast wallet private-key --mnemonic "${mnemonic}" --mnemonic-index 9)
# The hardhat task lives in the contracts that are on chain now, not the ones first deployed: on
# the production path the deploy is pinned to the previous release and has no propose task.
host_image=$(helm get values host-contracts -n "${NAMESPACE}" -o json | jq -r '"\(.scDeploy.image.name):\(.scDeploy.image.tag)"')
[[ "${host_image}" == *:* && "${host_image}" != *null* ]] || fail "could not resolve the host-contracts image"
live_tag=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.contracts\.version}' 2>/dev/null || true)
[[ -z "${live_tag}" ]] || host_image="${host_image%:*}:${live_tag}"
nb=$(helm list -n "${NAMESPACE}" -o json | jq '[.[] | select(.name | test("^coprocessor-[0-9]+$"))] | length')
gcs_version="${GCS_VERSION:-v$(yq -r '.commonConfig.stackVersion' "${root}/ci/preview-env/coprocessor/values-coprocessor-gcs-e2e.yaml")}"
# Unique and monotonic: the contract does not enforce uniqueness but a reused id is ignored.
proposal_id="${PROPOSAL_ID:-$(date -u +%s)}"

common=(
  NAMESPACE="${NAMESPACE}" NB_COPROCESSOR="${nb}" CHAIN_MODE=testnets EXTERNAL_CHAINS=true
  HOST_HTTP="${host_http}" GATEWAY_HTTP="${gateway_http}" HOST_CHAIN_ID=11155111
  DEPLOY_POLYGON="${deploy_polygon}" POLYGON_HTTP="${polygon_http}" POLYGON_CHAIN_ID=80002
  DEPLOYER_KEY_9="${owner_key}" HOST_CONTRACTS_IMAGE="${host_image}"
  PROPOSAL_ID="${proposal_id}" GCS_VERSION="${gcs_version}"
  START_LEAD_SECS="${START_LEAD_SECS:-300}" WINDOW_DURATION="${WINDOW_DURATION:-5h}"
  BUFFER="${BUFFER:-0}" TIMEOUT_SECS="${TIMEOUT_SECS:-900}"
)
echo "== bg-propose ${verb}: ${NAMESPACE}, ${nb} operators, chains ${chains}, proposal ${proposal_id}, version ${gcs_version}"
echo "   host ${host_http%%\?*} | polygon ${polygon_http%%\?*} | gateway ${gateway_http} | tool ${host_image##*/}"

case "${verb}" in
  calldata)     env "${common[@]}" PROPOSE_DRY_RUN=true  bash "${root}/ci/preview-env/scripts/propose-coprocessor-upgrade.sh" ;;
  send)         env "${common[@]}" PROPOSE_DRY_RUN=false bash "${root}/ci/preview-env/scripts/propose-coprocessor-upgrade.sh" ;;
  wait-cutover) env "${common[@]}" SKIP_PROPOSE=true ASSERT_CUTOVER=true bash "${root}/ci/preview-env/scripts/propose-coprocessor-upgrade.sh" ;;
  *) fail "unknown verb '${verb}' (calldata|send|wait-cutover)" ;;
esac

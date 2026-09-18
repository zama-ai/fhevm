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
#      WINDOW_DURATION (5h); BUFFER (0); TIMEOUT_SECS (900, DryRunStarted / cutover wait);
#      PROPOSE_EXTRA_ARGS (appended to the hardhat task, same as trailing CLI args).
set -euo pipefail

verb="${1:?usage: bg-propose.sh calldata|send|wait-cutover [-- <extra hardhat args>]}"
shift
# Anything after the verb goes to the hardhat task verbatim, so a round can test a parameter this
# wrapper does not model (PROPOSE_EXTRA_ARGS does the same from the environment).
extra_args="${PROPOSE_EXTRA_ARGS:-}"
[[ "${1:-}" == "--" ]] && shift
[[ $# -gt 0 ]] && extra_args="${extra_args:+${extra_args} }$*"
: "${NAMESPACE:?}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
fail() { echo "::error::$*" >&2; exit 1; }

secret_val() { kubectl get secret -n "${NAMESPACE}" "$1" -o jsonpath="{.data.$2}" 2>/dev/null | base64 -d 2>/dev/null || true; }
psql1() { kubectl exec -n "${NAMESPACE}" postgres-coprocessor-1-0 -- env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "$1"; }

# The rpc Secret is the testnets source of truth, so it stays first and testnets behaviour is
# unchanged. Off testnets there is no such Secret, and the idle test-suite Job carries the host
# chain's RPC in every mode.
job_env() {
  kubectl get job -n "${NAMESPACE}" test-suite -o jsonpath="{.spec.template.spec.containers[0].env[?(@.name=='$1')].value}" 2>/dev/null
}
host_http="${HOST_HTTP:-$(secret_val rpc ethereum-rpc-url)}"
[[ -n "${host_http}" ]] || host_http=$(job_env RPC_URL)
[[ -n "${host_http}" ]] || fail "could not resolve the host RPC (no rpc Secret and no test-suite Job RPC_URL)"
polygon_http=$(secret_val rpc polygon-rpc-url)
# The coprocessor talks to the gateway over ws://<host>:8548; the tool needs the http port on the same host.
# Any fleet's gw-listener carries the same URL, so take the first one: which slot is live moves
# between rounds, and on a second round "coprocessor-1" may not exist at all.
gw_deploy=$(kubectl get deploy -n "${NAMESPACE}" -o name 2>/dev/null | grep -E '/coprocessor-[0-9]+(-[a-z0-9]+)?-gw-listener$' | head -1)
[[ -n "${gw_deploy}" ]] || fail "no coprocessor gw-listener deployment found in ${NAMESPACE}"
gateway_ws=$(kubectl get -n "${NAMESPACE}" "${gw_deploy}" \
  -o jsonpath='{.spec.template.spec.containers[0].env[?(@.name=="GATEWAY_URL")].value}')
[[ -n "${gateway_ws}" ]] || fail "could not read GATEWAY_URL from ${gw_deploy}"
gateway_http="${GATEWAY_HTTP:-$(sed -E 's#^ws://#http://#; s#:8548$#:8547#' <<<"${gateway_ws}")}"

chains=$(psql1 "SELECT chain_id FROM host_chains ORDER BY chain_id;" | tr '\n' ' ')
deploy_polygon=false
grep -qE '(^| )80002( |$)' <<<"${chains}" && deploy_polygon=true
[[ "${deploy_polygon}" != "true" || -n "${polygon_http}" ]] || fail "host_chains has 80002 but the rpc Secret has no polygon-rpc-url"
# The host chain is the one that is not Polygon; its id decides the chain mode the tool runs in.
host_chain_id="${HOST_CHAIN_ID:-$(tr ' ' '\n' <<<"${chains}" | grep -E '^[0-9]+$' | grep -vx 80002 | head -1)}"
[[ -n "${host_chain_id}" ]] || fail "could not determine the host chain from host_chains (chains: ${chains})"
case "${host_chain_id}" in
  12345) chain_mode=anvil; external_chains=false ;;
  1337) chain_mode=blockchain-dev; external_chains=true ;;
  11155111) chain_mode=testnets; external_chains=true ;;
  *) fail "chain ${host_chain_id}: unknown host chain, cannot pick a chain mode" ;;
esac

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
  NAMESPACE="${NAMESPACE}" NB_COPROCESSOR="${nb}" CHAIN_MODE="${chain_mode}" EXTERNAL_CHAINS="${external_chains}"
  HOST_HTTP="${host_http}" GATEWAY_HTTP="${gateway_http}" HOST_CHAIN_ID="${host_chain_id}"
  DEPLOY_POLYGON="${deploy_polygon}" POLYGON_HTTP="${polygon_http}" POLYGON_CHAIN_ID=80002
  DEPLOYER_KEY_9="${owner_key}" HOST_CONTRACTS_IMAGE="${host_image}"
  PROPOSAL_ID="${proposal_id}" GCS_VERSION="${gcs_version}"
  START_LEAD_SECS="${START_LEAD_SECS:-300}" WINDOW_DURATION="${WINDOW_DURATION:-5h}"
  BUFFER="${BUFFER:-0}" TIMEOUT_SECS="${TIMEOUT_SECS:-900}"
  PROPOSE_EXTRA_ARGS="${extra_args}"
)
echo "== bg-propose ${verb}: ${NAMESPACE}, ${nb} operators, ${chain_mode} chains ${chains}, proposal ${proposal_id}, version ${gcs_version}"
echo "   host ${host_http%%\?*} | polygon ${polygon_http%%\?*} | gateway ${gateway_http} | tool ${host_image##*/}"

case "${verb}" in
  calldata)     env "${common[@]}" PROPOSE_DRY_RUN=true  bash "${root}/ci/preview-env/scripts/propose-coprocessor-upgrade.sh" ;;
  send)         env "${common[@]}" PROPOSE_DRY_RUN=false bash "${root}/ci/preview-env/scripts/propose-coprocessor-upgrade.sh" ;;
  wait-cutover) env "${common[@]}" SKIP_PROPOSE=true ASSERT_CUTOVER=true bash "${root}/ci/preview-env/scripts/propose-coprocessor-upgrade.sh" ;;
  *) fail "unknown verb '${verb}' (calldata|send|wait-cutover)" ;;
esac

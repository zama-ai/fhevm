#!/usr/bin/env bash
# Blue/Green QA: propose an upgrade with block numbers given literally, bypassing the window
# arithmetic in bg-propose.sh.
#
#   bg-propose-raw.sh --window <chainId>:<startBlock>:<endBlock> [--window ...] --gw-start <block>
#
# bg-propose.sh derives the windows from wall-clock time and refuses anything its own guards
# dislike: a window already past, or one shorter than a block. Those are exactly the cases the
# coprocessor has to survive, so this sends the call as written and lets the contract and the
# listener be the only validators.
#
# Signs locally and broadcasts through cluster-rpc.sh, because the host RPC is a ClusterIP.
#
# One --window per host chain: ingest.rs rejects a proposal whose chain set is not exactly equal
# to host_chains, so a two-chain preview needs both.
#
# Usage:
#   NAMESPACE=<ns> bash ci/preview-env/scripts/bg/bg-propose-raw.sh \
#     --window 12345:3840000:3840500 --window 80002:3840000:3840500 --gw-start 22900
# Env: NAMESPACE (required), PROPOSAL_ID (unix seconds), GCS_VERSION (gcs overlay stackVersion),
#      DRY_RUN (false: print the calldata and stop)
set -euo pipefail

: "${NAMESPACE:?}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
fail() { echo "::error::$*" >&2; exit 1; }

windows=()
gw_start=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --window) windows+=("${2:?--window needs <chainId>:<start>:<end>}"); shift 2 ;;
    --gw-start) gw_start="${2:?--gw-start needs a block}"; shift 2 ;;
    *) fail "unknown flag $1" ;;
  esac
done
[[ ${#windows[@]} -gt 0 ]] || fail "at least one --window is required"
[[ -n "${gw_start}" ]] || fail "--gw-start is required"

PROPOSAL_ID="${PROPOSAL_ID:-$(date +%s)}"
GCS_VERSION="${GCS_VERSION:-v$(yq -r '.commonConfig.stackVersion' \
  "${root}/ci/preview-env/coprocessor/values-coprocessor-gcs-e2e.yaml")}"
DRY_RUN="${DRY_RUN:-false}"

secret_val() { kubectl get secret -n "${NAMESPACE}" "$1" -o jsonpath="{.data.$2}" 2>/dev/null | base64 -d 2>/dev/null || true; }
rpc() { bash "${root}/ci/preview-env/scripts/deploy/cluster-rpc.sh" "${host_http}" "$1"; }
rpc_result() { rpc "$1" | sed 's/.*"result":"\([^"]*\)".*/\1/'; }

protocol_config=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" \
  -o jsonpath='{.data.protocol_config\.address}')
[[ -n "${protocol_config}" ]] || fail "host-sc-addresses has no protocol_config.address"

host_http=$(secret_val rpc ethereum-rpc-url)
[[ -n "${host_http}" ]] || host_http=$(kubectl get job -n "${NAMESPACE}" test-suite \
  -o jsonpath="{.spec.template.spec.containers[0].env[?(@.name=='RPC_URL')].value}" 2>/dev/null)
[[ -n "${host_http}" ]] || fail "could not resolve the host RPC"

# preview-wallets-mnemonic is generated only for external chains (EXTERNAL_CHAINS=true);
# an Anvil preview is seeded with the well-known test mnemonic, whose index 9 is the ACL owner.
mnemonic=$(secret_val preview-wallets-mnemonic mnemonic)
mnemonic="${mnemonic:-test test test test test test test test test test test junk}"
owner_key=$(cast wallet private-key --mnemonic "${mnemonic}" --mnemonic-index 9)
owner_addr=$(cast wallet address --private-key "${owner_key}")

# (uint64 chainId, uint64 startBlock, uint64 endBlock)[] as a cast tuple-array literal.
tuples=""
for w in "${windows[@]}"; do
  IFS=: read -r cid sb eb <<<"${w}"
  [[ -n "${cid}" && -n "${sb}" && -n "${eb}" ]] || fail "bad --window '${w}', want <chainId>:<start>:<end>"
  tuples+="${tuples:+,}(${cid},${sb},${eb})"
done

calldata=$(cast calldata \
  "proposeCoprocessorUpgrade(uint256,string,(uint64,uint64,uint64)[],uint64)" \
  "${PROPOSAL_ID}" "${GCS_VERSION}" "[${tuples}]" "${gw_start}")

echo "== propose-raw id=${PROPOSAL_ID} version=${GCS_VERSION} windows=[${tuples}] gw_start=${gw_start}"
echo "   target=${protocol_config} from=${owner_addr}"
if [[ "${DRY_RUN}" == "true" ]]; then
  echo "${calldata}"
  exit 0
fi

chain_id=$(rpc_result '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}')
nonce=$(rpc_result "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_getTransactionCount\",\"params\":[\"${owner_addr}\",\"pending\"]}")
# NONCE=<n> re-sends at a stuck nonce to replace that transaction instead of queueing behind it.
[[ -z "${NONCE:-}" ]] || nonce=$(printf '0x%x' "${NONCE}")
gas_price=$(rpc_result '{"jsonrpc":"2.0","id":1,"method":"eth_gasPrice","params":[]}')
# cast mktx builds EIP-1559: --gas-price is the max fee and the tip defaults to 1 wei, too low to
# mine once the base fee rises. Triple it and set a real tip; PRIORITY_FEE_WEI outbids a stuck tx.
gas_price=$(printf '0x%x' $(( $((gas_price)) * 3 )))
priority_fee="${PRIORITY_FEE_WEI:-1500000000}"
[[ -n "${chain_id}" && -n "${nonce}" && -n "${gas_price}" ]] || fail "could not read chainId/nonce/gasPrice"

# Supplying chain, nonce and gas makes mktx sign without reaching an RPC, which matters because
# the host RPC is only reachable from inside the namespace.
raw=$(cast mktx --private-key "${owner_key}" \
  --chain "$((chain_id))" --nonce "$((nonce))" \
  --gas-price "$((gas_price))" --priority-gas-price "${priority_fee}" --gas-limit 1000000 \
  "${protocol_config}" "${calldata}") \
  || fail "cast mktx failed; check the key and parameters"

tx=$(rpc_result "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_sendRawTransaction\",\"params\":[\"${raw}\"]}")
[[ "${tx}" == 0x* ]] || fail "broadcast failed: $(rpc "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_sendRawTransaction\",\"params\":[\"${raw}\"]}")"
echo "== broadcast tx ${tx}"

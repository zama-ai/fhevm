#!/usr/bin/env bash
# Reclaim leftover Sepolia/Amoy funds from a preview namespace before it is deleted.
# generate-mnemonic.cjs makes a FRESH mnemonic per run, so anything still sitting in
# these wallets when the namespace goes is lost for good - and the floors have to be
# sized for the worst gas we might hit, so most runs over-fund on purpose.
# Only chain_mode=testnets has real money: the `rpc` Secret exists only there, so its
# absence is the gate (anvil is fake, blockchain-dev is a PoW faucet).
# Env: NAMESPACE. Needs ethers on NODE_PATH.
set -euo pipefail

: "${NAMESPACE:?}"

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

secret_value() {
  kubectl get secret "$1" -n "${NAMESPACE}" -o "jsonpath={.data.$2}" 2>/dev/null | base64 -d
}

if ! kubectl get namespace "${NAMESPACE}" >/dev/null 2>&1; then
  echo "Namespace ${NAMESPACE} does not exist, nothing to sweep."
  exit 0
fi

mnemonic=$(secret_value preview-wallets-mnemonic mnemonic || true)
if [[ -z "${mnemonic}" ]]; then
  echo "No preview-wallets-mnemonic Secret (anvil preview or pre-sweep namespace) - nothing to sweep."
  exit 0
fi
# The mnemonic and the faucet keys are spendable, and the RPC URLs carry a QuickNode
# API key. Mask them before anything else can echo one into the log - ::add-mask::
# has to be written by the step itself, so it cannot come from deploy-rpc-secret.sh.
echo "::add-mask::${mnemonic}"

eth_url=$(secret_value rpc ethereum-rpc-url || true)
polygon_url=$(secret_value rpc polygon-rpc-url || true)
for url in "${eth_url}" "${polygon_url}"; do
  if [[ -n "${url}" ]]; then echo "::add-mask::${url}"; fi
done
if [[ -z "${eth_url}" && -z "${polygon_url}" ]]; then
  echo "No rpc Secret - not a testnets preview, nothing to sweep."
  exit 0
fi

eth_key=$(secret_value eth-faucet private-key || true)
polygon_key=$(secret_value polygon-faucet private-key || true)
for key in "${eth_key}" "${polygon_key}"; do
  if [[ -n "${key}" ]]; then echo "::add-mask::${key}"; fi
done

# The treasury is whatever address funded the run; recover it from the faucet key
# rather than hardcoding, so a rotated secret cannot send the refund into a void.
treasury_of() {
  MASKED_KEY="$1" node -e '
    const { ethers } = require("ethers");
    let k = (process.env.MASKED_KEY || "").trim();
    if (k && !k.startsWith("0x")) k = "0x" + k;
    process.stdout.write(k ? new ethers.Wallet(k).address : "");
  ' 2>/dev/null || true
}

chains="[]"
if [[ -n "${eth_url}" && -n "${eth_key}" ]]; then
  eth_treasury=$(treasury_of "${eth_key}")
  if [[ -n "${eth_treasury}" ]]; then
    chains=$(jq -c --argjson c "${chains}" --arg u "${eth_url}" --arg t "${eth_treasury}" \
      -n '$c + [{label:"sepolia",rpcUrl:$u,chainId:"11155111",treasury:$t}]')
  fi
fi
if [[ -n "${polygon_url}" && -n "${polygon_key}" ]]; then
  polygon_treasury=$(treasury_of "${polygon_key}")
  if [[ -n "${polygon_treasury}" ]]; then
    chains=$(jq -c --argjson c "${chains}" --arg u "${polygon_url}" --arg t "${polygon_treasury}" \
      -n '$c + [{label:"amoy",rpcUrl:$u,chainId:"80002",treasury:$t}]')
  fi
fi

if [[ "${chains}" == "[]" ]]; then
  echo "No chain had both an RPC URL and a faucet key - nothing to sweep."
  exit 0
fi

# Roles #0-#4 and #9 (fund-wallets.sh), then the KMS tx-senders from offset 10 and the
# coprocessor tx-senders right after them (preview-env-deploy.yml). Teardown does not
# know nb_kms_core / nb_coprocessor, so cover the whole span the largest topology can
# reach (13 KMS + 5 coprocessor = 10..27); deriving a wallet that was never funded just
# reads a zero balance and is skipped.
indices="0,1,2,3,4,9"
for i in $(seq 10 27); do indices="${indices},${i}"; done

MNEMONIC="${mnemonic}" CHAINS_JSON="${chains}" HD_INDICES="${indices}" \
  node "${script_dir}/sweep-wallets.cjs"

#!/usr/bin/env bash
# Fund every address this preview signs with; no-op on Anvil (prefunded).
# blockchain-dev: PoW faucets for host + gateway (Job). testnets: treasury key for Sepolia/Amoy (runner), Nitro faucet for the gateway (Job).
# Env: CHAIN_MODE, NAMESPACE, HOST_HTTP, GATEWAY_HTTP, HOST_FAUCET, GATEWAY_FAUCET, WALLETS_JSON, COPROC_WALLETS_JSON, ROLES_JSON_PATH;
#      testnets also POLYGON_HTTP, HOST_CHAIN_ID, POLYGON_CHAIN_ID, ETH_FUNDER_PRIVATE_KEY, POLYGON_FUNDER_PRIVATE_KEY, NODE_PATH.
set -euo pipefail

if [[ "${EXTERNAL_CHAINS:-false}" != "true" ]]; then
  echo "Anvil mode: accounts are prefunded, skipping faucet."
  exit 0
fi

: "${CHAIN_MODE:?}"
: "${NAMESPACE:?}"
: "${HOST_HTTP:?}"
: "${GATEWAY_HTTP:?}"
: "${GATEWAY_FAUCET:?}"
: "${ROLES_JSON_PATH:?}"

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
python_src="${script_dir}/fund-wallets.py"

# Role wallets sign on both sides; the coprocessor tx-senders only on the gateway.
# The KMS tx-senders also sign on the canonical host chain - see the testnets branch below.
role_addresses=$(python3 - <<'PY'
import json, os
roles = json.load(open(os.environ["ROLES_JSON_PATH"]))["roles"]
print("\n".join(roles[idx]["address"] for idx in ("0", "1", "2", "3", "4", "9")))
PY
)
all_addresses=$(python3 - <<'PY'
import json, os
addrs = []
roles = json.load(open(os.environ["ROLES_JSON_PATH"]))["roles"]
for idx in ("0", "1", "2", "3", "4", "9"):
    addrs.append(roles[idx]["address"])
for key in ("WALLETS_JSON", "COPROC_WALLETS_JSON"):
    raw = os.environ.get(key) or "[]"
    for w in json.loads(raw):
        addrs.append(w["address"])
# unique, preserve order
seen = set()
out = []
for a in addrs:
    if a.lower() not in seen:
        seen.add(a.lower())
        out.append(a)
print("\n".join(out))
PY
)

# Role #3 is the relayer's gateway signer (apply-chain-env.sh) and the only heavy gateway spender.
relayer_address=$(python3 -c 'import json,os; print(json.load(open(os.environ["ROLES_JSON_PATH"]))["roles"]["3"]["address"])')

fund_targets="host,gateway"
if [[ "${CHAIN_MODE}" == "testnets" ]]; then
  # HOST_HTTP/POLYGON_HTTP were read from Secret `rpc` by deploy-rpc-secret.sh.
  : "${HOST_HTTP:?}"
  : "${POLYGON_HTTP:?}"
  : "${HOST_CHAIN_ID:?}"
  : "${POLYGON_CHAIN_ID:?}"
  : "${ETH_FUNDER_PRIVATE_KEY:?}"
  : "${POLYGON_FUNDER_PRIVATE_KEY:?}"
  deployer=$(python3 -c 'import json,os; print(json.load(open(os.environ["ROLES_JSON_PATH"]))["roles"]["9"]["address"])')
  echo "Funding role wallets on Sepolia (ethereum-faucet) and Amoy (polygon-faucet)..."
  sepolia_json=$(jq -cn --arg h "${HOST_HTTP}" --arg hc "${HOST_CHAIN_ID}" \
    '[{label:"sepolia",rpcUrl:$h,chainId:$hc}]')
  amoy_json=$(jq -cn --arg p "${POLYGON_HTTP}" --arg pc "${POLYGON_CHAIN_ID}" \
    '[{label:"amoy",rpcUrl:$p,chainId:$pc}]')
  ADDRESSES="${role_addresses}" DEPLOYER_ADDRESS="${deployer}" \
    FUNDER_PRIVATE_KEY="${ETH_FUNDER_PRIVATE_KEY}" CHAINS_JSON="${sepolia_json}" \
    node "${script_dir}/fund-wallets-treasury.cjs"
  # Since RFC013 KMSGeneration lives on the canonical HOST chain, so each party's
  # kms-connector tx-sender signs PrepKeygen/Keygen/Crsgen responses on Sepolia - the
  # gateway faucet Job below is not enough. Left unfunded they burn their 4 retries on
  # "insufficient funds ... balance 0", the ceremony never reaches consensus, and the
  # keygen wait times out. Only those few ceremony txs land here (decryption responses
  # go to the gateway), so they need a far smaller float than a test signer.
  kms_tx_senders=$(python3 -c 'import json,os; print("\n".join(w["address"] for w in json.loads(os.environ.get("WALLETS_JSON") or "[]")))')
  if [[ -n "${kms_tx_senders}" ]]; then
    echo "Funding KMS tx-senders on Sepolia (KMSGeneration responses)..."
    ADDRESSES="${kms_tx_senders}" DEPLOYER_ADDRESS="${deployer}" \
      FLOOR_WEI="50000000000000000" \
      FUNDER_PRIVATE_KEY="${ETH_FUNDER_PRIVATE_KEY}" CHAINS_JSON="${sepolia_json}" \
      node "${script_dir}/fund-wallets-treasury.cjs"
  fi
  # Polygon's canonical-snapshot flow deploys the empty-proxy set TWICE - once as a
  # throwaway, then again once the canonical ProtocolConfig snapshot is written - on top
  # of the 9 implementations. Measured at 69 gwei: 5.5M gas (0.38 POL) for a single
  # contract and ~2.5 POL for the whole deploy, so the old 2.0 floor died two
  # implementations from the end. 6.0 absorbs a spike to ~165 gwei. A floor has to cover
  # the worst gas the run might meet, so it over-funds by design; sweep-wallets.sh
  # returns the remainder at teardown, which is what keeps that margin affordable.
  # The test signers need headroom too: one e2e fixture deploy on Amoy was measured at
  # 0.3 POL, so the 0.2 default drains them partway through the suite and every later
  # job dies on "insufficient funds" instead of exercising anything.
  ADDRESSES="${role_addresses}" DEPLOYER_ADDRESS="${deployer}" \
    FLOOR_WEI="6000000000000000000" \
    DEPLOYER_FLOOR_WEI="6000000000000000000" \
    FUNDER_PRIVATE_KEY="${POLYGON_FUNDER_PRIVATE_KEY}" CHAINS_JSON="${amoy_json}" \
    node "${script_dir}/fund-wallets-treasury.cjs"
  # Only the Nitro gateway has a faucet on this path.
  fund_targets="gateway"
else
  : "${HOST_FAUCET:?}"
fi

echo "Funding via in-cluster faucet Job (targets: ${fund_targets})..."
echo "${all_addresses}"
kubectl create configmap preview-fund-script -n "${NAMESPACE}" \
  --from-file=fund-wallets.py="${python_src}" \
  --from-literal=addresses="${all_addresses}" \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl delete job preview-fund-wallets -n "${NAMESPACE}" --ignore-not-found
kubectl apply -n "${NAMESPACE}" -f - <<EOF2
apiVersion: batch/v1
kind: Job
metadata:
  name: preview-fund-wallets
spec:
  backoffLimit: 1
  ttlSecondsAfterFinished: 600
  template:
    spec:
      restartPolicy: Never
      imagePullSecrets:
        - name: registry-credentials
      containers:
        - name: fund
          image: hub.zama.org/docker.io/library/python:3.12-alpine
          env:
            - name: FUND_TARGETS
              value: "${fund_targets}"
            - name: HOST_HTTP
              value: "${HOST_HTTP}"
            - name: GATEWAY_HTTP
              value: "${GATEWAY_HTTP}"
            - name: HOST_FAUCET
              value: "${HOST_FAUCET:-}"
            - name: GATEWAY_FAUCET
              value: "${GATEWAY_FAUCET}"
            - name: HOST_FLOOR_WEI
              value: "500000000000000000"
            - name: GATEWAY_FLOOR_WEI
              value: "200000000000000000"
            - name: GATEWAY_FLOOR_OVERRIDES
              value: "${relayer_address}:1000000000000000000"
            - name: MAX_CLAIMS_PER_ADDR
              value: "12"
          volumeMounts:
            - name: script
              mountPath: /fund
          command: ["python", "/fund/fund-wallets.py"]
      volumes:
        - name: script
          configMap:
            name: preview-fund-script
EOF2

kubectl wait -n "${NAMESPACE}" --for=condition=complete "job/preview-fund-wallets" --timeout=10m \
  || { kubectl logs -n "${NAMESPACE}" "job/preview-fund-wallets" || true; exit 1; }
kubectl logs -n "${NAMESPACE}" "job/preview-fund-wallets"

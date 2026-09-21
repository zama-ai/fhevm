#!/usr/bin/env bash
# Points the Solana e2e harness, the demo seed and the demo dapp at a preview-env namespace whose
# programs run on Solana devnet.
#
#   scripts/preview-env-solana-e2e.sh up <namespace> [state-dir]   # port-forwards + env file
#   scripts/preview-env-solana-e2e.sh down <state-dir>              # stops the port-forwards
#
# `up` writes <state-dir>/preview.env; load it and run the usual commands:
#   set -a; source <state-dir>/preview.env; set +a
#   bun run test:e2e e2e/scenarios/confidential-transfer.scenario.test.ts e2e/scenarios/token-vertical.scenario.test.ts
#   bun run demo:seed                                 # once per namespace deployment
#   bun run demo:operator &                           # DEMO_OPERATOR_URL, default http://127.0.0.1:8091
#   (cd ../../solana/demo-dapp && npm run dev)        # DEMO_DAPP_URL, default http://127.0.0.1:5173
#   bun run demo:smoke                                # the deposit arc, against the operator + namespace
#
# The demo boot capability (`DEMO_BOOT_ID` + the 0600 token file the dapp dev server presents to
# the operator) is created under <state-dir> in place of the local lifecycle's
# owned boot, and reused across `up` runs. Normal run cleanup sweeps saved actors; the operator
# replenishes keeper/mint-authority balances as needed from the deployer.
#
# What it gathers from the namespace, and why:
#   - gateway/host contract addresses (configmaps) into the fhevm-cli address layout under
#     <state-dir>, so `readGatewayBootstrapInputs`/`readActiveKmsPair` read them unchanged;
#   - the devnet RPC URL (secret `solana-rpc`; the websocket URL is the same endpoint over wss);
#   - the deployer keypair (secret `solana-deployer`) unless SOLANA_DEPLOYER_KEYPAIR already
#     names a funded devnet wallet; it funds every scenario actor by transfer;
#   - the listener proof endpoint bearer token (secret `solana-proof-api`) for the demo operator.
# The relayer, both anvil chains and the first coprocessor's proof endpoint are port-forwarded to
# the loopback ports the local stack uses, so no default URL changes.
set +x
set -euo pipefail
umask 077

usage() { echo "usage: $0 up <namespace> [state-dir] | down <state-dir>" >&2; exit 2; }

forward() { # name local:remote
  kubectl port-forward -n "$namespace" "$1" "$2" >>"$state/port-forward.log" 2>&1 &
  echo $! >>"$state/port-forward.pids"
}

secret_value() { kubectl get secret -n "$namespace" "$1" -o "jsonpath={.data.$2}" | base64 -d; }

configmap_value() { kubectl get configmap -n "$namespace" "$1" -o "jsonpath={.data.$2}"; }

case "${1:-}" in
down)
  state=${2:?state-dir}
  if [[ -f "$state/port-forward.pids" ]]; then
    xargs kill <"$state/port-forward.pids" 2>/dev/null || true
    rm -f "$state/port-forward.pids"
  fi
  echo "port-forwards stopped"
  ;;
up)
  namespace=${2:?namespace}
  state=${3:-$PWD/.fhevm-preview/$namespace}
  mkdir -p "$state/runtime/addresses/gateway" "$state/runtime/addresses/host"
  state=$(cd "$state" && pwd)
  chmod 700 "$state"
  [[ -f "$state/port-forward.pids" ]] && "$0" down "$state"

  {
    echo "GATEWAY_CONFIG_ADDRESS=$(configmap_value gw-sc-addresses 'gateway_config\.address')"
    echo "INPUT_VERIFICATION_ADDRESS=$(configmap_value gw-sc-addresses 'input_verification\.address')"
    echo "DECRYPTION_ADDRESS=$(configmap_value gw-sc-addresses 'decryption\.address')"
  } >"$state/runtime/addresses/gateway/.env.gateway"
  echo "PROTOCOL_CONFIG_CONTRACT_ADDRESS=$(configmap_value host-sc-addresses 'protocol_config\.address')" \
    >"$state/runtime/addresses/host/.env.host"

  deployer=${SOLANA_DEPLOYER_KEYPAIR:-$state/deployer.json}
  if [[ ! -f "$deployer" ]]; then
    (umask 077 && secret_value solana-deployer 'deployer\.json' >"$deployer")
  fi
  deployer=$(cd "$(dirname "$deployer")" && pwd)/$(basename "$deployer")
  rpc_url=$(secret_value solana-rpc 'rpc-url')
  proof_api_key=$(secret_value solana-proof-api 'api-key')

  # Demo boot capability: a UUID boot id and a 256-bit base64url token in a 0600 file (the shape
  # `demo/authorization.ts` validates), kept across `up` runs so the seeded runtime config stays valid.
  if [[ ! -f "$state/demo-boot-id" ]]; then
    python3 -c 'import uuid; print(uuid.uuid4())' >"$state/demo-boot-id"
    (umask 077 && python3 -c 'import base64, os; print(base64.urlsafe_b64encode(os.urandom(32)).decode().rstrip("="))' \
      >"$state/demo-authorization-token")
  fi
  demo_boot_id=$(<"$state/demo-boot-id")

  # Local ports for the forwards. The defaults mirror the local stack so a sourced preview.env needs
  # no other change; override any of them (PREVIEW_*_PORT) when a local stack is up at the same time.
  relayer_port=${PREVIEW_RELAYER_PORT:-3000}
  gateway_port=${PREVIEW_GATEWAY_RPC_PORT:-8546}
  host_port=${PREVIEW_HOST_RPC_PORT:-8545}
  leaf_proof_port=${PREVIEW_LEAF_PROOF_PORT:-18080}
  dapp_port=${PREVIEW_DAPP_PORT:-5173}
  operator_port=${PREVIEW_OPERATOR_PORT:-8091}
  relayer=$(kubectl get svc -n "$namespace" -l app.kubernetes.io/instance=relayer -o name | head -1)
  forward "${relayer:-svc/relayer}" "$relayer_port:3000"
  forward svc/anvil-gateway-anvil-node "$gateway_port:8546"
  forward svc/anvil-host-anvil-node "$host_port:8545"
  forward svc/coprocessor-1-solana-host-listener "$leaf_proof_port:8080"
  sleep 2

  # Shell-escape values, including quotes in credentials, without executing or logging them.
  env_line() { printf "%s=%q\n" "$1" "$2"; }
  {
    env_line SOLANA_HOST_CHAIN_ID 130140237723663404
    env_line SOLANA_E2E_SOURCE devnet
    env_line SOLANA_RECOVERY_DIR "$state/recovery"
    env_line SOLANA_PREVIEW_NAMESPACE "$namespace"
    env_line DEMO_RELAYER_API_KEY "${DEMO_RELAYER_API_KEY:-local}"
    env_line SOLANA_RPC_URL "$rpc_url"
    env_line SOLANA_WS_URL "${rpc_url/https:/wss:}"
    env_line SOLANA_RELAYER_URL "http://127.0.0.1:$relayer_port"
    env_line DEMO_RELAYER_URL "http://127.0.0.1:$relayer_port"
    env_line GW_RPC "http://127.0.0.1:$gateway_port"
    env_line HOST_RPC "http://127.0.0.1:$host_port"
    # The seeded demo config, boots and batch tables land under FHEVM_STATE_DIR (src/layout.ts).
    env_line FHEVM_STATE_DIR "$state"
    env_line COPROCESSOR_DB_PSQL "kubectl exec -n $namespace postgres-coprocessor-1-0 -- psql -U zama -d fhevm_e2e"
    env_line SOLANA_DEPLOYER_KEYPAIR "$deployer"
    # The same endpoint for the harness (SOLANA_*) and the dapp dev server (DEMO_*).
    env_line SOLANA_LEAF_PROOF_URL "http://127.0.0.1:$leaf_proof_port"
    env_line SOLANA_LEAF_PROOF_API_KEY "$proof_api_key"
    env_line DEMO_PROOF_URL "http://127.0.0.1:$leaf_proof_port"
    env_line DEMO_PROOF_API_KEY "$proof_api_key"
    env_line DEMO_BOOT_ID "$demo_boot_id"
    env_line DEMO_AUTH_TOKEN_FILE "$state/demo-authorization-token"
    env_line DEMO_DAPP_URL "http://127.0.0.1:$dapp_port"
    env_line DEMO_OPERATOR_URL "http://127.0.0.1:$operator_port"
    env_line DEMO_ALLOWED_ORIGIN "http://127.0.0.1:$dapp_port"
  } >"$state/preview.env"
  chmod 600 "$state/preview.env"
  echo "env written to $state/preview.env (secrets inside; not echoed)"
  echo "port-forwards: $(wc -l <"$state/port-forward.pids" | tr -d ' ') running, log $state/port-forward.log"
  ;;
*) usage ;;
esac

#!/usr/bin/env bash
# Resolve chain endpoints/ids for CHAIN_MODE (anvil | blockchain-dev | testnets) into $GITHUB_ENV.
# testnets = public Sepolia 11155111 + Amoy 80002 via the in-cluster eRPC proxy, Nitro 412346 gateway; forces DEPLOY_POLYGON.
set -euo pipefail

CHAIN_MODE="${CHAIN_MODE:-anvil}"
case "${CHAIN_MODE}" in
  anvil|blockchain-dev|testnets) ;;
  *)
    echo "::error::unknown CHAIN_MODE '${CHAIN_MODE}' (expected anvil|blockchain-dev|testnets)"
    exit 1
    ;;
esac

if [[ "${CHAIN_MODE}" == "blockchain-dev" && "${DEPLOY_POLYGON:-false}" == "true" ]]; then
  echo "::error::chain_mode=blockchain-dev cannot be combined with deploy_polygon: blockchain-dev has no Polygon node. Use chain_mode=testnets (Sepolia + Amoy) or stay on Anvil."
  exit 1
fi

# Both external modes use the blockchain-dev Nitro as gateway.
NITRO_HTTP="http://gateway-rpc-node.blockchain-dev:8547"
NITRO_WS="ws://gateway-rpc-node.blockchain-dev:8548"
NITRO_CHAIN_ID="412346"
NITRO_FAUCET="http://gateway-faucet-blockchain-dev-powfaucet.blockchain-dev:8080"

# http(s):// -> ws(s):// on the same host, for providers serving WS on the HTTP endpoint.
derive_ws() {
  local url="$1"
  case "${url}" in
    https://*) echo "wss://${url#https://}" ;;
    http://*) echo "ws://${url#http://}" ;;
    *) echo "${url}" ;;
  esac
}

case "${CHAIN_MODE}" in
  anvil)
    {
      echo "CHAIN_MODE=anvil"
      echo "EXTERNAL_CHAINS=false"
      echo "HOST_HTTP=http://anvil-host-anvil-node:8545"
      echo "HOST_WS=ws://anvil-host-anvil-node:8545"
      echo "GATEWAY_HTTP=http://anvil-gateway-anvil-node:8546"
      echo "GATEWAY_WS=ws://anvil-gateway-anvil-node:8546"
      echo "HOST_CHAIN_ID=12345"
      echo "GATEWAY_CHAIN_ID=54321"
      echo "HOST_FAUCET="
      echo "GATEWAY_FAUCET="
      echo "HEAD_BLOCK=0"
      echo "HARDHAT_NETWORK_TESTS=staging"
    } >> "${GITHUB_ENV}"
    echo "Chain mode: anvil (host 12345, gateway 54321$([[ "${DEPLOY_POLYGON:-false}" == "true" ]] && echo ', polygon 80002'))"
    ;;

  blockchain-dev)
    {
      echo "CHAIN_MODE=blockchain-dev"
      echo "EXTERNAL_CHAINS=true"
      echo "HOST_HTTP=http://ethereum-rpc-node.blockchain-dev:8545"
      echo "HOST_WS=ws://ethereum-rpc-node.blockchain-dev:8545"
      echo "GATEWAY_HTTP=${NITRO_HTTP}"
      echo "GATEWAY_WS=${NITRO_WS}"
      echo "HOST_CHAIN_ID=1337"
      echo "GATEWAY_CHAIN_ID=${NITRO_CHAIN_ID}"
      echo "HOST_FAUCET=http://host-faucet-blockchain-dev-powfaucet.blockchain-dev:8080"
      echo "GATEWAY_FAUCET=${NITRO_FAUCET}"
      # POLLER_SEED_START_BLOCK comes from the "Capture host head block" step.
      echo "HOST_FINALITY_DEPTH=0"
      echo "HOST_FINALITY_LAG=1"
      echo "HARDHAT_NETWORK_TESTS=zwsDev"
    } >> "${GITHUB_ENV}"
    echo "Chain mode: blockchain-dev (host 1337 Geth, gateway ${NITRO_CHAIN_ID} Nitro)"
    ;;

  testnets)
    : "${FUNDER_PRIVATE_KEY:?chain_mode=testnets needs the PREVIEW_TESTNETS_FUNDER_PRIVATE_KEY secret (treasury holding Sepolia ETH + Amoy POL)}"
    # Default: the eRPC public proxy (deploy-erpc.sh); RPC_URL_SEPOLIA/RPC_URL_AMOY optionally bypass it with a dedicated endpoint.
    erpc="http://preview-erpc:4000/listener-indexer/evm"
    host_http="${RPC_URL_SEPOLIA:-${erpc}/11155111}"
    polygon_http="${RPC_URL_AMOY:-${erpc}/80002}"
    # No host WS consumer on this topology (built-in hostListener disabled); derived only to fill the values.
    host_ws=$(derive_ws "${host_http}")
    polygon_ws=$(derive_ws "${polygon_http}")
    # The runner-side funder cannot reach the ClusterIP proxy: public endpoints (same defaults as environments.ts) or the override.
    funder_sepolia="${RPC_URL_SEPOLIA:-https://ethereum-sepolia-rpc.publicnode.com}"
    funder_amoy="${RPC_URL_AMOY:-https://rpc-amoy.polygon.technology}"
    {
      echo "CHAIN_MODE=testnets"
      echo "EXTERNAL_CHAINS=true"
      # Amoy is the second host chain: deploy_polygon-gated steps run, minus the Anvil node (EXTERNAL_CHAINS gate).
      echo "DEPLOY_POLYGON=true"
      echo "HOST_HTTP=${host_http}"
      echo "HOST_WS=${host_ws}"
      echo "POLYGON_HTTP=${polygon_http}"
      echo "POLYGON_WS=${polygon_ws}"
      echo "FUNDER_RPC_SEPOLIA=${funder_sepolia}"
      echo "FUNDER_RPC_AMOY=${funder_amoy}"
      echo "GATEWAY_HTTP=${NITRO_HTTP}"
      echo "GATEWAY_WS=${NITRO_WS}"
      echo "HOST_CHAIN_ID=11155111"
      echo "POLYGON_CHAIN_ID=80002"
      echo "GATEWAY_CHAIN_ID=${NITRO_CHAIN_ID}"
      echo "HOST_FAUCET="
      echo "GATEWAY_FAUCET=${NITRO_FAUCET}"
      # Negative = "behind head", resolved by the poller at startup, so no head capture and no drift during the long deploy.
      echo "POLLER_SEED_START_BLOCK=-10"
      # Public testnets reorg; Anvil/Geth --dev never do.
      echo "HOST_FINALITY_DEPTH=${HOST_FINALITY_DEPTH:-2}"
      echo "POLYGON_FINALITY_DEPTH=${POLYGON_FINALITY_DEPTH:-3}"
      echo "HOST_FINALITY_LAG=${HOST_FINALITY_LAG:-2}"
      echo "POLYGON_FINALITY_LAG=${POLYGON_FINALITY_LAG:-3}"
      # ETH e2e hardhat network (11155111, live path); the Polygon run already uses `-n polygonAmoy`.
      echo "HARDHAT_NETWORK_TESTS=sepolia"
      # 12s blocks stretch hardhat deploys and the keygen ceremony.
      echo "CONTRACTS_DEPLOY_TIMEOUT=30m"
      echo "KEYGEN_TIMEOUT=60m"
    } >> "${GITHUB_ENV}"
    rpc_src="eRPC public proxy"; [[ -n "${RPC_URL_SEPOLIA:-}${RPC_URL_AMOY:-}" ]] && rpc_src="RPC_URL_* override"
    echo "Chain mode: testnets (host 11155111 Sepolia, polygon 80002 Amoy via ${rpc_src}, gateway ${NITRO_CHAIN_ID} Nitro)"
    ;;
esac

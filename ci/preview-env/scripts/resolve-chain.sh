#!/usr/bin/env bash
# Resolve host/gateway RPC endpoints + chain IDs for this preview.
# Anvil (default): in-namespace anvil-host / anvil-gateway.
# blockchain-dev (USE_BLOCKCHAIN_DEV=true): shared Geth 1337 + Nitro 412346.
#
# Writes HOST_*/GATEWAY_* and HEAD_BLOCK to $GITHUB_ENV.
set -euo pipefail

if [[ "${USE_BLOCKCHAIN_DEV:-false}" == "true" && "${DEPLOY_POLYGON:-false}" == "true" ]]; then
  echo "::error::use_blockchain_dev cannot be combined with deploy_polygon: blockchain-dev has no Polygon node. Disable deploy_polygon or stay on Anvil."
  exit 1
fi

if [[ "${USE_BLOCKCHAIN_DEV:-false}" != "true" ]]; then
  {
    echo "USE_BLOCKCHAIN_DEV=false"
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
  echo "Chain mode: anvil (host 12345, gateway 54321)"
  exit 0
fi

{
  echo "USE_BLOCKCHAIN_DEV=true"
  echo "HOST_HTTP=http://ethereum-rpc-node.blockchain-dev:8545"
  echo "HOST_WS=ws://ethereum-rpc-node.blockchain-dev:8545"
  echo "GATEWAY_HTTP=http://gateway-rpc-node.blockchain-dev:8547"
  echo "GATEWAY_WS=ws://gateway-rpc-node.blockchain-dev:8548"
  echo "HOST_CHAIN_ID=1337"
  echo "GATEWAY_CHAIN_ID=412346"
  echo "HOST_FAUCET=http://host-faucet-blockchain-dev-powfaucet.blockchain-dev:8080"
  echo "GATEWAY_FAUCET=http://gateway-faucet-blockchain-dev-powfaucet.blockchain-dev:8080"
  echo "HARDHAT_NETWORK_TESTS=zwsDev"
} >> "${GITHUB_ENV}"

echo "Chain mode: blockchain-dev (host 1337 Geth, gateway 412346 Nitro)"

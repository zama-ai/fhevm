include .env.example

ENV_PATH=.env.example
FORGE_DAPP_OUT=out

# Deploy the empty proxy addresses.
deploy-empty-proxies:
	DOTENV_CONFIG_PATH=$(ENV_PATH) HARDHAT_NETWORK=hardhat npx hardhat task:deployEmptyUUPSProxies

# Deploy the pauser set contract.
deploy-pauser-set:
	DOTENV_CONFIG_PATH=$(ENV_PATH) HARDHAT_NETWORK=hardhat npx hardhat compile:specific --contract contracts/immutable
	DOTENV_CONFIG_PATH=$(ENV_PATH) HARDHAT_NETWORK=hardhat npx hardhat task:deployPauserSet

# Ensure that the empty proxy and pauser set addresses exists as these are required for contract compilation.
ensure-addresses: deploy-empty-proxies deploy-pauser-set

update-selectors: ensure-addresses
	DAPP_OUT=$(FORGE_DAPP_OUT) forge selectors list | tail -n +2 > ./docs/contract_selectors.txt

forge-test: ensure-addresses
	forge install foundry-rs/forge-std && forge test
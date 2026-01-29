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

forge-test: ensure-addresses
	forge soldeer install && forge test

prettier:
	npx prettier . --write

compile:
	npx hardhat compile

clean:
	npx hardhat clean

get-accounts:
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat get-accounts

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test --network hardhat $(if $(GREP),--grep '$(GREP)',)

# Here, we purposely use a logical OR (||) instead of an if statement with a negation to avoid having
# discrepancies between running locally and in the CI. This is because some shell environments
# handle exit statuses of pipelines differently.
check-selectors:
	DAPP_OUT=$(FORGE_DAPP_OUT) forge selectors list | tail -n +2 | diff ./docs/contract_selectors.txt - &> /dev/null || { \
		echo "Contract selectors are not up-to-date."; \
		echo "Please run 'make update-selectors' to update them."; \
		exit 1; \
	}

update-selectors:
	DAPP_OUT=$(FORGE_DAPP_OUT) forge selectors list | tail -n +2 > ./docs/contract_selectors.txt

check-bindings: ensure-addresses
	python3 ../ci/contracts_bindings_update.py --project host check

update-bindings: ensure-addresses
	python3 ../ci/contracts_bindings_update.py --project host update

lint-bindings:
	cd rust_bindings && cargo clippy -- -D warnings && cd ..

# Update auto-generated files for conformance checks
update-conformance: update-bindings update-selectors

# Conform to pre-commit checks
conformance: prettier update-conformance

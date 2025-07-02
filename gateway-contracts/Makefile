include .env.example

LOCAL_NETWORK_NAME=localGateway
ENV_PATH=.env.example
FORGE_DAPP_OUT=artifacts

prettier:
	npx prettier . --write

compile:
	npx hardhat compile

clean:
	npx hardhat clean

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test $(if $(GREP),--grep '$(GREP)',)

coverage: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat coverage

get-accounts:
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat get-accounts --num-accounts 20

start-local-node: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat node --port 8757

deploy-contracts-local:
	cp $(ENV_PATH) .env
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:deployAllGatewayContracts
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:addHostChainsToGatewayConfig --use-internal-gateway-config-address true

test-local:
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test $(if $(GREP),--grep '$(GREP)',) --network localGateway --skip-setup true

docker-compose-build:
	pwd
	cp .env.example .env
	docker compose -vvv build

docker-compose-up: docker-compose-down
	cp .env.example .env
	docker compose -vvv up -d

docker-compose-down:
	docker compose -vvv down -v --remove-orphans

check-bindings: ensure-addresses
	python3 scripts/bindings_update.py check

update-bindings: ensure-addresses
	python3 scripts/bindings_update.py update

lint-bindings:
	cd rust_bindings && cargo clippy -- -D warnings && cd ..

check-mocks:
	node scripts/mock_contracts_cli.js check

update-mocks:
	node scripts/mock_contracts_cli.js update

# Here, we purposely use a logical OR (||) instead of an if statement with a negation to avoid having
# discrepancies between running locally and in the CI. This is because some shell environments
# handle exit statuses of pipelines differently.
check-selectors: ensure-addresses
	DAPP_OUT=$(FORGE_DAPP_OUT) forge selectors list | tail -n +2 | diff ./selectors.txt - &> /dev/null || { \
		echo "Contract selectors are not up-to-date."; \
		echo "Please run 'make update-selectors' to update them."; \
		exit 1; \
	}

update-selectors: ensure-addresses
	DAPP_OUT=$(FORGE_DAPP_OUT) forge selectors list | tail -n +2 > ./selectors.txt

# Conform to pre-commit checks
conformance: prettier update-bindings update-mocks update-selectors

# Deploy the empty proxy addresses
deploy-empty-proxies:
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat task:deployEmptyUUPSProxies

# Ensure that the empty proxy addresses exists as these are required for contract compilation.
# In addition, ensure that the addresses match the ones used in local development in order to
# avoid discrepancies between local and CI (e.g., when generating and checking rust bindings).
ensure-addresses:
	ENV_PATH=$(ENV_PATH) npx ts-node scripts/ensure_proxy_addresses.ts

# Make sure we only use allowed licenses for dependencies
check-licenses:
	output=$$(npx license-checker --onlyAllow '0BSD; BSD; Apache-2.0; BSD-2-Clause; BSD-3-Clause; CC-BY-3.0; CC0-1.0; ISC; MIT; MPL-2.0; Python-2.0; WTFPL; PSF; LGPL-3.0' 2>&1); \
	status=$$?; \
	if [ $$status -ne 0 ]; then \
		printf '%s\n' "$$output"; \
		exit $$status; \
	fi

# Bump the prerelease version of the Gateway and its rust bindings crate
# This command:
# - Bumps the npm version of the Gateway, creates a new commit and tag
# - Updates the rust bindings and aligns the version with the npm version
# - Adds these changes to the above commit
prerelease:
	npm version prerelease
	$(MAKE) update-bindings
	git add ./rust_bindings && git commit --amend --no-edit

# Get the size of the contracts
get-contracts-sized:
	forge build --sizes

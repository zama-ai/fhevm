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

get-accounts:
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat get-accounts --num-accounts 20

start-local-node:
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat node --port 8546

deploy-contracts-local:
	cp $(ENV_PATH) .env
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:faucetToPrivate --private-key $(DEPLOYER_PRIVATE_KEY)
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:deployAllGatewayContracts
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:addNetworksToGatewayConfig --use-internal-gateway-config-address true

docker-compose-build:
	cp .env.example .env
	docker compose -vvv build

docker-compose-up: docker-compose-down
	cp .env.example .env
	docker compose -vvv up -d

docker-compose-down:
	docker compose -vvv down -v --remove-orphans

check-bindings:
	python3 scripts/bindings_update.py check

update-bindings:
	python3 scripts/bindings_update.py update

check-mocks:
	node scripts/mock_contracts_cli.js check

update-mocks:
	node scripts/mock_contracts_cli.js update

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

# Conform to pre-commit checks
conformance: prettier update-bindings update-mocks update-selectors

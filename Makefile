include .env.test

LOCAL_NETWORK_NAME=localHTTPZGateway
ENV_TEST_PATH=.env.test
FORGE_DAPP_OUT=artifacts

prettier:
	npx prettier . --write

compile:
	npx hardhat compile

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test:
	DOTENV_CONFIG_PATH=$(ENV_TEST_PATH) npx hardhat test

get-accounts:
	npx hardhat get-accounts --num-accounts 20

start-local-node:
	npx hardhat node --port 8546

deploy-contracts-local:
	cp $(ENV_TEST_PATH) .env
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:faucetToPrivate --private-key $(DEPLOYER_PRIVATE_KEY)
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:deployAllGatewayContracts
	HARDHAT_NETWORK=$(LOCAL_NETWORK_NAME) npx hardhat task:addNetworksToHttpz --use-internal-httpz-address true

docker-compose-build:
	docker compose -vvv build

docker-compose-up:
	docker compose -vvv up -d

docker-compose-down:
	docker compose -vvv down

check-bindings:
	python3 tasks/bindings_update.py check

update-bindings:
	python3 tasks/bindings_update.py update

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
conformance: prettier update-bindings update-selectors

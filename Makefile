prettier:
	npx prettier . --write

compile:
	npx hardhat compile

# Define it as a phony target to avoid conflicts with the test directory
TEST_DEPLOY_KEY = $(shell npx hardhat get-accounts --num-accounts 1 | grep Private | cut -d " " -f 3)
.PHONY: test
test:
	cp .env.example .env
	# Recompute addresses values that will be used for the tests, in case they have changed
	npx hardhat compile && npx hardhat task:deployEmptyUUPSProxies --deployer-private-key "$(TEST_DEPLOY_KEY)" --network hardhat
	npx hardhat test

get-accounts:
	npx hardhat get-accounts --num-accounts 15

deploy-contracts-local:
	cp .env.example .env
	./deploy-gateway-contracts localHTTPZGateway
	./add-httpz-networks.sh localHTTPZGateway

docker-compose-build:
	docker compose -vvv build

docker-compose-up:
	docker compose -vvv up -d

docker-compose-down:
	docker compose -vvv down

update-abi:
	python3 httpz_gateway_rust_bindings/abi_update.py update

start-local-node:
	npx hardhat node --port 8546

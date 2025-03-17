prettier:
	npx prettier . --write

compile:
	npx hardhat compile

# Define it as a phony target to avoid conflicts with the test directory
TEST_DEPLOY_KEY = $(shell npx hardhat get-accounts --num-accounts 1 | grep Private | cut -d " " -f 3)
.PHONY: test
test:
	# Recompute addresses values that will be used for the tests, in case they have changed
	npx hardhat compile && npx hardhat task:deployEmptyUUPSProxies --deployer-private-key "$(TEST_DEPLOY_KEY)" --network hardhat
	npx hardhat test

get-accounts:
	npx hardhat get-accounts --num-accounts 15

copy-env-example:
	cp .env.example .env

copy-env-example-deployment:
	cp .env.example.deployment .env

deploy-contracts-local: copy-env-example
	./deploy-httpz-gateway.sh localHTTPZGateway

deploy-contracts-local-deployment: copy-env-example-deployment
	./deploy-httpz-gateway-deployment.sh localHTTPZGateway

update-abi:
	python3 httpz_gateway_rust_bindings/abi_update.py update

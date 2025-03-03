prettier:
	npx prettier . --write

compile:
	npx hardhat compile

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test:
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

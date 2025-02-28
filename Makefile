prettier:
	pnpm exec prettier . --write

compile:
	pnpm exec hardhat compile

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test:
	pnpm exec hardhat test

get-accounts:
	pnpm exec hardhat get-accounts --num-accounts 15

copy-env-example:
	cp .env.example .env

copy-env-example-deployment:
	cp .env.example.deployment .env

deploy-contracts-local: copy-env-example
	./deploy-httpz-gateway.sh localHTTPZGateway

deploy-contracts-local-deployment: copy-env-example-deployment
	./deploy-httpz-gateway-deployment.sh localHTTPZGateway

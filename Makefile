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

deploy:
	./launch-local-gateway-layer2.sh

deploy-init:
	./launch-init-local-gateway-layer2.sh
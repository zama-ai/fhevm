prettier:
	pnpm exec prettier . --write

tests:
	pnpm exec hardhat test

get-accounts:
	pnpm exec hardhat get-accounts --num-accounts 15

deploy:
	./launch-local-gateway-layer2.sh

deploy-init:
	./launch-init-local-gateway-layer2.sh
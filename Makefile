prettier:
	pnpm exec prettier . --write

tests:
	pnpm exec hardhat test

get-accounts:
	pnpm exec hardhat get-accounts --num-accounts 15

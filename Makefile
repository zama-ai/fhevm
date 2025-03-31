prettier:
	npx prettier . --write

compile:
	npx hardhat compile

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test:
	DOTENV_CONFIG_PATH=.env.test npx hardhat test

get-accounts:
	npx hardhat get-accounts --num-accounts 20

start-local-node:
	npx hardhat node --port 8546

deploy-contracts-local:
	cp .env.example .env
	HARDHAT_NETWORK=localHTTPZGateway ./deploy-gateway-contracts.sh
	HARDHAT_NETWORK=localHTTPZGateway ./add-httpz-networks.sh

docker-compose-build:
	docker compose -vvv build

docker-compose-up:
	docker compose -vvv up -d

docker-compose-down:
	docker compose -vvv down

check-abi:
	python3 httpz_gateway_rust_bindings/abi_update.py check

update-abi:
	python3 httpz_gateway_rust_bindings/abi_update.py update

update-selectors:
	forge selectors list | tail -n +2 > ./docs/contract_selectors.txt


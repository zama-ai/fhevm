include .env.example

ENV_PATH=.env.example

prettier:
	npx prettier . --write

lint:
	npm run lint:sol:fix
	npm run lint:js:fix

compile:
	npx hardhat compile

clean:
	npx hardhat clean

get-accounts:
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat get-accounts

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test --network hardhat $(if $(GREP),--grep '$(GREP)',)

test-tasks: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test:tasks --network hardhat $(if $(GREP),--grep '$(GREP)',)


# Conform to pre-commit checks
conformance: prettier lint

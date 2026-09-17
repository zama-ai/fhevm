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
.PHONY: test check-selectors update-selectors
test: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test --network hardhat $(if $(GREP),--grep '$(GREP)',)

test-tasks: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test:tasks --network hardhat $(if $(GREP),--grep '$(GREP)',)


# Conform to pre-commit checks
conformance: prettier lint

# Selector drift guard. A logical OR rather than a negated if, so the exit status
# behaves the same locally and in CI — the same reason gateway-contracts does it this way.
check-selectors:
	forge selectors list | tail -n +2 | diff ./selectors.txt - &> /dev/null || { \
		echo "Contract selectors are not up-to-date."; \
		echo "Please run 'make update-selectors' to update them."; \
		exit 1; \
	}

update-selectors:
	forge selectors list | tail -n +2 > ./selectors.txt

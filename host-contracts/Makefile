include .env.example

ENV_PATH=.env.example
FORGE_DAPP_OUT=artifacts

prettier:
	npx prettier . --write

compile:
	npx hardhat compile

clean:
	npx hardhat clean

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test: clean
	DOTENV_CONFIG_PATH=$(ENV_PATH) npx hardhat test --network hardhat $(if $(GREP),--grep '$(GREP)',)

# Here, we purposely use a logical OR (||) instead of an if statement with a negation to avoid having
# discrepancies between running locally and in the CI. This is because some shell environments
# handle exit statuses of pipelines differently.
check-selectors:
	DAPP_OUT=$(FORGE_DAPP_OUT) forge selectors list | tail -n +2 | diff ./docs/contract_selectors.txt - &> /dev/null || { \
		echo "Contract selectors are not up-to-date."; \
		echo "Please run 'make update-selectors' to update them."; \
		exit 1; \
	}

update-selectors:
	DAPP_OUT=$(FORGE_DAPP_OUT) forge selectors list | tail -n +2 > ./docs/contract_selectors.txt

# Update auto-generated files for conformance checks
update-conformance: update-selectors

# Conform to pre-commit checks
conformance: prettier update-conformance
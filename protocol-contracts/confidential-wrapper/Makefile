prettier:
	npx prettier . --write

lint:
	npm run lint

compile:
	npx hardhat compile

clean:
	npx hardhat clean

# Define it as a phony target to avoid conflicts with the test directory
.PHONY: test
test: clean
	npx hardhat test --network hardhat $(if $(GREP),--grep '$(GREP)',)

# Conform to pre-commit checks
conformance: prettier lint

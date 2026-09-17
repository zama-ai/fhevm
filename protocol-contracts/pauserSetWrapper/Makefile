include .env.example

ENV_PATH=.env.example

prettier:
	npx prettier . --write

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


# Conform to pre-commit checks
conformance: prettier

# Selector drift guard. A logical OR rather than a negated if, so the exit status
# behaves the same locally and in CI — the same reason gateway-contracts does it this way.
check-selectors:
	@forge selectors list | tail -n +2 > ./selectors.actual
	@diff -u ./selectors.txt ./selectors.actual || { \
		echo ""; \
		echo "Contract selectors are not up-to-date. Run 'make update-selectors'."; \
		rm -f ./selectors.actual; \
		exit 1; \
	}
	@rm -f ./selectors.actual

update-selectors:
	forge selectors list | tail -n +2 > ./selectors.txt

# ── CI entry points ─────────────────────────────────────────────────────────
# Every step of this package's workflow is `make <target>`, so the same command
# runs locally and in CI. These delegate to package.json rather than restating
# the command, so there is one source of truth for what runs and one entry point
# for how to run it.
#
# NOTE `lint-check` not `lint`: where a `lint` target already exists it runs the
# --fix variants and REWRITES files, which is a local convenience and the wrong
# thing for CI — a fixing lint step passes while hiding what it changed.
install:
	npm ci

coverage:
	npm run coverage

coverage-check:
	npm run coverage:check

.PHONY: install coverage coverage-check

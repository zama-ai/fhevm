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

.PHONY: check-selectors update-selectors

# ── CI entry points ─────────────────────────────────────────────────────────
# Every step of this package's workflow is `make <target>`, so the same command
# runs locally and in CI. These delegate to package.json rather than restating
# the command, so there is one source of truth for what runs and one entry point
# for how to run it.
#
# feesBurner's package.json carries only `coverage` and `coverage:check`, and
# protocol-apps' CI ran `npx hardhat test` directly. So `compile` and `test`
# name the hardhat command, the way safe and pauserSetWrapper do, rather than a
# script that does not exist. There is no lint script, so there is no
# `lint-check` target — nothing was invented for a step the package never had.
install:
	npm ci

compile:
	npx hardhat compile

test:
	npx hardhat test

coverage:
	npm run coverage

coverage-check:
	npm run coverage:check

# `test` is phony so the test/ directory cannot satisfy it.
.PHONY: install compile test coverage coverage-check

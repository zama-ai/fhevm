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
# NOTE `lint-check` not `lint`: where a `lint` target already exists it runs the
# --fix variants and REWRITES files, which is a local convenience and the wrong
# thing for CI — a fixing lint step passes while hiding what it changed.
install:
	pnpm install --frozen-lockfile

lint-check:
	pnpm run lint:js

compile:
	pnpm run compile

test:
	pnpm test

coverage:
	pnpm run coverage

coverage-check:
	pnpm run coverage:check

.PHONY: install lint-check compile test coverage coverage-check

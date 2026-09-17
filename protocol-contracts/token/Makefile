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

.PHONY: check-selectors update-selectors

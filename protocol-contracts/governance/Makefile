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

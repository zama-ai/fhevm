# Protocol Contracts Makefile
# Common targets for all contract packages

# Dynamically discover all contract packages (subdirectories with foundry.toml)
PACKAGES := $(patsubst %/foundry.toml,%,$(wildcard */foundry.toml))

# Phony targets
.PHONY: help update-selectors check-selectors conformance prettier $(PACKAGES)

help:
	@echo "Protocol Contracts Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  update-selectors         - Update selectors.txt for all packages"
	@echo "  check-selectors          - Check selectors.txt for all packages"
	@echo ""
	@echo "Packages: $(PACKAGES)"

# Update selectors for all packages
update-selectors:
	@rm -f ./selectors.txt
	@for pkg in $(PACKAGES); do \
		cd $$pkg && ([ -d node_modules ] || ([ -f pnpm-lock.yaml ] && pnpm i || npm i)) && forge selectors list | tail -n +2 >> ../selectors.txt && cd ..; \
	done

# Check selectors for all packages
check-selectors:
	@rm -f ./selectors.txt.tmp
	@for pkg in $(PACKAGES); do \
		cd $$pkg && ([ -d node_modules ] || ([ -f pnpm-lock.yaml ] && pnpm i || npm i)) && forge selectors list | tail -n +2 >> ../selectors.txt.tmp && cd ..; \
	done
	@diff ./selectors.txt ./selectors.txt.tmp &> /dev/null || { \
		echo "Contract selectors are not up-to-date."; \
		echo "Please run 'make update-selectors' to update them."; \
		rm -f ./selectors.txt.tmp; \
		exit 1; \
	}
	@rm -f ./selectors.txt.tmp
	@echo "All selectors are up-to-date."

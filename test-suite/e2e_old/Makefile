#!/usr/bin/make -f

# Variables for paths and commands
ENV_FILE=.env
CONFIG_FILE=contracts/E2EFHEVMConfig.sol
HARDHAT_CONFIG=hardhat.config.ts
NPM_CMD=npm run task:accounts

.PHONY: all print-accounts prepare-test-local-coprocessor prepare-test-sepolia clean

# Default target
all: print-accounts

# Prints accounts with a reminder to ensure they are funded
print-accounts:
	@npm i hardhat
	@echo "Checking accounts. Ensure the first account has sufficient funds."
	@$(NPM_CMD)

# Prepares the environment for local coprocessor testing
prepare-test-local-coprocessor:
	@echo "Preparing environment for local coprocessor testing..."
	@cp -v .env.local $(ENV_FILE)
	@cp -v contracts/E2EFHEVMConfigLocal.sol $(CONFIG_FILE)
	@$(MAKE) set-local-network

# Prepares the environment for Sepolia network testing
prepare-test-sepolia:
	@echo "Preparing environment for Sepolia network testing..."
	@cp -v .env.example $(ENV_FILE)
	@cp -v contracts/E2EFHEVMConfigSepolia.sol $(CONFIG_FILE)

# Sets the default network to "localCoprocessor" in hardhat.config.ts
set-local-network:
	@echo "Setting default network to 'localCoprocessor' in $(HARDHAT_CONFIG)..."
	@if [ "$(shell uname)" = "Darwin" ]; then \
		sed -i '' 's/defaultNetwork: ".*"/defaultNetwork: "localCoprocessor"/' $(HARDHAT_CONFIG); \
	else \
		sed -i 's/defaultNetwork: ".*"/defaultNetwork: "localCoprocessor"/' $(HARDHAT_CONFIG); \
	fi

# Cleans up generated or temporary files
clean:
	@echo "Cleaning up temporary files..."
	@rm -fv $(ENV_FILE)
	@rm -fv $(CONFIG_FILE)

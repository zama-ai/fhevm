.PHONY: all setup run-hardhat deploy-fhevm-smart-contracts start-event-listener run-decrypt-test help

# Default target
all: help

# Colors for help message
GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)

# Setup target for initial configuration
setup-hardhat:
	@echo "$(GREEN)Setting up FHEVM environment...$(RESET)"
	@cd hardhat/contracts && npm install
	@cd hardhat/contracts && cp .env.example .env
	@echo "$(GREEN)Setup completed!$(RESET)"

# Copy from local.yaml.example
config-event-listener:
	@./setup-config.sh

# Run Hardhat node
run-hardhat:
	@echo "$(GREEN)Starting Hardhat node...$(RESET)"
	@cd hardhat/contracts && npx hardhat node --hostname 127.0.0.1 --port 8746

# Deploy FHEVM smart contracts
deploy-fhevm-smart-contracts:
	@echo "$(GREEN)Deploying FHEVM smart contracts...$(RESET)"
	@cd hardhat/contracts && sh launch-fhevm-local-coprocessor.sh

# Start event listener
start-event-listener:
	@echo "$(GREEN)Starting FHEVM event listener...$(RESET)"
	@cargo run --bin fhevm-relayer

# Run decrypt tests
run-test-decrypt:
	@echo "$(GREEN)Running decrypt tests...$(RESET)"
	@cd hardhat/contracts && npx hardhat test test/gatewayDecrypt/* --network localCoprocessorMocked

# Run ERC20 tests
run-test-erc20:
	@echo "$(GREEN)Running decrypt tests...$(RESET)"
	@cd hardhat/contracts && npx hardhat test test/encryptedERC20/EncryptedERC20.ts --network localCoprocessorMocked

# Help message
help:
	@echo ''
	@echo 'Available commands:'
	@echo ''
	@echo '  ${YELLOW}make setup-hardhat${RESET}                        - Initial setup of the environment'
	@echo '  ${YELLOW}make run-hardhat${RESET}                 - Start Hardhat node'
	@echo '  ${YELLOW}make deploy-fhevm-smart-contracts${RESET} - Deploy FHEVM smart contracts'
	@echo '  ${YELLOW}make config-event-listener${RESET}        - Configure FHEVM event listener'
	@echo '  ${YELLOW}make start-event-listener${RESET}        - Start FHEVM event listener'
	@echo '  ${YELLOW}make run-test-decrypt${RESET}            - Run decryption tests'
	@echo '  ${YELLOW}make run-test-erc20${RESET}            - Run erc20 tests'
	@echo ''
	@echo 'Run sequence:'
	@echo '  1. ${WHITE}make setup-hardhat${RESET}                        - First time only'
	@echo '  2. ${WHITE}make run-hardhat${RESET}                 - Terminal 1'
	@echo '  3. ${WHITE}make deploy-fhevm-smart-contracts${RESET} - Terminal 2'
	@echo '  4. ${WHITE}make config-event-listener${RESET}        - Terminal 2'
	@echo '  4. ${WHITE}make start-event-listener${RESET}        - Terminal 2'
	@echo '  5. ${WHITE}make run-test-decrypt${RESET}            - Terminal 3'
	@echo '  5. ${WHITE}make run-test-erc20${RESET}            - Terminal 3'
	@echo ''
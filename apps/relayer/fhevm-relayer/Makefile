#.PHONY: all setup run-hardhat deploy-fhevm-smart-contracts start-fhevm-relayer run-decrypt-test help
-include hardhat/contracts/.env
ifneq (,$(wildcard hardhat/contracts/.env))
    export $(shell cat hardhat/contracts/.env | sed 's/=.*//g')
endif

# Default target
all: help

# Colors for help message
GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)

# Setup target for initial configuration
setup-mock:
	@echo "$(GREEN)Setting up mock environment...$(RESET)"
	@cd hardhat/contracts && npm install
	@cd hardhat/contracts && cp .env.example .env
	@echo "$(GREEN)Setup completed!$(RESET)"

# Copy from local.yaml.example
config-fhevm-relayer:
	@./setup-config.sh

# Run Anvil node for httpz chain
run-httpz-mock:
	@echo "$(GREEN)Starting Anvil node for mock HTTPZ chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8756 --chain-id 123456 --accounts 15 --mnemonic $(MNEMONIC)

# Run Anvil node for gateway chain
run-gateway-mock:
	@echo "$(GREEN)Starting Anvil node for mock GATEWAY chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8757 --chain-id 654321 --accounts 15 --mnemonic $(MNEMONIC)

# Deploy HTTPZ smart contracts
deploy-httpz-smart-contracts:
	@echo "$(GREEN)Deploying HTTPZ smart contracts...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && sh ./launch-httpz-local-coprocessor.sh

# Deploy GATEWAY smart contracts
deploy-gateway-smart-contracts:
	@echo "$(GREEN)Deploying GATEWAY smart contracts...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && ./launch-gateway-local-coprocessor.sh

# Start event listener
start-fhevm-relayer:
	@echo "$(GREEN)Starting HTTPZ event listener...$(RESET)"
	@cargo run --bin fhevm-relayer

# Run decrypt tests
run-test-decrypt:
	@echo "$(GREEN)Running decrypt tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && RUST_RELAYER=true npx hardhat test test/gatewayDecrypt/* --network localCoprocessorL1

# Run ERC20 tests
run-test-erc20:
	@echo "$(GREEN)Running decrypt tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && RUST_RELAYER=true npx hardhat test test/encryptedERC20/EncryptedERC20.ts --network localCoprocessorL1

# Help message
help:
	@echo ''
	@echo 'Available commands:'
	@echo ''
	@echo '  ${YELLOW}make setup-mock${RESET}                        - Initial setup of the environment'
	@echo '  ${YELLOW}make run-httpz-mock${RESET}                 - Start Anvil node for httpz chain'
	@echo '  ${YELLOW}make run-gateway-mock${RESET}                 - Start Anvil node for gateway chain'
	@echo '  ${YELLOW}make deploy-httpz-smart-contracts${RESET} - Deploy HTTPZ smart contracts'
	@echo '  ${YELLOW}make deploy-gateway-smart-contracts${RESET} - Deploy Gateway smart contracts'
	@echo '  ${YELLOW}make config-fhevm-relayer${RESET}        - Configure HTTPZ event listener'
	@echo '  ${YELLOW}make start-fhevm-relayer${RESET}        - Start HTTPZ event listener'
	@echo '  ${YELLOW}make run-test-decrypt${RESET}            - Run decryption tests'
	@echo '  ${YELLOW}make run-test-erc20${RESET}            - Run erc20 tests'
	@echo ''
	@echo 'Run sequence:'
	@echo '  1. ${WHITE}make setup-mock${RESET}                        - First time only'
	@echo '  2. ${WHITE}make run-httpz-mock${RESET}                 - Terminal 1'
	@echo '  3. ${WHITE}make run-gateway-mock${RESET}                 - Terminal 2'
	@echo '  4. ${WHITE}make deploy-httpz-smart-contracts${RESET} - Terminal 3'
	@echo '  5. ${WHITE}make deploy-gateway-smart-contracts${RESET} - Terminal 3'
	@echo '  6. ${WHITE}make config-fhevm-relayer${RESET}        - Terminal 3'
	@echo '  7. ${WHITE}make start-fhevm-relayer${RESET}        - Terminal 3'
	@echo '  8. ${WHITE}make run-test-decrypt${RESET}            - Terminal 4'
	@echo '  9. ${WHITE}make run-test-erc20${RESET}            - Terminal 4'
	@echo ''

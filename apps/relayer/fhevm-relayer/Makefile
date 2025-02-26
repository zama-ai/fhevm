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

# Setup target at each start
setup-mock:
	@echo "$(GREEN)Setting up mock environment...$(RESET)"
	@cd hardhat/contracts && rm -f sql.db
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

# Run Anvil node for httpz chain
run-httpz-compose-mock:
	@echo "$(GREEN)Starting Anvil node for mock HTTPZ chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8756 --chain-id 123456 --accounts 15 --mnemonic $(MNEMONIC) --host 0.0.0.0

# Run Anvil node for gateway chain
run-gateway-compose-mock:
	@echo "$(GREEN)Starting Anvil node for mock GATEWAY chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8757 --chain-id 654321 --accounts 15 --mnemonic $(MNEMONIC) --host 0.0.0.0

# Deploy HTTPZ smart contracts
deploy-httpz-smart-contracts:
	@echo "$(GREEN)Deploying HTTPZ smart contracts...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && sh ./launch-httpz-local-coprocessor.sh

# Deploy GATEWAY smart contracts
deploy-gateway-smart-contracts:
	@echo "$(GREEN)Deploying GATEWAY smart contracts...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && ./launch-gateway-local-coprocessor.sh

# Deploy HTTPZ smart contracts
deploy-httpz-compose-smart-contracts:
	@echo "$(GREEN)Deploying HTTPZ smart contracts...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && sh ./launch-httpz-local-coprocessor.sh composeCoprocessorL1

# Deploy GATEWAY smart contracts
deploy-gateway-compose-smart-contracts:
	@echo "$(GREEN)Deploying GATEWAY smart contracts...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && ./launch-gateway-local-coprocessor.sh composeCoprocessorL2

# Start event listener
start-fhevm-relayer:
	@echo "$(GREEN)Starting HTTPZ event listener...$(RESET)"
	@cargo run --bin fhevm-relayer

# Start kms connector mock
start-gateway-processors-mock:
	@echo "$(GREEN)Starting gateway processors mock...$(RESET)"
	@cargo run --bin gateway-processors-mock

# Run decrypt tests
run-test-decrypt:
	@echo "$(GREEN)Running decrypt tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && npx hardhat compile && npx hardhat compile:specific --contract decryptionOracle && RUST_RELAYER=true npx hardhat test test/gatewayDecrypt/* --network localCoprocessorL1

run-test-input-proof-hardhat:
	@echo "$(GREEN)Running input proof tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && npx hardhat compile && npx hardhat compile:specific --contract decryptionOracle && RUST_RELAYER=true npx hardhat test test/gatewayDecrypt/* --network localCoprocessorL1Input


run-test-input-proof:
	@curl -X POST http://127.0.0.1:3000/input-proof \
		-H "Content-Type: application/json" \
		-d '{"contractChainId": "123456", "contractAddress": "0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d", "userAddress": "0x12B064FB845C1cc05e9493856a1D637a73e944bE", "ciphertextWithZkpok": "abcdef"}'

# Run ERC20 tests
run-test-erc20:
	@echo "$(GREEN)Running decrypt tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && RUST_RELAYER=true npx hardhat test test/encryptedERC20/EncryptedERC20.ts --network localCoprocessorL1


# Run CI test
# We need to run anvil nodes first
# Store PID file
PID_FILE_1 := .anvil1.pid
PID_FILE_2 := .anvil2.pid

# Escape the mnemonic phrase
ESCAPED_MNEMONIC := $(shell echo "$(MNEMONIC)" | sed 's/"/\\"/g')

.PHONY: test
test:
	@echo "Starting test process..."
	@cd hardhat/contracts && cp .env.example .env
	@echo "Starting first Anvil node on port 8756..."
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8756 --chain-id 123456 --accounts 15 --mnemonic "$(ESCAPED_MNEMONIC)" > /dev/null 2>&1 & echo $$! > $(PID_FILE_1)
	@echo "Starting second Anvil node on port 8757..."
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8757 --chain-id 654321 --accounts 15 --mnemonic "$(ESCAPED_MNEMONIC)" > /dev/null 2>&1 & echo $$! > $(PID_FILE_2)
	@echo "Waiting for Anvil nodes to start..." && sleep 2
	@cargo test || (make stop-anvil && exit 1)
	@make stop-anvil

.PHONY: stop-anvil
stop-anvil:
	@echo "Stopping Anvil nodes..."
	@for port in 8756 8757; do \
		pids=$$(lsof -ti:$$port); \
		if [ ! -z "$$pids" ]; then \
			echo "Killing processes on port $$port: $$pids"; \
			kill -9 $$pids; \
		fi \
	done
	@rm -f $(PID_FILE_1) $(PID_FILE_2)
	@echo "Anvil nodes stopped"

# Help message
help:
	@echo ''
	@echo 'Available commands:'
	@echo ''
	@echo '  ${YELLOW}make setup-mock${RESET}                       - Initial setup of the environment'
	@echo '  ${YELLOW}make run-httpz-mock${RESET}                 	- Start Anvil node for httpz chain'
	@echo '  ${YELLOW}make run-gateway-mock${RESET}                 - Start Anvil node for gateway chain'
	@echo '  ${YELLOW}make deploy-gateway-smart-contracts${RESET} 	- Deploy Gateway smart contracts'
	@echo '  ${YELLOW}make deploy-httpz-smart-contracts${RESET} 	- Deploy HTTPZ smart contracts'
	@echo '  ${YELLOW}make config-fhevm-relayer${RESET}        	- Configure HTTPZ event listener'
	@echo '  ${YELLOW}make start-fhevm-relayer${RESET}        	- Start HTTPZ event listener'
	@echo '  ${YELLOW}make start-gateway-processors-mock${RESET}    - Start gateway processors mock (copro + kms)'
	@echo '  ${YELLOW}make run-test-decrypt${RESET}            	- Run decryption tests'
	@echo '  ${YELLOW}make run-test-input-proof${RESET}           	- Run input proof tests'
	@echo '  ${YELLOW}make run-test-erc20${RESET}            	- Run erc20 tests'
	@echo ''
	@echo 'Run sequence:'
	@echo '  1. ${WHITE}make setup-mock${RESET}                      	- At each start'
	@echo '  2. ${WHITE}make run-httpz-mock${RESET}                  	- Terminal 1'
	@echo '  3. ${WHITE}make run-gateway-mock${RESET}                 	- Terminal 2'
	@echo '  4. ${WHITE}make deploy-gateway-smart-contracts${RESET} 	- Terminal 3'
	@echo '  5. ${WHITE}make deploy-httpz-smart-contracts${RESET} 		- Terminal 3'
	@echo '  6. ${WHITE}make config-fhevm-relayer${RESET}        		- Terminal 3'
	@echo '  7. ${WHITE}make start-gateway-processors-mock${RESET}   	- Terminal 4'
	@echo '  8. ${WHITE}make start-fhevm-relayer${RESET}        		- Terminal 5'
	@echo '  9. ${WHITE}make run-test-decrypt${RESET}            		- Terminal 6'
	@echo '  10.${WHITE}make run-test-input-proof${RESET}       		- Terminal 6'
	@echo '  11. ${WHITE}make run-test-erc20${RESET}            		- Terminal 6'
	@echo ''

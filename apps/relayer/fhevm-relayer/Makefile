#.PHONY: all setup run-hardhat deploy-fhevm-smart-contracts start-fhevm-relayer run-decrypt-test help
-include hardhat/contracts/.env

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

# Deploy GATEWAY smart contracts
deploy-gateway-smart-contracts:
	@echo "$(GREEN)Deploying GATEWAY smart contracts...$(RESET)"
	cd mock && ./launch-gateway-local-coprocessor.sh

build-fhevm-relayer:
	@echo "$(GREEN)Starting HTTPZ event listener...$(RESET)"
	@cargo build --bin fhevm-relayer
	@cargo build --bin gateway-processors-mock


start-fhevm-relayer:
	@echo "$(GREEN)Starting HTTPZ event listener...$(RESET)"
	@cargo run --bin fhevm-relayer

start-fhevm-relayer-mock-mode:
	@echo "$(GREEN)Starting HTTPZ event listener...$(RESET)"
	@APP_TRANSACTION__RETRY__MOCK_MODE=true cargo run --bin fhevm-relayer

# Start kms connector mock
start-gateway-processors-mock:
	@echo "$(GREEN)Starting gateway processors mock...$(RESET)"
	@cargo run --bin gateway-processors-mock

run-test-input-proof:
	@curl -s -S -X POST http://127.0.0.1:3000/v1/input-proof \
		-H "Content-Type: application/json" \
		-d '{"contractChainId": "123456", "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1", "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80", "ciphertextWithInputVerification": "abcdef"}'

run-test-user-decrypt:
	@curl -s -S -X POST http://127.0.0.1:3000/v1/user-decrypt \
-H "Content-Type: application/json" \
-d '{"ctHandleContractPairs":[{"ctHandle":"bf9b45c007d626278570aa9622a9c8646f1bfd4e25a5401bd576d15e05320000","contractAddress":"0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"}],"requestValidity":{"startTimestamp":"1742450894","durationDays":"10"},"contractsChainId":"123456","contractAddresses":["0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"],"userAddress":"0xa5e1defb98EFe38EBb2D958CEe052410247F4c80","signature":"f77ca89b541ca80645dfa2822a95354142b73d078429083569d9ec97e23868282a11bc8f2addeac311edbb0d6b4e2763ae1f8e69702f2ddb89ff952dded2c2d61c","publicKey":"2000000000000000127eae823019dbba103069c7d2ee53b16de8a29057911dfd8ba82c25abfb071a"}'

run-test-public-decrypt-http:
	@curl -X POST http://127.0.0.1:3000/v1/public-decrypt \
-H "Content-Type: application/json" \
-d '{"ciphertextHandles": ["0x5a88e7aa46f312ff70df6e84c85eb40cdfd42b18a9ff00000000000030390500"]}'

run-test-keyurl:
	@curl  http://127.0.0.1:3000/v1/keyurl



.PHONY: start-anvil-nodes
start-anvil-nodes:
	@echo "Starting anvil nodes..."
	@cd mock && \
		MNEMONIC_VALUE=$$(grep MNEMONIC .env.gateway | sed -E 's/MNEMONIC="(.*)"/\1/') && \
		MNEMONIC="$$MNEMONIC_VALUE" docker compose up -d
	@echo "Anvil nodes started"

.PHONY: stop-anvil-nodes
stop-anvil-nodes:
	@echo "Stopping anvil nodes..."
	@cd mock && docker compose down
	@echo "Anvil nodes stopped"

run-test-api: ## Run all API tests
	@bash mock/test-api.sh



# Help message
help:
	@echo ''
	@echo 'Available commands:'
	@echo ''
	@echo '  ${YELLOW}make start-anvil-nodes${RESET}                - Start Anvil node for gateway and fhevm'
	@echo '  ${YELLOW}make config-fhevm-relayer${RESET}        		- Configure HTTPZ event listener'
	@echo '  ${YELLOW}make start-fhevm-relayer${RESET}        		- Start relayer'
	@echo '  ${YELLOW}make start-fhevm-relayer-mock-mode${RESET}    - Start relayer mock mode'
	@echo '  ${YELLOW}make start-gateway-processors-mock${RESET}    - Start gateway processors mock (copro + kms)'
	@echo '  ${YELLOW}make run-test-input-proof${RESET}           	- Run input proof tests'
	@echo '  ${YELLOW}make run-test-user-decrypt${RESET}            - Run user decrypt test from curl (failing at user verififcation)'
	@echo '  ${YELLOW}make run-test-keyurl${RESET}            		- Run keyurl tests'
	@echo '  ${YELLOW}make run-test-api${RESET}            		- Run keyurl tests'
	@echo ''

	@echo 'Run sequence:'
	@echo '  1. ${WHITE}make start-anvil-nodes${RESET}                  - Terminal 1'
	@echo '  2. ${WHITE}make config-fhevm-relayer${RESET}               - Terminal 1'
	@echo '  7. ${WHITE}make start-gateway-processors-mock${RESET}   	- Terminal 1'
	@echo '  8. ${WHITE}make start-fhevm-relayer-mock-mode${RESET}      - Terminal 2'
	@echo '  11. ${WHITE}make run-test-input-proof${RESET}       		- Terminal 3'
	@echo '  13. ${WHITE}make run-test-keyurl${RESET}            		- Terminal 3'
	@echo '  15. ${WHITE}make run-test-user-decrypt${RESET}          	- Terminal 3'
	@echo ''

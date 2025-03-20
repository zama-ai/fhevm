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

# Run Anvil node for httpz chain
run-httpz-mock:
	@echo "$(GREEN)Starting Anvil node for mock HTTPZ chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 1 --port 8756 --chain-id 123456 --accounts 20 --mnemonic $(MNEMONIC)

# Run Anvil node for gateway chain
run-gateway-mock:
	@echo "$(GREEN)Starting Anvil node for mock GATEWAY chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 1 --port 8757 --chain-id 654321 --accounts 20 --mnemonic $(MNEMONIC)

# Run Anvil node for httpz chain
run-httpz-compose-mock:
	@echo "$(GREEN)Starting Anvil node for mock HTTPZ chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8756 --chain-id 123456 --accounts 20 --mnemonic $(MNEMONIC) --host 0.0.0.0

# Run Anvil node for gateway chain
run-gateway-compose-mock:
	@echo "$(GREEN)Starting Anvil node for mock GATEWAY chain...$(RESET)"
	@cd hardhat/contracts && anvil --block-time 0.1 --port 8757 --chain-id 654321 --accounts 20 --mnemonic $(MNEMONIC) --host 0.0.0.0

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

# Start python server for serving key and crs files:
start-key-server:
	@python3 -m http.server 3001 -d ./artifacts/test-keys

# Run decrypt tests
run-test-decrypt:
	@echo "$(GREEN)Running decrypt tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && npx hardhat compile && npx hardhat compile:specific --contract decryptionOracle && RUST_RELAYER=true npx hardhat test --grep "test async decrypt uint64$$" --network localCoprocessorL1

run-test-input-proof-hardhat:
	@echo "$(GREEN)Running input proof tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && npx hardhat compile && npx hardhat compile:specific --contract decryptionOracle && RUST_RELAYER=true npx hardhat test --grep "test user input uint64" --network localCoprocessorL1Input

run-test-input-proof-staging:
	@./run-input-proof.sh

run-test-input-proof:
	@curl -X POST http://127.0.0.1:3000/input-proof \
		-H "Content-Type: application/json" \
		-d '{"contractChainId": "123456", "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1", "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80", "ciphertextWithZkpok": "abcdef"}'

run-test-user-decrypt:
	@curl -X POST http://127.0.0.1:3000/user-decrypt \
-H "Content-Type: application/json" \
-d '{"ctHandleContractPairs":[{"ctHandle":"a5e1defb98efe38ebb2d958cee052410247f4c802410247f4c8010247f4c8076","contractAddress":"0xA674ef0e9b1AeBf8b4f5492b7A75d5B577208FCa"}],"requestValidity":{"startTimestamp":"1742407793301","durationDays":"120"},"contractsChainId":"123456","contractAddresses":["0xA674ef0e9b1AeBf8b4f5492b7A75d5B577208FCa"],"userAddress":"0xa44366bAA26296c1409AD1e284264212029F02f1","signature":"f488286b25a610aaff1a178ab0e3b508e5afb5cca448ed91b46ee47145dcdb79334a7ad3862b9f084708664bc78c6e5cab81176b8e231c92712e5ef5d6be90b41c","publicKey":"200000000000000043bcdaeb96544d5513b954982cbba73f4afb975f30507c170fe052a0c5d8f16c"}'


run-test-user-decrypt-from-working-hardhat:
	@curl -X POST http://127.0.0.1:3000/user-decrypt \
-H "Content-Type: application/json" \
-d '{"ctHandleContractPairs":[{"ctHandle":"bf9b45c007d626278570aa9622a9c8646f1bfd4e25a5401bd576d15e05320000","contractAddress":"0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"}],"requestValidity":{"startTimestamp":"1742450894","durationDays":"10"},"contractsChainId":"123456","contractAddresses":["0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"],"userAddress":"0xa5e1defb98EFe38EBb2D958CEe052410247F4c80","signature":"f77ca89b541ca80645dfa2822a95354142b73d078429083569d9ec97e23868282a11bc8f2addeac311edbb0d6b4e2763ae1f8e69702f2ddb89ff952dded2c2d61c","publicKey":"2000000000000000127eae823019dbba103069c7d2ee53b16de8a29057911dfd8ba82c25abfb071a"}'




run-test-user-decrypt-hardhat:
	@echo "$(GREEN)Running input proof tests...$(RESET)"
	@unset $$(cat hardhat/contracts/.env | sed 's/=.*//g') && cd hardhat/contracts && npx hardhat compile && npx hardhat compile:specific --contract decryptionOracle && RUST_RELAYER=true npx hardhat test --grep "test reencrypt ebool" --network localCoprocessorL1Input

run-test-keyurl:
	@curl  http://127.0.0.1:3000/keyurl

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

.PHONY: start-anvil-nodes
start-anvil-nodes:
	@echo "Starting anvil nodes..."
	@cd hardhat/contracts && cp .env.example .env
	@cd hardhat/contracts && \
		MNEMONIC_VALUE=$$(grep MNEMONIC .env | sed -E 's/export MNEMONIC="(.*)"/\1/') && \
		MNEMONIC="$$MNEMONIC_VALUE" docker compose up -d
	@echo "Anvil nodes started"

.PHONY: stop-anvil-nodes
stop-anvil-nodes:
	@echo "Stopping anvil nodes..."
	@cd hardhat/contracts && docker compose down
	@echo "Anvil nodes stopped"

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
	@echo '  ${YELLOW}make start-key-server${RESET}    		- Start key server for serving key and crs files'
	@echo '  ${YELLOW}make run-test-decrypt${RESET}            	- Run decryption tests'
	@echo '  ${YELLOW}make run-test-input-proof${RESET}           	- Run input proof tests'
	@echo '  ${YELLOW}make run-test-erc20${RESET}            	- Run erc20 tests'
	@echo '  ${YELLOW}make run-test-keyurl${RESET}            	- Run keyurl tests'
	@echo '  ${YELLOW}make run-test-input-proof-hardhat${RESET}     - Run input proof tests from hardhat'
	@echo '  ${YELLOW}make run-test-input-proof-staging${RESET}     - Run input proof tests from hardhat for staging'
	@echo '  ${YELLOW}make run-test-decrypt-hardhat${RESET}            	- Run decryption tests'
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
	@echo '  9. ${WHITE}make start-key-server${RESET}            		- Terminal 6'
	@echo '  10. ${WHITE}make run-test-decrypt${RESET}            		- Terminal 7'
	@echo '  11. ${WHITE}make run-test-input-proof${RESET}       		- Terminal 7'
	@echo '  12. ${WHITE}make run-test-erc20${RESET}            		- Terminal 7'
	@echo '  13. ${WHITE}make run-test-keyurl${RESET}            		- Terminal 7'
	@echo '  14. ${WHITE}make run-test-input-proof-hardhat${RESET}     	- Terminal 7'
	@echo '  15. ${WHITE}make run-test-decrypt-hardhat${RESET}          - Terminal 7'
	@echo ''

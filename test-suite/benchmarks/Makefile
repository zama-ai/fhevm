include .env

DECRYPTIONS_PER_BATCH=10
N_BATCH=1
SLEEP_BETWEEN_DECRYPTIONS=0
SLEEP_BETWEEN_BATCHES=1000
DEPLOY_NEW_CONTRACT=false
GENERATE_NEW_HANDLES=false
DEBUG_LOGS=false

# This command uses values from .env, NOT .env.example
public-decrypt-benchmark:
	MNEMONIC=$(MNEMONIC) RELAYER_URL=$(RELAYER_URL) RPC_URL=$(RPC_URL) npx hardhat public-decrypt-benchmark \
	--network $(NETWORK) \
	--decryptions-per-batch $(DECRYPTIONS_PER_BATCH) \
	--n-batch $(N_BATCH) \
	--sleep-between-decryptions $(SLEEP_BETWEEN_DECRYPTIONS) \
	--sleep-between-batches $(SLEEP_BETWEEN_BATCHES) \
	--deploy-new-contract $(DEPLOY_NEW_CONTRACT) \
	--debug-logs $(DEBUG_LOGS)

# This command uses values from .env, NOT .env.example
user-decrypt-benchmark:
	MNEMONIC=$(MNEMONIC) RELAYER_URL=$(RELAYER_URL) RPC_URL=$(RPC_URL) npx hardhat user-decrypt-benchmark \
	--network $(NETWORK) \
	--decryptions-per-batch $(DECRYPTIONS_PER_BATCH) \
	--n-batch $(N_BATCH) \
	--sleep-between-decryptions $(SLEEP_BETWEEN_DECRYPTIONS) \
	--sleep-between-batches $(SLEEP_BETWEEN_BATCHES) \
	--deploy-new-contract $(DEPLOY_NEW_CONTRACT) \
	--generate-new-handles $(GENERATE_NEW_HANDLES) \
	--debug-logs $(DEBUG_LOGS)


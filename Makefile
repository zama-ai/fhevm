TOP := $(dir $(firstword $(MAKEFILE_LIST)))
FHEVM_DEVOPS_PATH ?= $(TOP)external/fhevm-devops

.PHONY: publish-app-deployment-requested
publish-app-deployment-requested:
	aws --endpoint=http://localhost:4566 sns publish \
		--topic-arn 'arn:aws:sns:eu-central-1:000000000000:console-topic' \
		--region eu-central-1 \
		--message '{"_tag": "Event", "type": "app-deployment.requested", "payload": {"applicationId": "test-app", "deploymentId": "depl-id", "address": "0x12345", "chainId": "1"}}'

.PHONY: publish-app-deployment-discover-sc
publish-app-deployment-discover-sc:
	aws --endpoint=http://localhost:4566 sns publish \
		--topic-arn 'arn:aws:sns:eu-central-1:000000000000:console-topic' \
		--region eu-central-1 \
		--message '{"_tag": "Command", "type": "app-deployment.discover-sc", "payload": {"applicationId": "test-app", "deploymentId": "depl-id", "address": "0x278a72ccffee5dc758c1b573ca71f377609e39af", "chainId": "11155111"}}'

# simulate the event of the smart contract being discovered
.PHONY: publish-app-deployment.sc-discovered
publish-app-deployment-sc-discovered:
	aws --endpoint=http://localhost:4566 sns publish \
		--topic-arn 'arn:aws:sns:eu-central-1:000000000000:console-topic' \
		--region eu-central-1 \
		--message '{"_tag": "Event", "type": "app-deployment.sc-discovered", "meta": { "userId": "user_h8I8DmFLwF"}, "payload": {"applicationId": "dapp_cRcSlh0_the9", "deploymentId": "depl-id", "contractAddress": "0x278a72ccffee5dc758c1b573ca71f377609e39af", "creatorAddress": "0x278a72ccffee5dc758c1b573ca71f377609e39af"}}'

# run the blockchaion on docker
blockchain-install:
	bash scripts/install-blockchain-checks.sh && \
	bash scripts/install-blockchain-submodules.sh && \
	$(MAKE) clean-fhevm-devops && \
	$(MAKE) run-fhevm-devops

# test the blockchain
blockchain-test:
	bash scripts/blockchain-test.sh

# display blockchain operations
blockchain-listen:
	bash scripts/blockchain-listen.sh

run-fhevm-devops:
	$(MAKE) -C $(FHEVM_DEVOPS_PATH)/coprocessor run-full

clean-fhevm-devops:
	$(MAKE) -C $(FHEVM_DEVOPS_PATH)/coprocessor clean
	
# Hardhat
# First launch hardhat node
hardhat-run:
	bash scripts/hardhat-run.sh
# Then launch events listener	
hardhat-listen:
	bash scripts/hardhat-listen.sh
# Then launch some tests
hardhat-test:
	bash scripts/hardhat-test.sh

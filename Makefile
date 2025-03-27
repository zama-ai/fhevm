TOP := $(dir $(firstword $(MAKEFILE_LIST)))

.PHONY: publish-app-deployment-requested
publish-app-deployment-requested:
	aws --endpoint=http://localhost:4566 sqs send-message \
		--queue-url 'http://localhost:4566/000000000000/back-queue' \
		--region eu-central-1 \
		--message-body '{"_tag": "Event", "type": "app-deployment.requested", "payload": {"applicationId": "test-app", "deploymentId": "depl-id", "address": "0x12345", "chainId": "1"}}'

.PHONY: publish-app-deployment-discover-sc
publish-app-deployment-discover-sc:
	aws --endpoint=http://localhost:4566 sqs send-message \
		--queue-url 'http://localhost:4566/000000000000/back-queue' \
		--region eu-central-1 \
		--message '{"_tag": "Command", "type": "app-deployment.discover-sc", "payload": {"applicationId": "test-app", "deploymentId": "depl-id", "address": "0x278a72ccffee5dc758c1b573ca71f377609e39af", "chainId": "11155111"}}'

# simulate the event of the smart contract being discovered
.PHONY: publish-app-deployment.sc-discovered
publish-app-deployment-sc-discovered:
	aws --endpoint=http://localhost:4566 sqs send-message \
		--queue-url 'http://localhost:4566/000000000000/back-queue' \
		--region eu-central-1 \
		--message-body '{"_tag": "Event", "type": "app-deployment.sc-discovered", "meta": { "userId": "user_h8I8DmFLwF"}, "payload": {"applicationId": "dapp_cRcSlh0_the9", "deploymentId": "depl-id", "contractAddress": "0x278a72ccffee5dc758c1b573ca71f377609e39af", "creatorAddress": "0x278a72ccffee5dc758c1b573ca71f377609e39af"}}'

.PHONY: publish-back-dapp-stats-requests
publish-back-dapp-stats-requests:
	aws --endpoint=http://localhost:4566 sqs send-message \
		--queue-url 'http://localhost:4566/000000000000/back-queue' \
		--region eu-central-1 \
		--message-body '{"type": "back:dapp:stats-requested", "payload": {"chainId": "123456", "address": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80"}, "meta": {"correlationId": "ea0ca1c2-3fde-4f80-8abb-08aecee4107c"}}' 

.PHONY: publish-back-dapp-stats-available
publish-back-dapp-stats-available:
	aws --endpoint=http://localhost:4566 sqs send-message-batch \
		--queue-url 'http://localhost:4566/000000000000/back-queue' \
		--region eu-central-1 \
		--publish-batch-request-entries '[ \
			{"Id": "evt1", "Message": "{\"type\": \"back:dapp:stats-available\", \"payload\": {\"chainId\": \"123456\", \"address\": \"0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee43\", \"name\": \"FheAdd\", \"timestamp\": \"2022-01-01T00:00:00.000Z\", \"externalRef\": \"123456\"}, \"meta\": {\"correlationId\": \"ea0ca1c2-3fde-4f80-8abb-08aecee4107c\"}}"}, \
			{"Id": "evt2", "Message": "{\"type\": \"back:dapp:stats-available\", \"payload\": {\"chainId\": \"123456\", \"address\": \"0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee43\", \"name\": \"FheAdd\", \"timestamp\": \"2022-01-01T00:00:00.000Z\", \"externalRef\": \"12346\"}, \"meta\": {\"correlationId\": \"ea0ca1c2-3fde-4f80-8abb-08aecee4107c\"}}"}, \
			{"Id": "evt3", "Message": "{\"type\": \"back:dapp:stats-available\", \"payload\": {\"chainId\": \"123456\", \"address\": \"0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee43\", \"name\": \"FheAdd\", \"timestamp\": \"2022-01-01T00:00:00.000Z\", \"externalRef\": \"12347\"}, \"meta\": {\"correlationId\": \"ea0ca1c2-3fde-4f80-8abb-08aecee4107c\"}}"} \
		]'

publish-web3-fhe-event-requested:
	aws --endpoint=http://localhost:4566 sqs send-message \
		--queue-url 'http://localhost:4566/000000000000/web3-queue' \
		--region eu-central-1 \
		--message-body '{"type": "web3:fhe-event:detected", "payload": {"chainId": "123456", "address": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80"}, "meta": {"correlationId": "ea0ca1c2-3fde-4f80-8abb-08aecee4107c"}}'

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
	
# Hardhat
# First launch node and deploy contracts
httpz-run:
	bash scripts/httpz-run.sh
	
# Then launch some public decryption test
httpz-test-public-decrypt:
	bash scripts/httpz-test-public-decrypt.sh

httpz-test-private-decrypt:
	bash scripts/httpz-test-private-decrypt.sh

httpz-test-input:
	bash scripts/httpz-test-input.sh

# Then clean nodes
httpz-clean:
	bash scripts/httpz-clean.sh

console-side-clean:
	docker compose down --volumes --remove-orphans

console-side-run:
	docker compose up -d --wait

relayer-run:
	cd $(TOP)apps/relayer && cargo run --bin zws-relayer

relayer-run-debug:
	cd $(TOP)apps/relayer && cargo run --bin zws-relayer -- --config-file debug.toml

# `--ssh default` used to forward ssh agent to allow fhevm-relayer dependency to be reached
docker-compose-build:
	docker compose -f $(TOP)docker/docker-compose.yaml build --ssh default

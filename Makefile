TOP := $(dir $(firstword $(MAKEFILE_LIST)))

# simulate the event of the smart contract address being validated
.PHONY: back-address-validation-confirmed
back-address-validation-confirmed:
	aws --endpoint=http://localhost:4566 sqs send-message \
		--queue-url 'http://localhost:4566/000000000000/back-queue' \
		--region eu-central-1 \
		--message-body '{"_tag": "Event", "type": "back:address:validation:confirmed", "meta": {"correlationId":"a68ccede-f794-44c4-9c12-78a80418b78d"}, "payload": {"requestId": "01966856-a84e-7452-add1-5fa9b23ec93f", "address": "0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee99", "chainId": "11155111"}}'


.PHONY: publish-back-dapp-stats-requests
publish-back-dapp-stats-requests:
	aws --endpoint=http://localhost:4566 sqs send-message \
		--queue-url 'http://localhost:4566/000000000000/orchestrator-queue' \
		--region eu-central-1 \
		--message-body '{"type": "back:dapp:stats-requested", "payload": {"requestId":"ea0ca1c2-3fde-4f80-8abb-08aecee4107c", "dAppId": "dapp_eBNtPYLsxFUI", "chainId": "123456", "address": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80"}, "meta": {"correlationId": "ea0ca1c2-3fde-4f80-8abb-08aecee4107c"}}' 

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
	
# fhevm
.PHONY: fhevm-up fhevm-down
fhevm-up:
	bash scripts/fhevm-up.sh
fhevm-down:
	bash scripts/fhevm-down.sh
	
# fhevm Tests
.PHONY: fhevm-test-public-decrypt fhevm-test-private-decrypt fhevm-test-input
fhevm-test-public-decrypt:
	bash scripts/fhevm-test-public-decrypt.sh

fhevm-test-private-decrypt:
	bash scripts/fhevm-test-private-decrypt.sh

fhevm-test-input:
	bash scripts/fhevm-test-input.sh

# Console + Docker
.PHONY: console-build console-up console-down console-infra-up console-infra-down console-build-service console-up-service
console-build:
	docker compose -f ./docker-compose.02.console.build.yaml -f ./docker-compose.04.console.ghcr.yaml -f ./docker-compose.04.console.migrate.ghcr.yaml build

console-up:
	bash scripts/console-up.sh

console-down:
	docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.03.console.run.yaml -p console down --volumes --remove-orphans

console-infra-up:
	docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.04.console.migrate.ghcr.yaml -p console up -d --wait

console-infra-down:
	docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.04.console.migrate.ghcr.yaml -p console down --volumes --remove-orphans

console-build-service:
	docker compose -f ./docker-compose.02.console.build.yaml -f ./docker-compose.04.console.ghcr.yaml -f ./docker-compose.04.console.migrate.ghcr.yaml build $(service-name)

console-up-service:
	docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.03.console.run.yaml -f docker-compose.04.console.ghcr.yaml -f docker-compose.04.console.migrate.ghcr.yaml -p console up -d --wait --remove-orphans $(service-name)

# Relayer
.PHONY: relayer-run relayer-build relayer-run-debug relayer-lint
relayer-run:
	cd $(TOP)apps/relayer && cargo run --bin zws-relayer

relayer-build:
	cd $(TOP)apps/relayer && cargo build --bin zws-relayer

relayer-lint:
	cd $(TOP)apps/relayer && cargo clippy --all-targets --all-features --workspace --exclude fhevm-relayer -- -D warnings

relayer-run-debug:
	cd $(TOP)apps/relayer && cargo run --bin zws-relayer -- --config-file debug.toml

.PHONY: down
down: fhevm-down console-down

.PHONY: build-and-up
build-and-up: console-build down
	$(MAKE) fhevm-up
	$(MAKE) console-up

.PHONY: test
test:
	$(MAKE) fhevm-test-input
	$(MAKE) fhevm-test-private-decrypt
	$(MAKE) fhevm-test-public-decrypt

all: 
	$(MAKE) build-and-up
	echo "Waiting to make sure that everything is ready (all healthcheck are not implemented in the Console stack)"
	sleep 10
	$(MAKE) test





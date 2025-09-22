TOP := $(dir $(firstword $(MAKEFILE_LIST)))

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
	docker compose --progress=plain -f ./docker-compose.02.console.build.yaml -f ./docker-compose.04.console.ghcr.yaml build

console-up:
	bash scripts/console-up.sh

console-down:
	docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.run.yaml -p console down --volumes --remove-orphans

console-infra-up:
	docker compose -f ./docker-compose.01.infra.yaml -p console up -d --wait

console-infra-down:
	docker compose -f ./docker-compose.01.infra.yaml -p console down --volumes --remove-orphans

console-build-service:
	docker compose -f ./docker-compose.02.console.build.yaml -f ./docker-compose.04.console.ghcr.yaml build $(service-name)

console-up-service:
	docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.run.yaml -f docker-compose.04.console.ghcr.yaml -f docker-compose.04.console.migrate.ghcr.yaml -p console up -d --wait --remove-orphans $(service-name)

# Relayer
.PHONY: relayer-run relayer-build relayer-run-debug relayer-lint
zws-relayer-run:
	cd $(TOP)apps/relayer && cargo run --bin zws-relayer

zws-relayer-build:
	cd $(TOP)apps/relayer && cargo build --bin zws-relayer

zws-relayer-lint:
	cd $(TOP)apps/relayer && cargo clippy --all-targets --all-features --workspace --exclude fhevm-relayer -- -D warnings

zws-relayer-run-debug:
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

relayer-debug-all:
	DEBUG=1 RELAYER_URL=http://console-relayer:4324 $(MAKE) all

relayer-debug-test:
	DEBUG=1 RELAYER_URL=http://console-relayer:4324 $(MAKE) test

relayer-debug-console-up:
	DEBUG=1 RELAYER_URL=http://console-relayer:4324 $(MAKE) console-up







#!/usr/bin/make -f

# Load environment variables to makefile and export it to shell environment
include .env
export $(shell cat .env | sed 's/=.*//g')

BINDIR ?= $(GOPATH)/bin
ETHERMINT_BINARY = ethermintd
ETHERMINT_DIR = ethermint
BUILDDIR ?= $(CURDIR)/build

WORKDIR ?= $(CURDIR)/work_dir
SUDO := $(shell which sudo)

OS := $(shell uname -s)

ifeq ($(OS),Linux)
    IS_LINUX := true
else
    IS_LINUX := false
endif

check_os:
	@echo "Operating System: $(OS)"
	@if [ "$(IS_LINUX)" = "true" ]; then \
	    echo "This is a Linux system."; \
	else \
	    echo "This is not a Linux system."; \
	fi



# This version must the same as in docker-compose-full.yml
# TODO add check
KMS_DEV_VERSION ?= $(DOCKER_IMAGES_TAG)
FHEVM_SOLIDITY_REPO ?= fhevm
FHEVM_SOLIDITY_PATH ?= $(WORKDIR)/$(FHEVM_SOLIDITY_REPO)
FHEVM_SOLIDITY_PATH_EXISTS := $(shell test -d $(FHEVM_SOLIDITY_PATH)/.git && echo "true" || echo "false")
FHEVM_SOLIDITY_VERSION ?= v0.6.0-2-test

export GO111MODULE = on

# Default target executed when no arguments are given to make.
default_target: all

.PHONY: default_target

# process build tags



###############################################################################
###                                Single validator                         ###
###############################################################################


$(WORKDIR)/:
	$(info WORKDIR)
	mkdir -p $(WORKDIR)

clone-fhevm-solidity: $(WORKDIR)/
	$(info Cloning fhevm-solidity version $(FHEVM_SOLIDITY_VERSION))
	cd $(WORKDIR) && git clone git@github.com:zama-ai/fhevm.git
	cd $(FHEVM_SOLIDITY_PATH) && git checkout $(FHEVM_SOLIDITY_VERSION)

check-fhevm-solidity: $(WORKDIR)/
	$(info check-fhevm-solidity)
ifeq ($(FHEVM_SOLIDITY_PATH_EXISTS), true)
	@echo "fhevm-solidity exists in $(FHEVM_SOLIDITY_PATH)"
	@if [ ! -d $(WORKDIR)/fhevm ]; then \
        echo 'fhevm-solidity is not available in $(WORKDIR)'; \
        echo "FHEVM_SOLIDITY_PATH is set to a custom value"; \
    else \
        echo 'fhevm-solidity is already available in $(WORKDIR)'; \
    fi
else
	@echo "fhevm does not exist"
	echo "We clone it for you!"
	echo "If you want your own version please update FHEVM_SOLIDITY_PATH pointing to your fhevm-solidity folder!"
	$(MAKE) clone-fhevm-solidity
endif


init-db: check-all-test-repo  
ifeq ($(CENTRALIZED_KMS),false)
	@echo "CENTRALIZED_KMS is false, we are extracting keys from kms-core-1"
	$(MAKE) copy-keys-threshold-key-gen
else ifeq ($(CENTRALIZED_KMS),true)
	@echo "CENTRALIZED_KMS is true, copying fhe keys from dev image"
	$(MAKE) copy-keys-centralized-key-gen
else
	@echo "CENTRALIZED_KMS is set to an unrecognized value: $(CENTRALIZED_KMS)"
endif
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv -f docker-compose/docker-compose-setup-fhevm-coprocessor-db.yml up -d --wait || true
	@if [ "$$(docker inspect -f '{{.State.ExitCode}}' $(DOCKER_COMPOSE_PROJECT)-setup-fhevm-coprocessor-db-1)" -eq 0 ]; then \
		echo "Container exited with code 0, proceeding..."; \
	else \
		echo "Container did not exit successfully, failing..."; \
		exit 1; \
	fi


setup-fhevm-contracts:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv -f docker-compose/docker-compose-setup-fhevm-contracts.yml up -d --wait || true
	@if [ "$$(docker inspect -f '{{.State.ExitCode}}' zama-setup-fhevm-contracts-1)" -eq 0 ]; then \
		echo "Container exited with code 0, proceeding..."; \
	else \
		echo "Container did not exit successfully, failing..."; \
		exit 1; \
	fi

check-all-test-repo: check-fhevm-solidity


copy-keys-threshold-key-gen:
	@bash ./scripts/copy_fhe_keys_threshold_key_gen.sh $(PWD)/network-fhe-keys
	@bash ./scripts/update_signers.sh $(PWD)/env/.env.example.deployment $(PWD)/network-fhe-keys 4

copy-keys-centralized-key-gen:
	@bash ./scripts/copy_fhe_keys_centralized_key_gen.sh $(PWD)/network-fhe-keys
	@bash ./scripts/update_signers.sh $(PWD)/env/.env.example.deployment $(PWD)/network-fhe-keys 1


trigger-key-gen-threshold:
	cargo run --bin simulator -- \
		-f config/local_threshold.toml \
		insecure-key-gen

trigger-crs-gen-threshold:
	cargo run --bin simulator -- \
		-f config/local_threshold.toml \
		insecure-crs-gen --max-num-bits 256

run-kms-threshold:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-threshold.yml \
		up -d --wait

run-kms-threshold-with-gateway:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-threshold.yml \
		-f docker-compose/docker-compose-kms-gateway-threshold.yml \
		-f docker-compose/docker-compose-fhevm-coprocessor.yml \
		up -d --wait

stop-kms-threshold:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-threshold.yml \
		down --volumes --remove-orphans

run-kms-centralized:
	docker -p $(DOCKER_COMPOSE_PROJECT) compose -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-centralized.yml \
		up -d --wait

stop-kms-centralized:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-centralized.yml \
		down

run-kms-centralized-with-gateway:
	docker compose  -p $(DOCKER_COMPOSE_PROJECT) -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-centralized.yml \
		-f docker-compose/docker-compose-kms-gateway-centralized.yml \
		-f docker-compose/docker-compose-fhevm-coprocessor.yml \
		up -d --wait

stop-kms-centralized-with-gateway:
	docker compose  -p $(DOCKER_COMPOSE_PROJECT) -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-centralized.yml \
		-f docker-compose/docker-compose-kms-gateway-centralized.yml \
		-f docker-compose/docker-compose-fhevm-coprocessor.yml \
		down -v --remove-orphans

stop-kms-threshold-with-gateway:
	docker compose  -p $(DOCKER_COMPOSE_PROJECT) -vvv \
		-f docker-compose/docker-compose-kms-base.yml \
		-f docker-compose/docker-compose-kms-threshold.yml \
		-f docker-compose/docker-compose-kms-gateway-threshold.yml \
		-f docker-compose/docker-compose-fhevm-coprocessor.yml \
		down -v --remove-orphans


run-kms:
ifeq ($(CENTRALIZED_KMS),true)
	@echo "CENTRALIZED_KMS is true, running centralized KMS...."
	sleep 2
	$(MAKE) run-kms-centralized-with-gateway
	
else ifeq ($(CENTRALIZED_KMS),false)
	@echo "CENTRALIZED_KMS is false, running threshold KMS...."
	sleep 2
	$(MAKE) run-kms-threshold-with-gateway
else
	@echo "CENTRALIZED_KMS is set to an unrecognized value: $(CENTRALIZED_KMS)"
endif



stop-kms:
ifeq ($(CENTRALIZED_KMS),true)
	@echo "CENTRALIZED_KMS is true, Stopping centralized KMS...."
	$(MAKE) stop-kms-centralized-with-gateway
	
else ifeq ($(CENTRALIZED_KMS),false)
	@echo "CENTRALIZED_KMS is false, Stopping threshold KMS...."
	$(MAKE) stop-kms-threshold-with-gateway
else
	@echo "CENTRALIZED_KMS is set to an unrecognized value: $(CENTRALIZED_KMS)"
endif

TEST_FILE := run_tests.sh
TEST_IF_FROM_REGISTRY :=

run-e2e-test: check-all-test-repo
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test


install-packages:
	@cd $(FHEVM_SOLIDITY_PATH) && npm i
	@if [ "$(IS_LINUX)" = "true" ]; then \
	    cd $(FHEVM_SOLIDITY_PATH) && npm i solidity-comments-linux-x64-gnu; \
	fi



prepare-e2e-test: check-all-test-repo
	@bash scripts/fund_tests_addresses_docker.sh
	$(MAKE) setup-fhevm-contracts
	@cp ./scripts/hardhat-compile.sh $(FHEVM_SOLIDITY_PATH)
	@cp ./env/.env.example.deployment $(FHEVM_SOLIDITY_PATH)/.env
	@cd $(FHEVM_SOLIDITY_PATH) &&  ./hardhat-compile.sh
	

run-async-test:
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint64' --network localCoprocessor


run-true-input-async-test:
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint64 non-trivial'

run-all-tests:
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt bool$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint4$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint8$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint16$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint32$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint64$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint128$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint256$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt address$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt mixed$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt bool$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt uint4$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt uint8$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt uint16$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt uint32$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt uint64$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt uint128$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt uint256$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test reencrypt address$'
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint64 non-trivial'


e2e-test:
	@$(MAKE) check-all-test-repo
	$(MAKE) run-full
	$(MAKE) prepare-e2e-test
	$(MAKE) run-e2e-test
	$(MAKE) stop-full


clean-keys:
	$(MAKE) stop-kms
	rm -rf network-fhe-keys
	rm -rf kms-fhe-keys
	rm -rf res


clean:
	$(MAKE) stop-kms
	rm -rf $(WORKDIR)/
	rm -rf network-fhe-keys
	rm -rf kms-fhe-keys
	rm -rf res


print-info:
	@echo 'KMS_DEV_VERSION: $(KMS_DEV_VERSION) for KEY_GEN---extracted from Makefile'
	@echo 'FHEVM_SOLIDITY_VERSION: $(FHEVM_SOLIDITY_VERSION) ---extracted from Makefile'
	@bash scripts/get_repository_info.sh fhevm $(FHEVM_SOLIDITY_PATH)



run-gw: 
	@docker compose -vvv -f docker-compose/docker-compose-kms-base.yml -f docker-compose/docker-compose-kms-centralized.yml -f docker-compose/docker-compose-kms-gateway-centralized.yml up dev-kms-gateway

restart-kv:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv \
    -f docker-compose/docker-compose-kms-base.yml \
    -f docker-compose/docker-compose-kms-threshold.yml \
    -f docker-compose/docker-compose-kms-gateway-threshold.yml \
    -f docker-compose/docker-compose-fhevm-coprocessor.yml \
    up -d --no-deps dev-kv-store


restart-coproc:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv \
    -f docker-compose/docker-compose-kms-base.yml \
    -f docker-compose/docker-compose-kms-threshold.yml \
    -f docker-compose/docker-compose-kms-gateway-threshold.yml \
    -f docker-compose/docker-compose-fhevm-coprocessor.yml \
    up -d --no-deps coproc


restart-gw:
	docker compose -p $(DOCKER_COMPOSE_PROJECT) -vvv \
    -f docker-compose/docker-compose-kms-base.yml \
    -f docker-compose/docker-compose-kms-threshold.yml \
    -f docker-compose/docker-compose-kms-gateway-threshold.yml \
    -f docker-compose/docker-compose-fhevm-coprocessor.yml \
    up -d --no-deps dev-kms-gateway


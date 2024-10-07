#!/usr/bin/make -f

include .env


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
KMS_DEV_VERSION ?= v0.9.0-rc11-test-signature

FHEVM_SOLIDITY_REPO ?= fhevm
FHEVM_SOLIDITY_PATH ?= $(WORKDIR)/$(FHEVM_SOLIDITY_REPO)
FHEVM_SOLIDITY_PATH_EXISTS := $(shell test -d $(FHEVM_SOLIDITY_PATH)/.git && echo "true" || echo "false")
FHEVM_SOLIDITY_VERSION ?= v0.5.4-0

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
	cd $(WORKDIR) && git clone https://github.com/zama-ai/fhevm.git
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
	@echo "fhevm-solidity does not exist"
	echo "We clone it for you!"
	echo "If you want your own version please update FHEVM_SOLIDITY_PATH pointing to your fhevm-solidity folder!"
	$(MAKE) clone-fhevm-solidity
endif


check-all-test-repo: check-fhevm-solidity


generate-fhe-keys-registry-dev-image:
ifeq ($(KEY_GEN),false)
	@echo "KEY_GEN is false, executing corresponding commands..."
	@bash ./scripts/copy_fhe_keys.sh $(KMS_DEV_VERSION) $(PWD)/network-fhe-keys $(PWD)/kms-fhe-keys
else ifeq ($(KEY_GEN),true)
	@echo "KEY_GEN is true, executing corresponding commands..."
	@bash ./scripts/prepare_volumes_from_kms_core.sh $(KMS_DEV_VERSION) $(PWD)/network-fhe-keys $(PWD)/kms-fhe-keys
else
	@echo "KEY_GEN is set to an unrecognized value: $(KEY_GEN)"
endif


run-full:
	$(MAKE) generate-fhe-keys-registry-dev-image
ifeq ($(KEY_GEN),false)
	@echo "KEY_GEN is false, executing corresponding commands..."
	@docker compose  -f docker-compose/docker-compose-full.yml  up --detach
else ifeq ($(KEY_GEN),true)
	@echo "KEY_GEN is true, mounting fhe keys into kms-core..."
	@docker compose  -f docker-compose/docker-compose-full.yml -f docker-compose/docker-compose-full.override.yml up --detach
else
	@echo "KEY_GEN is set to an unrecognized value: $(KEY_GEN)"
endif

	@echo 'sleep a little to let the docker start up'
	sleep 5

stop-full:
	@docker compose  -f docker-compose/docker-compose-full.yml down

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
	$(MAKE) install-packages
	@sleep 5
	@./scripts/fund_test_addresses_docker.sh
	@cd $(FHEVM_SOLIDITY_PATH) && cp .env.example .env
	@cd $(FHEVM_SOLIDITY_PATH) && ./setup-local-fhevm.sh

run-async-test:
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint8'

run-true-input-async-test:
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint64 non-trivial'

e2e-test:
	@$(MAKE) check-all-test-repo
	$(MAKE) run-full
	$(MAKE) prepare-e2e-test
	$(MAKE) run-e2e-test
	$(MAKE) stop-full



clean:
	$(MAKE) stop-full
	rm -rf $(BUILDDIR)/
	rm -rf $(WORKDIR)/
	rm -rf network-fhe-keys
	rm -rf kms-fhe-keys
	rm -rf res


print-info:
	@echo 'KMS_DEV_VERSION: $(KMS_DEV_VERSION) for KEY_GEN---extracted from Makefile'
	@echo 'FHEVM_SOLIDITY_VERSION: $(FHEVM_SOLIDITY_VERSION) ---extracted from Makefile'
	@bash scripts/get_repository_info.sh fhevm $(FHEVM_SOLIDITY_PATH)

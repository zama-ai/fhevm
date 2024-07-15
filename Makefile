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
KMS_DEV_VERSION ?= v0.7.1

FHEVM_SOLIDITY_REPO ?= fhevm
FHEVM_SOLIDITY_PATH ?= $(WORKDIR)/$(FHEVM_SOLIDITY_REPO)
FHEVM_SOLIDITY_PATH_EXISTS := $(shell test -d $(FHEVM_SOLIDITY_PATH)/.git && echo "true" || echo "false")
FHEVM_SOLIDITY_VERSION ?= v0.5.2

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


change-running-node-owner:
	@$(SUDO) chown -R $(USER): running_node/


init-ethermint-node:
	@$(MAKE) init-ethermint-node-from-registry

init-ethermint-node-from-registry:
	$(MAKE) generate-fhe-keys-registry-dev-image

generate-fhe-keys-registry:
ifeq ($(KEY_GEN),false)
	@echo "KEY_GEN is false, executing corresponding commands..."
	@bash ./scripts/copy_fhe_keys.sh $(KMS_DEV_VERSION) $(PWD)/running_node/node2/.ethermintd/zama/keys/network-fhe-keys $(PWD)/running_node/node2/.ethermintd/zama/keys/kms-fhe-keys
else ifeq ($(KEY_GEN),true)
	@echo "KEY_GEN is true, executing corresponding commands..."
	@bash ./scripts/prepare_volumes_from_kms_core.sh $(KMS_DEV_VERSION) $(PWD)/running_node/node2/.ethermintd/zama/keys/network-fhe-keys $(PWD)/running_node/node2/.ethermintd/zama/keys/kms-fhe-keys
else
	@echo "KEY_GEN is set to an unrecognized value: $(KEY_GEN)"
endif

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
	@docker compose  -f docker-compose/docker-compose-full.yml  up --detach
	@echo 'sleep a little to let the docker start up'
	sleep 10

stop-full:
	@docker compose  -f docker-compose/docker-compose-full.yml down

TEST_FILE := run_tests.sh
TEST_IF_FROM_REGISTRY := 

run-e2e-test: check-all-test-repo
	$(MAKE) prepare-e2e-test
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
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test

run-async-test:
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint8' 

run-true-input-async-test:
	@cd $(FHEVM_SOLIDITY_PATH) && npx hardhat test --grep 'test async decrypt uint64 non-trivial' 

e2e-test:
	@$(MAKE) check-all-test-repo
	@$(MAKE) init-ethermint-node-from-registry
	$(MAKE) run-full
	$(MAKE) run-e2e-test
	$(MAKE) stop-full


clean-node-storage:
	@echo 'clean node storage'
	sudo rm -rf running_node

clean: clean-node-storage
	$(MAKE) stop-full
	rm -rf $(BUILDDIR)/
	rm -rf $(WORKDIR)/ 


print-info:
	@echo 'KMS_DEV_VERSION: $(KMS_DEV_VERSION) for KEY_GEN---extracted from Makefile'
	@echo 'FHEVM_SOLIDITY_VERSION: $(FHEVM_SOLIDITY_VERSION) ---extracted from Makefile'
	@bash scripts/get_repository_info.sh fhevm $(FHEVM_SOLIDITY_PATH)


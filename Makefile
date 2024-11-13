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
KMS_DEV_VERSION ?= $(DOCKER_IMAGES_TAG)

FHEVM_SOLIDITY_REPO ?= fhevm
FHEVM_SOLIDITY_PATH ?= $(WORKDIR)/$(FHEVM_SOLIDITY_REPO)
FHEVM_SOLIDITY_PATH_EXISTS := $(shell test -d $(FHEVM_SOLIDITY_PATH)/.git && echo "true" || echo "false")
FHEVM_SOLIDITY_VERSION ?= fhevm/non-trivial
COPROCESSOR_REPO ?= fhevm-backend
COPROCESSOR_PATH ?= $(WORKDIR)/$(COPROCESSOR_REPO)
COPROCESSOR_PATH_EXISTS := $(shell test -d $(COPROCESSOR_PATH)/.git && echo "true" || echo "false")
COPROCESSOR_VERSION ?= mano/update-to-latest-fhevm

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

clone-coprocessor: $(WORKDIR)/ 
	$(info Cloning coprocessor version $(COPROCESSOR_VERSION))
	cd $(WORKDIR) && git clone git@github.com:zama-ai/fhevm-backend.git
	cd $(COPROCESSOR_PATH) && git checkout $(COPROCESSOR_VERSION)


init-db:
	$(MAKE) copy-keys-threshold-key-gen
	cp -v network-fhe-keys/* $(COPROCESSOR_PATH)/fhevm-engine/fhevm-keys
	cd $(COPROCESSOR_PATH)/fhevm-engine/coprocessor && make init



run-coprocessor: $(WORKDIR)/ check-coprocessor check-all-test-repo
ifeq ($(CENTRALIZED_KMS),false)
	@echo "CENTRALIZED_KMS is false, we are extracting keys from kms-core-1"
	
	
else ifeq ($(CENTRALIZED_KMS),true)
	@echo "CENTRALIZED_KMS is true, copying fhe keys from dev image"
	$(MAKE) generate-fhe-keys-registry-dev-image
else
	@echo "CENTRALIZED_KMS is set to an unrecognized value: $(CENTRALIZED_KMS)"
	cp -v network-fhe-keys/* $(COPROCESSOR_PATH)/fhevm-engine/fhevm-keys
endif
	cd $(COPROCESSOR_PATH)/fhevm-engine/coprocessor && make cleanup
	cd $(COPROCESSOR_PATH)/fhevm-engine/coprocessor && cargo install sqlx-cli
	cd $(COPROCESSOR_PATH)/fhevm-engine/coprocessor && make run

stop-coprocessor: $(WORKDIR)/ 
	cd $(COPROCESSOR_PATH)/fhevm-engine/coprocessor && make cleanup


check-coprocessor: $(WORKDIR)/
	$(info check-coprocessor)
ifeq ($(COPROCESSOR_PATH_EXISTS), true)
	@echo "coprocessor exists in $(COPROCESSOR_PATH)"
	@if [ ! -d $(WORKDIR)/fhevm ]; then \
        echo 'coprocessor is not available in $(WORKDIR)'; \
        echo "COPROCESSOR_PATH is set to a custom value"; \
    else \
        echo 'coprocessor is already available in $(WORKDIR)'; \
    fi
else
	@echo "coprocessor does not exist"
	echo "We clone it for you!"
	echo "If you want your own version please update COPROCESSOR_PATH pointing to your coprocessor folder!"
	$(MAKE) clone-coprocessor
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

copy-keys-threshold:
	@bash ./scripts/copy_fhe_keys_threshold.sh zama-kms-threshold-dev-kms-core-1-1 $(PWD)/network-fhe-keys
	@bash ./scripts/update_signers.sh $(PWD)/work_dir/fhevm/.env.example.deployment $(PWD)/network-fhe-keys

copy-keys-threshold-key-gen:
	@bash ./scripts/copy_fhe_keys_threshold_key_gen.sh $(PWD)/network-fhe-keys
	@bash ./scripts/update_signers.sh $(PWD)/work_dir/fhevm/.env.example.deployment $(PWD)/network-fhe-keys 

run-full:
	@echo "Running kms"
	$(MAKE) run-kms
	
	@echo "Running co-processor"
	$(MAKE) run-coprocessor
	
	@echo "Predeployment of SCs"
	$(MAKE) prepare-e2e-test


stop-full:
	$(MAKE) stop-kms
	$(MAKE) stop-coprocessor


trigger-key-gen-threshold:
	cargo run --bin simulator -- -f config/local_threshold.toml insecure-key-gen

trigger-crs-gen-threshold:
	cargo run --bin simulator -- -f config/local_threshold.toml crs-gen --max-num-bits 256

run-kms-threshold:
	docker compose -vvv -f docker-compose/docker-compose-kms-base.yml -f docker-compose/docker-compose-kms-threshold.yml up -d --wait

run-kms-threshold-with-gateway:
	docker compose -vvv -f docker-compose/docker-compose-kms-base.yml -f docker-compose/docker-compose-kms-threshold.yml -f docker-compose/docker-compose-kms-gateway-threshold.yml up -d --wait

stop-kms-threshold:
	docker compose -vvv -f docker-compose/docker-compose-kms-base.yml -f docker-compose/docker-compose-kms-threshold.yml down --volumes --remove-orphans

run-kms-centralized:
	docker compose -vvv -f docker-compose/docker-compose-kms-base.yml -f docker-compose/docker-compose-kms-centralized.yml up -d --wait

run-kms-centralized-with-gateway:
	docker compose -vvv -f docker-compose/docker-compose-kms-base.yml -f docker-compose/docker-compose-kms-centralized.yml -f docker-compose/docker-compose-kms-gateway-centralized.yml up -d --wait

run-kms:
ifeq ($(CENTRALIZED_KMS),true)
	@echo "CENTRALIZED_KMS is true, running centralized KMS...."
	sleep 2
	$(MAKE) run-kms-centralized
	
else ifeq ($(CENTRALIZED_KMS),false)
	@echo "CENTRALIZED_KMS is false, running threshold KMS...."
	sleep 2
	$(MAKE) run-kms-threshold
else
	@echo "CENTRALIZED_KMS is set to an unrecognized value: $(CENTRALIZED_KMS)"
endif



stop-kms:
ifeq ($(CENTRALIZED_KMS),true)
	@echo "CENTRALIZED_KMS is true, Stopping centralized KMS...."
	@docker compose  -f docker-compose/docker-compose-full.yml down
	
else ifeq ($(CENTRALIZED_KMS),false)
	@echo "CENTRALIZED_KMS is false, Stopping threshold KMS...."
	$(MAKE) stop-kms-threshold
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
	# $(MAKE) install-packages
	# @sleep 5
	@echo "fund test addresses"
	@cd $(FHEVM_SOLIDITY_PATH) && cp .env.example.deployment .env
	@cd $(FHEVM_SOLIDITY_PATH) && rm -rf ./.openzeppelin
	@cd $(FHEVM_SOLIDITY_PATH) && ./fund_tests_addresses_docker.sh
	@cd $(FHEVM_SOLIDITY_PATH) && npm i
	@cd $(FHEVM_SOLIDITY_PATH) && ./precompute-addresses.sh
	@cd $(FHEVM_SOLIDITY_PATH) && ./launch-fhevm-coprocessor.sh

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


clean-keys:
	$(MAKE) stop-full
	rm -rf network-fhe-keys
	rm -rf kms-fhe-keys
	rm -rf res


clean:
	$(MAKE) stop-full
	rm -rf $(WORKDIR)/
	rm -rf network-fhe-keys
	rm -rf kms-fhe-keys
	rm -rf res


print-info:
	@echo 'KMS_DEV_VERSION: $(KMS_DEV_VERSION) for KEY_GEN---extracted from Makefile'
	@echo 'FHEVM_SOLIDITY_VERSION: $(FHEVM_SOLIDITY_VERSION) ---extracted from Makefile'
	@bash scripts/get_repository_info.sh fhevm $(FHEVM_SOLIDITY_PATH)

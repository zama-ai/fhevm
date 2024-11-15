#!/usr/bin/make -f

include .env

KEY_GEN = false
BINDIR ?= $(GOPATH)/bin
ETHERMINT_BINARY = ethermintd
ETHERMINT_DIR = ethermint


# This version must the same as in docker-compose-full.yml
# TODO add check
KMS_DEV_VERSION ?= v0.7.1

export GO111MODULE = on

# Default target executed when no arguments are given to make.
default_target: all

.PHONY: default_target

# process build tags

###############################################################################
###                                Single validator                         ###
###############################################################################



generate-fhe-keys:
	@bash ./scripts/copy_fhe_keys.sh $(KMS_DEV_VERSION) $(PWD)/network-fhe-keys $(PWD)/kms-fhe-keys

run-full:
	$(MAKE) generate-fhe-keys
	@docker compose --env-file .env.docker -f docker-compose/docker-compose-full.yml up --detach
	@echo 'sleep a little to let the docker start up'
	sleep 5

stop-full:
	@docker compose --env-file .env.docker -f docker-compose/docker-compose-full.yml down


clean:
	$(MAKE) stop-full
	rm -rf network-fhe-keys
	rm -rf kms-fhe-keys


print-info:
	@echo 'KMS_DEV_VERSION: $(KMS_DEV_VERSION) for KEY_GEN---extracted from Makefile'

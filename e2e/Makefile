#!/usr/bin/make -f





# Default target executed when no arguments are given to make.
default_target: prepare-test-local-coprocessor


.PHONY: default_target


prepare-test-local-coprocessor:
	@cp -v .env.local .env
	@cp -v contracts/E2EFHEVMConfigLocal.sol contracts/E2EFHEVMConfig.sol
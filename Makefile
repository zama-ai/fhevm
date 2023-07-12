HOST_ARCH := $(shell uname -m)
WORKDIR ?= $(CURDIR)/work_dir

FHEVM_TFHE_CLI_PATH ?= $(WORKDIR)/fhevm-tfhe-cli
FHEVM_TFHE_CLI_PATH_EXISTS := $(shell test -d $(FHEVM_TFHE_CLI_PATH)/.git && echo "true" || echo "false")
FHEVM_TFHE_CLI_VERSION ?= v0.1.1

install-fhevm-tfhe-cli: $(WORKDIR)/ check-fhevm-tfhe-cli

check-fhevm-tfhe-cli: 
	$(info check-fhevm-tfhe-cli)
	@echo "FHEVM_TFHE_CLI_PATH_EXISTS  $(FHEVM_TFHE_CLI_PATH_EXISTS)"
ifeq ($(FHEVM_TFHE_CLI_PATH_EXISTS), true)
	@echo "fhevm-tfhe-cli exists in $(FHEVM_TFHE_CLI_PATH)"
	@if [ ! -d $(WORKDIR)/fhevm-tfhe-cli ]; then \
        echo 'fhevm-tfhe-cli is not available in $(WORKDIR)'; \
        echo "FHEVM_TFHE_CLI_PATH is set to a custom value"; \
    else \
        echo 'fhevm-tfhe-cli is already available in $(WORKDIR)'; \
    fi
else
	@echo "fhevm-tfhe-cli does not exist in $(FHEVM_TFHE_CLI_PATH)"
	echo "We clone it for you!"
	echo "If you want your own version please update FHEVM_TFHE_CLI_PATH pointing to your fhevm-tfhe-cli folder!"
	$(MAKE) clone_fhevm_tfhe_cli
endif
	echo 'Call build zbc fhe'
	$(MAKE) build_fhevm_tfhe_cli

clone_fhevm_tfhe_cli: $(WORKDIR)/
	$(info Cloning fhevm-tfhe-cli version $(FHEVM_TFHE_CLI_VERSION))
	cd $(WORKDIR) && git clone git@github.com:zama-ai/fhevm-tfhe-cli.git
	cd $(WORKDIR)/fhevm-tfhe-cli && git checkout $(FHEVM_TFHE_CLI_VERSION)


build_fhevm_tfhe_cli:
ifeq ($(HOST_ARCH), x86_64)
	@echo 'Arch is x86'
	@ARCH_TO_BUILD_FHEVM_TFHE_CLI=$$(cd $(FHEVM_TFHE_CLI_PATH) && ./scripts/get_arch.sh) && cd $(FHEVM_TFHE_CLI_PATH) && cargo build --release --features tfhe/$${ARCH_TO_BUILD_FHEVM_TFHE_CLI}
else
	@echo 'Arch is not x86'
	@ARCH_TO_BUILD_FHEVM_TFHE_CLI=$$(cd $(FHEVM_TFHE_CLI_PATH) && ./scripts/get_arch.sh) && cd $(FHEVM_TFHE_CLI_PATH) && cargo +nightly build --release --features tfhe/$${ARCH_TO_BUILD_FHEVM_TFHE_CLI}
endif


$(WORKDIR)/:
	$(info WORKDIR)
	mkdir -p $(WORKDIR)

clean: 
	$(WORKDIR)/



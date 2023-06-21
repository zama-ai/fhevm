HOST_ARCH := $(shell uname -m)
WORKDIR ?= $(CURDIR)/work_dir

ZBC_FHE_TOOL_PATH ?= $(WORKDIR)/zbc-fhe-tool
ZBC_FHE_TOOL_PATH_EXISTS := $(shell test -d $(ZBC_FHE_TOOL_PATH)/.git && echo "true" || echo "false")
ZBC_FHE_TOOL_VERSION ?= v0.1.0

install-zbc-fhe-tool: $(WORKDIR)/ check-zbc-fhe-tool

check-zbc-fhe-tool: 
	$(info check-zbc-fhe-tool)
	@echo "ZBC_FHE_TOOL_PATH_EXISTS  $(ZBC_FHE_TOOL_PATH_EXISTS)"
ifeq ($(ZBC_FHE_TOOL_PATH_EXISTS), true)
	@echo "zbc-fhe-tool exists in $(ZBC_FHE_TOOL_PATH)"
	@if [ ! -d $(WORKDIR)/zbc-fhe-tool ]; then \
        echo 'zbc-fhe-tool is not available in $(WORKDIR)'; \
        echo "ZBC_FHE_TOOL_PATH is set to a custom value"; \
    else \
        echo 'zbc-fhe-tool is already available in $(WORKDIR)'; \
    fi
else
	@echo "zbc-fhe-tool does not exist in $(ZBC_FHE_TOOL_PATH)"
	echo "We clone it for you!"
	echo "If you want your own version please update ZBC_FHE_TOOL_PATH pointing to your zbc-fhe-tool folder!"
	$(MAKE) clone_zbc_fhe_tool
endif
	echo 'Call build zbc fhe'
	$(MAKE) build_zbc_fhe_tool

clone_zbc_fhe_tool: $(WORKDIR)/
	$(info Cloning zbc-fhe-tool version $(ZBC_FHE_TOOL_VERSION))
	cd $(WORKDIR) && git clone git@github.com:zama-ai/zbc-fhe-tool.git
	cd $(WORKDIR)/zbc-fhe-tool && git checkout $(ZBC_FHE_TOOL_VERSION)


build_zbc_fhe_tool:
ifeq ($(HOST_ARCH), x86_64)
	@echo 'Arch is x86'
	@ARCH_TO_BUIL_ZBC_FHE_TOOL=$$(cd $(ZBC_FHE_TOOL_PATH) && ./scripts/get_arch.sh) && cd $(ZBC_FHE_TOOL_PATH) && cargo build --release --features tfhe/$${ARCH_TO_BUIL_ZBC_FHE_TOOL}
else
	@echo 'Arch is not x86'
	@ARCH_TO_BUIL_ZBC_FHE_TOOL=$$(cd $(ZBC_FHE_TOOL_PATH) && ./scripts/get_arch.sh) && cd $(ZBC_FHE_TOOL_PATH) && cargo +nightly build --release --features tfhe/$${ARCH_TO_BUIL_ZBC_FHE_TOOL}
endif


$(WORKDIR)/:
	$(info WORKDIR)
	mkdir -p $(WORKDIR)

clean: 
	$(WORKDIR)/



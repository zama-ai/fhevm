# Single source of truth for the host address constants lives in host-contracts.
HOST_ADDRESSES := ../host-contracts/addresses/FHEVMHostAddresses.sol
# examples/CoprocessorSetup.sol imports this hardhat-convention path. The forge-only flow does
# not run the hardhat deploy task that normally generates it, so we mirror it here.
FHEVM_ADDRESSES := fhevmTemp/addresses/FHEVMHostAddresses.sol

.PHONY: check-spdx-headers ensure-addresses forge-test

# Make sure all Solidity source files use the expected SPDX license identifier (BSD-3-Clause-Clear).
# We check lib/ and config/ but exclude external dependencies (OpenZeppelin-derived FhevmECDSA).
check-spdx-headers:
	bash ../ci/check_spdx_licenses.sh lib config --exclude cryptography/FhevmECDSA.sol

# Generate the host address constants via the host-contracts flow (only when missing).
$(HOST_ADDRESSES):
	$(MAKE) -C ../host-contracts ensure-addresses

# Mirror the generated host addresses into the path expected by examples/CoprocessorSetup.sol,
# so forge can compile the examples (pulled in by the bridge tests) without running hardhat.
$(FHEVM_ADDRESSES): $(HOST_ADDRESSES)
	mkdir -p $(dir $@)
	cp $< $@

# Prepare the address constants required to compile the examples.
ensure-addresses: $(FHEVM_ADDRESSES)

# Run the Foundry test suite from a clean checkout (no hardhat required).
forge-test: ensure-addresses
	forge soldeer install && forge test

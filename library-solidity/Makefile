# Make sure all Solidity source files use the expected SPDX license identifier (BSD-3-Clause-Clear).
# We check lib/ and config/ but exclude external dependencies (OpenZeppelin-derived FhevmECDSA).
check-spdx-headers:
	bash ../ci/check_spdx_licenses.sh lib config --exclude cryptography/FhevmECDSA.sol

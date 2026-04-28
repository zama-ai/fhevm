// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, FHE} from "@fhevm/solidity/lib/FHE.sol";

library DefaultCoprocessorConfig {
    /// @dev These addresses are placeholders. They are patched at runtime via sed
    /// in the E2E test runner script with the actual deployment addresses.
    function getConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D,
                CoprocessorAddress: 0xe3a9105a3a932253A70F126eb1E3b589C643dD24,
                KMSVerifierAddress: 0x901F8942346f7AB3a01F6D7613119Bca447Bb030
            });
    }
}

contract E2ECoprocessorConfig {
    constructor() {
        FHE.setCoprocessor(DefaultCoprocessorConfig.getConfig());
    }
}

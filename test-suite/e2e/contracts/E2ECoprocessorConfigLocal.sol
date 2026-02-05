// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, FHE} from "@fhevm/solidity/lib/FHE.sol";

library DefaultCoprocessorConfig {
    /// @dev These addresses are placeholders. They are patched at runtime via sed
    /// in the E2E test runner script with the actual deployment addresses.
    function getConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c,
                CoprocessorAddress: 0xcCAe95fF1d11656358E782570dF0418F59fA40e1,
                KMSVerifierAddress: 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11
            });
    }
}

contract E2ECoprocessorConfig {
    constructor() {
        FHE.setCoprocessor(DefaultCoprocessorConfig.getConfig());
    }
}

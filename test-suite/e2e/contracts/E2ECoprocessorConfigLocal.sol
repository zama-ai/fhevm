// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, FHE} from "@fhevm/solidity/lib/FHE.sol";

library DefaultCoprocessorConfig {
    function getConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c,
                CoprocessorAddress: 0x12B064FB845C1cc05e9493856a1D637a73e944bE,
                DecryptionOracleAddress: 0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d,
                KMSVerifierAddress: 0xcCAe95fF1d11656358E782570dF0418F59fA40e1
            });
    }
}

contract E2ECoprocessorConfig {
    constructor() {
        FHE.setCoprocessor(DefaultCoprocessorConfig.getConfig());
    }
}

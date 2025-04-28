// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMConfigStruct, FHE} from "@fhevm/solidity/lib/FHE.sol";

address constant DECRYPTION_ORACLE_ADDRESS = 0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d;

library DefaultFHEVMConfig {
    function getConfig() internal pure returns (FHEVMConfigStruct memory) {
        return
            FHEVMConfigStruct({
                ACLAddress: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c,
                FHEVMExecutorAddress: 0x12B064FB845C1cc05e9493856a1D637a73e944bE,
                KMSVerifierAddress: 0xcCAe95fF1d11656358E782570dF0418F59fA40e1,
                InputVerifierAddress: 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11
            });
    }
}

contract E2EFHEVMConfig {
    constructor() {
        FHE.setCoprocessor(DefaultFHEVMConfig.getConfig());
        FHE.setDecryptionOracle(DECRYPTION_ORACLE_ADDRESS);
    }
}
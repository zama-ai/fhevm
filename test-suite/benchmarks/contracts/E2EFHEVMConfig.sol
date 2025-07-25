// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHEVMConfigStruct, FHE } from "@fhevm/solidity/lib/FHE.sol";

address constant DECRYPTION_ORACLE_ADDRESS = 0xa02Cda4Ca3a71D7C46997716F4283aa851C28812;

library DefaultFHEVMConfig {
    function getConfig() internal pure returns (FHEVMConfigStruct memory) {
        return
            FHEVMConfigStruct({
                ACLAddress: 0x687820221192C5B662b25367F70076A37bc79b6c,
                FHEVMExecutorAddress: 0x848B0066793BcC60346Da1F49049357399B8D595,
                KMSVerifierAddress: 0x1364cBBf2cDF5032C47d8226a6f6FBD2AFCDacAC,
                InputVerifierAddress: 0xbc91f3daD1A5F19F8390c400196e58073B6a0BC4
            });
    }
}

contract E2EFHEVMConfig {
    constructor() {
        FHE.setCoprocessor(DefaultFHEVMConfig.getConfig());
        FHE.setDecryptionOracle(DECRYPTION_ORACLE_ADDRESS);
    }
}

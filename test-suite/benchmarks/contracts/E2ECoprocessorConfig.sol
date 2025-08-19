// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { CoprocessorConfig, FHE } from "@fhevm/solidity/lib/FHE.sol";

library DefaultCoprocessorConfig {
    function getConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x687820221192C5B662b25367F70076A37bc79b6c,
                CoprocessorAddress: 0x848B0066793BcC60346Da1F49049357399B8D595,
                DecryptionOracleAddress: 0xa02Cda4Ca3a71D7C46997716F4283aa851C28812,
                KMSVerifierAddress: 0x1364cBBf2cDF5032C47d8226a6f6FBD2AFCDacAC
            });
    }
}

contract E2ECoprocessorConfig {
    constructor() {
        FHE.setCoprocessor(DefaultCoprocessorConfig.getConfig());
    }
}

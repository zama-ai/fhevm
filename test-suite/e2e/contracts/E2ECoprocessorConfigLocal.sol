// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, FHE} from "@fhevm/solidity/lib/FHE.sol";

library DefaultCoprocessorConfig {
    function getConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: REPLACE_WITH_DEPLOYED_ACL_ADDRESS,
                CoprocessorAddress: REPLACE_WITH_DEPLOYED_FHEVMEXECUTOR_ADDRESS,
                KMSVerifierAddress: REPLACE_WITH_DEPLOYED_KMSVERIFIER_ADDRESS
            });
    }
}

contract E2ECoprocessorConfig {
    constructor() {
        FHE.setCoprocessor(DefaultCoprocessorConfig.getConfig());
    }
}

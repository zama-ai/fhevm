// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";

library MainnetConfig {
    function getMainnetConfig() internal pure returns (CoprocessorConfig memory) {
        return CoprocessorConfig({
            ACLAddress: 0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6,
            CoprocessorAddress: 0xD82385dADa1ae3E969447f20A3164F6213100e75,
            KMSVerifierAddress: 0x77627828a55156b04Ac0DC0eb30467f1a552BB03
        });
    }
}

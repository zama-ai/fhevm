// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";

library SepoliaConfig {
    function getDevnetConfig() internal pure returns (CoprocessorConfig memory) {
        return CoprocessorConfig({
            ACLAddress: 0xBCA6F8De823a399Dc431930FD5EE550Bf1C0013e,
            CoprocessorAddress: 0x5cc8c5A366E733d4f1e677B2A9C08Bc2ea49b302,
            KMSVerifierAddress: 0x3F3819BeBE4bD0EFEf8078Df6f9B574ADa80CCA4
        });
    }

    function getTestnetConfig() internal pure returns (CoprocessorConfig memory) {
        return CoprocessorConfig({
            ACLAddress: 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D,
            CoprocessorAddress: 0x92C920834Ec8941d2C77D188936E1f7A6f49c127,
            KMSVerifierAddress: 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A
        });
    }
}

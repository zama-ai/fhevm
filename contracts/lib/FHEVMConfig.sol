// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../addresses/ACLAddress.sol";
import "../addresses/KMSVerifierAddress.sol";
import "../addresses/InputVerifierAddress.sol";
import "../addresses/FHEVMExecutorAddress.sol";

import {FHEVMConfigStruct} from "./FHE.sol";

library FHEVMConfig {
    /// @dev Function to return an immutable struct
    function defaultConfig() internal pure returns (FHEVMConfigStruct memory) {
        return
            FHEVMConfigStruct({
                ACLAddress: aclAdd,
                FHEVMExecutorAddress: fhevmExecutorAdd,
                KMSVerifierAddress: kmsVerifierAdd,
                InputVerifierAddress: inputVerifierAdd
            });
    }
}

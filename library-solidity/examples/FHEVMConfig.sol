// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd, inputVerifierAdd} from "../fhevmTemp/addresses/FHEVMHostAddresses.sol";

import {FHEVMConfigStruct} from "../lib/Impl.sol";

/**
 * @title   FHEVMConfig
 * @notice  This library returns all addresses for the ACL, FHEVMExecutor, InputVerifier,
 *          and KMSVerifier contracts.
 */
library FHEVMConfig {
    /**
     * @notice This function returns a struct containing all contract addresses.
     * @dev    It returns an immutable struct.
     */
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

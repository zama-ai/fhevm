// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../httpzTemp/addresses/ACLAddress.sol";
import "../httpzTemp/addresses/InputVerifierAddress.sol";
import "../httpzTemp/addresses/KMSVerifierAddress.sol";
import "../httpzTemp/addresses/TFHEExecutorAddress.sol";

import {HTTPZConfigStruct} from "../lib/Impl.sol";

/**
 * @title   FHEVMConfig
 * @notice  This library returns all addresses for the ACL, TFHEExecutor, InputVerifier,
 *          and KMSVerifier contracts.
 */
library FHEVMConfig {
    /**
     * @notice This function returns a struct containing all contract addresses.
     * @dev    It returns an immutable struct.
     */
    function defaultConfig() internal pure returns (HTTPZConfigStruct memory) {
        return
            HTTPZConfigStruct({
                ACLAddress: aclAdd,
                TFHEExecutorAddress: tfheExecutorAdd,
                KMSVerifierAddress: kmsVerifierAdd,
                InputVerifierAddress: inputVerifierAdd
            });
    }
}

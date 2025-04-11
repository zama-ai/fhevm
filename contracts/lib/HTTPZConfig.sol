// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../addresses/ACLAddress.sol";
import "../addresses/KMSVerifierAddress.sol";
import "../addresses/InputVerifierAddress.sol";
import "../addresses/HTTPZExecutorAddress.sol";

import {HTTPZConfigStruct} from "./HTTPZ.sol";

library HTTPZConfig {
    /// @dev Function to return an immutable struct
    function defaultConfig() internal pure returns (HTTPZConfigStruct memory) {
        return
            HTTPZConfigStruct({
                ACLAddress: aclAdd,
                HTTPZExecutorAddress: httpzExecutorAdd,
                KMSVerifierAddress: kmsVerifierAdd,
                InputVerifierAddress: inputVerifierAdd
            });
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "fhevm-core-contracts/addresses/ACLAddress.sol";
import "fhevm-core-contracts/addresses/InputVerifierAddress.sol";
import "fhevm-core-contracts/addresses/KMSVerifierAddress.sol";
import "fhevm-core-contracts/addresses/TFHEExecutorAddress.sol";
import "../lib/Impl.sol";

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
    function getTestConfig() internal pure returns (FHEVMConfigStruct memory) {
        return
            FHEVMConfigStruct({
                ACLAddress: aclAdd,
                TFHEExecutorAddress: tfheExecutorAdd,
                KMSVerifierAddress: kmsVerifierAdd,
                InputVerifierAddress: inputVerifierAdd
            });
    }
}

/**
 * @title   TestZamaFHEVMConfig.
 */
contract TestZamaFHEVMConfig {
    constructor() {
        TFHE.setFHEVM(FHEVMConfig.getTestConfig());
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../addresses/FHEVMHostAddresses.sol";
import "../addresses/DecryptionOracleAddress.sol";

import {CoprocessorConfig} from "./Impl.sol";

/**
 * @title   Coprocessor
 * @notice  This library returns all addresses for the ACL, FHEVMExecutor, DecryptionOracle,
 *          and KMSVerifier contracts.
 */
library CoprocessorSetup {
    /**
     * @notice This function returns a struct containing all contract addresses.
     * @dev    It returns an immutable struct.
     */
    function defaultConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: aclAdd,
                CoprocessorAddress: fhevmExecutorAdd,
                DecryptionOracleAddress: decryptionOracleAdd,
                KMSVerifierAddress: kmsVerifierAdd
            });
    }
}

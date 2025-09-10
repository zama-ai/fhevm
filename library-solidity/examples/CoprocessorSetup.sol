// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {decryptionOracleAdd} from "../fhevmTemp/addresses/DecryptionOracleAddress.sol";
import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd} from "../fhevmTemp/addresses/FHEVMHostAddresses.sol";

import {CoprocessorConfig} from "../lib/Impl.sol";

/**
 * @title   CoprocessorSetup
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

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../addresses/FHEVMHostAddresses.sol";
import "../addresses/DecryptionOracleAddress.sol";

import {CoprocessorConfigStruct} from "./Impl.sol";

/**
 * @title   CoprocessorConfig
 * @notice  This library returns all addresses for the ACL, FHEVMExecutor, DecryptionOracle,
 *          and KMSVerifier contracts.
 */
library CoprocessorConfig {
    /**
     * @notice This function returns a struct containing all contract addresses.
     * @dev    It returns an immutable struct.
     */
    function defaultConfig() internal pure returns (CoprocessorConfigStruct memory) {
        return
            CoprocessorConfigStruct({
                ACLAddress: aclAdd,
                CoprocessorAddress: fhevmExecutorAdd,
                DecryptionOracleAddress: DECRYPTION_ORACLE_ADDRESS,
                KMSVerifierAddress: kmsVerifierAdd
            });
    }
}

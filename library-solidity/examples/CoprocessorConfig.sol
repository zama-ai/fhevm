// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {aclAdd} from "../fhevmTemp/addresses/ACLAddress.sol";
import {fhevmExecutorAdd} from "../fhevmTemp/addresses/FHEVMExecutorAddress.sol";
import {DECRYPTION_ORACLE_ADDRESS} from "../fhevmTemp/addresses/DecryptionOracleAddress.sol";
import {kmsVerifierAdd} from "../fhevmTemp/addresses/KMSVerifierAddress.sol";

import {CoprocessorConfigStruct} from "../lib/Impl.sol";

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

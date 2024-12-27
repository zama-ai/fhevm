// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "fhevm-core-contracts/addresses/DecryptionOracleAddress.sol";

/**
 * @title   DecryptionOracleConfig
 * @notice  This library returns the DecryptionOracle address
 */
library DecryptionOracleConfig {
    /**
     * @notice This function returns a the gateway contract address.
     */
    function defaultDecryptionOracle() internal pure returns (address) {
        return DECRYPTION_ORACLE_ADDRESS;
    }
}

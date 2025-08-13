// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

/// @notice Contract for testing asynchronous decryption using the Gateway
contract HTTPPublicDecrypt is E2ECoprocessorConfig {
    /// @dev Encrypted state variables
    ebool public xBool;
    euint32 public xUint32;
    eaddress public xAddress;

    /// @notice Constructor to initialize the contract and set up encrypted values
    constructor() {
        /// @dev Initialize encrypted variables with sample values
        xBool = FHE.asEbool(true);
        FHE.makePubliclyDecryptable(xBool);

        xUint32 = FHE.asEuint32(242);
        FHE.makePubliclyDecryptable(xUint32);

        xAddress = FHE.asEaddress(0xfC4382C084fCA3f4fB07c3BCDA906C01797595a8);
        FHE.makePubliclyDecryptable(xAddress);
    }
}

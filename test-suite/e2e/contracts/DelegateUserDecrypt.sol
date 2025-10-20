// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";


/// @notice Contract for demonstrating user decryption with delegation
contract DelegateUserDecrypt is E2ECoprocessorConfig {
    /// @dev Encrypted boolean
    ebool public xBool;

    /// @notice Constructor to initialize encrypted values and set permissions
    constructor() {
        // Initialize and set permissions for xBool
        xBool = FHE.asEbool(true);
        FHE.allowThis(xBool);
        FHE.allow(xBool, msg.sender);
        FHE.delegateForUserDecryption(xBool, msg.sender);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import {FHE, ebool} from "@fhevm/solidity/lib/FHE.sol";

/// @notice Contract for testing uninitialized FHE contract
contract TestFHENotInitialized {
    /// @dev Encrypted state variables
    ebool private xBool;

    /// @dev Decrypted state variables
    bool public yBool;

    /// @notice Constructor where the user forgot to initialize the FHE environment
    constructor() {}

    /// @notice Perform a simple FHE operation. Should revert since FHE is not initialized
    function computeFheBoolAndRevert() public {
        xBool = FHE.asEbool(true);
        FHE.allowThis(xBool);
    }
}

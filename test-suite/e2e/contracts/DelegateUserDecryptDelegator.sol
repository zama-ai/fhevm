// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";


/// @notice Contract for demonstrating user decryption with delegation
contract DelegateUserDecryptDelegator is E2ECoprocessorConfig {
    /// @dev Encrypted boolean
    ebool public xBool;

    /// @notice Constructor to initialize encrypted values and set permissions
    constructor() {
        xBool = FHE.asEbool(true);
        FHE.allowThis(xBool);
        FHE.allow(xBool, msg.sender);
    }

    function delegate(address contract_delegate_address) public {
        FHE.delegateUserDecryption(msg.sender, contract_delegate_address, uint64(block.timestamp + 1 days));
    }

    function revoke(address contract_delegate_address) public {
        FHE.revokeUserDecryptionDelegation(msg.sender, contract_delegate_address);
    }
}

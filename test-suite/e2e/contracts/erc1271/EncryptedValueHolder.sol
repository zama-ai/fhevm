// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice Holds a trivially-encrypted value on behalf of a user — the
///         realistic user-decryption shape: the handle lives on a dapp/token
///         contract and the user (e.g. an ERC-1271 wallet) is only the
///         `userAddress`. Needed for the SDK-client suite because both the
///         SDK (`checkPersistAllowed`) and the KMS connector
///         (`userAddress` listed in `allowedContracts`) reject requests where
///         the userAddress IS the contract holding the handle — a shape the
///         raw-envelope protocol suite can still exercise via permissive
///         `allowedContracts: []`, which the SDK does not expose.
contract EncryptedValueHolder is E2ECoprocessorConfig {
    euint64 public value;

    /// @notice Store a trivially-encrypted value and authorize `user` (and
    ///         this contract) to decrypt it.
    function initValueFor(uint64 v, address user) external {
        value = FHE.asEuint64(v);
        FHE.allowThis(value); // isAllowed(value, address(this)) == true
        FHE.allow(value, user); // isAllowed(value, user) == true
    }
}

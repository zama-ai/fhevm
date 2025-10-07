// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract ACLEvents {
    /// @notice         Emitted when a handle is allowed.
    /// @param caller   account calling the allow function.
    /// @param account  account being allowed for the handle.
    /// @param handle   handle being allowed.
    event Allowed(address indexed caller, address indexed account, bytes32 handle);

    /// @notice             Emitted when a list of handles is allowed for decryption.
    /// @param caller       account calling the allowForDecryption function.
    /// @param handlesList  List of handles allowed for decryption.
    event AllowedForDecryption(address indexed caller, bytes32[] handlesList);

    /// @notice                 Emitted when a new delegation is requested.
    /// @param delegator        Delegator address.
    /// @param delegatee        Delegatee address.
    /// @param contractAddress  Contract address.
    /// @param delegationCounter    Delegation counter.
    /// @param oldExpiryDate    Previous Expiry Date.
    /// @param newExpiryDate    New Expiry Date.
    event NewDelegation(
        address indexed delegator,
        address indexed delegatee,
        address contractAddress,
        uint64 delegationCounter,
        uint64 oldExpiryDate,
        uint64 newExpiryDate
    );

    /// @notice                 Emitted when a delegation is revoked.
    /// @param delegator        Delegator address.
    /// @param delegatee        Delegatee address.
    /// @param contractAddress  Contract address.
    /// @param delegationCounter    Delegation counter.
    /// @param oldExpiryDate    Previous Expiry Date.
    event RevokedDelegation(
        address indexed delegator,
        address indexed delegatee,
        address contractAddress,
        uint64 delegationCounter,
        uint64 oldExpiryDate
    );

    /// @notice Emitted when the pauser address is updated.
    /// @param newPauser New pauser address.
    event UpdatePauser(address indexed newPauser);
}

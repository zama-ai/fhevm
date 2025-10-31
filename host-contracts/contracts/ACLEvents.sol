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

    /// @notice                     Emitted when an account is delegated for user decryption.
    /// @param delegator            The address of the account that delegates access to its handles.
    /// @param delegate             The address of the account that receives the delegation.
    /// @param contractAddress      The contract address to delegate access to.
    /// @param delegationCounter    Counter that tracks the order of each delegation or revocation.
    /// @param oldExpirationDate    The previous UNIX timestamp when the user decryption delegation expires.
    /// @param newExpirationDate    The new UNIX timestamp when the user decryption delegation expires.
    event DelegatedForUserDecryption(
        address indexed delegator,
        address indexed delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 oldExpirationDate,
        uint64 newExpirationDate
    );

    /// @notice                     Emitted when a delegation for user decryption is revoked.
    /// @param delegator            The address of the account that delegates access to its handles.
    /// @param delegate             The address of the account that receives the delegation.
    /// @param contractAddress      The contract address to delegate access to.
    /// @param delegationCounter    Counter that tracks the order of each delegation or revocation.
    /// @param oldExpirationDate    The expiration UNIX timestamp of the revoked user decryption delegation.
    event RevokedDelegationForUserDecryption(
        address indexed delegator,
        address indexed delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 oldExpirationDate
    );

    /// @notice Emitted when an account is added to the deny list.
    /// @param account The address of the account that is blocked.
    event BlockedAccount(address indexed account);

    /// @notice Emitted when an account is removed from the deny list.
    /// @param account The address of the account that is unblocked.
    event UnblockedAccount(address indexed account);
}

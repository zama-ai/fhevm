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

    /// @notice                 Emitted when a new delegatee address is added.
    /// @param caller           caller address
    /// @param delegatee        Delegatee address.
    /// @param contractAddresses  Contract addresses.
    event NewDelegation(address indexed caller, address indexed delegatee, address[] contractAddresses);

    /// @notice                 Emitted when a delegatee address is revoked.
    /// @param caller           caller address
    /// @param delegatee        Delegatee address.
    /// @param contractAddresses  Contract addresses.
    event RevokedDelegation(address indexed caller, address indexed delegatee, address[] contractAddresses);

    /// @notice Emitted when the pauser address is updated.
    /// @param newPauser New pauser address.
    event UpdatePauser(address indexed newPauser);
}

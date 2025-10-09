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
}

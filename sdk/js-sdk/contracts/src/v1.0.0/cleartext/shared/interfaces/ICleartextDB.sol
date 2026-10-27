// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title ICleartextDB
 * @notice Shared store mapping ciphertext handles to their cleartext values, extracted from the
 *         executor so that multiple `FHEVMExecutor` instances can share one cleartext database.
 * @dev Writes are gated to a set of registered writers (the cleartext-arithmetic layer that computes
 *      and persists results), managed by the ACL owner — mirroring the `PauserSet` pattern. Reads
 *      are public.
 */
interface ICleartextDB {
    /// @notice Emitted when an address is granted write access.
    event AddWriter(address indexed account);
    /// @notice Emitted when an address's write access is revoked.
    event RemoveWriter(address indexed account);

    error CleartextErrorInvalidNullWriter();
    error CleartextErrorAccountAlreadyWriter(address account);
    error CleartextErrorAccountNotWriter(address account);
    error CleartextErrorNotWriter(address account);

    /// @notice Returns the cleartext value stored for `handle` (0 if unset).
    function get(bytes32 handle) external view returns (uint256);

    /// @notice Whether anything has ever been stored for `handle`.
    /// @dev    `get` returns 0 for an unwritten handle exactly as it does for one worth zero, so this
    ///         is the only way to tell them apart — which a reader on a fork must be able to do.
    function has(bytes32 handle) external view returns (bool);

    /// @notice Stores `value` for `handle`. Callable only by a registered writer.
    function set(bytes32 handle, uint256 value) external;

    /// @notice Grants write access to `account`. ACL-owner only.
    function addWriter(address account) external;

    /// @notice Revokes write access from `account`. ACL-owner only.
    function removeWriter(address account) external;

    /// @notice Whether `account` may write to the store.
    function isWriter(address account) external view returns (bool);
}

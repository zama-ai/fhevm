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

    error InvalidNullWriter();
    error AccountAlreadyWriter(address account);
    error AccountNotWriter(address account);
    error NotWriter(address account);

    /// @notice Returns the cleartext value stored for `handle` (0 if unset).
    function get(bytes32 handle) external view returns (uint256);

    /// @notice Stores `value` for `handle`. Callable only by a registered writer.
    function set(bytes32 handle, uint256 value) external;

    /// @notice Grants write access to `account`. ACL-owner only.
    function addWriter(address account) external;

    /// @notice Revokes write access from `account`. ACL-owner only.
    function removeWriter(address account) external;

    /// @notice Whether `account` may write to the store.
    function isWriter(address account) external view returns (bool);
}

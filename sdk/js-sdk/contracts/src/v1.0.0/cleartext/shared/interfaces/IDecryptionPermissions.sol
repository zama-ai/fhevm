// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title IDecryptionPermissions
 * @notice Who is allowed to see a handle — the ACL, as decryption cares about it.
 *
 * @dev The sibling of `IPlaintexts`: that one answers what a handle is WORTH, this one answers who may
 *      LEARN it, and every decryption path needs both. Three questions, one per path — publicly
 *      decryptable, allowed for a user, delegated from one user to another.
 *
 * @dev A SLICE of the ACL, not the whole thing. `_internal/interfaces/IACL.sol` is the generated, full
 *      surface; this is the handful of methods the decryption code actually calls, named for what it
 *      asks rather than for the contract that answers. Callers that only need permission checks should
 *      not have to name a thirty-function interface, nor reach into the generated tree to find it.
 */
interface IDecryptionPermissions {
    /// @notice Whether `handle` is publicly decryptable.
    function isAllowedForDecryption(bytes32 handle) external view returns (bool);

    /// @notice Whether `account` is persistently allowed to access `handle`.
    function persistAllowed(bytes32 handle, address account) external view returns (bool);

    /// @notice Whether `delegate` may decrypt `handle` for `delegator`.
    function isHandleDelegatedForUserDecryption(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle
    ) external view returns (bool);
}

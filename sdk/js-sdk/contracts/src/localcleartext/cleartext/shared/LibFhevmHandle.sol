// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "./LibFheType.sol";

/// ----------------------------------------------------------------------------
///   ⚠️ Private Library:
///     - Host-internal, shipped with the forge payload
///     - Should not be exposed to the public
///     - not maintained
///     - can change at any time
/// ----------------------------------------------------------------------------

/// @notice The internal binary format of an FHEVM handle.
///
/// @dev Handle layout (see `LibForgeFhevmEncrypt._inputHandle`):
///      `hash21[0:20] || index[21] || chainId[22:29] || fheTypeId[30] || version[31]`
///
///      TODO: isExternal(), toString().
library LibFhevmHandle {
    /**
     * @notice Returned when a handle carries a chain id other than the current one.
     * @param handle The offending handle.
     * @param handleChainId The chain id read from the handle.
     * @param blockChainId The current `block.chainid`.
     */
    error CleartextErrorHandleChainIdMismatch(bytes32 handle, uint64 handleChainId, uint64 blockChainId);
    /// @notice The zero word is not a handle: it is what an encrypted value reads as before anything was
    ///         ever written to it — `FHE.isInitialized` is false, and no ciphertext, no cleartext and no
    ///         ACL entry exist for it. Reading it is a mistake in the caller, named as such.
    error CleartextErrorHandleUninitialized(bytes32 handle);

    /// @notice The FHE type `handle` was minted for.
    /// @dev Byte 30 of the layout.
    function typeOf(bytes32 handle) internal pure returns (FheType) {
        return FheType(uint8(handle[30]));
    }

    /// @notice The position `handle` had in the batch it was minted with.
    /// @dev Byte 21. `encryptValues` returns the handles in call order, so this is the same `i` the
    ///      caller reads them back with.
    function indexOf(bytes32 handle) internal pure returns (uint8) {
        return uint8(handle[21]);
    }

    /// @notice The handle format `handle` was minted in.
    /// @dev Byte 31, the last one.
    function versionOf(bytes32 handle) internal pure returns (uint8) {
        return uint8(handle[31]);
    }

    /// @notice The chain `handle` was minted on.
    /// @dev Bytes 22..29, written as `uint64(block.chainid)`.
    function chainIdOf(bytes32 handle) internal pure returns (uint64) {
        return uint64(uint256(handle) >> 16);
    }

    /**
     * @notice Refuses a handle that was not minted on this chain.
     *
     * @dev The layout accessors above say what a handle CONTAINS; this says what the cleartext stack
     *      will ACCEPT, and it lives here because the two cannot be allowed to drift — the check is
     *      only meaningful while it reads the same bytes `chainIdOf` does.
     *
     *      A handle carrying another chain id was never produced by this stack: in a test that mostly
     *      means it was created before `vm.chainId` moved, or was copied in from another network's
     *      trace. The real host contracts do not check this; the cleartext ones can afford to, and a
     *      precise error beats a silent wrong answer — the handle would otherwise name a slot in THIS
     *      chain's database that is either empty or some unrelated handle's value.
     */
    function checkChainId(bytes32 handle) internal view {
        // FIRST, before any field is read out of it: the zero word carries chain id 0, so without this it
        // would fail below as a "chain mismatch" — a message that sends the reader to forks when the
        // real story is an uninitialized value, typically a confidential balance nothing ever credited.
        // Uninitialized-ness is what `FHE.isInitialized` reports, and the fix is to check that first.
        if (handle == bytes32(0)) revert CleartextErrorHandleUninitialized(handle);
        uint64 handleChainId = chainIdOf(handle);
        if (handleChainId != uint64(block.chainid)) {
            revert CleartextErrorHandleChainIdMismatch(handle, handleChainId, uint64(block.chainid));
        }
    }
}

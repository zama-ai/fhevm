// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, euint64} from "../FHE.sol";
import {MessagingFee, MessagingReceipt} from "./IConfidentialBridge.sol";
import {ConfidentialOAppCore} from "./ConfidentialOAppCore.sol";

/**
 * @title   ConfidentialOAppSender
 * @notice  Send side of a confidential omnichain app. Looks up the destination peer from the
 *          shared {ConfidentialOAppCore} registry and bridges encrypted handles to it through
 *          `FHE.bridge`, so subclasses never deal with the bridge contract or peer encoding
 *          directly. A single-handle convenience and a type-agnostic multi-handle overload are
 *          provided (a message may carry up to the bridge's `MAX_HANDLES`).
 * @dev     The destination is taken from {peers} (a `bytes32`, so non-EVM peers work too).
 *          Quote the fee with {_quoteBridge} and forward it as `msg.value` to {_bridge}.
 */
abstract contract ConfidentialOAppSender is ConfidentialOAppCore {
    /// @notice `lzComposeGas` is 0; the destination receive callback would never run, so the
    ///         bridged value would never be delivered. (The bridge also enforces a per-`dstEid`
    ///         minimum; this is a cheap pre-check for the clearly-invalid zero case.)
    error ZeroComposeGas();

    /**
     * @notice Bridge a single encrypted `euint64` handle to the peer on `dstEid`.
     * @dev    Convenience wrapper over the multi-handle {_bridge}; the payload must reference the
     *         handle at index 0. See the list overload for the allowance/gas rules.
     */
    function _bridge(
        uint32 dstEid,
        bytes memory payload,
        euint64 handle,
        uint64 lzComposeGas,
        uint256 nativeFee
    ) internal returns (MessagingReceipt memory) {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = euint64.unwrap(handle);
        return _bridge(dstEid, payload, handles, lzComposeGas, nativeFee);
    }

    /**
     * @notice Bridge a list of encrypted handles to the peer on `dstEid` in a single message.
     * @dev    Type-agnostic: pass raw `bytes32` handles (e.g. `euint64.unwrap(h)`); mixing
     *         encrypted types within one list is fine. The destination receiver gets the derived
     *         handles aligned one-to-one by index.
     *
     *         The bridge checks `isAllowed(handle, msg.sender)` for every handle, and since
     *         `_bridge` runs in this contract's context `msg.sender` is THIS contract — so the
     *         contract must hold ACL allowance on each handle (e.g. via `FHE.allowThis`), not the
     *         external caller. {_bridge} performs no caller authorization of its own; subclasses
     *         must gate their public entrypoint (as `ConfidentialOFTViaLib.send` does with
     *         `FHE.isSenderAllowed`).
     * @param dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param payload       Opaque app payload (the receiver decodes it); reference handles by index.
     * @param handles       Raw `bytes32` handles to bridge; this contract must hold ACL allowance
     *                      on each. Up to the bridge's `MAX_HANDLES`.
     * @param lzComposeGas  Gas budget for the destination receive callback (lzCompose leg); must
     *                      meet the bridge's per-`dstEid` minimum (and be non-zero).
     * @param nativeFee     LayerZero native fee to forward (query via {_quoteBridge}).
     */
    function _bridge(
        uint32 dstEid,
        bytes memory payload,
        bytes32[] memory handles,
        uint64 lzComposeGas,
        uint256 nativeFee
    ) internal returns (MessagingReceipt memory) {
        if (lzComposeGas == 0) revert ZeroComposeGas();
        return FHE.bridge(dstEid, _getPeerOrRevert(dstEid), payload, handles, lzComposeGas, nativeFee);
    }

    /// @notice Quotes the native fee for the single-handle {_bridge} call.
    function _quoteBridge(
        uint32 dstEid,
        bytes memory payload,
        euint64 handle,
        uint64 lzComposeGas
    ) internal view returns (MessagingFee memory) {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = euint64.unwrap(handle);
        return _quoteBridge(dstEid, payload, handles, lzComposeGas);
    }

    /// @notice Quotes the native fee for the multi-handle {_bridge} call.
    function _quoteBridge(
        uint32 dstEid,
        bytes memory payload,
        bytes32[] memory handles,
        uint64 lzComposeGas
    ) internal view returns (MessagingFee memory) {
        return FHE.quoteBridge(dstEid, address(this), _getPeerOrRevert(dstEid), payload, handles, lzComposeGas);
    }
}

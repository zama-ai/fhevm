// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE} from "../FHE.sol";
import {Impl} from "../Impl.sol";
import {MessagingFee, MessagingReceipt} from "./IConfidentialBridge.sol";
import {ConfidentialOAppCore} from "./ConfidentialOAppCore.sol";

/**
 * @title   ConfidentialOAppSender
 * @notice  Send side of a confidential omnichain app. Looks up the destination peer from the
 *          shared {ConfidentialOAppCore} registry and bridges encrypted handles to it through
 *          `FHE.bridge`, so subclasses never deal with the bridge contract or peer encoding
 *          directly (a message may carry up to the bridge's `MAX_HANDLES`).
 * @dev     The destination is taken from {peers} (a `bytes32`, so non-EVM peers work too).
 *          Quote the fee with {_quoteBridge} and forward it as `msg.value`.
 *
 *          AUTHORIZATION — read before exposing a send entrypoint. The bridge checks
 *          `isAllowed(handle, msg.sender)`, and `_bridge*` runs in THIS contract's context, so the
 *          check is against the contract — which typically holds `allowThis` allowance on its
 *          handles. The external caller is therefore NOT authorized by the bridge. There are two
 *          send helpers, named so the call site declares its safety posture:
 *          - {_bridgeFrom}: safe default. Asserts the external caller is ACL-allowed on every handle
 *            (the raw-handle equivalent of `FHE.isSenderAllowed`) before bridging. Type-agnostic:
 *            covers any encrypted type, single or multi, via `bytes32` handles. Use when the caller
 *            owns the handle(s) being sent.
 *          - {_bridgeUnchecked}: NO caller authorization. Use only for handles the caller does not
 *            (and should not) hold allowance on — e.g. a value derived inside the app such as a
 *            burn result — and gate the entrypoint yourself first (see `ConfidentialOFTViaLib`).
 *            A public entrypoint that calls this without its own gate lets anyone bridge the
 *            contract's handles to a configured peer.
 */
abstract contract ConfidentialOAppSender is ConfidentialOAppCore {
    /// @notice `lzComposeGas` is 0; the destination receive callback would never run, so the
    ///         bridged value would never be delivered (the bridge rejects a zero gas budget too).
    error ZeroComposeGas();

    /// @notice {_bridgeFrom} was called with a `handle` the external `sender` is not ACL-allowed on.
    error UnauthorizedSender(bytes32 handle, address sender);

    /**
     * @notice Safe send: bridge a list of encrypted handles the external caller owns to the peer on
     *         `dstEid` in a single message.
     * @dev    Asserts the external `msg.sender` is ACL-allowed on EVERY handle (`Impl.isAllowed`, the
     *         raw-handle check `FHE.isSenderAllowed` performs per type) before bridging, reverting
     *         {UnauthorizedSender} with the first handle that fails. This removes the footgun of an
     *         unguarded entrypoint. Type-agnostic: pass raw `bytes32` handles (e.g. `euint64.unwrap(h)`),
     *         mixing types within one list is fine. For a handle the caller does not hold allowance on
     *         (e.g. a value derived inside the app), gate the entrypoint yourself and use
     *         {_bridgeUnchecked}. The payload references handles by their index.
     */
    function _bridgeFrom(
        uint32 dstEid,
        bytes memory payload,
        bytes32[] memory handles,
        uint64 lzComposeGas,
        uint256 nativeFee
    ) internal returns (MessagingReceipt memory) {
        for (uint256 i = 0; i < handles.length; i++) {
            if (!Impl.isAllowed(handles[i], msg.sender)) revert UnauthorizedSender(handles[i], msg.sender);
        }
        return _bridgeUnchecked(dstEid, payload, handles, lzComposeGas, nativeFee);
    }

    /**
     * @notice Safe send: single-handle convenience over the multi-handle {_bridgeFrom}.
     * @dev    Type-agnostic — pass the raw `bytes32` handle (`T.unwrap(h)` for any encrypted type).
     *         Asserts the caller owns `handle`, then bridges it; the payload must reference it at
     *         index 0.
     */
    function _bridgeFrom(
        uint32 dstEid,
        bytes memory payload,
        bytes32 handle,
        uint64 lzComposeGas,
        uint256 nativeFee
    ) internal returns (MessagingReceipt memory) {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = handle;
        return _bridgeFrom(dstEid, payload, handles, lzComposeGas, nativeFee);
    }

    /**
     * @notice Unauthorized send: single-handle convenience over the multi-handle {_bridgeUnchecked},
     *         WITHOUT checking the external caller.
     * @dev    Type-agnostic — pass the raw `bytes32` handle (`T.unwrap(h)` for any encrypted type).
     *         The payload must reference it at index 0. See that overload for the authorization/gas
     *         rules — in particular, the caller is NOT authorized here, so gate the entrypoint yourself.
     */
    function _bridgeUnchecked(
        uint32 dstEid,
        bytes memory payload,
        bytes32 handle,
        uint64 lzComposeGas,
        uint256 nativeFee
    ) internal returns (MessagingReceipt memory) {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = handle;
        return _bridgeUnchecked(dstEid, payload, handles, lzComposeGas, nativeFee);
    }

    /**
     * @notice Unauthorized send: bridge a list of encrypted handles to the peer on `dstEid` in a
     *         single message, WITHOUT checking the external caller.
     * @dev    Type-agnostic: pass raw `bytes32` handles (e.g. `euint64.unwrap(h)`); mixing
     *         encrypted types within one list is fine. The destination receiver gets the derived
     *         handles aligned one-to-one by index.
     *
     *         No caller authorization is performed. The bridge checks `isAllowed(handle, msg.sender)`
     *         for every handle, and since this runs in the contract's context `msg.sender` is THIS
     *         contract — so the contract must hold ACL allowance on each handle (e.g. via
     *         `FHE.allowThis`), and the external caller is NOT authorized. Subclasses MUST gate their
     *         public entrypoint (as `ConfidentialOFTViaLib.send` does with `FHE.isSenderAllowed` on
     *         the input it then burns). For the common "caller owns the handle" case, prefer the safe
     *         {_bridgeFrom}.
     * @param dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param payload       Opaque app payload (the receiver decodes it); reference handles by index.
     * @param handles       Raw `bytes32` handles to bridge; this contract must hold ACL allowance
     *                      on each. Up to the bridge's `MAX_HANDLES`.
     * @param lzComposeGas  Gas budget for the destination receive callback (lzCompose leg); must be
     *                      non-zero (the bridge reverts otherwise).
     * @param nativeFee     LayerZero native fee to forward (query via {_quoteBridge}).
     */
    function _bridgeUnchecked(
        uint32 dstEid,
        bytes memory payload,
        bytes32[] memory handles,
        uint64 lzComposeGas,
        uint256 nativeFee
    ) internal returns (MessagingReceipt memory) {
        if (lzComposeGas == 0) revert ZeroComposeGas();
        return FHE.bridge(dstEid, _getPeerOrRevert(dstEid), payload, handles, lzComposeGas, nativeFee);
    }

    /// @notice Quotes the native fee for the single-handle send (pass `T.unwrap(h)` for any type).
    function _quoteBridge(
        uint32 dstEid,
        bytes memory payload,
        bytes32 handle,
        uint64 lzComposeGas
    ) internal view returns (MessagingFee memory) {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = handle;
        return _quoteBridge(dstEid, payload, handles, lzComposeGas);
    }

    /// @notice Quotes the native fee for the multi-handle send.
    function _quoteBridge(
        uint32 dstEid,
        bytes memory payload,
        bytes32[] memory handles,
        uint64 lzComposeGas
    ) internal view returns (MessagingFee memory) {
        return FHE.quoteBridge(dstEid, address(this), _getPeerOrRevert(dstEid), payload, handles, lzComposeGas);
    }
}

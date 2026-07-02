// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE} from "../FHE.sol";
import {Impl} from "../Impl.sol";
import {ConfidentialOAppCore} from "./ConfidentialOAppCore.sol";
import "encrypted-types/EncryptedTypes.sol";

/**
 * @title   ConfidentialOAppSender
 * @notice  Send side of a confidential omnichain app (cOApp) relying on a LayerZero-enabled ConfidentialBridge.
 *          Looks up the destination peer from the shared {ConfidentialOAppCore} registry and bridges encrypted
 *          handles to it through `FHE.sendLZConfidentialBridge`.
 * @dev     A message may carry up to 32 handes.
 * @dev     The destination is taken from {peers} (a `bytes32`, so non-EVM peers work too).
 * @dev     Quote the fee with {_quoteSendSingleHandleToPeer/_quoteSendHandlesToPeer} and forward it as `msg.value`
 *          when sending, i.e before calling any of {_sendSingleHandleToPeer/_sendHandlesToPeer}.
 */
abstract contract ConfidentialOAppSender is ConfidentialOAppCore {
    /**
     * @notice Bridges a single raw `bytes32` handle to the peer cOApp configured for `dstEid`.
     * @dev    *Private* core behind the per-type {_sendSingleHandleToPeer} overloads. Wraps `handle` in a
     *         one-element list; the destination receiver references it at index 0. Forwards `msg.value` as the
     *         LayerZero native fee, so the calling entrypoint must be `payable` and funded with
     *         the amount returned by {_quoteSendSingleHandleToPeer}.
     * @dev    Reverts {NoPeer} if no peer is configured for `dstEid`.
     * @param dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param payload       Opaque app payload; decoded by the destination receiver, which references the handle by index.
     * @param handle        Raw `bytes32` handle to bridge; this contract must hold ACL allowance on it.
     * @param lzComposeGas  Gas budget for the destination app callback `onConfidentialBridgeReceived` (lzCompose leg). The amount needed is
     *                      app-specific, apps should size it for their `onConfidentialBridgeReceived` workload.
     * @return guid         The LayerZero message guid.
     * @return nonce        The LayerZero message nonce.
     */
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        bytes32 handle,
        uint64 lzComposeGas
    ) private returns (bytes32 guid, uint64 nonce) {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = handle;
        (guid, nonce) = FHE.sendLZConfidentialBridge(
            dstEid,
            _getPeerOrRevert(dstEid),
            payload,
            handles,
            lzComposeGas,
            msg.value
        );
    }

    /// @notice Type-safe {ebool} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        ebool handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /// @notice Type-safe {euint8} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        euint8 handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /// @notice Type-safe {euint16} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        euint16 handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /// @notice Type-safe {euint32} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        euint32 handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /// @notice Type-safe {euint64} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        euint64 handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /// @notice Type-safe {euint128} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        euint128 handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /// @notice Type-safe {euint256} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        euint256 handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /// @notice Type-safe {eaddress} overload of the single-handle send; unwraps `handle` and bridges it to the peer on `dstEid`.. See the private {_sendSingleHandleToPeer} core for the full semantics.
    function _sendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        eaddress handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendSingleHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }

    /**
     * @notice Bridges a list of encrypted handles to the peer cOApp configured for `dstEid` in a single message.
     * @dev    Forwards `msg.value` as the LayerZero native fee, so the calling entrypoint must be `payable` and
     *         funded with the amount returned by {_quoteSendHandlesToPeer}. The destination receiver gets the
     *         derived handles aligned one-to-one by index. Reverts {NoPeer} if no peer is configured for `dstEid`.
     * @param dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param payload       Opaque app payload; decoded by the destination receiver.
     * @param handles       Raw `bytes32` handles to bridge (mixing encrypted types is fine); this contract must hold
     *                      ACL allowance on each. Should be non-empty and contain a maximum of 32 handles.
     * @param lzComposeGas  Gas budget for the destination app callback `onConfidentialBridgeReceived` (lzCompose leg). The amount needed is
     *                      app-specific, apps should size it for their `onConfidentialBridgeReceived` workload.
     * @return guid         The LayerZero message guid.
     * @return nonce        The LayerZero message nonce.
     */
    function _sendHandlesToPeer(
        uint32 dstEid,
        bytes memory payload,
        bytes32[] memory handles,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = FHE.sendLZConfidentialBridge(
            dstEid,
            _getPeerOrRevert(dstEid),
            payload,
            handles,
            lzComposeGas,
            msg.value
        );
    }

    /**
     * @notice Quotes the LayerZero native fee to bridge a single handle to the peer configured for `dstEid`.
     * @dev    Call this function before calling {_sendSingleHandleToPeer} and forward the result as `msg.value`.
     * @dev    Reverts {NoPeer} if no peer is configured for `dstEid`.
     * @dev    See {FHE-quoteLZConfidentialBridge} for the race-condition caveat.
     * @param  dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param  payload       Opaque app payload matching the intended send (only its length affects the fee).
     * @param  lzComposeGas  Gas budget for the destination app callback `onConfidentialBridgeReceived` (lzCompose leg). The amount needed is
     *                       app-specific, apps should size it for their `onConfidentialBridgeReceived` workload.
     * @return nativeFee     The native fee to forward as `msg.value` to {_sendSingleHandleToPeer}.
     */
    function _quoteSendSingleHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        uint64 lzComposeGas
    ) internal view returns (uint256 nativeFee) {
        bytes32[] memory handles = new bytes32[](1); // null handle works for the quote
        nativeFee = FHE.quoteLZConfidentialBridge(
            dstEid,
            address(this),
            _getPeerOrRevert(dstEid),
            payload,
            handles,
            lzComposeGas
        );
    }

    /**
     * @notice Quotes the LayerZero native fee for a multi-handle send to the peer configured for `dstEid`.
     * @dev    Call before {_sendHandlesToPeer} and forward the result as `msg.value`. Null handles are used for
     *         the quote (the fee depends only on the number of handles, not their values), so pass `numHandles`
     *         equal to the size of the list you intend to send. Reverts {NoPeer} if no peer is configured for
     *         `dstEid`. See {FHE-quoteLZConfidentialBridge} for the race-condition caveat.
     * @param dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param payload       Opaque app payload matching the intended send (only its length affects the fee).
     * @param numHandles    Number of handles the intended send will carry (must match for a correct estimate).
     * @param lzComposeGas  Gas budget for the destination app callback `onConfidentialBridgeReceived` (lzCompose leg). The amount needed is
     *                      app-specific, apps should size it for their `onConfidentialBridgeReceived` workload.
     * @return nativeFee    The native fee to forward as `msg.value` to {_sendHandlesToPeer}.
     */
    function _quoteSendHandlesToPeer(
        uint32 dstEid,
        bytes memory payload,
        uint256 numHandles,
        uint64 lzComposeGas
    ) internal view returns (uint256 nativeFee) {
        bytes32[] memory handles = new bytes32[](numHandles); // null handles work for the quote
        nativeFee = FHE.quoteLZConfidentialBridge(
            dstEid,
            address(this),
            _getPeerOrRevert(dstEid),
            payload,
            handles,
            lzComposeGas
        );
    }
}

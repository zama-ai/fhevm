// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, euint64} from "../FHE.sol";
import {MessagingFee, MessagingReceipt} from "./IConfidentialBridge.sol";
import {ConfidentialOAppCore} from "./ConfidentialOAppCore.sol";

/**
 * @title   ConfidentialOAppSender
 * @notice  Send side of a confidential omnichain app. Looks up the destination peer from the
 *          shared {ConfidentialOAppCore} registry and bridges encrypted handles to it through
 *          `FHE.bridge`, so subclasses never deal with the bridge contract, LayerZero options,
 *          or peer encoding directly.
 * @dev     The destination is taken from {peers} (a `bytes32`, so non-EVM peers work too).
 *          Quote the fee with {_quoteBridge} and forward it as `msg.value` to {_bridge}.
 */
abstract contract ConfidentialOAppSender is ConfidentialOAppCore {
    /**
     * @notice Bridge a single encrypted `euint64` handle to the peer on `dstEid`.
     * @param dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param payload       Opaque app payload (the receiver decodes it).
     * @param handle        The encrypted value to bridge; the caller must hold ACL allowance on it.
     * @param lzComposeGas  Gas budget for the destination receive callback (lzCompose leg).
     * @param nativeFee     LayerZero native fee to forward (query via {_quoteBridge}).
     */
    function _bridge(
        uint32 dstEid,
        bytes memory payload,
        euint64 handle,
        uint128 lzComposeGas,
        uint256 nativeFee
    ) internal returns (MessagingReceipt memory) {
        bytes32 peer = _getPeerOrRevert(dstEid);
        bytes32[] memory handleList = new bytes32[](1);
        handleList[0] = euint64.unwrap(handle);
        return FHE.bridge(dstEid, peer, payload, handleList, lzComposeGas, "", nativeFee);
    }

    /// @notice Quotes the native fee for the matching {_bridge} call.
    function _quoteBridge(
        uint32 dstEid,
        bytes memory payload,
        euint64 handle,
        uint128 lzComposeGas
    ) internal view returns (MessagingFee memory) {
        bytes32 peer = _getPeerOrRevert(dstEid);
        bytes32[] memory handleList = new bytes32[](1);
        handleList[0] = euint64.unwrap(handle);
        return FHE.quoteBridge(dstEid, address(this), peer, payload, handleList, lzComposeGas, "");
    }
}

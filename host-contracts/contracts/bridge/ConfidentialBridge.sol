// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {OAppCore} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/OAppCore.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

import {HandlesSender} from "./HandlesSender.sol";
import {HandlesReceiver} from "./HandlesReceiver.sol";

/**
 * @title ConfidentialBridge
 * @notice Sole deployed artifact for confidential handle bridging on a given chain.
 *         Combines the source-side {HandlesSender} mixin (`send` + ACL check + outbound
 *         `BridgeHandle` event) and the destination-side {HandlesReceiver} mixin
 *         (`_lzReceive` + handle derivation + `HandleBridged` event + lzCompose
 *         dispatch). One bridge instance per chain serves both directions: outbound
 *         sends as source, inbound receives as destination.
 *
 * @dev    The base mixins are intentionally abstract; this contract supplies the single
 *         `OAppCore` constructor and the merged `oAppVersion`. ACL and external apps
 *         track this contract via {ACL.getConfidentialBridgeAddress}.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ConfidentialBridge is HandlesSender, HandlesReceiver {
    /// @notice Returned when `dstEids` and `dstChainIds` constructor arrays differ in length.
    error DstChainIdArrayLengthMismatch(uint256 dstEidsLength, uint256 dstChainIdsLength);

    /**
     * @param _lzEndpoint   LayerZero V2 endpoint address on this chain.
     * @param _owner        Initial owner (governance — also authorized to call grantFallback).
     * @param dstEids       LayerZero endpoint ids to seed the dstEid → dstChainId map with.
     *                      May be empty; pairs can also be added later via {setDstChainId}.
     * @param dstChainIds   Destination chain ids paired index-by-index with `dstEids`. Must
     *                      have the same length as `dstEids`.
     */
    constructor(
        address _lzEndpoint,
        address _owner,
        uint32[] memory dstEids,
        uint64[] memory dstChainIds
    ) OAppCore(_lzEndpoint, _owner) Ownable(_owner) {
        if (dstEids.length != dstChainIds.length) {
            revert DstChainIdArrayLengthMismatch(dstEids.length, dstChainIds.length);
        }
        for (uint256 i = 0; i < dstEids.length; i++) {
            _setDstChainId(dstEids[i], dstChainIds[i]);
        }
    }

    /// @notice OApp version tuple — both send (1) and receive (2) paths are active.
    function oAppVersion()
        public
        pure
        override(HandlesSender, HandlesReceiver)
        returns (uint64 senderVersion, uint64 receiverVersion)
    {
        return (1, 2);
    }
}

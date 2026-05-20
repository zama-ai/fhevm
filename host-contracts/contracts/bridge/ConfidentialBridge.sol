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
    /**
     * @param _lzEndpoint  LayerZero V2 endpoint address on this chain.
     * @param _owner       Initial owner (governance — also authorized to call grantFallback).
     */
    constructor(address _lzEndpoint, address _owner) OAppCore(_lzEndpoint, _owner) Ownable(_owner) {}

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

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {OApp, Origin, MessagingFee, MessagingReceipt} from "@layerzerolabs/oapp-evm/contracts/oapp/OApp.sol";
import {ILayerZeroComposer} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroComposer.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @dev Test-only minimal OApp (sender/receiver/composer) proving LocalSimpleMessageLib +
///      EndpointV2Mock + the relay sequence deliver a message end-to-end, without DVN/Executor.
contract MockOApp is OApp, ILayerZeroComposer {
    bytes public lastReceived;
    bytes public lastComposed;

    constructor(address _endpoint, address _delegate) OApp(_endpoint, _delegate) Ownable(_delegate) {}

    function send(
        uint32 _dstEid,
        bytes calldata _message,
        bytes calldata _options
    ) external payable returns (MessagingReceipt memory) {
        return _lzSend(_dstEid, _message, _options, MessagingFee(msg.value, 0), payable(msg.sender));
    }

    function _lzReceive(
        Origin calldata,
        bytes32 _guid,
        bytes calldata _message,
        address,
        bytes calldata
    ) internal override {
        lastReceived = _message;
        // Queue a compose to self so the compose relay step is exercised too.
        endpoint.sendCompose(address(this), _guid, 0, _message);
    }

    function lzCompose(
        address,
        bytes32,
        bytes calldata _message,
        address,
        bytes calldata
    ) external payable override {
        require(msg.sender == address(endpoint), "only endpoint");
        lastComposed = _message;
    }
}

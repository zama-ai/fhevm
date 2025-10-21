// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import { IOAppComposer } from "@layerzerolabs/oapp-evm/contracts/oapp/interfaces/IOAppComposer.sol";

contract OFTComposerMock is IOAppComposer {
    // default empty values for testing a lzCompose received message
    address public from;
    bytes32 public guid;
    bytes public message;
    address public executor;
    bytes public extraData;

    function lzCompose(
        address _from,
        bytes32 _guid,
        bytes calldata _message,
        address _executor,
        bytes calldata /*_extraData*/
    ) external payable {
        from = _from;
        guid = _guid;
        message = _message;
        executor = _executor;
        extraData = _message;
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IPauserSet } from "@fhevm/host-contracts/contracts/interfaces/IPauserSet.sol";

interface IPausableToken {
    function pauseMinting() external;
}

contract PauserSetWrapper {
    IPauserSet public immutable PAUSER_SET;
    address public immutable CONTRACT_TARGET;
    bytes4 public immutable FUNCTION_SELECTOR;
    string public FUNCTION_SIGNATURE;

    error ExecutionFailed(bytes errorData);
    error NoCodeAtTarget(address target);
    error SenderNotPauser();

    constructor(address _target, string memory _functionSignature, address _pauserSet) {
        if (_target.code.length == 0) revert NoCodeAtTarget(_target);
        CONTRACT_TARGET = _target;
        FUNCTION_SIGNATURE = _functionSignature;
        FUNCTION_SELECTOR = bytes4(keccak256(bytes(_functionSignature)));
        PAUSER_SET = IPauserSet(_pauserSet);
    }

    function callFunction(bytes memory args) external payable {
        if (!PAUSER_SET.isPauser(msg.sender)) revert SenderNotPauser();
        bytes memory data = abi.encodePacked(FUNCTION_SELECTOR, args);
        (bool success, bytes memory errorData) = CONTRACT_TARGET.call{ value: msg.value }(data);
        if (!success) {
            revert ExecutionFailed(errorData);
        }
    }
}

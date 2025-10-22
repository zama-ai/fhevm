// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IPauserSet} from "@fhevm/host-contracts/contracts/interfaces/IPauserSet.sol";

contract PauserSetWrapper {
  IPauserSet immutable public PAUSER_SET;

  error ExecutionFailed(bytes errorData);
  error SenderNotPauser();

  constructor(address _pauserSet) {
    PAUSER_SET = IPauserSet(_pauserSet);
  }

  function execute(address callee, bytes memory data) external payable {
    if(!PAUSER_SET.isPauser(msg.sender)) revert SenderNotPauser();
    (bool success, bytes memory err) = callee.call{ value: msg.value }(data);
        if (!success) {
            revert ExecutionFailed(err);
        }
  }
}

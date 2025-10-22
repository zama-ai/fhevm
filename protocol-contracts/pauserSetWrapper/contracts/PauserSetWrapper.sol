// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IPauserSet } from "@fhevm/host-contracts/contracts/interfaces/IPauserSet.sol";

interface IPausableToken {
    function pauseMinting() external;
}

contract PauserSetWrapper {
    IPausableToken public immutable PAUSABLE_TOKEN;
    IPauserSet public immutable PAUSER_SET;

    error SenderNotPauser();

    constructor(address _pausableToken, address _pauserSet) {
        PAUSABLE_TOKEN = IPausableToken(_pausableToken);
        PAUSER_SET = IPauserSet(_pauserSet);
    }

    function pauseMinting() external payable {
        if (!PAUSER_SET.isPauser(msg.sender)) revert SenderNotPauser();
        PAUSABLE_TOKEN.pauseMinting();
    }
}

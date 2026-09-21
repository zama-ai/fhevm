// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.22;

contract SelfDestructableMock {
    constructor(address _target) payable {
        selfdestruct(payable(_target));
    }
}

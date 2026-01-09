// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

interface Vm {
    function warp(uint256 newTimestamp) external;
    function prank(address newSender) external;
    function expectRevert(bytes calldata) external;
    function expectEmit(bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData) external;
    function etch(address where, bytes calldata code) external;
    function store(address target, bytes32 slot, bytes32 value) external;
}

contract TestBase {
    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
    bytes32 internal constant INITIALIZABLE_STORAGE_SLOT =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;

    function assertEq(uint256 a, uint256 b) internal pure {
        if (a != b) {
            revert("assertEq(uint256) failed");
        }
    }

    function assertEq(address a, address b) internal pure {
        if (a != b) {
            revert("assertEq(address) failed");
        }
    }

    function assertEq(bytes32 a, bytes32 b) internal pure {
        if (a != b) {
            revert("assertEq(bytes32) failed");
        }
    }

    function setInitializedVersion(address target, uint64 version) internal {
        vm.store(target, INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(version)));
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {CiphertextRoots} from "@fhevm-host-contracts/contracts/CiphertextRoots.sol";
import {ICiphertextRoots} from "@fhevm-host-contracts/contracts/interfaces/ICiphertextRoots.sol";
import {CiphertextRootsUpgradedExample} from "@fhevm-host-contracts/examples/CiphertextRootsUpgradedExample.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {protocolConfigAdd, ciphertextRootsAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ProtocolConfigMock} from "./ProtocolConfigMock.sol";

contract CiphertextRootsTest is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);
    address internal constant outsider = address(0xBEEF);
    uint64 internal constant CHAIN_ID = 11155111;
    uint64 internal constant OTHER_CHAIN_ID = 137;
    uint256 internal constant START_BLOCK = 1000;
    uint256 internal constant EPOCH_LENGTH = 100;
    uint256 internal constant EPOCH = 7;
    uint256 internal constant NODE_COUNT = 5;
    uint256 internal constant MAJORITY_THRESHOLD = 3;
    /// @dev The number of nodes in the small contexts some tests define.
    uint256 internal constant SMALL_NODE_COUNT = 3;
    /// @dev RFC 039 coprocessor context IDs: type byte 0x09 followed by a counter.
    uint256 internal constant COPROCESSOR_CONTEXT_COUNTER_BASE = uint256(9) << 248;
    uint256 internal constant CONTEXT_1 = COPROCESSOR_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant CONTEXT_2 = COPROCESSOR_CONTEXT_COUNTER_BASE + 2;
    uint256 internal constant CONTEXT_3 = COPROCESSOR_CONTEXT_COUNTER_BASE + 3;
    bytes32 internal constant ROOT_1 = keccak256("root-1");
    bytes32 internal constant ROOT_2 = keccak256("root-2");

    address internal nodeA = address(0xA1);
    address internal nodeB = address(0xA2);
    address internal nodeC = address(0xA3);
    address internal nodeD = address(0xA4);
    address internal nodeE = address(0xA5);
    address internal nodeF = address(0xA6);

    CiphertextRoots internal ciphertextRoots;
    ProtocolConfigMock internal protocolConfigMock;

    function setUp() public {
        _deployACL(owner);
        deployCodeTo("test/ciphertextRoots/ProtocolConfigMock.sol:ProtocolConfigMock", protocolConfigAdd);
        protocolConfigMock = ProtocolConfigMock(protocolConfigAdd);
        vm.label(protocolConfigAdd, "ProtocolConfigMock");
        (ciphertextRoots, ) = _deployCiphertextRoots(owner);

        // Context 1: `NODE_COUNT` nodes, majority threshold `MAJORITY_THRESHOLD`.
        protocolConfigMock.defineContext(CONTEXT_1, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), MAJORITY_THRESHOLD);
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(CHAIN_ID, START_BLOCK, EPOCH_LENGTH);
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    function _nodes(address[NODE_COUNT] memory fixedNodes) internal pure returns (address[] memory nodes) {
        nodes = new address[](fixedNodes.length);
        for (uint256 i = 0; i < fixedNodes.length; i++) {
            nodes[i] = fixedNodes[i];
        }
    }

    function _nodes(address[NODE_COUNT + 1] memory fixedNodes) internal pure returns (address[] memory nodes) {
        nodes = new address[](fixedNodes.length);
        for (uint256 i = 0; i < fixedNodes.length; i++) {
            nodes[i] = fixedNodes[i];
        }
    }

    function _nodes(address[SMALL_NODE_COUNT] memory fixedNodes) internal pure returns (address[] memory nodes) {
        nodes = new address[](fixedNodes.length);
        for (uint256 i = 0; i < fixedNodes.length; i++) {
            nodes[i] = fixedNodes[i];
        }
    }

    function _post(address sender, uint256 epoch, bytes32 root) internal {
        vm.prank(sender);
        ciphertextRoots.postRoot(CHAIN_ID, epoch, root);
    }

    function _expectRecorded(uint256 epoch, bytes32 root, uint256 contextId) internal {
        vm.expectEmit(true, true, false, true, address(ciphertextRoots));
        emit ICiphertextRoots.EpochRootRecorded(CHAIN_ID, epoch, root, contextId);
    }

    function _assertRecorded(uint256 epoch, bytes32 expectedRoot) internal view {
        assertEq(ciphertextRoots.getEpochRoot(CHAIN_ID, epoch), expectedRoot);
    }

    function _assertNotRecorded(uint256 epoch) internal {
        _assertNotRecorded(CHAIN_ID, epoch);
    }

    function _assertNotRecorded(uint64 chainId, uint256 epoch) internal {
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochRoot.selector, chainId, epoch));
        ciphertextRoots.getEpochRoot(chainId, epoch);
    }

    function _record(uint256 epoch) internal {
        _post(nodeA, epoch, ROOT_1);
        _post(nodeB, epoch, ROOT_1);
        _post(nodeC, epoch, ROOT_1);
        _assertRecorded(epoch, ROOT_1);
    }

    function _setEpochLength(uint256 fromEpoch, uint256 epochLength) internal {
        vm.prank(owner);
        ciphertextRoots.setEpochLength(CHAIN_ID, fromEpoch, epochLength);
    }

    function _assertEntry(
        ICiphertextRoots.EpochScheduleEntry memory entry,
        uint256 fromEpoch,
        uint256 length
    ) internal pure {
        assertEq(entry.fromEpoch, fromEpoch);
        assertEq(entry.epochLength, length);
    }

    function _assertEpochBlocks(
        uint64 chainId,
        uint256 epoch,
        uint256 expectedStartBlock,
        uint256 expectedEndBlock
    ) internal view {
        (uint256 startBlock, uint256 endBlock) = ciphertextRoots.getEpochBlocks(chainId, epoch);
        assertEq(startBlock, expectedStartBlock);
        assertEq(endBlock, expectedEndBlock);
    }

    // -----------------------------------------------------------------------
    // Initialization and upgrade tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public {
        assertEq(address(ciphertextRoots), ciphertextRootsAdd);
        assertEq(ciphertextRoots.getVersion(), "CiphertextRoots v0.1.0");
        _assertNotRecorded(EPOCH);
    }

    function test_revertDoubleInitFromEmptyProxy() public {
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        ciphertextRoots.initializeFromEmptyProxy();
    }

    function test_storageLocationPinned() public view {
        // `chains[CHAIN_ID]` lives at the mapping slot under the storage location, and the
        // schedule's `startBlock` is its first field.
        bytes32 location = 0x3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f12000;
        bytes32 slot = keccak256(abi.encode(uint256(CHAIN_ID), location));
        uint256 stored = uint256(vm.load(address(ciphertextRoots), slot));
        assertEq(stored, ciphertextRoots.getEpochSchedule(CHAIN_ID).startBlock);
        assertEq(stored, START_BLOCK);
    }

    function test_revertUpgradeNotOwner() public {
        address newImplementation = address(new CiphertextRootsUpgradedExample());
        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, outsider));
        EmptyUUPSProxy(ciphertextRootsAdd).upgradeToAndCall(newImplementation, "");
    }

    function test_upgradePreservesEntries() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
        _setEpochLength(EPOCH + 1, 50);

        address newImplementation = address(new CiphertextRootsUpgradedExample());
        vm.prank(owner);
        EmptyUUPSProxy(ciphertextRootsAdd).upgradeToAndCall(newImplementation, "");

        assertEq(ciphertextRoots.getVersion(), "CiphertextRoots v0.2.0");
        ICiphertextRoots.EpochSchedule memory schedule = ciphertextRoots.getEpochSchedule(CHAIN_ID);
        assertEq(schedule.startBlock, START_BLOCK);
        assertEq(schedule.entries.length, 2);
        _assertEntry(schedule.entries[0], 0, EPOCH_LENGTH);
        _assertEntry(schedule.entries[1], EPOCH + 1, 50);
        _assertRecorded(EPOCH, ROOT_1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_1).length, 3);
    }

    // -----------------------------------------------------------------------
    // Epoch schedule tests
    // -----------------------------------------------------------------------

    function test_setEpochScheduleEmitsAndStores() public {
        vm.expectEmit(true, false, false, true, address(ciphertextRoots));
        emit ICiphertextRoots.EpochScheduleSet(OTHER_CHAIN_ID, 0, 50);
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, 0, 50);

        ICiphertextRoots.EpochSchedule memory schedule = ciphertextRoots.getEpochSchedule(OTHER_CHAIN_ID);
        assertEq(schedule.startBlock, 0);
        assertEq(schedule.entries.length, 1);
        _assertEntry(schedule.entries[0], 0, 50);
    }

    function test_revertSetEpochScheduleNotOwner() public {
        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, outsider));
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, START_BLOCK, EPOCH_LENGTH);
    }

    function test_revertSetEpochScheduleNullChainId() public {
        vm.prank(owner);
        vm.expectRevert(ICiphertextRoots.InvalidChainId.selector);
        ciphertextRoots.setEpochSchedule(0, START_BLOCK, EPOCH_LENGTH);
    }

    function test_revertSetEpochScheduleNullEpochLength() public {
        vm.prank(owner);
        vm.expectRevert(ICiphertextRoots.InvalidEpochLength.selector);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, START_BLOCK, 0);
    }

    function test_revertSetEpochScheduleAlreadySet() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.EpochScheduleAlreadySet.selector, CHAIN_ID));
        ciphertextRoots.setEpochSchedule(CHAIN_ID, START_BLOCK + 1, EPOCH_LENGTH + 1);
    }

    function test_revertGetEpochScheduleUnset() public {
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID));
        ciphertextRoots.getEpochSchedule(OTHER_CHAIN_ID);
    }

    // -----------------------------------------------------------------------
    // Epoch length tests
    // -----------------------------------------------------------------------

    /// @dev Epochs of 5 blocks from block 10, then of 20 from epoch 5, then of 10 from epoch 8.
    function test_setEpochLengthEmitsAndStores() public {
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, 10, 5);
        vm.expectEmit(true, false, false, true, address(ciphertextRoots));
        emit ICiphertextRoots.EpochLengthSet(OTHER_CHAIN_ID, 5, 20);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 5, 20);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 8, 10);

        ICiphertextRoots.EpochSchedule memory schedule = ciphertextRoots.getEpochSchedule(OTHER_CHAIN_ID);
        assertEq(schedule.startBlock, 10);
        assertEq(schedule.entries.length, 3);
        _assertEntry(schedule.entries[0], 0, 5);
        _assertEntry(schedule.entries[1], 5, 20);
        _assertEntry(schedule.entries[2], 8, 10);
    }

    function test_revertSetEpochLengthNotOwner() public {
        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, outsider));
        ciphertextRoots.setEpochLength(CHAIN_ID, EPOCH, 50);
    }

    function test_revertSetEpochLengthNoEpochSchedule() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID));
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, EPOCH, 50);
    }

    function test_revertSetEpochLengthNullEpochLength() public {
        vm.prank(owner);
        vm.expectRevert(ICiphertextRoots.InvalidEpochLength.selector);
        ciphertextRoots.setEpochLength(CHAIN_ID, EPOCH, 0);
    }

    function test_revertSetEpochLengthNotAfterLast() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, 0, 1));
        ciphertextRoots.setEpochLength(CHAIN_ID, 0, 50);

        _setEpochLength(5, 50);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, 5, 6));
        ciphertextRoots.setEpochLength(CHAIN_ID, 5, 60);
    }

    /// @dev Roots can be recorded out of order: the highest recorded epoch is the bound.
    function test_revertSetEpochLengthNotAfterHighestRecordedEpoch() public {
        _record(EPOCH);
        _record(EPOCH - 4);

        // Above the epoch recorded last, but not above the highest recorded one.
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, EPOCH - 3, EPOCH + 1)
        );
        ciphertextRoots.setEpochLength(CHAIN_ID, EPOCH - 3, 50);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, EPOCH, EPOCH + 1));
        ciphertextRoots.setEpochLength(CHAIN_ID, EPOCH, 50);

        // The first epoch above the highest recorded one is accepted.
        vm.prank(owner);
        ciphertextRoots.setEpochLength(CHAIN_ID, EPOCH + 1, 50);
    }

    function test_removeEpochLengthEmitsAndStores() public {
        _setEpochLength(5, 50);
        _setEpochLength(8, 60);
        _setEpochLength(12, 70);

        // Removing a middle one keeps the others in order.
        vm.expectEmit(true, false, false, true, address(ciphertextRoots));
        emit ICiphertextRoots.EpochLengthRemoved(CHAIN_ID, 8);
        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, 8);
        ICiphertextRoots.EpochSchedule memory schedule = ciphertextRoots.getEpochSchedule(CHAIN_ID);
        assertEq(schedule.entries.length, 3);
        _assertEntry(schedule.entries[0], 0, EPOCH_LENGTH);
        _assertEntry(schedule.entries[1], 5, 50);
        _assertEntry(schedule.entries[2], 12, 70);

        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, 12);
        schedule = ciphertextRoots.getEpochSchedule(CHAIN_ID);
        assertEq(schedule.entries.length, 2);
        _assertEntry(schedule.entries[1], 5, 50);
    }

    /// @dev Changing the epoch length of a pending entry is a removal and a new set.
    function test_removeEpochLengthThenSetSameFromEpoch() public {
        _setEpochLength(5, 50);
        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, 5);
        _setEpochLength(5, 60);

        ICiphertextRoots.EpochSchedule memory schedule = ciphertextRoots.getEpochSchedule(CHAIN_ID);
        assertEq(schedule.entries.length, 2);
        _assertEntry(schedule.entries[1], 5, 60);
    }

    function test_revertRemoveEpochLengthNotOwner() public {
        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, outsider));
        ciphertextRoots.removeEpochLength(CHAIN_ID, 5);
    }

    function test_revertRemoveEpochLengthNoEpochSchedule() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID));
        ciphertextRoots.removeEpochLength(OTHER_CHAIN_ID, 5);
    }

    function test_revertRemoveEpochLengthNotFound() public {
        _setEpochLength(5, 50);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.EpochLengthNotFound.selector, CHAIN_ID, 6));
        ciphertextRoots.removeEpochLength(CHAIN_ID, 6);

        // The first entry, from epoch 0, is never removed.
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.EpochLengthNotFound.selector, CHAIN_ID, 0));
        ciphertextRoots.removeEpochLength(CHAIN_ID, 0);
    }

    function test_revertRemoveEpochLengthNotAfterHighestRecordedEpoch() public {
        _setEpochLength(EPOCH - 2, 50);
        _setEpochLength(EPOCH + 2, 60);
        _record(EPOCH);

        // The entry below the recorded epoch cannot be removed: the recorded root was built under it.
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, EPOCH - 2, EPOCH + 1)
        );
        ciphertextRoots.removeEpochLength(CHAIN_ID, EPOCH - 2);

        // The entry above the recorded epoch can.
        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, EPOCH + 2);
    }

    /// @dev The boundary case: the entry that starts exactly at the highest recorded epoch
    /// is the one that epoch's root was built under, so it cannot be removed.
    function test_revertRemoveEpochLengthAtHighestRecordedEpoch() public {
        _setEpochLength(EPOCH, 50);
        _record(EPOCH);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, EPOCH, EPOCH + 1));
        ciphertextRoots.removeEpochLength(CHAIN_ID, EPOCH);
    }

    /// @dev Adding and removing schedule entries does not change which blocks an earlier epoch
    /// covers.
    function test_entriesLeaveEarlierEpochsUnchanged() public {
        (uint256 startBlock, uint256 endBlock) = ciphertextRoots.getEpochBlocks(CHAIN_ID, EPOCH);

        _setEpochLength(EPOCH + 1, 60);
        _setEpochLength(EPOCH + 2, 70);
        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, EPOCH + 1);

        (uint256 startBlockAfter, uint256 endBlockAfter) = ciphertextRoots.getEpochBlocks(CHAIN_ID, EPOCH);
        assertEq(startBlockAfter, startBlock);
        assertEq(endBlockAfter, endBlock);
    }

    // -----------------------------------------------------------------------
    // Epoch of block tests
    // -----------------------------------------------------------------------

    function test_getEpochOfBlockSingleEpochLength() public {
        vm.expectRevert(
            abi.encodeWithSelector(ICiphertextRoots.BlockBeforeStartBlock.selector, CHAIN_ID, START_BLOCK - 1)
        );
        ciphertextRoots.getEpochOfBlock(CHAIN_ID, START_BLOCK - 1);
        assertEq(ciphertextRoots.getEpochOfBlock(CHAIN_ID, START_BLOCK), 0);
        assertEq(ciphertextRoots.getEpochOfBlock(CHAIN_ID, START_BLOCK + EPOCH_LENGTH - 1), 0);
        assertEq(ciphertextRoots.getEpochOfBlock(CHAIN_ID, START_BLOCK + EPOCH_LENGTH), 1);
        assertEq(ciphertextRoots.getEpochOfBlock(CHAIN_ID, START_BLOCK + EPOCH * EPOCH_LENGTH + 1), EPOCH);
    }

    /// @dev Epochs of 5 blocks from block 10, then of 20 from epoch 5 (block 35), then of 10 from
    /// epoch 8 (block 95).
    function test_getEpochOfBlockSeveralEpochLengths() public {
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, 10, 5);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 5, 20);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 8, 10);

        // The schedule, with the blocks checked below marked:
        //
        //  from epoch 0, 5 blocks   from epoch 5, 20 blocks          from epoch 8, 10 blocks
        //  |                        |                                |
        //  | e0 | e1 | e2 | e3 | e4 |    e5    |    e6    |    e7    |  e8  |  e9  | ...
        //  10   15   20   25   30   35         55         75         95     105    115   <- first block
        //  ^                       ^^                               ^^      ^
        //  10                    34,35                            94,95     105

        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 10), 0);

        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 34), 4);
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 35), 5);

        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 94), 7);
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 95), 8);

        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 105), 9);
    }

    function testFuzz_getEpochOfBlockMatchesFormula(uint64 offset) public view {
        assertEq(ciphertextRoots.getEpochOfBlock(CHAIN_ID, START_BLOCK + offset), uint256(offset) / EPOCH_LENGTH);
    }

    /// @dev Removing the 20-block entry gives its epochs back to the 5-block entry before it.
    function test_getEpochOfBlockAfterRemoval() public {
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, 10, 5);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 5, 20);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 8, 10);
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 54), 5);

        vm.prank(owner);
        ciphertextRoots.removeEpochLength(OTHER_CHAIN_ID, 5);

        // Epochs 0 to 7 are now 5 blocks each, so epoch 8 covers blocks 50 to 59.
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 54), 8);
        (uint256 startBlock, uint256 endBlock) = ciphertextRoots.getEpochBlocks(OTHER_CHAIN_ID, 8);
        assertEq(startBlock, 50);
        assertEq(endBlock, 59);

        // A block before the removed entry is in the same epoch as before.
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 34), 4);
    }

    function test_revertGetEpochOfBlockNoEpochSchedule() public {
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID));
        ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, 0);
    }

    function test_getEpochBlocksSingleEpochLength() public view {
        // Epoch 0 starts at the start block.
        (uint256 startBlock, uint256 endBlock) = ciphertextRoots.getEpochBlocks(CHAIN_ID, 0);
        assertEq(startBlock, START_BLOCK);
        assertEq(endBlock, START_BLOCK + EPOCH_LENGTH - 1);

        // Epoch e covers S + e * M through S + (e + 1) * M - 1 (RFC 034).
        (startBlock, endBlock) = ciphertextRoots.getEpochBlocks(CHAIN_ID, EPOCH);
        assertEq(startBlock, START_BLOCK + EPOCH * EPOCH_LENGTH);
        assertEq(endBlock, START_BLOCK + (EPOCH + 1) * EPOCH_LENGTH - 1);
    }

    /// @dev For any epoch, the two views agree.
    function testFuzz_getEpochBlocksMatchesGetEpochOfBlock(uint16 epoch) public {
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, 10, 5);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 5, 20);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 8, 10);

        (uint256 startBlock, uint256 endBlock) = ciphertextRoots.getEpochBlocks(OTHER_CHAIN_ID, epoch);

        // The first and the last block of the epoch are in the epoch.
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, startBlock), epoch);
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, endBlock), epoch);

        // The block after the epoch is in the next epoch.
        assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, endBlock + 1), uint256(epoch) + 1);

        // The block before the epoch is in the previous epoch. Epoch 0 has none.
        if (epoch > 0) {
            assertEq(ciphertextRoots.getEpochOfBlock(OTHER_CHAIN_ID, startBlock - 1), uint256(epoch) - 1);
        }
    }

    function test_getEpochBlocksSeveralEpochLengths() public {
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, 10, 5);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 5, 20);
        vm.prank(owner);
        ciphertextRoots.setEpochLength(OTHER_CHAIN_ID, 8, 10);

        // The schedule, with the epochs checked below marked:
        //
        //  from epoch 0, 5 blocks   from epoch 5, 20 blocks          from epoch 8, 10 blocks
        //  |                        |                                |
        //  | e0 | e1 | e2 | e3 | e4 |    e5    |    e6    |    e7    |  e8  |  e9  | ...
        //  10   15   20   25   30   35         55         75         95     105    115   <- first block
        //    ^                   ^       ^                     ^        ^

        _assertEpochBlocks(OTHER_CHAIN_ID, 0, 10, 14);

        _assertEpochBlocks(OTHER_CHAIN_ID, 4, 30, 34);
        _assertEpochBlocks(OTHER_CHAIN_ID, 5, 35, 54);

        _assertEpochBlocks(OTHER_CHAIN_ID, 7, 75, 94);
        _assertEpochBlocks(OTHER_CHAIN_ID, 8, 95, 104);
    }

    function test_revertGetEpochBlocksNoEpochSchedule() public {
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID));
        ciphertextRoots.getEpochBlocks(OTHER_CHAIN_ID, 0);
    }

    /// @dev Every getter of roots and their senders requires a host chain with a schedule.
    function test_revertRootGettersNoEpochSchedule() public {
        bytes memory expected = abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID);
        vm.expectRevert(expected);
        ciphertextRoots.getEpochRoot(OTHER_CHAIN_ID, EPOCH);
        vm.expectRevert(expected);
        ciphertextRoots.getHighestRecordedEpoch(OTHER_CHAIN_ID);
        vm.expectRevert(expected);
        ciphertextRoots.getPostedRoots(OTHER_CHAIN_ID, EPOCH);
        vm.expectRevert(expected);
        ciphertextRoots.getRootSenders(OTHER_CHAIN_ID, EPOCH, ROOT_1);
    }

    function test_revertGetHighestRecordedEpochNoneRecorded() public {
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochRootRecorded.selector, CHAIN_ID));
        ciphertextRoots.getHighestRecordedEpoch(CHAIN_ID);
    }

    function test_getHighestRecordedEpochNotLoweredByLowerEpoch() public {
        _record(EPOCH);
        _record(EPOCH - 4);
        assertEq(ciphertextRoots.getHighestRecordedEpoch(CHAIN_ID), EPOCH);
    }

    function test_getHighestRecordedEpochRaisedByHigherEpoch() public {
        _record(EPOCH);
        _record(EPOCH + 2);
        assertEq(ciphertextRoots.getHighestRecordedEpoch(CHAIN_ID), EPOCH + 2);
    }

    function test_getHighestRecordedEpochIsPerChain() public {
        _record(EPOCH);
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, START_BLOCK, EPOCH_LENGTH);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochRootRecorded.selector, OTHER_CHAIN_ID));
        ciphertextRoots.getHighestRecordedEpoch(OTHER_CHAIN_ID);
    }

    /// @dev Epoch 0 is an ordinary epoch: once it has a root, it is the highest recorded one.
    function test_getHighestRecordedEpochZero() public {
        _record(0);
        assertEq(ciphertextRoots.getHighestRecordedEpoch(CHAIN_ID), 0);
    }

    // -----------------------------------------------------------------------
    // Post tests
    // -----------------------------------------------------------------------

    function test_postRootEmitsAndStores() public {
        vm.expectEmit(true, true, true, true, address(ciphertextRoots));
        emit ICiphertextRoots.RootPosted(CHAIN_ID, EPOCH, nodeA, ROOT_1);
        _post(nodeA, EPOCH, ROOT_1);

        bytes32[] memory roots = ciphertextRoots.getPostedRoots(CHAIN_ID, EPOCH);
        assertEq(roots.length, 1);
        assertEq(roots[0], ROOT_1);
        address[] memory senders = ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_1);
        assertEq(senders.length, 1);
        assertEq(senders[0], nodeA);
        _assertNotRecorded(EPOCH);
    }

    function test_postRootRecordsAtMajorityThreshold() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _assertNotRecorded(EPOCH);

        _expectRecorded(EPOCH, ROOT_1, CONTEXT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    function test_postRootTracksDistinctRoots() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_2);
        _post(nodeC, EPOCH, ROOT_1);

        bytes32[] memory roots = ciphertextRoots.getPostedRoots(CHAIN_ID, EPOCH);
        assertEq(roots.length, 2);
        assertEq(roots[0], ROOT_1);
        assertEq(roots[1], ROOT_2);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_1).length, 2);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_2).length, 1);
        _assertNotRecorded(EPOCH);
    }

    function test_postRootFirstRootToReachThresholdIsFinal() public {
        // Six nodes, so that two roots can each get three posts. CONTEXT_1 created at setUp is destroyed.
        protocolConfigMock.defineContext(
            CONTEXT_2,
            _nodes([nodeA, nodeB, nodeC, nodeD, nodeE, nodeF]),
            MAJORITY_THRESHOLD
        );
        protocolConfigMock.destroyContext(CONTEXT_1);

        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);

        // A second root reaching the threshold is stored as evidence but never replaces the recorded one.
        _post(nodeD, EPOCH, ROOT_2);
        _post(nodeE, EPOCH, ROOT_2);
        _post(nodeF, EPOCH, ROOT_2);
        _assertRecorded(EPOCH, ROOT_1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_2).length, 3);
    }

    function test_postRootAcceptedAfterEpochRootRecorded() public {
        _record(EPOCH);

        // A late post is still accepted and stored.
        vm.expectEmit(true, true, true, true, address(ciphertextRoots));
        emit ICiphertextRoots.RootPosted(CHAIN_ID, EPOCH, nodeD, ROOT_1);
        _post(nodeD, EPOCH, ROOT_1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_1).length, 4);
        _assertRecorded(EPOCH, ROOT_1);
    }

    function test_postRootAnyEpochAccepted() public {
        _post(nodeA, 0, ROOT_1);
        _post(nodeA, type(uint256).max, ROOT_1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, 0, ROOT_1).length, 1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, type(uint256).max, ROOT_1).length, 1);
    }

    function test_postRootPreviousContextMemberAllowed() public {
        // A is only in the previous context, which is still valid.
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), MAJORITY_THRESHOLD);
        _post(nodeA, EPOCH, ROOT_1);

        address[] memory senders = ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_1);
        assertEq(senders.length, 1);
        assertEq(senders[0], nodeA);
    }

    function test_revertPostRootDestroyedPreviousContextMember() public {
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), MAJORITY_THRESHOLD);
        protocolConfigMock.destroyContext(CONTEXT_1);
        vm.prank(nodeA);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NotCoprocessorTxSender.selector, nodeA));
        ciphertextRoots.postRoot(CHAIN_ID, EPOCH, ROOT_1);
    }

    function test_revertPostRootOlderContextMember() public {
        // Context 1 stays valid but is neither current nor previous once context 3 is current.
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), MAJORITY_THRESHOLD);
        protocolConfigMock.defineContext(CONTEXT_3, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), MAJORITY_THRESHOLD);
        vm.prank(nodeA);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NotCoprocessorTxSender.selector, nodeA));
        ciphertextRoots.postRoot(CHAIN_ID, EPOCH, ROOT_1);
    }

    function test_revertPostRootNoEpochSchedule() public {
        vm.prank(nodeA);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID));
        ciphertextRoots.postRoot(OTHER_CHAIN_ID, EPOCH, ROOT_1);
    }

    function test_revertPostRootNotCoprocessorTxSender() public {
        vm.prank(outsider);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NotCoprocessorTxSender.selector, outsider));
        ciphertextRoots.postRoot(CHAIN_ID, EPOCH, ROOT_1);
    }

    function test_revertPostRootAlreadyPosted() public {
        _post(nodeA, EPOCH, ROOT_1);
        vm.prank(nodeA);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.RootAlreadyPosted.selector, CHAIN_ID, EPOCH, nodeA));
        ciphertextRoots.postRoot(CHAIN_ID, EPOCH, ROOT_2);
    }

    // -----------------------------------------------------------------------
    // Counting tests
    // -----------------------------------------------------------------------

    /// @dev The example of RFC 034, section "Why membership is read when counting". Five nodes,
    /// threshold 4. A and B post under the context {A, B, C, D, E}. Then E is replaced by F, which
    /// gives the context {A, B, C, D, F}, and C and D post.
    /// If a post counted only for the context it was made under, the old context would have A and
    /// B and the new one C and D, and neither would reach 4. The contract instead checks membership
    /// when it counts: A and B are members of the new context too, so it has A, B, C, D and the root
    /// is recorded.
    function test_countMembershipWhenCounting() public {
        // The old context: {A, B, C, D, E}, threshold 4.
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 4);
        protocolConfigMock.destroyContext(CONTEXT_1);
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);

        // The new context: {A, B, C, D, F}, threshold 4. With C it has A, B, C: not enough yet.
        protocolConfigMock.defineContext(CONTEXT_3, _nodes([nodeA, nodeB, nodeC, nodeD, nodeF]), 4);
        _post(nodeC, EPOCH, ROOT_1);
        _assertNotRecorded(EPOCH);

        // With D it has A, B, C, D: the root is recorded under the new context.
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_3);
        _post(nodeD, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev A replaced context keeps counting, with its own members and threshold, until it is
    /// destroyed. Here the current context does not reach its threshold but the previous one does,
    /// so the root is recorded under the previous context.
    function test_countFallsBackToPreviousContext() public {
        // The old context: {A, B, C, D, E}, threshold 4.
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 4);
        protocolConfigMock.destroyContext(CONTEXT_1);
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);

        // The new context: {A, B, C, D, F}, threshold 4. The old one is now the previous context.
        protocolConfigMock.defineContext(CONTEXT_3, _nodes([nodeA, nodeB, nodeC, nodeD, nodeF]), 4);

        // E is only in the previous context: its post is accepted and counts only there.
        _post(nodeE, EPOCH, ROOT_1);
        _assertNotRecorded(EPOCH);

        // C posts. The senders are now A, B, C and E.
        // Members of the current context among them: A, B, C. That is 3, below its threshold of 4.
        // Members of the previous context among them: A, B, C, E. That is 4, its threshold.
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev When two roots reach a threshold, the one that reached it first is recorded. A context
    /// switch brings root 1 to the new context's threshold without any post, so nothing records it
    /// yet. A member of the previous context then posts root 2, which brings root 2 to the previous
    /// context's threshold. The contract records root 1: before adding a post, it first records a
    /// root that already reached a threshold.
    function test_postRootRecordsRootThatReachedThresholdFirst() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_2);
        _post(nodeD, EPOCH, ROOT_2);
        _assertNotRecorded(EPOCH);

        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeF]), 2);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        _post(nodeE, EPOCH, ROOT_2);
        _assertRecorded(EPOCH, ROOT_1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_2).length, 3);
    }

    function test_postRootAndRecordEpochRootAgree() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_2);
        _post(nodeD, EPOCH, ROOT_2);
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeF]), 2);

        uint256 snapshot = vm.snapshotState();
        _post(nodeE, EPOCH, ROOT_2);
        bytes32 rootViaPost = ciphertextRoots.getEpochRoot(CHAIN_ID, EPOCH);
        vm.revertToState(snapshot);

        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        bytes32 rootViaRecord = ciphertextRoots.getEpochRoot(CHAIN_ID, EPOCH);
        assertEq(rootViaPost, rootViaRecord);
        assertEq(rootViaRecord, ROOT_1);
    }

    function test_overlappingMembersCountedForBothContexts() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        // A and B stay members; the new threshold is out of reach but the previous one is not.
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeF]), 5);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev Counts are never added across the two contexts. A and B are only in the previous
    /// context (2 of its threshold 3) and F only in the current one (1 of its threshold 2): three
    /// posts of the same root, yet neither context reaches its threshold.
    function test_countsNotAddedAcrossContexts() public {
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeD, nodeE, nodeF]), 2);
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeF, EPOCH, ROOT_1);
        _assertNotRecorded(EPOCH);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoRootReachesThreshold.selector, CHAIN_ID, EPOCH));
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
    }

    /// @dev The previous context is the current ID minus one. For the first context that ID was
    /// never defined, so no previous context counts, and the reads of it that would revert on the
    /// mock are never made.
    function test_firstContextHasNoPreviousContext() public {
        assertFalse(protocolConfigMock.isValidCoprocessorContext(CONTEXT_1 - 1));
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    function test_epochRootRecordedEmittedOnce() public {
        vm.recordLogs();
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _post(nodeD, EPOCH, ROOT_1);
        _post(nodeE, EPOCH, ROOT_2);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        uint256 recordedEvents;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == ICiphertextRoots.EpochRootRecorded.selector) {
                recordedEvents++;
            }
        }
        assertEq(recordedEvents, 1);
    }

    function test_chainsAreIsolated() public {
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(OTHER_CHAIN_ID, START_BLOCK, EPOCH_LENGTH);
        vm.prank(nodeA);
        ciphertextRoots.postRoot(OTHER_CHAIN_ID, EPOCH, ROOT_2);

        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);

        _assertNotRecorded(OTHER_CHAIN_ID, EPOCH);
        assertEq(ciphertextRoots.getRootSenders(OTHER_CHAIN_ID, EPOCH, ROOT_2).length, 1);
        assertEq(ciphertextRoots.getRootSenders(OTHER_CHAIN_ID, EPOCH, ROOT_1).length, 0);
    }

    function testFuzz_recordsExactlyAtThreshold(uint8 nodeCount, uint8 threshold) public {
        nodeCount = uint8(bound(nodeCount, 1, 16));
        threshold = uint8(bound(threshold, 1, nodeCount));
        address[] memory nodes = new address[](nodeCount);
        for (uint256 i = 0; i < nodeCount; i++) {
            nodes[i] = address(uint160(0x1000 + i));
        }
        protocolConfigMock.defineContext(CONTEXT_2, nodes, threshold);
        protocolConfigMock.destroyContext(CONTEXT_1);

        // One post short of the threshold.
        for (uint256 i = 0; i < threshold - 1; i++) {
            _post(nodes[i], EPOCH, ROOT_1);
        }
        _assertNotRecorded(EPOCH);

        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        _post(nodes[threshold - 1], EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    // -----------------------------------------------------------------------
    // Record tests
    // -----------------------------------------------------------------------

    function test_recordEpochRootAfterThresholdLowered() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _assertNotRecorded(EPOCH);

        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 2);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        vm.prank(outsider);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        _assertRecorded(EPOCH, ROOT_1);
    }

    function test_recordEpochRootPicksFirstPostedRoot() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_2);
        _post(nodeC, EPOCH, ROOT_2);
        _assertNotRecorded(EPOCH);

        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 1);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev Two switches in a row: the entries reach the threshold of the context that became the
    /// previous one, with no post in between.
    function test_recordEpochRootUnderPreviousContext() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC]), 2);
        protocolConfigMock.defineContext(CONTEXT_3, _nodes([nodeD, nodeE, nodeF]), 1);

        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        _assertRecorded(EPOCH, ROOT_1);
    }

    function test_recordEpochRootSkipsRootsBelowThreshold() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_2);
        _post(nodeC, EPOCH, ROOT_2);
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), 2);

        _expectRecorded(EPOCH, ROOT_2, CONTEXT_2);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        _assertRecorded(EPOCH, ROOT_2);
    }

    function test_recordEpochRootIgnoresDestroyedPreviousContext() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        protocolConfigMock.defineContext(CONTEXT_2, _nodes([nodeD, nodeE, nodeF]), 1);
        protocolConfigMock.destroyContext(CONTEXT_1);

        // The mock reverts on reads of a destroyed context, so this proves they are never made.
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoRootReachesThreshold.selector, CHAIN_ID, EPOCH));
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
    }

    function test_revertRecordEpochRootAlreadyRecorded() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.EpochRootAlreadyRecorded.selector, CHAIN_ID, EPOCH));
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
    }

    function test_revertRecordEpochRootNoRootReachesThreshold() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoRootReachesThreshold.selector, CHAIN_ID, EPOCH));
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
    }

    function test_revertRecordEpochRootNoEpochSchedule() public {
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NoEpochSchedule.selector, OTHER_CHAIN_ID));
        ciphertextRoots.recordEpochRoot(OTHER_CHAIN_ID, EPOCH);
    }
}

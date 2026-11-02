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
import {CoprocessorContextsMock} from "./CoprocessorContextsMock.sol";

contract CiphertextRootsTest is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);
    address internal constant outsider = address(0xBEEF);
    uint64 internal constant CHAIN_ID = 11155111;
    uint64 internal constant OTHER_CHAIN_ID = 137;
    uint256 internal constant START_BLOCK = 1000;
    uint256 internal constant EPOCH_LENGTH = 100;
    uint256 internal constant EPOCH = 7;
    /// @dev RFC 039 coprocessor context IDs: type byte 0x09 followed by a counter.
    uint256 internal constant COPROCESSOR_CONTEXT_COUNTER_BASE = uint256(9) << 248;
    uint256 internal constant CONTEXT_1 = COPROCESSOR_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant CONTEXT_2 = COPROCESSOR_CONTEXT_COUNTER_BASE + 2;
    uint256 internal constant CONTEXT_3 = COPROCESSOR_CONTEXT_COUNTER_BASE + 3;
    bytes32 internal constant ROOT_1 = keccak256("root-1");
    bytes32 internal constant ROOT_2 = keccak256("root-2");
    bytes32 internal constant CIPHERTEXT_ROOTS_STORAGE_LOCATION =
        0x3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f12000;

    address internal nodeA = address(0xA1);
    address internal nodeB = address(0xA2);
    address internal nodeC = address(0xA3);
    address internal nodeD = address(0xA4);
    address internal nodeE = address(0xA5);
    address internal nodeF = address(0xA6);

    CiphertextRoots internal ciphertextRoots;
    CoprocessorContextsMock internal coprocessorContexts;

    function setUp() public {
        _deployACL(owner);
        deployCodeTo("test/ciphertextRoots/CoprocessorContextsMock.sol:CoprocessorContextsMock", protocolConfigAdd);
        coprocessorContexts = CoprocessorContextsMock(protocolConfigAdd);
        vm.label(protocolConfigAdd, "CoprocessorContextsMock");
        (ciphertextRoots, ) = _deployCiphertextRoots(owner);

        /// @dev Context 1: five nodes, majority threshold 3.
        coprocessorContexts.defineContext(CONTEXT_1, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 3);
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(CHAIN_ID, START_BLOCK, EPOCH_LENGTH);
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    function _nodes(address[5] memory fixedNodes) internal pure returns (address[] memory nodes) {
        nodes = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            nodes[i] = fixedNodes[i];
        }
    }

    function _nodes(address[6] memory fixedNodes) internal pure returns (address[] memory nodes) {
        nodes = new address[](6);
        for (uint256 i = 0; i < 6; i++) {
            nodes[i] = fixedNodes[i];
        }
    }

    function _nodes(address[3] memory fixedNodes) internal pure returns (address[] memory nodes) {
        nodes = new address[](3);
        for (uint256 i = 0; i < 3; i++) {
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
        (bytes32 root, bool recorded) = ciphertextRoots.getEpochRoot(CHAIN_ID, epoch);
        assertTrue(recorded);
        assertEq(root, expectedRoot);
    }

    function _assertNotRecorded(uint256 epoch) internal view {
        (bytes32 root, bool recorded) = ciphertextRoots.getEpochRoot(CHAIN_ID, epoch);
        assertFalse(recorded);
        assertEq(root, bytes32(0));
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

    function _assertEpochLength(
        ICiphertextRoots.EpochLength memory epochLength,
        uint256 fromEpoch,
        uint256 length
    ) internal pure {
        assertEq(epochLength.fromEpoch, fromEpoch);
        assertEq(epochLength.epochLength, length);
    }

    // -----------------------------------------------------------------------
    // Initialization and upgrade tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public view {
        assertEq(address(ciphertextRoots), ciphertextRootsAdd);
        assertEq(ciphertextRoots.getVersion(), "CiphertextRoots v0.1.0");
        _assertNotRecorded(EPOCH);
    }

    function test_revertDoubleInitFromEmptyProxy() public {
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        ciphertextRoots.initializeFromEmptyProxy();
    }

    function test_storageLocationPinned() public view {
        /// @dev `epochSchedules` is the first field, so `epochSchedules[CHAIN_ID]` lives at the mapping slot of the
        /// namespace: `startBlock` there, the epoch length count in the next slot and the epoch lengths at its hash.
        bytes32 scheduleSlot = keccak256(abi.encode(uint256(CHAIN_ID), CIPHERTEXT_ROOTS_STORAGE_LOCATION));
        assertEq(uint256(vm.load(address(ciphertextRoots), scheduleSlot)), START_BLOCK);
        bytes32 epochLengthsSlot = bytes32(uint256(scheduleSlot) + 1);
        assertEq(uint256(vm.load(address(ciphertextRoots), epochLengthsSlot)), 1);
        uint256 firstEpochLengthSlot = uint256(keccak256(abi.encode(epochLengthsSlot)));
        assertEq(uint256(vm.load(address(ciphertextRoots), bytes32(firstEpochLengthSlot))), 0);
        assertEq(uint256(vm.load(address(ciphertextRoots), bytes32(firstEpochLengthSlot + 1))), EPOCH_LENGTH);
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
        assertEq(schedule.epochLengths.length, 2);
        _assertEpochLength(schedule.epochLengths[0], 0, EPOCH_LENGTH);
        _assertEpochLength(schedule.epochLengths[1], EPOCH + 1, 50);
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
        assertEq(schedule.epochLengths.length, 1);
        _assertEpochLength(schedule.epochLengths[0], 0, 50);
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
        assertEq(schedule.epochLengths.length, 3);
        _assertEpochLength(schedule.epochLengths[0], 0, 5);
        _assertEpochLength(schedule.epochLengths[1], 5, 20);
        _assertEpochLength(schedule.epochLengths[2], 8, 10);
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
        _record(3);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, 4, EPOCH + 1));
        ciphertextRoots.setEpochLength(CHAIN_ID, 4, 50);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.FromEpochTooLow.selector, CHAIN_ID, EPOCH, EPOCH + 1));
        ciphertextRoots.setEpochLength(CHAIN_ID, EPOCH, 50);
        _setEpochLength(EPOCH + 1, 50);
    }

    function test_removeEpochLengthEmitsAndStores() public {
        _setEpochLength(5, 50);
        _setEpochLength(8, 60);
        _setEpochLength(12, 70);

        /// @dev Removing a middle one keeps the others in order.
        vm.expectEmit(true, false, false, true, address(ciphertextRoots));
        emit ICiphertextRoots.EpochLengthRemoved(CHAIN_ID, 8);
        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, 8);
        ICiphertextRoots.EpochSchedule memory schedule = ciphertextRoots.getEpochSchedule(CHAIN_ID);
        assertEq(schedule.epochLengths.length, 3);
        _assertEpochLength(schedule.epochLengths[0], 0, EPOCH_LENGTH);
        _assertEpochLength(schedule.epochLengths[1], 5, 50);
        _assertEpochLength(schedule.epochLengths[2], 12, 70);

        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, 12);
        schedule = ciphertextRoots.getEpochSchedule(CHAIN_ID);
        assertEq(schedule.epochLengths.length, 2);
        _assertEpochLength(schedule.epochLengths[1], 5, 50);
    }

    /// @dev Changing the length of a pending epoch length is a removal and a new set.
    function test_removeEpochLengthThenSetSameFromEpoch() public {
        _setEpochLength(5, 50);
        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, 5);
        _setEpochLength(5, 60);

        ICiphertextRoots.EpochSchedule memory schedule = ciphertextRoots.getEpochSchedule(CHAIN_ID);
        assertEq(schedule.epochLengths.length, 2);
        _assertEpochLength(schedule.epochLengths[1], 5, 60);
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

        /// @dev The first epoch length, from epoch 0, is never removed.
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.EpochLengthNotFound.selector, CHAIN_ID, 0));
        ciphertextRoots.removeEpochLength(CHAIN_ID, 0);
    }

    function test_revertRemoveEpochLengthInUse() public {
        _setEpochLength(5, 50);
        _setEpochLength(9, 60);
        _record(EPOCH);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.EpochLengthInUse.selector, CHAIN_ID, 5));
        ciphertextRoots.removeEpochLength(CHAIN_ID, 5);
        vm.prank(owner);
        ciphertextRoots.removeEpochLength(CHAIN_ID, 9);
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
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE, nodeF]), 3);
        coprocessorContexts.destroyContext(CONTEXT_1);

        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);

        /// @dev A second root reaching the threshold is stored as evidence but never replaces the recorded one.
        _post(nodeD, EPOCH, ROOT_2);
        _post(nodeE, EPOCH, ROOT_2);
        _post(nodeF, EPOCH, ROOT_2);
        _assertRecorded(EPOCH, ROOT_1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_2).length, 3);
    }

    function test_postRootAcceptedAfterEpochRootRecorded() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);

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
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), 3);
        _post(nodeA, EPOCH, ROOT_1);
        assertEq(ciphertextRoots.getRootSenders(CHAIN_ID, EPOCH, ROOT_1).length, 1);
    }

    function test_revertPostRootDestroyedPreviousContextMember() public {
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), 3);
        coprocessorContexts.destroyContext(CONTEXT_1);
        vm.prank(nodeA);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NotCoprocessorTxSender.selector, nodeA));
        ciphertextRoots.postRoot(CHAIN_ID, EPOCH, ROOT_1);
    }

    function test_revertPostRootOlderContextMember() public {
        /// @dev Context 1 stays valid but is neither current nor previous once context 3 is current.
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), 3);
        coprocessorContexts.defineContext(CONTEXT_3, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), 3);
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

    function test_revertPostRootNoCoprocessorContext() public {
        coprocessorContexts.setCurrentContextId(0);
        vm.prank(nodeA);
        vm.expectRevert(ICiphertextRoots.NoCoprocessorContext.selector);
        ciphertextRoots.postRoot(CHAIN_ID, EPOCH, ROOT_1);
    }

    // -----------------------------------------------------------------------
    // Counting tests
    // -----------------------------------------------------------------------

    /// @dev RFC 034's example: n = 5, k = 4, A and B post before E is replaced by F. Counting by
    /// membership when counting lets C and D complete the epoch under the new context.
    function test_countMembershipWhenCounting() public {
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 4);
        coprocessorContexts.destroyContext(CONTEXT_1);
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);

        coprocessorContexts.defineContext(CONTEXT_3, _nodes([nodeA, nodeB, nodeC, nodeD, nodeF]), 4);
        _post(nodeC, EPOCH, ROOT_1);
        _assertNotRecorded(EPOCH);

        _expectRecorded(EPOCH, ROOT_1, CONTEXT_3);
        _post(nodeD, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    function test_countFallsBackToPreviousContext() public {
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 4);
        coprocessorContexts.destroyContext(CONTEXT_1);
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);

        coprocessorContexts.defineContext(CONTEXT_3, _nodes([nodeA, nodeB, nodeC, nodeD, nodeF]), 4);
        /// @dev E is only in the previous context; its post counts there during the grace period.
        _post(nodeE, EPOCH, ROOT_1);
        _assertNotRecorded(EPOCH);

        /// @dev Current context: A, B, C = 3 < 4. Previous context: A, B, C, E = 4.
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev A context switch makes r1 reach the new threshold with no post. A retiring member then
    /// posts r2, which reaches the previous threshold: r1 reached one first and must be recorded.
    function test_postRootRecordsRootThatReachedThresholdFirst() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_2);
        _post(nodeD, EPOCH, ROOT_2);
        _assertNotRecorded(EPOCH);

        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeF]), 2);
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
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeF]), 2);

        uint256 snapshot = vm.snapshotState();
        _post(nodeE, EPOCH, ROOT_2);
        (bytes32 rootViaPost, ) = ciphertextRoots.getEpochRoot(CHAIN_ID, EPOCH);
        vm.revertToState(snapshot);

        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        (bytes32 rootViaRecord, ) = ciphertextRoots.getEpochRoot(CHAIN_ID, EPOCH);
        assertEq(rootViaPost, rootViaRecord);
        assertEq(rootViaRecord, ROOT_1);
    }

    function test_overlappingMembersCountedForBothContexts() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        /// @dev A and B stay members; the new threshold is out of reach but the previous one is not.
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeF]), 5);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev The previous context is the current ID minus one. For the first context that ID was
    /// never defined, so no previous context counts, and the reads of it that would revert on the
    /// mock are never made.
    function test_firstContextHasNoPreviousContext() public {
        assertFalse(coprocessorContexts.isValidCoprocessorContext(CONTEXT_1 - 1));
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev Context 2 was never defined, so context 3 has no valid previous context: context 1,
    /// although still valid, does not count, and the reads of context 2 that would revert on the
    /// mock are never made.
    function test_undefinedPreviousContextIgnored() public {
        coprocessorContexts.defineContext(CONTEXT_3, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), 3);
        vm.prank(nodeA);
        vm.expectRevert(abi.encodeWithSelector(ICiphertextRoots.NotCoprocessorTxSender.selector, nodeA));
        ciphertextRoots.postRoot(CHAIN_ID, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        _post(nodeC, EPOCH, ROOT_1);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_3);
        _post(nodeD, EPOCH, ROOT_1);
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

        (, bool recorded) = ciphertextRoots.getEpochRoot(OTHER_CHAIN_ID, EPOCH);
        assertFalse(recorded);
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
        coprocessorContexts.defineContext(CONTEXT_2, nodes, threshold);
        coprocessorContexts.destroyContext(CONTEXT_1);

        for (uint256 i = 0; i + 1 < threshold; i++) {
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

        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 2);
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

        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC, nodeD, nodeE]), 1);
        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        _assertRecorded(EPOCH, ROOT_1);
    }

    /// @dev Two switches in a row: the entries reach the threshold of the context that became the
    /// previous one, with no post in between.
    function test_recordEpochRootUnderPreviousContext() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeA, nodeB, nodeC]), 2);
        coprocessorContexts.defineContext(CONTEXT_3, _nodes([nodeD, nodeE, nodeF]), 1);

        _expectRecorded(EPOCH, ROOT_1, CONTEXT_2);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        _assertRecorded(EPOCH, ROOT_1);
    }

    function test_recordEpochRootSkipsRootsBelowThreshold() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_2);
        _post(nodeC, EPOCH, ROOT_2);
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeB, nodeC, nodeD, nodeE, nodeF]), 2);

        _expectRecorded(EPOCH, ROOT_2, CONTEXT_2);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
        _assertRecorded(EPOCH, ROOT_2);
    }

    function test_recordEpochRootIgnoresDestroyedPreviousContext() public {
        _post(nodeA, EPOCH, ROOT_1);
        _post(nodeB, EPOCH, ROOT_1);
        coprocessorContexts.defineContext(CONTEXT_2, _nodes([nodeD, nodeE, nodeF]), 1);
        coprocessorContexts.destroyContext(CONTEXT_1);

        /// @dev The mock reverts on reads of a destroyed context, so this proves they are never made.
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

    function test_revertRecordEpochRootNoCoprocessorContext() public {
        _post(nodeA, EPOCH, ROOT_1);
        coprocessorContexts.setCurrentContextId(0);
        vm.expectRevert(ICiphertextRoots.NoCoprocessorContext.selector);
        ciphertextRoots.recordEpochRoot(CHAIN_ID, EPOCH);
    }
}

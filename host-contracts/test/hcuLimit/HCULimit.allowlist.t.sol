// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {FheType} from "../../contracts/shared/FheType.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {aclAdd, confidentialBridgeAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {MockHCULimit} from "./HCULimit.t.sol";
import {HCULimitV040TestDouble} from "./HCULimitV040TestDouble.sol";

/// @dev Supplies ACL's ownership-transfer callback; delegate synchronization is covered by the bridge tests.
contract MockConfidentialBridge {
    function syncDelegate() external {}
}

/// @dev Executes owner actions atomically for governance cleanup regression tests.
contract MockGovernanceBatchExecutor {
    function executeBatch(address target, bytes[] calldata actions) external {
        for (uint256 i; i < actions.length; ++i) {
            (bool success, bytes memory reason) = target.call(actions[i]);
            if (!success) {
                assembly {
                    revert(add(reason, 32), mload(reason))
                }
            }
        }
    }
}

contract HCULimitAllowlistTest is Test {
    enum CleanupRoleState {
        Active,
        Renounced,
        ExpiredAtDeadline,
        ExpiredPastDeadline,
        Revoked
    }

    MockHCULimit internal hcuLimit;
    address internal constant OWNER = address(456);
    address internal constant ALLOWLISTER = address(0xA11CE);
    address internal constant DAPP = address(0xDABB);
    address internal executor;
    bytes32 internal constant INPUT = bytes32(uint256(1));
    bytes32 internal constant RESULT = bytes32(uint256(2));

    function setUp() public {
        vm.warp(1 days);
        vm.etch(aclAdd, address(new ACL()).code);
        if (confidentialBridgeAdd != address(0)) {
            vm.etch(confidentialBridgeAdd, address(new MockConfidentialBridge()).code);
        }
        vm.store(
            aclAdd,
            // ERC-7201: openzeppelin.storage.Ownable; ACL's _owner is at offset zero.
            0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300,
            bytes32(uint256(uint160(OWNER)))
        );
        address proxy = _emptyProxy();
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            proxy,
            address(new MockHCULimit()),
            abi.encodeCall(HCULimit.initializeFromEmptyProxy, (type(uint48).max, 5_000_000, 20_000_000))
        );
        vm.stopPrank();
        hcuLimit = MockHCULimit(proxy);
        executor = hcuLimit.getFHEVMExecutorAddress();
    }

    function _emptyProxy() internal returns (address) {
        return
            UnsafeUpgrades.deployUUPSProxy(
                address(new EmptyUUPSProxy()),
                abi.encodeCall(EmptyUUPSProxy.initialize, ())
            );
    }

    function _accounts(address account) internal pure returns (address[] memory accounts) {
        accounts = new address[](1);
        accounts[0] = account;
    }

    function _grant(address account) internal {
        vm.prank(OWNER);
        hcuLimit.addAllowlister(account, block.timestamp + 1 days);
    }

    function _allow(address account) internal {
        vm.prank(OWNER);
        hcuLimit.addToAllowlist(_accounts(account));
    }

    function _activate(bool activated) internal {
        vm.prank(OWNER);
        hcuLimit.setAllowlistActivated(activated);
    }

    function _compute(address caller) internal {
        vm.prank(executor);
        hcuLimit.checkHCUForCast(FheType.Uint8, INPUT, RESULT, caller);
    }

    function _assertGovernanceCleanupBatch(CleanupRoleState state) internal {
        _activate(true);
        _grant(ALLOWLISTER);
        uint256 deadline = hcuLimit.allowlisterDeadline(ALLOWLISTER);
        address[] memory dapps = new address[](2);
        dapps[0] = DAPP;
        dapps[1] = address(0xBEEF);
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(dapps);
        _compute(DAPP);

        MockGovernanceBatchExecutor governance = new MockGovernanceBatchExecutor();
        vm.prank(OWNER);
        ACL(aclAdd).transferOwnership(address(governance));
        vm.prank(address(governance));
        ACL(aclAdd).acceptOwnership();

        // The proposal is prepared before the role can change ahead of execution.
        bytes[] memory actions = new bytes[](2);
        actions[0] = abi.encodeCall(HCULimit.removeAllowlister, (ALLOWLISTER));
        actions[1] = abi.encodeCall(HCULimit.removeFromAllowlist, (dapps));
        if (state == CleanupRoleState.Renounced) {
            vm.prank(ALLOWLISTER);
            hcuLimit.renounceAllowlister();
        } else if (state == CleanupRoleState.ExpiredAtDeadline) {
            vm.warp(deadline);
        } else if (state == CleanupRoleState.ExpiredPastDeadline) {
            vm.warp(deadline + 1);
        } else if (state == CleanupRoleState.Revoked) {
            vm.prank(address(governance));
            hcuLimit.removeAllowlister(ALLOWLISTER);
        }

        governance.executeBatch(address(hcuLimit), actions);
        assertFalse(hcuLimit.isAllowlister(ALLOWLISTER));
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), 0);
        for (uint256 i; i < dapps.length; ++i) {
            assertFalse(hcuLimit.isAllowlisted(dapps[i]));
            vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, dapps[i]));
            _compute(dapps[i]);
        }
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(address(0xCAFE)));
    }

    function test_governanceCleanupBatchRevokesActiveAllowlister() public {
        _assertGovernanceCleanupBatch(CleanupRoleState.Active);
    }

    function test_governanceCleanupBatchSurvivesRenunciation() public {
        _assertGovernanceCleanupBatch(CleanupRoleState.Renounced);
    }

    function test_governanceCleanupBatchSurvivesExpiry(bool pastDeadline) public {
        _assertGovernanceCleanupBatch(
            pastDeadline ? CleanupRoleState.ExpiredPastDeadline : CleanupRoleState.ExpiredAtDeadline
        );
    }

    function test_governanceCleanupBatchSurvivesPriorRevocation() public {
        _assertGovernanceCleanupBatch(CleanupRoleState.Revoked);
    }

    function test_freshDeploymentIsOpenAndConsumesInitializerVersion() public {
        assertFalse(hcuLimit.isAllowlistActivated());
        assertFalse(hcuLimit.isAllowlisted(DAPP));
        assertFalse(hcuLimit.isAllowlister(ALLOWLISTER));
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), 0);
        _compute(DAPP);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        hcuLimit.reinitializeV5();
    }

    function test_upgradePreservesLegacyStorageAndStartsInactive() public {
        address proxy = _emptyProxy();
        vm.startPrank(OWNER);
        UnsafeUpgrades.upgradeProxy(
            proxy,
            address(new HCULimitV040TestDouble()),
            abi.encodeCall(HCULimitV040TestDouble.initializeFromEmptyProxy, (DAPP))
        );
        UnsafeUpgrades.upgradeProxy(proxy, address(new MockHCULimit()), abi.encodeCall(HCULimit.reinitializeV5, ()));
        vm.stopPrank();
        MockHCULimit upgraded = MockHCULimit(proxy);
        assertEq(upgraded.getGlobalHCUCapPerBlock(), 30_000_000);
        assertEq(upgraded.getMaxHCUPerTx(), 20_000_000);
        assertEq(upgraded.getMaxHCUDepthPerTx(), 5_000_000);
        (uint48 blockNumber, uint48 usedHCU) = upgraded.getBlockMeter();
        assertEq(blockNumber, block.number);
        assertEq(usedHCU, 12_345);
        assertTrue(upgraded.isBlockHCUWhitelisted(DAPP));
        assertFalse(upgraded.isAllowlistActivated());
        assertFalse(upgraded.isAllowlisted(DAPP));
        assertFalse(upgraded.isAllowlister(ALLOWLISTER));
        assertEq(upgraded.allowlisterDeadline(ALLOWLISTER), 0);
        assertEq(upgraded.getVersion(), "HCULimit v0.5.0");
        vm.prank(executor);
        upgraded.checkHCUForCast(FheType.Uint8, INPUT, RESULT, DAPP);
        (, usedHCU) = upgraded.getBlockMeter();
        assertEq(usedHCU, 12_345);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        upgraded.reinitializeV5();
    }

    function test_ownerAndAllowlisterBatchAdditionsAndEvents(bool activated) public {
        _activate(activated);
        address[] memory accounts = new address[](2);
        accounts[0] = DAPP;
        accounts[1] = address(0xBEEF); // Preapproval does not require deployed code.
        vm.expectEmit(true, true, false, true, address(hcuLimit));
        emit HCULimit.AllowlistAdded(accounts[0], OWNER);
        vm.expectEmit(true, true, false, true, address(hcuLimit));
        emit HCULimit.AllowlistAdded(accounts[1], OWNER);
        vm.prank(OWNER);
        hcuLimit.addToAllowlist(accounts);
        assertTrue(hcuLimit.isAllowlisted(accounts[0]));
        assertTrue(hcuLimit.isAllowlisted(accounts[1]));
        assertFalse(hcuLimit.isAllowlister(OWNER));
        assertFalse(hcuLimit.isBlockHCUWhitelisted(DAPP));

        vm.expectEmit(true, false, false, true, address(hcuLimit));
        emit HCULimit.AllowlistRemoved(accounts[0]);
        vm.expectEmit(true, false, false, true, address(hcuLimit));
        emit HCULimit.AllowlistRemoved(accounts[1]);
        vm.prank(OWNER);
        hcuLimit.removeFromAllowlist(accounts);
        assertFalse(hcuLimit.isAllowlisted(accounts[0]));
        assertFalse(hcuLimit.isAllowlisted(accounts[1]));

        _grant(ALLOWLISTER);
        vm.expectEmit(true, true, false, true, address(hcuLimit));
        emit HCULimit.AllowlistAdded(accounts[0], ALLOWLISTER);
        vm.expectEmit(true, true, false, true, address(hcuLimit));
        emit HCULimit.AllowlistAdded(accounts[1], ALLOWLISTER);
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(accounts);
        assertTrue(hcuLimit.isAllowlisted(accounts[0]));
        assertTrue(hcuLimit.isAllowlisted(accounts[1]));
    }

    function test_allowlistAuthorityFollowsAcceptedACLOwnership(bool activated) public {
        _activate(activated);
        ACL acl = ACL(aclAdd);
        address newOwner = address(0xB0B);
        assertFalse(hcuLimit.isAllowlister(OWNER));
        assertFalse(hcuLimit.isAllowlister(newOwner));

        vm.prank(OWNER);
        acl.transferOwnership(newOwner);
        assertEq(acl.owner(), OWNER);
        assertEq(acl.pendingOwner(), newOwner);

        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, newOwner));
        vm.prank(newOwner);
        hcuLimit.addToAllowlist(_accounts(DAPP));
        assertFalse(hcuLimit.isAllowlisted(DAPP));

        // The current owner retains authority while the transfer is pending.
        _allow(DAPP);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, newOwner));
        vm.prank(newOwner);
        hcuLimit.removeFromAllowlist(_accounts(DAPP));
        assertTrue(hcuLimit.isAllowlisted(DAPP));

        vm.prank(newOwner);
        acl.acceptOwnership();
        assertEq(acl.owner(), newOwner);
        assertEq(acl.pendingOwner(), address(0));

        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, OWNER));
        vm.prank(OWNER);
        hcuLimit.addToAllowlist(_accounts(address(0xBEEF)));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, OWNER));
        vm.prank(OWNER);
        hcuLimit.removeFromAllowlist(_accounts(DAPP));

        vm.expectEmit(true, true, false, true, address(hcuLimit));
        emit HCULimit.AllowlistAdded(address(0xBEEF), newOwner);
        vm.prank(newOwner);
        hcuLimit.addToAllowlist(_accounts(address(0xBEEF)));
        assertTrue(hcuLimit.isAllowlisted(address(0xBEEF)));
        assertFalse(hcuLimit.isAllowlister(newOwner));

        vm.prank(newOwner);
        hcuLimit.removeFromAllowlist(_accounts(DAPP));
        assertFalse(hcuLimit.isAllowlisted(DAPP));
        assertEq(hcuLimit.isAllowlistActivated(), activated);
    }

    function test_ownershipTransferPreservesExplicitAllowlisterGrants() public {
        ACL acl = ACL(aclAdd);
        address newOwner = address(0xB0B);
        _grant(OWNER);
        _grant(newOwner);
        uint256 deadline = block.timestamp + 1 days;

        vm.prank(OWNER);
        acl.transferOwnership(newOwner);
        // Pending ownership grants no authority, but an explicit allowlister role still permits additions.
        vm.prank(newOwner);
        hcuLimit.addToAllowlist(_accounts(DAPP));
        vm.prank(newOwner);
        acl.acceptOwnership();
        assertEq(hcuLimit.allowlisterDeadline(OWNER), deadline);
        assertEq(hcuLimit.allowlisterDeadline(newOwner), deadline);

        // The former owner keeps its delegated role, without owner-only removal rights.
        vm.prank(OWNER);
        hcuLimit.addToAllowlist(_accounts(address(0xBEEF)));
        assertTrue(hcuLimit.isAllowlisted(DAPP));
        assertTrue(hcuLimit.isAllowlisted(address(0xBEEF)));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, OWNER));
        vm.prank(OWNER);
        hcuLimit.removeFromAllowlist(_accounts(DAPP));

        vm.prank(newOwner);
        hcuLimit.removeAllowlister(OWNER);
        assertFalse(hcuLimit.isAllowlister(OWNER));
        assertEq(hcuLimit.allowlisterDeadline(OWNER), 0);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, OWNER));
        vm.prank(OWNER);
        hcuLimit.addToAllowlist(_accounts(address(0xCAFE)));
    }

    function test_ownerOnlyAdministrationRejectsOtherActiveAndExpiredAllowlisters(uint8 role) public {
        role = uint8(bound(role, 0, 2));
        if (role != 0) _grant(ALLOWLISTER);
        if (role == 2) vm.warp(block.timestamp + 1 days);
        bytes[] memory calls = new bytes[](7);
        calls[0] = abi.encodeCall(HCULimit.setAllowlistActivated, (true));
        calls[1] = abi.encodeCall(HCULimit.removeFromAllowlist, (_accounts(DAPP)));
        calls[2] = abi.encodeCall(HCULimit.addAllowlister, (DAPP, block.timestamp + 1 days));
        calls[3] = abi.encodeCall(HCULimit.removeAllowlister, (ALLOWLISTER));
        calls[4] = abi.encodeCall(HCULimit.addToBlockHCUWhitelist, (DAPP));
        calls[5] = abi.encodeCall(HCULimit.removeFromBlockHCUWhitelist, (DAPP));
        calls[6] = abi.encodeCall(HCULimit.removeFromAllowlist, (new address[](0)));
        for (uint256 i; i < calls.length; ++i) {
            vm.prank(ALLOWLISTER);
            (bool success, bytes memory reason) = address(hcuLimit).call(calls[i]);
            assertFalse(success);
            assertEq(reason, abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, ALLOWLISTER));
        }
    }

    function test_onlyOwnerOrActiveAllowlisterCanAdd(bool expired) public {
        if (expired) {
            _grant(ALLOWLISTER);
            vm.warp(block.timestamp + 1 days);
        }
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(DAPP));
        // Existing members still require authorization, even when the entire batch would be skipped.
        _allow(DAPP);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(DAPP));
        // Empty batches still enforce authorization.
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(new address[](0));
    }

    function test_emptyAdmissionBatchRevertsForAuthorizedSenders(bool delegated, bool activated) public {
        _activate(activated);
        if (delegated) _grant(ALLOWLISTER);
        vm.expectRevert(HCULimit.EmptyBatch.selector);
        vm.prank(delegated ? ALLOWLISTER : OWNER);
        hcuLimit.addToAllowlist(new address[](0));
    }

    function test_emptyRemovalBatchRevertsForOwner(bool activated) public {
        _activate(activated);
        vm.expectRevert(HCULimit.EmptyBatch.selector);
        vm.prank(OWNER);
        hcuLimit.removeFromAllowlist(new address[](0));
    }

    function test_batchAdditionSkipsExistingMembersAndDuplicates(bool delegated, bool activated) public {
        _activate(activated);
        _grant(ALLOWLISTER);
        // Another authorized account admits a member before this batch executes.
        vm.prank(delegated ? OWNER : ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(DAPP));

        address sender = delegated ? ALLOWLISTER : OWNER;
        address[] memory accounts = new address[](6);
        accounts[0] = DAPP;
        accounts[1] = address(0xBEEF);
        accounts[2] = DAPP;
        accounts[3] = address(0xBEEF);
        accounts[4] = address(0xCAFE);
        accounts[5] = address(0xCAFE);
        vm.recordLogs();
        vm.prank(sender);
        hcuLimit.addToAllowlist(accounts);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, 2);
        for (uint256 i; i < logs.length; ++i) {
            assertEq(logs[i].emitter, address(hcuLimit));
            assertEq(logs[i].topics.length, 3);
            assertEq(logs[i].topics[0], keccak256("AllowlistAdded(address,address)"));
            assertEq(logs[i].topics[1], bytes32(uint256(uint160(i == 0 ? accounts[1] : accounts[4]))));
            assertEq(logs[i].topics[2], bytes32(uint256(uint160(sender))));
            assertEq(logs[i].data.length, 0);
        }
        assertTrue(hcuLimit.isAllowlisted(DAPP));
        assertTrue(hcuLimit.isAllowlisted(address(0xBEEF)));
        assertTrue(hcuLimit.isAllowlisted(address(0xCAFE)));

        // Replaying a batch containing only existing members succeeds without any events.
        vm.recordLogs();
        vm.prank(sender);
        hcuLimit.addToAllowlist(accounts);
        assertEq(vm.getRecordedLogs().length, 0);
    }

    function test_batchAdditionRemainsAtomicForZeroAddress(bool delegated, uint8 zeroIndex) public {
        _allow(DAPP);
        if (delegated) _grant(ALLOWLISTER);
        address[] memory accounts = new address[](4);
        accounts[0] = DAPP;
        accounts[1] = address(0xBEEF);
        accounts[2] = address(0xBEEF);
        accounts[3] = address(0xCAFE);
        accounts[bound(zeroIndex, 0, accounts.length - 1)] = address(0);
        vm.expectRevert(HCULimit.InvalidZeroAddress.selector);
        vm.prank(delegated ? ALLOWLISTER : OWNER);
        hcuLimit.addToAllowlist(accounts);
        assertTrue(hcuLimit.isAllowlisted(DAPP));
        assertFalse(hcuLimit.isAllowlisted(address(0xBEEF)));
        assertFalse(hcuLimit.isAllowlisted(address(0xCAFE)));
        assertFalse(hcuLimit.isAllowlisted(address(0)));
    }

    function test_batchRemovalIsAtomic(bool repeated) public {
        _allow(DAPP);
        address[] memory accounts = new address[](2);
        accounts[0] = DAPP;
        accounts[1] = repeated ? DAPP : address(0xBEEF);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotAllowlisted.selector, accounts[1]));
        vm.prank(OWNER);
        hcuLimit.removeFromAllowlist(accounts);
        assertTrue(hcuLimit.isAllowlisted(DAPP));
    }

    function test_addAllowlisterRejectsZeroAddress() public {
        vm.expectRevert(HCULimit.InvalidZeroAddress.selector);
        vm.prank(OWNER);
        hcuLimit.addAllowlister(address(0), block.timestamp + 1);
    }

    function test_addAllowlisterRejectsNonfutureDeadlines(uint256 deadline) public {
        deadline = bound(deadline, 0, block.timestamp);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidAllowlisterDeadline.selector, deadline));
        vm.prank(OWNER);
        hcuLimit.addAllowlister(ALLOWLISTER, deadline);
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), 0);
    }

    function test_grantExtendShortenExpireAndRenew() public {
        uint256 deadline = block.timestamp + 1 days;
        for (uint256 i; i < 3; ++i) {
            uint256 replacement = i == 1 ? deadline + 1 days : deadline;
            vm.expectEmit(true, false, false, true, address(hcuLimit));
            emit HCULimit.AllowlisterAdded(ALLOWLISTER, replacement);
            vm.prank(OWNER);
            hcuLimit.addAllowlister(ALLOWLISTER, replacement);
            assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), replacement);
            assertTrue(hcuLimit.isAllowlister(ALLOWLISTER));
        }
        vm.warp(deadline - 1);
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(DAPP));
        vm.warp(deadline);
        assertFalse(hcuLimit.isAllowlister(ALLOWLISTER));
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), deadline);
        assertTrue(hcuLimit.isAllowlisted(DAPP));
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(address(0xBEEF)));
        uint256 renewedDeadline = deadline + 1 days;
        vm.expectEmit(true, false, false, true, address(hcuLimit));
        emit HCULimit.AllowlisterAdded(ALLOWLISTER, renewedDeadline);
        vm.prank(OWNER);
        hcuLimit.addAllowlister(ALLOWLISTER, renewedDeadline);
        assertTrue(hcuLimit.isAllowlister(ALLOWLISTER));
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), renewedDeadline);
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(address(0xBEEF)));
    }

    function test_revocationAndRenunciation(bool activated, bool renounce) public {
        _activate(activated);
        _grant(ALLOWLISTER);
        _grant(DAPP);
        uint256 otherDeadline = hcuLimit.allowlisterDeadline(DAPP);
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(DAPP));
        vm.expectEmit(true, false, false, true, address(hcuLimit));
        emit HCULimit.AllowlisterRemoved(ALLOWLISTER);
        vm.prank(renounce ? ALLOWLISTER : OWNER);
        if (renounce) hcuLimit.renounceAllowlister();
        else hcuLimit.removeAllowlister(ALLOWLISTER);
        assertFalse(hcuLimit.isAllowlister(ALLOWLISTER));
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), 0);
        assertEq(hcuLimit.allowlisterDeadline(DAPP), otherDeadline);
        assertTrue(hcuLimit.isAllowlisted(DAPP));
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(address(0xBEEF)));
        if (renounce) {
            vm.expectRevert(abi.encodeWithSelector(HCULimit.NotAllowlister.selector, ALLOWLISTER));
            vm.prank(ALLOWLISTER);
            hcuLimit.renounceAllowlister();
        }
    }

    function test_absentRoleCannotRenounce() public {
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.renounceAllowlister();
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), 0);
    }

    function test_expiredRoleCanRenounce(bool pastDeadline) public {
        _grant(ALLOWLISTER);
        uint256 deadline = hcuLimit.allowlisterDeadline(ALLOWLISTER);
        vm.warp(deadline + (pastDeadline ? 1 : 0));
        assertFalse(hcuLimit.isAllowlister(ALLOWLISTER));
        vm.expectEmit(true, false, false, true, address(hcuLimit));
        emit HCULimit.AllowlisterRemoved(ALLOWLISTER);
        vm.prank(ALLOWLISTER);
        hcuLimit.renounceAllowlister();
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), 0);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.renounceAllowlister();
    }

    function test_ownerRevocationClearsInactiveGrants(bool expired, bool pastDeadline, bool activated) public {
        _activate(activated);
        _allow(DAPP);
        if (expired) _grant(ALLOWLISTER);
        uint256 deadline = hcuLimit.allowlisterDeadline(ALLOWLISTER);
        if (expired) vm.warp(deadline + (pastDeadline ? 1 : 0));

        vm.recordLogs();
        vm.prank(OWNER);
        hcuLimit.removeAllowlister(ALLOWLISTER);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, expired ? 1 : 0);
        if (expired) {
            assertEq(logs[0].emitter, address(hcuLimit));
            assertEq(logs[0].topics.length, 2);
            assertEq(logs[0].topics[0], keccak256("AllowlisterRemoved(address)"));
            assertEq(logs[0].topics[1], bytes32(uint256(uint160(ALLOWLISTER))));
            assertEq(logs[0].data.length, 0);
        }
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), 0);
        assertFalse(hcuLimit.isAllowlister(ALLOWLISTER));
        assertTrue(hcuLimit.isAllowlisted(DAPP));
        assertEq(hcuLimit.isAllowlistActivated(), activated);

        vm.recordLogs();
        vm.startPrank(OWNER);
        hcuLimit.removeAllowlister(ALLOWLISTER);
        hcuLimit.removeAllowlister(address(0));
        vm.stopPrank();
        assertEq(vm.getRecordedLogs().length, 0);
    }

    function test_ownerRenouncesOnlyAnActiveGrantAndKeepsOwnerPowers() public {
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotAllowlister.selector, OWNER));
        vm.prank(OWNER);
        hcuLimit.renounceAllowlister();
        _grant(OWNER);
        vm.prank(OWNER);
        hcuLimit.renounceAllowlister();
        assertEq(hcuLimit.allowlisterDeadline(OWNER), 0);
        _allow(DAPP);
    }

    function test_allowlistDeactivationAndReactivationRetainMembershipAndDeadlines() public {
        _grant(ALLOWLISTER);
        uint256 deadline = hcuLimit.allowlisterDeadline(ALLOWLISTER);
        _allow(DAPP);
        vm.expectEmit(false, false, false, true, address(hcuLimit));
        emit HCULimit.AllowlistActivationChanged(true);
        _activate(true);
        _compute(DAPP);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, address(0xBEEF)));
        _compute(address(0xBEEF));
        vm.expectEmit(false, false, false, true, address(hcuLimit));
        emit HCULimit.AllowlistActivationChanged(false);
        _activate(false);
        _compute(address(0xBEEF));
        vm.warp(deadline);
        _activate(true);
        assertTrue(hcuLimit.isAllowlisted(DAPP));
        assertEq(hcuLimit.allowlisterDeadline(ALLOWLISTER), deadline);
        assertFalse(hcuLimit.isAllowlister(ALLOWLISTER));
        _compute(DAPP);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, address(0xBEEF)));
        _compute(address(0xBEEF));
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotACLOwnerOrAllowlister.selector, ALLOWLISTER));
        vm.prank(ALLOWLISTER);
        hcuLimit.addToAllowlist(_accounts(address(0xBEEF)));
        vm.prank(OWNER);
        hcuLimit.removeFromAllowlist(_accounts(DAPP));
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, DAPP));
        _compute(DAPP);
    }

    function test_allAllowlistAndBlockWhitelistCombinations(bool activated, bool allowed, bool whitelisted) public {
        if (allowed) _allow(DAPP);
        if (whitelisted) {
            vm.prank(OWNER);
            hcuLimit.addToBlockHCUWhitelist(DAPP);
        }
        _activate(activated);
        if (activated && !allowed) {
            vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, DAPP));
            _compute(DAPP);
            assertEq(hcuLimit.getHCUForTransaction(), 0);
        } else {
            _compute(DAPP);
            assertEq(hcuLimit.getHCUForTransaction(), 32);
        }
        (, uint48 usedHCU) = hcuLimit.getBlockMeter();
        assertEq(usedHCU, (activated && !allowed) || whitelisted ? 0 : 32);
    }

    function test_allowlistRejectionPrecedesBlockCapAndLeavesMetersUnchanged(bool whitelisted) public {
        _compute(address(0xBEEF));
        if (whitelisted) {
            vm.prank(OWNER);
            hcuLimit.addToBlockHCUWhitelist(DAPP);
        }
        hcuLimit.setHCUPerBlockUnsafeForTest(32);
        _activate(true);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, DAPP));
        _compute(DAPP);
        (, uint48 usedHCU) = hcuLimit.getBlockMeter();
        assertEq(usedHCU, 32);
        assertEq(hcuLimit.getHCUForTransaction(), 32);
    }

    function test_allowlistedDappsRemainSubjectToHcuLimits(bool whitelisted) public {
        _allow(DAPP);
        _activate(true);
        if (whitelisted) {
            vm.prank(OWNER);
            hcuLimit.addToBlockHCUWhitelist(DAPP);
        }
        hcuLimit.setHCUForTransaction(20_000_000);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        _compute(DAPP);
        hcuLimit.setHCUForTransaction(0);
        hcuLimit.setHCUForHandle(INPUT, 5_000_000);
        vm.expectRevert(HCULimit.HCUTransactionDepthLimitExceeded.selector);
        _compute(DAPP);
        hcuLimit.setHCUForHandle(INPUT, 0);
        hcuLimit.setHCUPerBlockUnsafeForTest(0);
        if (whitelisted) _compute(DAPP);
        else {
            vm.expectRevert(HCULimit.HCUBlockLimitExceeded.selector);
            _compute(DAPP);
        }
    }

    function test_everyComputationPathEnforcesAllowlistAndExecutor() public {
        _activate(true);
        bytes[] memory calls = _computationCalls(DAPP);
        for (uint256 i; i < calls.length; ++i) {
            vm.prank(executor);
            (bool success, bytes memory reason) = address(hcuLimit).call(calls[i]);
            assertFalse(success, string(abi.encodePacked("ungated path ", vm.toString(i))));
            assertEq(reason, abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, DAPP));
        }
        (, uint48 usedHCU) = hcuLimit.getBlockMeter();
        assertEq(usedHCU, 0);
        assertEq(hcuLimit.getHCUForTransaction(), 0);
        _allow(DAPP);
        for (uint256 i; i < calls.length; ++i) {
            // A forged dapp argument must not bypass executor-only entry points.
            (bool success, bytes memory reason) = address(hcuLimit).call(calls[i]);
            assertFalse(success);
            assertEq(reason, abi.encodeWithSelector(HCULimit.CallerMustBeFHEVMExecutorContract.selector));
            hcuLimit.setHCUForTransaction(0);
            hcuLimit.setHCUForHandle(RESULT, 0);
            vm.prank(executor);
            (success, reason) = address(hcuLimit).call(calls[i]);
            assertTrue(
                success,
                string(abi.encodePacked("allowlisted path failed ", vm.toString(i), vm.toString(reason)))
            );
        }
    }

    function _computationCalls(address caller) internal pure returns (bytes[] memory calls) {
        bytes4[20] memory binarySelectors = [
            HCULimit.checkHCUForFheAdd.selector,
            HCULimit.checkHCUForFheSub.selector,
            HCULimit.checkHCUForFheMul.selector,
            HCULimit.checkHCUForFheDiv.selector,
            HCULimit.checkHCUForFheRem.selector,
            HCULimit.checkHCUForFheBitAnd.selector,
            HCULimit.checkHCUForFheBitOr.selector,
            HCULimit.checkHCUForFheBitXor.selector,
            HCULimit.checkHCUForFheShl.selector,
            HCULimit.checkHCUForFheShr.selector,
            HCULimit.checkHCUForFheRotl.selector,
            HCULimit.checkHCUForFheRotr.selector,
            HCULimit.checkHCUForFheEq.selector,
            HCULimit.checkHCUForFheNe.selector,
            HCULimit.checkHCUForFheGe.selector,
            HCULimit.checkHCUForFheGt.selector,
            HCULimit.checkHCUForFheLe.selector,
            HCULimit.checkHCUForFheLt.selector,
            HCULimit.checkHCUForFheMin.selector,
            HCULimit.checkHCUForFheMax.selector
        ];
        calls = new bytes[](49);
        uint256 n;
        for (uint256 i; i < binarySelectors.length; ++i) {
            calls[n++] = abi.encodeWithSelector(
                binarySelectors[i],
                FheType.Uint8,
                bytes1(0x01),
                INPUT,
                INPUT,
                RESULT,
                caller
            );
            if (
                binarySelectors[i] != HCULimit.checkHCUForFheDiv.selector &&
                binarySelectors[i] != HCULimit.checkHCUForFheRem.selector
            ) {
                calls[n++] = abi.encodeWithSelector(
                    binarySelectors[i],
                    FheType.Uint8,
                    bytes1(0x00),
                    INPUT,
                    INPUT,
                    RESULT,
                    caller
                );
            }
        }
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheNeg, (FheType.Uint8, INPUT, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheNot, (FheType.Uint8, INPUT, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForCast, (FheType.Uint8, INPUT, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForTrivialEncrypt, (FheType.Uint8, RESULT, caller));
        calls[n++] = abi.encodeCall(
            HCULimit.checkHCUForIfThenElse,
            (FheType.Uint8, INPUT, INPUT, INPUT, RESULT, caller)
        );
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheRand, (FheType.Uint8, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheRandBounded, (FheType.Uint8, RESULT, caller));
        bytes32[] memory values = new bytes32[](2);
        values[0] = INPUT;
        values[1] = INPUT;
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheSum, (FheType.Uint8, values, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheIsIn, (FheType.Uint8, INPUT, values, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheMulDiv, (FheType.Uint8, 0x01, INPUT, INPUT, RESULT, caller));
        calls[n++] = abi.encodeCall(HCULimit.checkHCUForFheMulDiv, (FheType.Uint8, 0x03, INPUT, INPUT, RESULT, caller));
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ACL} from "../../contracts/ACL.sol";
import {FHEVMExecutor} from "../../contracts/FHEVMExecutor.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {HCUConfigurator} from "../../contracts/HCUConfigurator.sol";
import {FheType} from "../../contracts/shared/FheType.sol";
import {HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {aclAdd, fhevmExecutorAdd, hcuConfiguratorAdd, hcuLimitAdd} from "../../addresses/FHEVMHostAddresses.sol";

/// @dev A contract that keeps one encrypted value and extends it by one addition per transaction.
contract DepthUser {
    bytes32 public value;

    function start() external {
        value = FHEVMExecutor(fhevmExecutorAdd).trivialEncrypt(0, FheType.Uint64);
        ACL(aclAdd).allow(value, address(this));
    }

    function addOne(bytes32 input) public {
        value = FHEVMExecutor(fhevmExecutorAdd).fheAdd(input, bytes32(uint256(1)), 0x01);
        ACL(aclAdd).allow(value, address(this));
    }

    function add(bytes32 lhs, bytes32 rhs) public {
        value = FHEVMExecutor(fhevmExecutorAdd).fheAdd(lhs, rhs, 0x00);
        ACL(aclAdd).allow(value, address(this));
    }

    function sum(bytes32 first, bytes32 second) external {
        bytes32[] memory values = new bytes32[](2);
        values[0] = first;
        values[1] = second;
        value = FHEVMExecutor(fhevmExecutorAdd).fheSum(values, FheType.Uint64);
        ACL(aclAdd).allow(value, address(this));
    }

    function isIn(bytes32 member, bytes32 candidate) external {
        bytes32[] memory values = new bytes32[](1);
        values[0] = candidate;
        value = FHEVMExecutor(fhevmExecutorAdd).fheIsIn(member, values, FheType.Uint64);
        ACL(aclAdd).allow(value, address(this));
    }

    function select(bytes32 ifTrue, bytes32 ifFalse) external {
        bytes32 condition = FHEVMExecutor(fhevmExecutorAdd).trivialEncrypt(1, FheType.Bool);
        value = FHEVMExecutor(fhevmExecutorAdd).fheIfThenElse(condition, ifTrue, ifFalse);
        ACL(aclAdd).allow(value, address(this));
    }

    /// @dev Builds two additions within this transaction, then adds `stored` to the result.
    function addToNewChain(bytes32 stored) external {
        bytes32 chain = FHEVMExecutor(fhevmExecutorAdd).trivialEncrypt(0, FheType.Uint64);
        chain = FHEVMExecutor(fhevmExecutorAdd).fheAdd(chain, bytes32(uint256(1)), 0x01);
        chain = FHEVMExecutor(fhevmExecutorAdd).fheAdd(chain, bytes32(uint256(1)), 0x01);
        add(stored, chain);
    }

    /// @dev Computes on `input` without keeping the result: only FHEVMExecutor grants it, transiently, to this contract.
    function compute(bytes32 input) external {
        FHEVMExecutor(fhevmExecutorAdd).fheAdd(input, bytes32(uint256(1)), 0x01);
    }

    /// @dev Computes on `input` without keeping the result, with three operands (0), an array (1), a value and an
    ///      array (2), or one operand (3).
    function computeShape(bytes32 input, uint8 shape) external {
        bytes32[] memory values = new bytes32[](2);
        values[0] = input;
        values[1] = input;
        if (shape == 0) {
            bytes32 condition = FHEVMExecutor(fhevmExecutorAdd).trivialEncrypt(1, FheType.Bool);
            FHEVMExecutor(fhevmExecutorAdd).fheIfThenElse(condition, input, input);
        } else if (shape == 1) {
            FHEVMExecutor(fhevmExecutorAdd).fheSum(values, FheType.Uint64);
        } else if (shape == 2) {
            FHEVMExecutor(fhevmExecutorAdd).fheIsIn(input, values, FheType.Uint64);
        } else {
            FHEVMExecutor(fhevmExecutorAdd).fheNeg(input);
        }
    }

    /// @dev Keeps `input`, which another contract handed over in this transaction, without computing on it.
    function keep(bytes32 input) external {
        value = input;
        ACL(aclAdd).allow(input, address(this));
    }

    /// @dev Hands this contract's value to `recipient` for this transaction, and `recipient` keeps it.
    function handOver(DepthUser recipient) external {
        ACL(aclAdd).allowTransient(value, address(recipient));
        recipient.keep(value);
    }

    function forward(DepthUser recipient) external {
        ACL(aclAdd).allowTransient(value, address(recipient));
        recipient.addOne(value);
    }

    function share(address account) external {
        ACL(aclAdd).allow(value, account);
    }
}

/// @dev Each call runs as its own transaction, so depth can only cross calls through ACL storage.
/// forge-config: default.isolate = true
contract CarriedDepthTest is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);
    uint16 internal constant DEFAULT_QUOTA = 0;
    uint16 internal constant PARTNER_QUOTA = 5;

    ACL internal acl;
    HCULimit internal hcu;
    HCUConfigurator internal hcuConfigurator;
    DepthUser internal alice;
    DepthUser internal bob;

    function setUp() public {
        _deployACL(owner);
        _deployHCULimit(owner);
        _deployHCUConfigurator(owner);
        _deployFHEVMExecutor(owner);
        acl = ACL(aclAdd);
        hcu = HCULimit(hcuLimitAdd);
        hcuConfigurator = HCUConfigurator(hcuConfiguratorAdd);
        alice = new DepthUser();
        bob = new DepthUser();
    }

    function test_SmallChainedTransactionsAccumulateUntilTheLimit() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setLimit(start + 3 * step, 1);

        alice.addOne(alice.value());
        assertEq(_storedDepth(alice), start + 2 * step);
        alice.addOne(alice.value());
        bytes32 last = alice.value();
        // One step over the limit, at one HCU of decay per block.
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, step));
        alice.addOne(last);

        // An independent chain is unaffected.
        bob.start();
        bob.addOne(bob.value());
    }

    function test_CarriedDepthDecaysWithBlocks() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        // One block of decay falls one short of the excess, so the reported wait rounds up to two blocks.
        _setLimit(start + 2 * step, step - 1);
        alice.addOne(alice.value());
        bytes32 last = alice.value();
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, 2));
        alice.addOne(last);

        vm.roll(block.number + 1);
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, 1));
        alice.addOne(last);

        vm.roll(block.number + 1);
        alice.addOne(last);
        assertEq(_storedDepth(alice), start + step + 2);
    }

    /// @dev ACL before 0.7.0 stored `true` in the permission's slot: it reads as allowed with no carried depth, so a
    ///      computation on that handle starts a new chain. A permission stored now keeps `true` in its low byte, which
    ///      is all the previous ACL reads.
    function test_PermissionStoredBeforeUpgradeCarriesNoDepth() public {
        (, uint48 step) = _startAndMeasure(alice);
        bytes32 handle = alice.value();
        bytes32 aclStorageLocation = 0xa688f31953c2015baaf8c0a488ee1ee22eb0e05273cc1fd31ea4cbee42febc00;
        bytes32 slot = keccak256(abi.encode(address(alice), keccak256(abi.encode(handle, aclStorageLocation))));
        assertEq(uint8(uint256(vm.load(aclAdd, slot))), 1);

        vm.store(aclAdd, slot, bytes32(uint256(1)));
        assertTrue(acl.isAllowed(handle, address(alice)));
        (uint48 depth, uint48 storedBlock) = acl.getStoredDepth(handle, address(alice));
        assertEq(depth, 0);
        assertEq(storedBlock, 0);

        alice.addOne(handle);
        assertEq(_storedDepth(alice), step);
    }

    function test_DeepestInputSetsCarriedDepth() public {
        _startAndMeasure(alice);
        for (uint256 i = 0; i < 20; i++) {
            alice.addOne(alice.value());
        }
        bytes32 deep = alice.value();
        uint48 deepDepth = _storedDepth(alice);
        alice.share(address(bob));

        bob.start();
        bob.add(bob.value(), deep);
        assertGt(_storedDepth(bob), deepDepth);

        bob.start();
        bob.sum(bob.value(), deep);
        assertGt(_storedDepth(bob), deepDepth);

        bob.start();
        bob.isIn(deep, bob.value());
        assertGt(_storedDepth(bob), deepDepth);

        bob.start();
        bob.select(bob.value(), deep);
        assertGt(_storedDepth(bob), deepDepth);
    }

    function test_TransactionDepthComesFromDeepestInputWithinTransaction() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        for (uint256 i = 0; i < 10; i++) {
            alice.addOne(alice.value());
        }
        alice.share(address(bob));
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(start + 2 * step);

        // The stored handle carries more depth, but the new chain is deeper within this transaction.
        bytes32 stored = alice.value();
        vm.expectRevert(HCULimit.HCUTransactionDepthLimitExceeded.selector);
        bob.addToNewChain(stored);
    }

    function test_TransientForwardCarriesDepth() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        alice.forward(bob);
        assertEq(_storedDepth(bob), start + 2 * step);
    }

    function test_SharedPermissionCarriesDepth() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        alice.share(address(bob));
        bob.addOne(alice.value());
        assertEq(_storedDepth(bob), start + 2 * step);
    }

    function test_FirstGrantWritesOneWordAndRepeatedGrantWritesNothing() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);

        vm.record();
        alice.share(address(bob));
        (, bytes32[] memory writes) = vm.accesses(aclAdd);
        assertEq(writes.length, 1);
        assertTrue(acl.isAllowed(alice.value(), address(bob)));
        (uint48 depth, uint48 storedBlock) = acl.getStoredDepth(alice.value(), address(bob));
        assertEq(depth, start + step);
        assertEq(storedBlock, block.number);

        vm.record();
        alice.share(address(bob));
        (, writes) = vm.accesses(aclAdd);
        assertEq(writes.length, 0);
        assertTrue(acl.isAllowed(alice.value(), address(bob)));
    }

    function test_TransactionDepthLimitIgnoresCarriedDepth() public {
        (, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        for (uint256 i = 0; i < 4; i++) {
            alice.addOne(alice.value());
        }
        assertGt(_storedDepth(alice), 4 * step);
    }

    function test_PermissionGrantedWhileDisabledStartsNewChain() public {
        alice.start();
        for (uint256 i = 0; i < 5; i++) {
            alice.addOne(alice.value());
        }
        assertEq(_storedDepth(alice), 0);

        (, uint48 step) = _startAndMeasure(bob);
        alice.addOne(alice.value());
        assertEq(_storedDepth(alice), step);
    }

    function test_CacheDepthRejectsOtherCallers() public {
        vm.expectRevert(HCULimit.CallerMustBeACLContract.selector);
        hcu.cacheDepth(keccak256("handle"), address(this));
    }

    function test_LimitAndDecayMustBeBothSetOrBothZero() public {
        vm.startPrank(owner);
        vm.expectRevert(HCUConfigurator.InvalidCarriedDepthLimit.selector);
        hcuConfigurator.setCarriedDepthLimit(1_000_000, 0);
        vm.expectRevert(HCUConfigurator.InvalidCarriedDepthLimit.selector);
        hcuConfigurator.setCarriedDepthLimit(0, 1);
        vm.stopPrank();
    }

    function test_PerTransactionLimitsAreLiftedOnceTheDepthLimitIsOn() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        // The rollout: enable the chain's carried-depth limit and quotas' limits, both below maxHCUDepthPerTx, then lift the
        // per-transaction limits.
        _setLimit(start + step, 1);
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, start + step);
        vm.startPrank(owner);
        hcuConfigurator.setMaxHCUPerTx(type(uint48).max);
        hcuConfigurator.setMaxHCUDepthPerTx(type(uint48).max);
        vm.stopPrank();
        assertEq(hcu.getMaxHCUPerTx(), type(uint48).max);
        assertEq(hcu.getMaxHCUDepthPerTx(), type(uint48).max);

        // The depth limit still bounds a transaction on fresh inputs: its second addition is one step over, and
        // waiting cannot fit a chain built within the transaction.
        bob.start();
        bytes32 fresh = bob.value();
        vm.expectRevert(_depthExceeded(bob, DEFAULT_QUOTA, type(uint256).max));
        bob.addToNewChain(fresh);
    }

    function test_LimitCannotBeEnabledBeforeACLIsUpgraded() public {
        // An ACL without getStoredDepth, like ACL before this upgrade.
        vm.mockCallRevert(aclAdd, abi.encodeWithSelector(ACL.getStoredDepth.selector), "");
        vm.startPrank(owner);
        vm.expectRevert();
        hcuConfigurator.setCarriedDepthLimit(20_000_000, 10_000_000);
        hcuConfigurator.setCarriedDepthLimit(0, 0);
        vm.stopPrank();
    }

    /// A contract in a stricter quota can keep a handle that a looser quota's contract hands over: the grant creates no
    /// work. It cannot compute on the handle until it decays under its own limit.
    function test_AStricterQuotaKeepsAHandedOverHandleButCannotComputeOnIt() public {
        (uint48 start, uint48 step) = _startAndMeasure(bob);
        bob.addOne(bob.value());
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, start + step);
        vm.startPrank(owner);
        hcuConfigurator.setQuota(PARTNER_QUOTA, type(uint48).max, type(uint48).max);
        hcuConfigurator.assignQuota(PARTNER_QUOTA, address(bob));
        vm.stopPrank();
        _setQuotaMaxCarriedDepth(PARTNER_QUOTA, start + 2 * step);

        bob.handOver(alice);
        assertEq(alice.value(), bob.value());
        assertEq(_storedDepth(alice), start + 2 * step);

        bytes32 kept = alice.value();
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, 2 * step));
        alice.compute(kept);
        vm.roll(block.number + 2 * step);
        alice.compute(kept);
    }

    function test_GrantsAreNotCheckedAgainstTheDepthLimit() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        alice.addOne(alice.value());
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, start + step);

        alice.share(address(bob));
        assertEq(_storedDepth(alice), start + 2 * step);
        (uint48 depth, ) = acl.getStoredDepth(alice.value(), address(bob));
        assertEq(depth, start + 2 * step);
    }

    /// A contract computes no value deeper than its quota's limit, even one it never grants: it could otherwise
    /// make the value public with allowForDecryption, which checks no depth.
    function test_QuotaDepthLimitRefusesEveryResultNotOnlyGrants() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, start + step);
        bytes32 value = alice.value();
        // One step over the limit, at one HCU of decay per block.
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, step));
        alice.compute(value);

        // Every operation shape checks it.
        for (uint8 shape; shape < 4; ++shape) {
            vm.expectPartialRevert(HCULimit.HCUCarriedDepthExceeded.selector);
            alice.computeShape(value, shape);
        }
    }

    function test_QuotaDepthLimitIsOffWhileTheDepthLimitIsOff() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, start + step);
        _setLimit(0, 0);
        alice.addOne(alice.value());
    }

    function test_EachQuotaComputesUpToItsOwnQuotaDepthLimit() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, start + step);
        vm.startPrank(owner);
        hcuConfigurator.setQuota(PARTNER_QUOTA, type(uint48).max, type(uint48).max);
        hcuConfigurator.assignQuota(PARTNER_QUOTA, address(bob));
        vm.stopPrank();
        _setQuotaMaxCarriedDepth(PARTNER_QUOTA, start + 3 * step);

        alice.share(address(bob));
        bob.addOne(alice.value());
        assertEq(_storedDepth(bob), start + 2 * step);
        bytes32 aliceValue = alice.value();
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, step));
        alice.addOne(aliceValue);
    }

    /// A quota without a limit uses the chain's; a quota's own limit replaces it, looser or stricter.
    function test_AQuotaLimitReplacesTheChainLimitInEitherDirection() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setLimit(start + 2 * step, 1);
        vm.startPrank(owner);
        hcuConfigurator.setQuota(PARTNER_QUOTA, type(uint48).max, type(uint48).max);
        hcuConfigurator.assignQuota(PARTNER_QUOTA, address(bob));
        vm.stopPrank();
        alice.share(address(bob));
        bytes32 shared = alice.value();

        // Unset: alice's quota uses the chain's limit.
        alice.addOne(shared);
        bytes32 atChainLimit = alice.value();
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, step));
        alice.addOne(atChainLimit);

        // Looser: bob's quota goes past the chain's limit.
        _setQuotaMaxCarriedDepth(PARTNER_QUOTA, start + 4 * step);
        bob.addOne(shared);
        bob.addOne(bob.value());
        assertEq(_storedDepth(bob), start + 3 * step);

        // Stricter: alice's quota refuses below the chain's limit.
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, start + step);
        vm.expectRevert(_depthExceeded(alice, DEFAULT_QUOTA, step));
        alice.addOne(shared);
    }

    function test_AnyQuotaTakesAnyLimitWhileTheChainLimitIsOff() public {
        _setQuotaMaxCarriedDepth(1, 6_000_000);
        _setQuotaMaxCarriedDepth(2, 30_000_000);
        _setQuotaMaxCarriedDepth(DEFAULT_QUOTA, 5_000_000);
        assertEq(hcu.getQuotaMaxCarriedDepth(2), 30_000_000);
    }

    function _setQuotaMaxCarriedDepth(uint16 quotaId, uint48 maxCarriedDepth) internal {
        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.QuotaMaxCarriedDepthSet(quotaId, maxCarriedDepth, owner);
        vm.prank(owner);
        hcuConfigurator.setQuotaMaxCarriedDepth(quotaId, maxCarriedDepth);
    }

    /// @dev Enables the limit without bound, starts `user`'s chain and returns its starting depth and the depth one addition adds.
    function _startAndMeasure(DepthUser user) internal returns (uint48 start, uint48 step) {
        _setLimit(type(uint48).max, 1);
        user.start();
        start = _storedDepth(user);
        user.addOne(user.value());
        step = _storedDepth(user) - start;
    }

    function _setLimit(uint48 maxDepth, uint48 decayPerBlock) internal {
        vm.prank(owner);
        hcuConfigurator.setCarriedDepthLimit(maxDepth, decayPerBlock);
    }

    function _depthExceeded(
        DepthUser user,
        uint16 quotaId,
        uint256 retryAfterBlocks
    ) internal pure returns (bytes memory) {
        return
            abi.encodeWithSelector(HCULimit.HCUCarriedDepthExceeded.selector, address(user), quotaId, retryAfterBlocks);
    }

    function _storedDepth(DepthUser user) internal view returns (uint48 depth) {
        (depth, ) = acl.getStoredDepth(user.value(), address(user));
    }
}

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
contract CrossTxDepthTest is HostContractsDeployerTestUtils {
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
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUCrossTxDepthLimitExceeded.selector, uint256(step)));
        alice.addOne(last);

        // An independent chain is unaffected.
        bob.start();
        bob.addOne(bob.value());
    }

    function test_CarriedDepthDecaysWithBlocks() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setLimit(start + 2 * step, step);
        alice.addOne(alice.value());
        bytes32 last = alice.value();
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUCrossTxDepthLimitExceeded.selector, uint256(1)));
        alice.addOne(last);

        vm.roll(block.number + 1);
        alice.addOne(alice.value());
        assertEq(_storedDepth(alice), start + 2 * step);
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
        vm.expectRevert(HCUConfigurator.InvalidCrossTxDepthLimit.selector);
        hcuConfigurator.setCrossTxDepthLimit(1_000_000, 0);
        vm.expectRevert(HCUConfigurator.InvalidCrossTxDepthLimit.selector);
        hcuConfigurator.setCrossTxDepthLimit(0, 1);
        vm.stopPrank();
    }

    function test_LimitCoversTransactionDepthLimitFromBothSetters() public {
        uint48 maxDepthPerTx = hcu.getMaxHCUDepthPerTx();
        vm.startPrank(owner);
        vm.expectRevert(HCUConfigurator.CrossTxDepthBelowMaxDepthPerTx.selector);
        hcuConfigurator.setCrossTxDepthLimit(maxDepthPerTx - 1, 1);
        hcuConfigurator.setCrossTxDepthLimit(maxDepthPerTx, 1);
        vm.expectRevert(HCUConfigurator.CrossTxDepthBelowMaxDepthPerTx.selector);
        hcuConfigurator.setMaxHCUDepthPerTx(maxDepthPerTx + 1);
        vm.stopPrank();
    }

    function test_LimitCannotBeEnabledBeforeACLIsUpgraded() public {
        // An ACL without getStoredDepth, like ACL before this upgrade.
        vm.mockCallRevert(aclAdd, abi.encodeWithSelector(ACL.getStoredDepth.selector), "");
        vm.startPrank(owner);
        vm.expectRevert();
        hcuConfigurator.setCrossTxDepthLimit(20_000_000, 10_000_000);
        hcuConfigurator.setCrossTxDepthLimit(0, 0);
        vm.stopPrank();
    }

    function test_StoredDepthLimitRefusesADeepGrant() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setStoredDepthLimit(DEFAULT_QUOTA, start + 2 * step);

        alice.addOne(alice.value());
        assertEq(_storedDepth(alice), start + 2 * step);
        bytes32 last = alice.value();
        // The result is computed in the same transaction, so its depth comes from transient storage. One step over the
        // limit, at one HCU of decay per block.
        vm.expectRevert(
            abi.encodeWithSelector(
                HCULimit.HCUStoredDepthExceeded.selector,
                address(alice),
                DEFAULT_QUOTA,
                uint256(step)
            )
        );
        alice.addOne(last);
    }

    function test_StoredDepthLimitChecksEveryGrantOfAStoredHandle() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        alice.addOne(alice.value());
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setStoredDepthLimit(DEFAULT_QUOTA, start + step);
        bytes memory refused = abi.encodeWithSelector(
            HCULimit.HCUStoredDepthExceeded.selector,
            address(alice),
            DEFAULT_QUOTA,
            uint256(step)
        );

        vm.expectRevert(refused);
        alice.share(address(bob));
        vm.expectRevert(refused);
        alice.forward(bob);
        // A grant already persisted writes nothing and is not checked.
        alice.share(address(alice));

        vm.roll(block.number + step);
        alice.share(address(bob));
    }

    function test_StoredDepthLimitLeavesTheExecutorsTransientGrantsUnchecked() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setStoredDepthLimit(DEFAULT_QUOTA, start + step);
        // FHEVMExecutor hands the deeper result to alice; only a grant by alice would be checked.
        alice.compute(alice.value());
    }

    function test_StoredDepthLimitIsOffWhileTheDepthLimitIsOff() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setStoredDepthLimit(DEFAULT_QUOTA, start + step);
        _setLimit(0, 0);
        alice.addOne(alice.value());
    }

    function test_EachQuotaGrantsUpToItsOwnStoredDepthLimit() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(step);
        _setStoredDepthLimit(DEFAULT_QUOTA, start + step);
        vm.startPrank(owner);
        hcuConfigurator.setQuota(PARTNER_QUOTA, type(uint48).max, type(uint48).max);
        hcuConfigurator.assignQuota(PARTNER_QUOTA, address(bob));
        vm.stopPrank();
        _setStoredDepthLimit(PARTNER_QUOTA, start + 3 * step);

        alice.share(address(bob));
        bob.addOne(alice.value());
        assertEq(_storedDepth(bob), start + 2 * step);
        bytes32 aliceValue = alice.value();
        vm.expectRevert(
            abi.encodeWithSelector(
                HCULimit.HCUStoredDepthExceeded.selector,
                address(alice),
                DEFAULT_QUOTA,
                uint256(step)
            )
        );
        alice.addOne(aliceValue);
    }

    function test_SetQuotaMaxStoredDepthKeepsTheDefaultQuotaTheStrictest() public {
        // maxHCUDepthPerTx is 5,000,000 after deployment.
        _setLimit(20_000_000, 1);
        vm.startPrank(owner);
        // Quotas 1 and 2 take no limit.
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidMaxStoredDepth.selector, uint16(1)));
        hcuConfigurator.setQuotaMaxStoredDepth(1, 6_000_000);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidMaxStoredDepth.selector, uint16(2)));
        hcuConfigurator.setQuotaMaxStoredDepth(2, 0);
        // A limit lies between maxHCUDepthPerTx and maxDepth.
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidMaxStoredDepth.selector, DEFAULT_QUOTA));
        hcuConfigurator.setQuotaMaxStoredDepth(DEFAULT_QUOTA, 4_999_999);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidMaxStoredDepth.selector, DEFAULT_QUOTA));
        hcuConfigurator.setQuotaMaxStoredDepth(DEFAULT_QUOTA, 20_000_001);
        // Another quota's limit needs the default quota's first, and is not below it; its 0 is always accepted.
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidMaxStoredDepth.selector, PARTNER_QUOTA));
        hcuConfigurator.setQuotaMaxStoredDepth(PARTNER_QUOTA, 10_000_000);
        hcuConfigurator.setQuotaMaxStoredDepth(PARTNER_QUOTA, 0);
        hcuConfigurator.setQuotaMaxStoredDepth(DEFAULT_QUOTA, 8_000_000);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidMaxStoredDepth.selector, PARTNER_QUOTA));
        hcuConfigurator.setQuotaMaxStoredDepth(PARTNER_QUOTA, 7_999_999);
        hcuConfigurator.setQuotaMaxStoredDepth(PARTNER_QUOTA, 8_000_000);
        vm.stopPrank();
        assertEq(hcu.getQuotaMaxStoredDepth(PARTNER_QUOTA), 8_000_000);
    }

    function test_StoredDepthLimitNeedsTheDepthLimit() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidMaxStoredDepth.selector, DEFAULT_QUOTA));
        hcuConfigurator.setQuotaMaxStoredDepth(DEFAULT_QUOTA, 5_000_000);
    }

    function _setStoredDepthLimit(uint16 quotaId, uint48 maxStoredDepth) internal {
        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.QuotaMaxStoredDepthSet(quotaId, maxStoredDepth, owner);
        vm.prank(owner);
        hcuConfigurator.setQuotaMaxStoredDepth(quotaId, maxStoredDepth);
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
        hcuConfigurator.setCrossTxDepthLimit(maxDepth, decayPerBlock);
    }

    function _storedDepth(DepthUser user) internal view returns (uint48 depth) {
        (depth, ) = acl.getStoredDepth(user.value(), address(user));
    }
}

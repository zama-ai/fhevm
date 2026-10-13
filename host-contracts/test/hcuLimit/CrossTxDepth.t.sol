// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ACL} from "../../contracts/ACL.sol";
import {FHEVMExecutor} from "../../contracts/FHEVMExecutor.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {FheType} from "../../contracts/shared/FheType.sol";
import {HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {aclAdd, fhevmExecutorAdd, hcuLimitAdd} from "../../addresses/FHEVMHostAddresses.sol";

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
    bytes32 internal constant ACL_STORAGE_ROOT = 0xa688f31953c2015baaf8c0a488ee1ee22eb0e05273cc1fd31ea4cbee42febc00;

    ACL internal acl;
    HCULimit internal hcu;
    DepthUser internal alice;
    DepthUser internal bob;

    function setUp() public {
        _deployACL(owner);
        _deployHCULimit(owner);
        _deployFHEVMExecutor(owner);
        acl = ACL(aclAdd);
        hcu = HCULimit(hcuLimitAdd);
        alice = new DepthUser();
        bob = new DepthUser();
    }

    function test_SmallChainedTransactionsAccumulateUntilTheLimit() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        _setLimit(start + 3 * step, 1);

        alice.addOne(alice.value());
        assertEq(_storedDepth(alice), start + 2 * step);
        alice.addOne(alice.value());
        bytes32 last = alice.value();
        vm.expectRevert(HCULimit.HCUCrossTxDepthLimitExceeded.selector);
        alice.addOne(last);

        // An independent chain is unaffected.
        bob.start();
        bob.addOne(bob.value());
    }

    function test_CarriedDepthDecaysWithBlocks() public {
        (uint48 start, uint48 step) = _startAndMeasure(alice);
        _setLimit(start + 2 * step, step);
        alice.addOne(alice.value());
        bytes32 last = alice.value();
        vm.expectRevert(HCULimit.HCUCrossTxDepthLimitExceeded.selector);
        alice.addOne(last);

        vm.roll(block.number + 1);
        alice.addOne(alice.value());
        assertEq(_storedDepth(alice), start + 2 * step);
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
        assertEq(uint256(vm.load(aclAdd, writes[0])), 1 | (uint256(start + step) << 8) | (block.number << 56));

        vm.record();
        alice.share(address(bob));
        (, writes) = vm.accesses(aclAdd);
        assertEq(writes.length, 0);
        assertTrue(acl.isAllowed(alice.value(), address(bob)));
    }

    function test_TransactionDepthLimitIgnoresCarriedDepth() public {
        (, uint48 step) = _startAndMeasure(alice);
        vm.prank(owner);
        hcu.setMaxHCUDepthPerTx(step);
        for (uint256 i = 0; i < 4; i++) alice.addOne(alice.value());
        assertGt(_storedDepth(alice), 4 * step);
    }

    function test_VerifiedInputsStartNewChains() public {
        (, uint48 step) = _startAndMeasure(alice);
        // A verified input carries its index in byte 21; any depth stored next to its permission is ignored.
        bytes32 input = keccak256("input") & ~bytes32(uint256(type(uint88).max));
        input |= bytes32(uint256(uint8(FheType.Uint64)) << 8);
        vm.store(aclAdd, _permissionSlot(input, address(alice)), bytes32(1 | (uint256(type(uint48).max) << 8)));

        alice.addOne(input);
        assertEq(_storedDepth(alice), step);
    }

    function test_PermissionsWithoutStoredDepthReadAsZero() public {
        bytes32 handle = keccak256("old permission");
        vm.store(aclAdd, _permissionSlot(handle, address(alice)), bytes32(uint256(1)));
        (bool allowed, uint48 depth, uint48 storedBlock) = acl.getStoredDepth(handle, address(alice));
        assertTrue(allowed);
        assertTrue(acl.isAllowed(handle, address(alice)));
        assertEq(depth, 0);
        assertEq(storedBlock, 0);
    }

    function test_DisabledLimitStoresNoDepth() public {
        alice.start();
        for (uint256 i = 0; i < 5; i++) alice.addOne(alice.value());
        assertEq(_storedDepth(alice), 0);
    }

    function test_OnlyProtocolContractsSetDepth() public {
        _setLimit(1_000_000, 1);
        vm.expectRevert(HCULimit.CallerMustBeACLOrBridge.selector);
        hcu.loadDepth(keccak256("handle"), address(this));
        vm.expectRevert(HCULimit.CallerMustBeConfidentialBridge.selector);
        hcu.setBridgedDepths(new bytes32[](0), new uint48[](0), uint48(block.number));
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
        hcu.setCrossTxDepthLimit(maxDepth, decayPerBlock);
    }

    function _storedDepth(DepthUser user) internal view returns (uint48 depth) {
        (, depth, ) = acl.getStoredDepth(user.value(), address(user));
    }

    function _permissionSlot(bytes32 handle, address account) internal pure returns (bytes32) {
        return keccak256(abi.encode(account, keccak256(abi.encode(handle, ACL_STORAGE_ROOT))));
    }
}

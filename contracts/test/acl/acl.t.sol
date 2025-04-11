// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {httpzExecutorAdd} from "../../addresses/HTTPZExecutorAddress.sol";

contract ACLTest is Test {
    ACL internal acl;

    address internal constant owner = address(456);

    address internal proxy;
    address internal implementation;
    address internal httpzExecutor;

    /// @dev This helper function allows to add any handle for any account.
    function _allowHandle(bytes32 handle, address account) internal {
        vm.prank(httpzExecutor);
        acl.allowTransient(handle, account);
        vm.prank(account);
        acl.allow(handle, account);
        acl.cleanTransientStorage();
    }

    function setUp() public {
        /// @dev It uses UnsafeUpgrades for measuring code coverage.
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, owner)
        );

        implementation = address(new ACL());
        UnsafeUpgrades.upgradeProxy(proxy, implementation, "", owner);
        acl = ACL(proxy);
        httpzExecutor = acl.getHTTPZExecutorAddress();

        assertEq(acl.owner(), owner);
    }

    function test_getVersion() public view {
        assertEq(acl.getVersion(), string(abi.encodePacked("ACL v0.1.0")));
    }

    function test_HTTPZExecutorAddress() public view {
        assertEq(acl.getHTTPZExecutorAddress(), httpzExecutorAdd);
    }

    function test_isAllowedReturnsFalseIfNotAllowed(bytes32 handle, address account) public view {
        assertFalse(acl.isAllowed(handle, account));
    }

    function test_isAllowedForDecryptionReturnsFalseIfNotAllowed(bytes32 handle) public view {
        assertFalse(acl.isAllowedForDecryption(handle));
    }

    function test_allowedTransientReturnsFalseIfNotAllowed(bytes32 handle, address account) public view {
        assertFalse(acl.allowedTransient(handle, account));
    }

    function test_persistAllowedReturnsFalseIfNotAllowed(bytes32 handle, address account) public view {
        assertFalse(acl.persistAllowed(handle, account));
    }

    function test_CannotAllowIfNotAllowedToUseTheHandle(bytes32 handle, address account) public {
        vm.expectPartialRevert(ACL.SenderNotAllowed.selector);
        acl.allow(handle, account);
    }

    function test_CannotAllowTrasientIfNotAllowedToUseTheHandle(bytes32 handle, address account) public {
        vm.expectPartialRevert(ACL.SenderNotAllowed.selector);
        acl.allowTransient(handle, account);
    }

    function test_CanAllowTransientIfhttpzAddress(bytes32 handle, address account) public {
        vm.prank(httpzExecutor);
        acl.allowTransient(handle, account);
        assertTrue(acl.allowedTransient(handle, account));
        assertTrue(acl.isAllowed(handle, account));
    }

    function test_CanAllowTransientIfhttpzAddressButOnlyUntilItGetsCleaned(bytes32 handle, address account) public {
        vm.prank(httpzExecutor);
        acl.allowTransient(handle, account);
        acl.cleanTransientStorage();
        assertFalse(acl.allowedTransient(handle, account));
        assertFalse(acl.isAllowed(handle, account));
    }

    function test_CanAllow(bytes32 handle, address account) public {
        assertFalse(acl.isAllowed(handle, account));
        _allowHandle(handle, account);
        assertTrue(acl.isAllowed(handle, account));
        assertTrue(acl.persistAllowed(handle, account));
    }

    function test_CanDelegateAccountButItIsAllowedOnBehalfOnlyIfBothContractAndSenderAreAllowed(
        bytes32 handle,
        address sender,
        address delegatee,
        address delegateeContract
    ) public {
        vm.assume(sender != delegateeContract);

        vm.prank(sender);
        vm.expectEmit(address(acl));

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;
        emit ACL.NewDelegation(sender, delegatee, contractAddresses);
        acl.delegateAccount(delegatee, contractAddresses);
        vm.assertFalse(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));

        /// @dev The sender and the delegatee contract must be allowed to use the handle before it delegates.
        _allowHandle(handle, sender);
        vm.assertFalse(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));
        _allowHandle(handle, delegateeContract);
        vm.assertTrue(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));
    }

    function test_CannotDelegateAccountToSameAccountTwice(
        bytes32 handle,
        address sender,
        address delegatee,
        address delegateeContract
    ) public {
        /// @dev We call the other test to avoid repeating the same code.
        test_CanDelegateAccountButItIsAllowedOnBehalfOnlyIfBothContractAndSenderAreAllowed(
            handle,
            sender,
            delegatee,
            delegateeContract
        );

        vm.prank(sender);
        vm.expectPartialRevert(ACL.AlreadyDelegated.selector);
        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;
        acl.delegateAccount(delegatee, contractAddresses);
    }

    function test_CannotDelegateIfSenderIsDelegateeContract(address sender, address delegatee) public {
        vm.prank(sender);
        vm.expectPartialRevert(ACL.SenderCannotBeContractAddress.selector);
        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = sender;
        acl.delegateAccount(delegatee, contractAddresses);
    }

    function test_CanDelegateAccountIfAccountNotAllowed(
        bytes32 handle,
        address sender,
        address delegatee,
        address delegateeContract
    ) public {
        vm.assume(sender != delegateeContract);
        /// @dev Only the delegatee contract must be allowed to use the handle before it delegates.
        _allowHandle(handle, delegateeContract);

        vm.prank(sender);
        vm.expectEmit(address(acl));
        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;
        emit ACL.NewDelegation(sender, delegatee, contractAddresses);
        acl.delegateAccount(delegatee, contractAddresses);

        vm.assertFalse(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));
    }

    function test_NoOneCanAllowForDecryptionIfEmptyList(address sender) public {
        bytes32[] memory handlesList = new bytes32[](0);
        vm.prank(sender);
        vm.expectRevert(ACL.HandlesListIsEmpty.selector);
        acl.allowForDecryption(handlesList);
    }

    function test_CanAllowForDecryptionIfSenderIsApprovedToUseHandle(
        address sender,
        bytes32 handle0,
        bytes32 handle1
    ) public {
        bytes32[] memory handlesList = new bytes32[](2);
        handlesList[0] = handle0;
        handlesList[1] = handle1;

        _allowHandle(handle0, sender);
        _allowHandle(handle1, sender);

        vm.prank(sender);
        vm.expectEmit(address(acl));
        emit ACL.AllowedForDecryption(address(sender), handlesList);
        acl.allowForDecryption(handlesList);

        assertTrue(acl.isAllowedForDecryption(handle0));
        assertTrue(acl.isAllowedForDecryption(handle1));
    }

    function test_CannotAllowForDecryptionIfSenderIsNotAllowedToUseTheHandle(bytes32 handle0, bytes32 handle1) public {
        bytes32[] memory handlesList = new bytes32[](2);
        handlesList[0] = handle0;
        handlesList[1] = handle1;
        vm.expectPartialRevert(ACL.SenderNotAllowed.selector);
        acl.allowForDecryption(handlesList);
    }

    function test_CannotAllowForDecryptionIfSenderIsNotAllowedToUseOneOfTheHandles(
        address sender,
        bytes32 handle0,
        bytes32 handle1
    ) public {
        vm.assume(handle0 != handle1);
        bytes32[] memory handlesList = new bytes32[](2);
        handlesList[0] = handle0;
        handlesList[1] = handle1;

        /// @dev Only the handle0 is allowed.
        _allowHandle(handle0, sender);

        vm.prank(sender);
        vm.expectPartialRevert(ACL.SenderNotAllowed.selector);
        acl.allowForDecryption(handlesList);
    }

    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        this.upgrade(randomAccount);
    }

    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", owner);
    }
}

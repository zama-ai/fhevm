// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMExecutorAddress.sol";

contract ACLTest is Test {
    ACL internal acl;

    address internal constant owner = address(456);

    address internal proxy;
    address internal implementation;
    address internal fhevmExecutor;

    /**
     * @dev Grants permissions for a hnadle for an account for testing purposes.
     *
     * @param handle The handle identifier.
     * @param account The account to grant permissions to.
     */
    function _allowHandle(bytes32 handle, address account) internal {
        vm.prank(fhevmExecutor);
        acl.allowTransient(handle, account);
        vm.prank(account);
        acl.allow(handle, account);
        acl.cleanTransientStorage();
    }

    /**
     * @dev Sets up the testing environment by deploying a proxy contract and initializing signers.
     * This function is executed before each test to ensure a consistent and isolated state.
     */
    function setUp() public {
        /// @dev It uses UnsafeUpgrades for measuring code coverage.
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, owner)
        );

        implementation = address(new ACL());
        UnsafeUpgrades.upgradeProxy(proxy, implementation, abi.encodeCall(acl.reinitialize, ()), owner);
        acl = ACL(proxy);
        fhevmExecutor = acl.getFHEVMExecutorAddress();
    }

    /**
     * @dev Tests that the post-upgrade check for the proxy contract works as expected.
     * It checks that the version is correct, the owner is set to the expected address, and the fhevmExecutor address is correct.
     */
    function test_PostProxyUpgradeCheck() public view {
        assertEq(acl.getVersion(), string(abi.encodePacked("ACL v0.1.0")));
        assertEq(acl.owner(), owner);
        assertEq(acl.getFHEVMExecutorAddress(), fhevmExecutorAdd);
    }

    /**
     * @dev Tests that the contract isAllowed returns false if the handle is not allowed for the account.
     */
    function test_IsAllowedReturnsFalseIfNotAllowed(bytes32 handle, address account) public view {
        assertFalse(acl.isAllowed(handle, account));
    }

    /**
     * @dev Tests that the contract isAllowedForDecryption returns false if the handle is not allowed for decryption.
     */
    function test_IsAllowedForDecryptionReturnsFalseIfNotAllowed(bytes32 handle) public view {
        assertFalse(acl.isAllowedForDecryption(handle));
    }

    /**
     * @dev Tests that the contract allowedTransient returns false if the handle is not allowed for the account.
     */
    function test_AllowedTransientReturnsFalseIfNotAllowed(bytes32 handle, address account) public view {
        assertFalse(acl.allowedTransient(handle, account));
    }

    /**
     * @dev Tests that the contract persistAllowed returns false if the handle is not allowed for the account.
     */
    function test_PersistAllowedReturnsFalseIfNotAllowed(bytes32 handle, address account) public view {
        assertFalse(acl.persistAllowed(handle, account));
    }

    /**
     * @dev Tests that the function allow reverts if the sender is not allowed to use the handle.
     */
    function test_CannotAllowIfNotAllowedToUseTheHandle(bytes32 handle, address account) public {
        vm.expectPartialRevert(ACL.SenderNotAllowed.selector);
        acl.allow(handle, account);
    }

    /**
     * @dev Tests that the function allowTransient reverts if the sender is not allowed to use the handle.
     */
    function test_CannotAllowTransientIfNotAllowedToUseTheHandle(bytes32 handle, address account) public {
        vm.expectPartialRevert(ACL.SenderNotAllowed.selector);
        acl.allowTransient(handle, account);
    }

    /**
     * @dev Tests that the function allow works if the sender address is the fhevmExecutor address.
     */
    function test_CanAllowTransientIfFhevmExecutor(bytes32 handle, address account) public {
        vm.prank(fhevmExecutor);
        acl.allowTransient(handle, account);
        assertTrue(acl.allowedTransient(handle, account));
        assertTrue(acl.isAllowed(handle, account));
    }

    /**
     * @dev Tests that the function allowTransient works if the sender address is the fhevmExecutor address until the transient storage gets cleaned.
     */
    function test_CanAllowTransientIfFhevmExecutorButOnlyUntilItGetsCleaned(bytes32 handle, address account) public {
        vm.prank(fhevmExecutor);
        acl.allowTransient(handle, account);
        acl.cleanTransientStorage();
        assertFalse(acl.allowedTransient(handle, account));
        assertFalse(acl.isAllowed(handle, account));
    }

    /**
     * @dev Tests that the function allow works if the sender address is allowed to use the handle.
     */
    function test_CanAllow(bytes32 handle, address account) public {
        assertFalse(acl.isAllowed(handle, account));
        _allowHandle(handle, account);
        assertTrue(acl.isAllowed(handle, account));
        assertTrue(acl.persistAllowed(handle, account));
    }

    /**
     * @dev Tests that the sender can delegate to another account only if both contract and sender addresses are allowed
     * to use the handle.
     */
    function test_CanDelegateAccountButItIsAllowedOnBehalfOnlyIfBothContractAndSenderAreAllowed(
        bytes32 handle,
        address sender,
        address delegatee,
        address delegateeContract
    ) public {
        vm.assume(sender != delegateeContract);

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;

        vm.prank(sender);
        vm.expectEmit(address(acl));
        emit ACL.NewDelegation(sender, delegatee, contractAddresses);
        acl.delegateAccount(delegatee, contractAddresses);
        vm.assertFalse(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));

        /// @dev The sender and the delegatee contract must be allowed to use the handle before it delegates.
        _allowHandle(handle, sender);
        vm.assertFalse(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));
        _allowHandle(handle, delegateeContract);
        vm.assertTrue(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));
    }

    /**
     * @dev Tests that the sender cannot delegate to the same account twice.
     */
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

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;

        vm.prank(sender);
        vm.expectPartialRevert(ACL.AlreadyDelegated.selector);
        acl.delegateAccount(delegatee, contractAddresses);
    }

    /**
     * @dev Tests that the sender cannot delegate to the account if contractAddresses are empty.
     */
    function test_CannotDelegateIfContractAddressesAreEmpty(address sender, address delegatee) public {
        vm.assume(sender != delegatee);
        address[] memory contractAddresses = new address[](0);

        vm.prank(sender);
        vm.expectRevert(ACL.ContractAddressesIsEmpty.selector);
        acl.delegateAccount(delegatee, contractAddresses);
    }

    function test_CannotDelegateIfContractAddressesAboveMaxNumberContractAddresses(
        address sender,
        address delegatee
    ) public {
        vm.assume(sender != delegatee);
        /// @dev The max number of contract addresses is hardcoded to 10 in the ACL contract.
        address[] memory contractAddresses = new address[](11);

        /// @dev Fill the array with 11 distinct addresses.
        for (uint256 i = 0; i < 11; i++) {
            contractAddresses[i] = address(uint160(i));
        }

        vm.prank(sender);
        vm.expectRevert(ACL.ContractAddressesMaxLengthExceeded.selector);
        acl.delegateAccount(delegatee, contractAddresses);
    }

    /**
     * @dev Tests that the sender cannot delegate to a contract address.
     */
    function test_CannotDelegateIfSenderIsDelegateeContract(address sender, address delegatee) public {
        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = sender;

        vm.prank(sender);
        vm.expectPartialRevert(ACL.SenderCannotBeContractAddress.selector);
        acl.delegateAccount(delegatee, contractAddresses);
    }

    /**
     * @dev Tests that the sender cannot delegate if account is not allowed to use the handle.
     */
    function test_CanDelegateAccountIfAccountNotAllowed(
        bytes32 handle,
        address sender,
        address delegatee,
        address delegateeContract
    ) public {
        vm.assume(sender != delegateeContract);
        /// @dev Only the delegatee contract must be allowed to use the handle before it delegates.
        _allowHandle(handle, delegateeContract);

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;

        vm.prank(sender);
        vm.expectEmit(address(acl));
        emit ACL.NewDelegation(sender, delegatee, contractAddresses);
        acl.delegateAccount(delegatee, contractAddresses);

        vm.assertFalse(acl.allowedOnBehalf(delegatee, handle, delegateeContract, sender));
    }

    /**
     * @dev Tests that the sender can revoke delegation if the sender has already delegated.
     */
    function test_CanRevokeDelegation(address sender, address delegatee, address delegateeContract) public {
        vm.assume(sender != delegateeContract);

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;

        /// @dev Delegate the account first.
        vm.prank(sender);
        acl.delegateAccount(delegatee, contractAddresses);

        vm.prank(sender);
        vm.expectEmit(address(acl));
        emit ACL.RevokedDelegation(sender, delegatee, contractAddresses);
        acl.revokeDelegation(delegatee, contractAddresses);
    }

    /**
     * @dev Tests that the sender cannot revoke delegation if the sender has not delegated yet.
     */
    function test_CannotRevokeDelegationIfNotDelegatedYet(
        address sender,
        address delegatee,
        address delegateeContract
    ) public {
        vm.assume(sender != delegateeContract);

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = delegateeContract;

        vm.prank(sender);
        vm.expectPartialRevert(ACL.NotDelegatedYet.selector);
        acl.revokeDelegation(delegatee, contractAddresses);
    }

    /**
     * @dev Tests that the sender cannot revoke delegation if the contract addresses are empty.
     */
    function test_CannotRevokeDelegationIfEmptyConctractAddresses(address sender, address delegatee) public {
        vm.assume(sender != delegatee);
        address[] memory contractAddresses = new address[](0);

        vm.prank(sender);
        vm.expectRevert(ACL.ContractAddressesIsEmpty.selector);
        acl.revokeDelegation(delegatee, contractAddresses);
    }

    /**
     * @dev Tests that the sender cannot delegate if the handle list is empty.
     */
    function test_NoOneCanAllowForDecryptionIfEmptyList(address sender) public {
        bytes32[] memory handlesList = new bytes32[](0);
        vm.prank(sender);
        vm.expectRevert(ACL.HandlesListIsEmpty.selector);
        acl.allowForDecryption(handlesList);
    }

    /**
     * @dev Tests that the sender can allow for decryption if the sender is approved to use the handle.
     */
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

    /**
     * @dev Tests that the sender cannot allow for decryption if the sender is not allowed to use the handle.
     */
    function test_CannotAllowForDecryptionIfSenderIsNotAllowedToUseTheHandle(bytes32 handle0, bytes32 handle1) public {
        bytes32[] memory handlesList = new bytes32[](2);
        handlesList[0] = handle0;
        handlesList[1] = handle1;

        vm.expectPartialRevert(ACL.SenderNotAllowed.selector);
        acl.allowForDecryption(handlesList);
    }

    /**
     * @dev Tests that the sender cannot allow for decryption if the sender is not allowed to use one of the handles.
     */
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

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        this.upgrade(randomAccount);
    }

    /**
     * @dev This function is used to test that only the owner can authorize an upgrade.
     *      It attempts to upgrade the proxy contract to a new implementation using a random account.
     *      The upgrade should fail if the random account is not the owner.
     */
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", owner);
    }
}

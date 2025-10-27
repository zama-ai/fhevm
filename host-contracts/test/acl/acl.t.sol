// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {StdStorage, stdStorage} from "forge-std/StdStorage.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {ACL} from "../../contracts/ACL.sol";
import {PauserSet} from "../../contracts/immutable/PauserSet.sol";
import {ACLEvents} from "../../contracts/ACLEvents.sol";
import {EmptyUUPSProxyACL} from "../../contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {fhevmExecutorAdd, pauserSetAdd, aclAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract ACLTest is HostContractsDeployerTestUtils {
    using stdStorage for StdStorage;
    ACL internal acl;
    PauserSet internal pauserSet;

    address internal constant owner = address(456);
    address internal constant pauser = address(789);

    address internal proxy;
    address internal fhevmExecutor;

    /**
     * @dev Grants permissions for a handle for an account for testing purposes.
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
     * @dev Reconstructs `UserDecryptionDelegation` directly from storage.
     *
     * `ACL.userDecryptionDelegations` stores three packed `uint64` fields in one slot. We point `stdstore`
     * at `getUserDecryptionDelegationExpirationDate`, which walks the same nested mapping and records the
     * slot that is read. With `enable_packed_slots()` the helper returns offsets for that packed word, so we
     * can `vm.load` the slot and shift/mask the raw value back into the struct fields—no mock accessor needed.
     */
    function _getUserDecryptionDelegation(
        address delegator,
        address delegate,
        address contractAddress
    ) internal returns (ACL.UserDecryptionDelegation memory userDecryptionDelegation) {
        uint256 slot = stdstore
            .target(address(acl))
            .sig("getUserDecryptionDelegationExpirationDate(address,address,address)")
            .with_key(delegator)
            .with_key(delegate)
            .with_key(contractAddress)
            .enable_packed_slots()
            .find();
        bytes32 data = vm.load(address(acl), bytes32(slot));
        uint256 raw = uint256(data);
        userDecryptionDelegation.expirationDate = uint64(raw);
        userDecryptionDelegation.lastBlockDelegateOrRevoke = uint64(raw >> 64);
        userDecryptionDelegation.delegationCounter = uint64(raw >> 128);
    }

    /**
     * @dev Sets up the testing environment by deploying a proxy contract and initializing signers.
     * This function is executed before each test to ensure a consistent and isolated state.
     */
    function setUp() public {
         _deployACL(owner);
        acl = ACL(aclAdd);
        proxy = aclAdd;
        _deployPauserSet();
        pauserSet = PauserSet(pauserSetAdd);
        vm.prank(owner);
        pauserSet.addPauser(pauser);
        fhevmExecutor = acl.getFHEVMExecutorAddress();
    }

    /**
     * @dev Tests that the post-upgrade check for the proxy contract works as expected.
     * It checks that the version is correct, the owner/pauser are set to the expected addresses, and the fhevmExecutor address is correct.
     */
    function test_PostProxyUpgradeCheck() public view {
        assertEq(acl.getVersion(), string(abi.encodePacked("ACL v0.2.0")));
        assertEq(acl.owner(), owner);
        assertEq(acl.isPauser(pauser), true);
        assertEq(acl.getFHEVMExecutorAddress(), fhevmExecutorAdd);
        assertEq(acl.getPauserSetAddress(), pauserSetAdd);
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
    function test_CannotAllowIfNotAllowedToUseTheHandle(address sender, bytes32 handle, address account) public {
        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderNotAllowed.selector, sender));
        acl.allow(handle, account);
    }

    /**
     * @dev Tests that the function allowTransient reverts if the sender is not allowed to use the handle.
     */
    function test_CannotAllowTransientIfNotAllowedToUseTheHandle(
        address sender,
        bytes32 handle,
        address account
    ) public {
        vm.assume(sender != fhevmExecutorAdd); // fhevmExecutor is privileged for transientAllow
        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderNotAllowed.selector, sender));
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
    function test_CanDelegateForUserDecryptionAndIsHandleDelegatedForUserDecryptionOnlyIfBothContractAndSenderAreAllowed(
        bytes32 handle,
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);
        vm.assume(sender != delegate);
        vm.assume(delegate != contractAddress);

        ACL.UserDecryptionDelegation memory userDecryptionDelegation;
        uint64 expirationDate = uint64(block.timestamp) + 7 hours;
        uint64 oldExpirationDate = userDecryptionDelegation.expirationDate;
        uint64 newExpirationDate = expirationDate;

        vm.prank(sender);
        vm.expectEmit(address(acl));
        emit ACLEvents.DelegatedForUserDecryption(
            sender,
            delegate,
            contractAddress,
            ++userDecryptionDelegation.delegationCounter,
            oldExpirationDate,
            newExpirationDate
        );
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);

        /// @dev Check that even that the delegation was made, neither the delegator nor the contract are allowed to use the handle.
        vm.assertFalse(acl.isHandleDelegatedForUserDecryption(sender, delegate, contractAddress, handle));

        /// @dev The delegator and the contract must be allowed to use the handle before it delegates.
        _allowHandle(handle, sender);
        vm.assertFalse(acl.isHandleDelegatedForUserDecryption(sender, delegate, contractAddress, handle));
        _allowHandle(handle, contractAddress);
        vm.assertTrue(acl.isHandleDelegatedForUserDecryption(sender, delegate, contractAddress, handle));

        /// @dev Check that the delegation is stored correctly.
        ACL.UserDecryptionDelegation memory storedUserDecryptionDelegation = _getUserDecryptionDelegation(
            sender,
            delegate,
            contractAddress
        );
        assertEq(storedUserDecryptionDelegation.expirationDate, expirationDate);
        assertEq(storedUserDecryptionDelegation.delegationCounter, userDecryptionDelegation.delegationCounter);
    }

    /**
     * @dev Tests that the sender cannot delegate in the same block twice.
     */
    function test_CannotDelegateForUserDecryptionInSameBlockTwice(
        bytes32 handle,
        address sender,
        address delegate,
        address contractAddress
    ) public {
        /// @dev We call the other test to avoid repeating the same code.
        test_CanDelegateForUserDecryptionAndIsHandleDelegatedForUserDecryptionOnlyIfBothContractAndSenderAreAllowed(
            handle,
            sender,
            delegate,
            contractAddress
        );

        uint64 expirationDate = uint64(block.timestamp) + 7 hours;

        vm.prank(sender);
        vm.expectRevert(
            abi.encodeWithSelector(
                ACL.AlreadyDelegatedOrRevokedInSameBlock.selector,
                sender,
                delegate,
                contractAddress,
                block.number
            )
        );
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);
    }

    /**
     * @dev Tests that the sender cannot delegate for user decryption if delegate and contract address are the same.
     */
    function test_CannotDelegateForUserDecryptionForSameDelegateAndContractAddress(
        address sender,
        address delegate
    ) public {
        vm.assume(sender != delegate);

        uint64 expirationDate = uint64(block.timestamp) + 7 hours;

        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(ACL.DelegateCannotBeContractAddress.selector, delegate));
        acl.delegateForUserDecryption(delegate, delegate, expirationDate);
    }

    /**
     * @dev Tests that the user decryption delegation cannot be created with the same expiration date.
     */
    function test_CannotDelegateUserDecryptionWithSameExpirationDate(
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);
        vm.assume(sender != delegate);
        vm.assume(delegate != contractAddress);

        /// @dev Delegate user decryption for the first time.
        uint64 expirationDate = uint64(block.timestamp) + 7 hours;
        vm.prank(sender);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);

        /// @dev Increase block number to avoid "AlreadyDelegatedOrRevokedInSameBlock" error.
        vm.roll(block.number + 1);

        /// @dev Delegate user decryption for the second time with the same expiration date.
        vm.prank(sender);
        vm.expectRevert(
            abi.encodeWithSelector(
                ACL.ExpirationDateAlreadySetToSameValue.selector,
                sender,
                delegate,
                contractAddress,
                expirationDate
            )
        );
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);
    }

    /**
     * @dev Tests that the sender cannot delegate for user decryption with expiration date before one hour.
     */
    function test_CannotDelegateForUserDecryptionWithExpirationDateBeforeOneHour(
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);
        vm.assume(sender != delegate);
        vm.assume(delegate != contractAddress);

        uint64 expirationDate = uint64(block.timestamp);

        vm.prank(sender);
        vm.expectRevert(ACL.ExpirationDateBeforeOneHour.selector);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);
    }

    /**
     * @dev Tests that the sender cannot delegate to itself as the contract address.
     */
    function test_CannotDelegateIfSenderIsContractAddress(address sender, address delegate) public {
        uint64 expirationDate = uint64(block.timestamp) + 7 hours;

        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderCannotBeContractAddress.selector, sender));
        acl.delegateForUserDecryption(delegate, sender, expirationDate);
    }

    /**
     * @dev Tests that the sender cannot delegate to itself as delegate.
     */
    function test_CannotDelegateIfSenderIsDelegate(address sender, address contractAddress) public {
        vm.assume(sender != contractAddress);
        uint64 expirationDate = uint64(block.timestamp) + 7 hours;

        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderCannotBeDelegate.selector, sender));
        acl.delegateForUserDecryption(sender, contractAddress, expirationDate);
    }

    /**
     * @dev Tests that the sender can revoke delegation for user decryption if the sender has already delegated.
     */
    function test_CanRevokeDelegationForUserDecryption(
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);
        vm.assume(sender != delegate);
        vm.assume(delegate != contractAddress);

        uint64 expirationDate = uint64(block.timestamp) + 7 hours;
        uint64 oldExpirationDate = expirationDate;

        /// @dev Delegate the account first.
        vm.prank(sender);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);

        /// @dev After delegation above, the counter should be 2.
        uint64 revokeDelegationCounter = 2;

        /// @dev Increase block number to avoid "AlreadyDelegatedOrRevokedInSameBlock" error.
        vm.roll(block.number + 1);

        vm.prank(sender);
        vm.expectEmit(address(acl));
        emit ACLEvents.RevokedDelegationForUserDecryption(
            sender,
            delegate,
            contractAddress,
            revokeDelegationCounter,
            oldExpirationDate
        );
        acl.revokeDelegationForUserDecryption(delegate, contractAddress);

        /// @dev Check that the delegation is stored correctly after revocation.
        ACL.UserDecryptionDelegation memory storedUserDecryptionDelegation = _getUserDecryptionDelegation(
            sender,
            delegate,
            contractAddress
        );
        assertEq(storedUserDecryptionDelegation.expirationDate, 0);
        assertEq(storedUserDecryptionDelegation.delegationCounter, revokeDelegationCounter);
    }

    /**
     * @dev Tests that the delegation and revocation counter is stored in a sequential order.
     */
    function test_UserDecryptionDelegationAndRevocationCounterIsSequential(
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);
        vm.assume(sender != delegate);
        vm.assume(delegate != contractAddress);
        uint64 userDecryptionDelegationCounter = 0;

        /// @dev Delegate user decryption for the first time.
        uint64 expirationDate = uint64(block.timestamp) + 3 hours;
        vm.prank(sender);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);
        ACL.UserDecryptionDelegation memory userDecryptionDelegation = _getUserDecryptionDelegation(
            sender,
            delegate,
            contractAddress
        );
        assertEq(userDecryptionDelegation.delegationCounter, ++userDecryptionDelegationCounter);

        /// @dev Increase block number to avoid "AlreadyDelegatedOrRevokedInSameBlock" error.
        vm.roll(block.number + 1);

        /// @dev Delegate user decryption for the second time.
        expirationDate = uint64(block.timestamp) + 5 hours;
        vm.prank(sender);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);
        userDecryptionDelegation = _getUserDecryptionDelegation(sender, delegate, contractAddress);
        assertEq(userDecryptionDelegation.delegationCounter, ++userDecryptionDelegationCounter);

        /// @dev Increase block number to avoid "AlreadyDelegatedOrRevokedInSameBlock" error.
        vm.roll(block.number + 1);

        /// @dev Revoke user decryption delegation for the first time.
        vm.prank(sender);
        acl.revokeDelegationForUserDecryption(delegate, contractAddress);
        userDecryptionDelegation = _getUserDecryptionDelegation(sender, delegate, contractAddress);
        assertEq(userDecryptionDelegation.delegationCounter, ++userDecryptionDelegationCounter);

        /// @dev Increase block number to avoid "AlreadyDelegatedOrRevokedInSameBlock" error.
        vm.roll(block.number + 1);

        /// @dev Delegate user decryption for the second time.
        expirationDate = uint64(block.timestamp) + 7 hours;
        vm.prank(sender);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);
        userDecryptionDelegation = _getUserDecryptionDelegation(sender, delegate, contractAddress);
        assertEq(userDecryptionDelegation.delegationCounter, ++userDecryptionDelegationCounter);

        /// @dev Increase block number to avoid "AlreadyDelegatedOrRevokedInSameBlock" error.
        vm.roll(block.number + 1);

        /// @dev Revoke user decryption delegation for the second time.
        vm.prank(sender);
        acl.revokeDelegationForUserDecryption(delegate, contractAddress);
        userDecryptionDelegation = _getUserDecryptionDelegation(sender, delegate, contractAddress);
        assertEq(userDecryptionDelegation.delegationCounter, ++userDecryptionDelegationCounter);
    }

    /**
     * @dev Tests that the sender cannot revoke delegation if the sender has not delegated yet.
     */
    function test_CannotRevokeDelegationIfNotDelegatedYet(
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);

        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(ACL.NotDelegatedYet.selector, sender, delegate, contractAddress));
        acl.revokeDelegationForUserDecryption(delegate, contractAddress);
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
        emit ACLEvents.AllowedForDecryption(address(sender), handlesList);
        acl.allowForDecryption(handlesList);

        assertTrue(acl.isAllowedForDecryption(handle0));
        assertTrue(acl.isAllowedForDecryption(handle1));
    }

    /**
     * @dev Tests that the sender cannot allow for decryption if the sender is not allowed to use the handle.
     */
    function test_CannotAllowForDecryptionIfSenderIsNotAllowedToUseTheHandle(
        address sender,
        bytes32 handle0,
        bytes32 handle1
    ) public {
        bytes32[] memory handlesList = new bytes32[](2);
        handlesList[0] = handle0;
        handlesList[1] = handle1;

        vm.prank(sender);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderNotAllowed.selector, sender));
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
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderNotAllowed.selector, sender));
        acl.allowForDecryption(handlesList);
    }

    /**
     * @dev Tests that only the pauser can pause the contract.
     */
    function test_OnlyPauserCanPause(address randomAccount) public {
        vm.assume(randomAccount != pauser);
        vm.expectRevert(abi.encodeWithSelector(ACL.NotPauser.selector, randomAccount));
        vm.prank(randomAccount);
        acl.pause();
    }

    /**
     * @dev Tests that only the owner can unpause the contract.
     */
    function test_OnlyOwnerCanUnpause(address randomAccount) public {
        vm.assume(randomAccount != owner);
        vm.prank(pauser);
        acl.pause();
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, randomAccount));
        vm.prank(randomAccount);
        acl.unpause();
    }

    /**
     * @dev Tests that only the pauser cannot unpause the contract.
     */
    function test_PauserCannotUnpause() public {
        vm.prank(pauser);
        acl.pause();
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, pauser));
        vm.prank(pauser);
        acl.unpause();
    }

    /**
     * @dev Tests that allow() cannot be called if the contract is paused.
     */
    function test_CannotCallAllowIfPaused() public {
        bytes32 mockHandle = keccak256(abi.encodePacked("handle"));
        vm.prank(pauser);
        acl.pause();

        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        vm.prank(fhevmExecutor);
        acl.allow(mockHandle, address(123));
    }

    /**
     * @dev Tests that allowTransient() cannot be called if the contract is paused.
     */
    function test_CannotCallAllowTransientIfPaused() public {
        bytes32 mockHandle = keccak256(abi.encodePacked("handle"));

        vm.prank(pauser);
        acl.pause();

        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        vm.prank(fhevmExecutor);
        acl.allowTransient(mockHandle, address(123));
    }

    /**
     * @dev Tests that allowForDecryption() cannot be called if the contract is paused.
     */
    function test_CannotCallAllowForDecryptionIfPaused() public {
        vm.prank(pauser);
        acl.pause();

        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        vm.prank(fhevmExecutor);
        acl.allowForDecryption(new bytes32[](1));
    }

    /**
     * @dev Tests that user decryption delegation cannot be called if the contract is paused.
     */
    function test_CannotDelegateForUserDecryptionIfPaused(
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);
        vm.assume(sender != delegate);
        vm.assume(delegate != contractAddress);

        uint64 expirationDate = uint64(block.timestamp) + 7 hours;

        vm.prank(sender);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);

        vm.prank(pauser);
        acl.pause();

        vm.prank(sender);
        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);
    }

    /**
     * @dev Tests that revoke delegation for user decryption cannot be called if the contract is paused.
     */
    function test_CannotRevokeDelegationForUserDecryptionIfPaused(
        address sender,
        address delegate,
        address contractAddress
    ) public {
        vm.assume(sender != contractAddress);
        vm.assume(sender != delegate);
        vm.assume(delegate != contractAddress);

        uint64 expirationDate = uint64(block.timestamp) + 7 hours;

        vm.prank(sender);
        acl.delegateForUserDecryption(delegate, contractAddress, expirationDate);

        vm.prank(pauser);
        acl.pause();

        vm.prank(sender);
        vm.expectRevert(PausableUpgradeable.EnforcedPause.selector);
        acl.revokeDelegationForUserDecryption(delegate, contractAddress);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, randomAccount));
        this.upgrade(randomAccount);
    }

    /**
     * @dev This function is used to test that only the owner can authorize an upgrade.
     *      It attempts to upgrade the proxy contract to a new implementation using a random account.
     *      The upgrade should fail if the random account is not the owner.
     */
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxyACL()), "", randomAccount);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxyACL()), "", owner);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Deny List
    ////////////////////////////////////////////////////////////////////////////

    function _oneRandomAddress() internal view returns (address randomAddress) {
        randomAddress = vm.randomAddress();
        vm.assume(randomAddress != owner);
        vm.assume(randomAddress != pauser);
    }

    function _twoRandomAddresses() internal view returns (address randomAddress1, address randomAddress2) {
        randomAddress1 = vm.randomAddress();
        randomAddress2 = vm.randomAddress();

        vm.assume(randomAddress1 != owner);
        vm.assume(randomAddress2 != owner);
        vm.assume(randomAddress1 != pauser);
        vm.assume(randomAddress2 != pauser);
    }

    function _cheatAllowTransient(bytes32 handle, address account) internal {
        address fhevmExecutorAddress = acl.getFHEVMExecutorAddress();
        vm.prank(fhevmExecutorAddress);
        acl.allowTransient(handle, account);
        assertEq(acl.allowedTransient(handle, account), true);
    }

    /**
     * @dev Tests that a non-owner cannot block an account
     */
    function test_NonOwnerCannotBlockAccount() public {
        (address randomCaller, address randomAccount) = _twoRandomAddresses();

        assertEq(acl.isAccountDenied(randomAccount), false);

        vm.prank(randomCaller);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, randomCaller));
        acl.blockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), false);
    }

    /**
     * @dev Tests that a non-owner cannot unblock an account
     */
    function test_NonOwnerCannotUnblockAccount() public {
        (address randomCaller, address randomAccount) = _twoRandomAddresses();

        assertEq(acl.isAccountDenied(randomAccount), false);

        vm.prank(randomCaller);
        vm.expectRevert(abi.encodeWithSelector(OwnableUpgradeable.OwnableUnauthorizedAccount.selector, randomCaller));
        acl.unblockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), false);
    }

    /**
     * @dev Tests that the owner can block an account
     */
    function test_OwnerCanBlockAccount() public {
        address randomAccount = _oneRandomAddress();
        address ownerAddress = acl.owner();

        assertEq(acl.isAccountDenied(randomAccount), false);

        vm.prank(ownerAddress);
        vm.expectEmit(true, true, true, true, address(acl));
        emit ACLEvents.BlockedAccount(randomAccount);
        acl.blockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), true);
    }

    /**
     * @dev Tests that the owner can unblock an account
     */
    function test_OwnerCanUnblockAccount() public {
        address randomAccount = _oneRandomAddress();
        address ownerAddress = acl.owner();

        assertEq(acl.isAccountDenied(randomAccount), false);

        vm.prank(ownerAddress);
        acl.blockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), true);

        vm.prank(ownerAddress);
        vm.expectEmit(true, true, true, true, address(acl));
        emit ACLEvents.UnblockedAccount(randomAccount);
        acl.unblockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), false);
    }

    /**
     * @dev Tests that the owner cannot block an already blocked account
     */
    function test_OwnerCannotBlockAccountTwice() public {
        address randomAccount = _oneRandomAddress();
        address ownerAddress = acl.owner();

        vm.prank(ownerAddress);
        acl.blockAccount(randomAccount);

        vm.prank(ownerAddress);
        vm.expectRevert(abi.encodeWithSelector(ACL.AccountAlreadyBlocked.selector, randomAccount));
        acl.blockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), true);
    }

    /**
     * @dev Tests that the owner cannot unblock an account that is not blocked
     */
    function test_OwnerCannotUnblockAccountIfNotBlocked() public {
        address randomAccount = _oneRandomAddress();
        address ownerAddress = acl.owner();

        vm.prank(ownerAddress);
        vm.expectRevert(abi.encodeWithSelector(ACL.AccountNotBlocked.selector, randomAccount));
        acl.unblockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), false);
    }

    /**
     * @dev Tests that the owner cannot unblock an account twice
     */
    function test_OwnerCannotUnblockAccountTwice() public {
        address randomAccount = _oneRandomAddress();
        address ownerAddress = acl.owner();

        vm.prank(ownerAddress);
        acl.blockAccount(randomAccount);

        vm.prank(ownerAddress);
        acl.unblockAccount(randomAccount);

        vm.prank(ownerAddress);
        vm.expectRevert(abi.encodeWithSelector(ACL.AccountNotBlocked.selector, randomAccount));
        acl.unblockAccount(randomAccount);

        assertEq(acl.isAccountDenied(randomAccount), false);
    }

    /**
     * @dev Tests that a denied account cannot allow
     */
    function test_DeniedAccountCannotAllow() public {
        (address randomAccount, address randomUser) = _twoRandomAddresses();

        vm.prank(acl.owner());
        acl.blockAccount(randomAccount);

        bytes32 handle = bytes32(vm.randomUint());

        _cheatAllowTransient(handle, randomAccount);

        vm.prank(randomAccount);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderDenied.selector, randomAccount));
        acl.allow(handle, randomUser);
    }

    /**
     * @dev Tests that a denied account cannot allowTransient
     */
    function test_DeniedAccountCannotAllowTransient() public {
        (address randomAccount, address randomUser) = _twoRandomAddresses();

        vm.prank(acl.owner());
        acl.blockAccount(randomAccount);

        bytes32 handle = bytes32(vm.randomUint());

        _cheatAllowTransient(handle, randomAccount);

        vm.prank(randomAccount);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderDenied.selector, randomAccount));
        acl.allowTransient(handle, randomUser);
    }

    /**
     * @dev Tests that a denied account cannot allowForDecryption
     */
    function test_DeniedAccountCannotAllowForDecryption() public {
        address randomAccount = _oneRandomAddress();

        vm.prank(acl.owner());
        acl.blockAccount(randomAccount);

        bytes32 handle = bytes32(vm.randomUint());
        bytes32[] memory handlesList = new bytes32[](1);
        handlesList[0] = handle;

        _cheatAllowTransient(handle, randomAccount);

        vm.prank(randomAccount);
        vm.expectRevert(abi.encodeWithSelector(ACL.SenderDenied.selector, randomAccount));
        acl.allowForDecryption(handlesList);
    }
}

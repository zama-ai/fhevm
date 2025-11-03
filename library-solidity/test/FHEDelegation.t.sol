// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "encrypted-types/EncryptedTypes.sol";
import {FHE} from "../lib/FHE.sol";
import {CoprocessorConfig} from "../lib/Impl.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract DelegationLibraryAdapter {
    function setCoprocessorConfig(CoprocessorConfig memory config) external {
        FHE.setCoprocessor(config);
    }

    function delegateUserDecryption(address delegate, address contractAddress, uint64 expirationDate) external {
        FHE.delegateUserDecryption(delegate, contractAddress, expirationDate);
    }

    function delegateUserDecryptionWithoutExpiration(address delegate, address contractAddress) external {
        FHE.delegateUserDecryptionWithoutExpiration(delegate, contractAddress);
    }

    function delegateUserDecryptions(
        address delegate,
        address[] memory contractAddresses,
        uint64 expirationDate
    ) external {
        FHE.delegateUserDecryptions(delegate, contractAddresses, expirationDate);
    }

    function delegateUserDecryptionsWithoutExpiration(address delegate, address[] memory contractAddresses) external {
        FHE.delegateUserDecryptionsWithoutExpiration(delegate, contractAddresses);
    }

    function revokeUserDecryptionDelegation(address delegate, address contractAddress) external {
        FHE.revokeUserDecryptionDelegation(delegate, contractAddress);
    }

    function revokeUserDecryptionDelegations(address delegate, address[] memory contractAddresses) external {
        FHE.revokeUserDecryptionDelegations(delegate, contractAddresses);
    }

    function allowHandle(bytes32 handle, address account) external {
        FHE.allow(euint256.wrap(handle), account);
    }

    function mintAndPersistHandle(uint256 plaintext) external returns (bytes32) {
        euint256 ciphertext = FHE.asEuint256(plaintext);
        FHE.allowThis(ciphertext);
        return euint256.unwrap(ciphertext);
    }

    function isUserDecryptable(bytes32 handle, address user, address contractAddress) external view returns (bool) {
        return FHE.isUserDecryptable(handle, user, contractAddress);
    }

    function isDelegatedForUserDecryption(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle
    ) external view returns (bool) {
        return FHE.isDelegatedForUserDecryption(delegator, delegate, contractAddress, handle);
    }

    function getDelegatedUserDecryptionExpirationDate(
        address delegator,
        address delegate,
        address contractAddress
    ) external view returns (uint64) {
        return FHE.getDelegatedUserDecryptionExpirationDate(delegator, delegate, contractAddress);
    }
}

contract FHEDelegationTest is HostContractsDeployerTestUtils {
    DelegationLibraryAdapter internal adapter;
    ACL internal acl;

    address internal constant OWNER = address(0xAA11);
    address internal constant PAUSER = address(0xBB22);
    address internal constant GATEWAY_SOURCE = address(0xCC33);
    uint64 internal constant GATEWAY_CHAIN_ID = 31337;
    function setUp() public {
        vm.warp(1_000_000);

        adapter = new DelegationLibraryAdapter();

        address[] memory kmsSigners = new address[](1);
        kmsSigners[0] = address(0x1111);
        address[] memory inputSigners = new address[](1);
        inputSigners[0] = address(0x2222);

        _deployFullHostStack(
            OWNER,
            PAUSER,
            GATEWAY_SOURCE,
            GATEWAY_SOURCE,
            GATEWAY_CHAIN_ID,
            kmsSigners,
            1,
            inputSigners,
            1
        );

        acl = ACL(aclAdd);

        CoprocessorConfig memory config = CoprocessorConfig({
            ACLAddress: aclAdd,
            CoprocessorAddress: fhevmExecutorAdd,
            KMSVerifierAddress: kmsVerifierAdd
        });

        adapter.setCoprocessorConfig(config);
    }

    function _expectActiveDelegation(address delegate, address contractContext, uint64 expectedExpiry) internal view {
        uint64 stored = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contractContext);
        assertEq(stored, expectedExpiry, "delegation expiry mismatch");
    }

    function _boundValidFutureExpiry(uint256 expirationDate) internal view returns (uint64) {
        uint256 minExpiry = block.timestamp + 1 hours;
        uint256 maxExpiry = type(uint64).max;
        return uint64(bound(expirationDate, minExpiry, maxExpiry));
    }

    function _assumeDelegateAndContext(address delegate, address contractContext) internal view {
        vm.assume(delegate != address(adapter));
        vm.assume(contractContext != address(adapter));
        vm.assume(delegate != contractContext);
    }

    function testFuzz_IsUserDecryptable_ReturnsFalseWhenUserEqualsContract(
        uint256 plaintext,
        address contractContext
    ) public {
        bytes32 handle = adapter.mintAndPersistHandle(plaintext);
        adapter.allowHandle(handle, contractContext);

        bool allowed = adapter.isUserDecryptable(handle, contractContext, contractContext);
        assertFalse(allowed, "user == contract should not be decryptable");
    }

    function testFuzz_IsUserDecryptable_ReturnsFalseWhenUserNotPersistAllowed(
        uint256 plaintext,
        address contractContext,
        address unauthorizedUser
    ) public {
        _assumeDelegateAndContext(unauthorizedUser, contractContext);
        bytes32 handle = adapter.mintAndPersistHandle(plaintext);
        adapter.allowHandle(handle, contractContext);

        bool allowed = adapter.isUserDecryptable(handle, unauthorizedUser, contractContext);
        assertFalse(allowed, "missing user allowance should return false");
    }

    function testFuzz_IsUserDecryptable_ReturnsFalseWhenContractNotPersistAllowed(
        uint256 plaintext,
        address user,
        address contractContext
    ) public {
        vm.assume(contractContext != address(adapter));
        bytes32 handle = adapter.mintAndPersistHandle(plaintext);
        adapter.allowHandle(handle, user);

        bool allowed = adapter.isUserDecryptable(handle, user, contractContext);
        assertFalse(allowed, "missing contract allowance should return false");
    }

    function testFuzz_IsUserDecryptable_ReturnsTrueWhenBothPersistAllowed(
        uint256 plaintext,
        address user,
        address contractContext
    ) public {
        bytes32 handle = adapter.mintAndPersistHandle(plaintext);
        vm.assume(user != contractContext);
        adapter.allowHandle(handle, user);
        adapter.allowHandle(handle, contractContext);

        bool allowed = adapter.isUserDecryptable(handle, user, contractContext);
        assertTrue(allowed, "expected user decryptable context");
    }

    function testFuzz_IsDelegatedForUserDecryption_ReturnsTrueWhenActive(
        uint256 plaintext,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        bytes32 handle = adapter.mintAndPersistHandle(plaintext);
        adapter.allowHandle(handle, contractContext);

        uint64 expirationDate = uint64(block.timestamp + 2 hours);
        adapter.delegateUserDecryption(delegate, contractContext, expirationDate);

        bool delegated = adapter.isDelegatedForUserDecryption(address(adapter), delegate, contractContext, handle);
        assertTrue(delegated, "delegated handle should be active");
    }

    function testFuzz_IsDelegatedForUserDecryption_ReturnsFalseWhenExpired(
        uint256 plaintext,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        bytes32 handle = adapter.mintAndPersistHandle(plaintext);
        adapter.allowHandle(handle, contractContext);

        uint64 expirationDate = uint64(block.timestamp + 2 hours);
        adapter.delegateUserDecryption(delegate, contractContext, expirationDate);

        vm.warp(uint256(expirationDate) + 1);

        bool delegated = adapter.isDelegatedForUserDecryption(address(adapter), delegate, contractContext, handle);
        assertFalse(delegated, "delegation past expiry should be inactive");
    }

    function testFuzz_DelegateUserDecryption_PersistsExpiryInACL(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);

        _expectActiveDelegation(delegate, contractContext, boundedExpiry);
    }

    function testFuzz_DelegateUserDecryption_RevertsWhenExpiryTooSoon(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        uint256 maxExpiry = block.timestamp + 1 hours - 1;
        uint64 boundedExpiry = uint64(bound(expirationDate, block.timestamp, maxExpiry));

        vm.expectRevert(ACL.ExpirationDateBeforeOneHour.selector);
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);
    }

    function test_DelegateUserDecryption_RevertsWhenSenderIsContractAddress(address delegate) public {
        vm.assume(delegate != address(adapter));
        uint64 expirationDate = uint64(block.timestamp + 2 hours);

        vm.expectRevert(abi.encodeWithSelector(ACL.SenderCannotBeContractAddress.selector, address(adapter)));
        adapter.delegateUserDecryption(delegate, address(adapter), expirationDate);
    }

    function test_DelegateUserDecryption_RevertsWhenDelegateEqualsSender(address contractContext) public {
        vm.assume(contractContext != address(adapter));
        uint64 expirationDate = uint64(block.timestamp + 2 hours);

        vm.expectRevert(abi.encodeWithSelector(ACL.SenderCannotBeDelegate.selector, address(adapter)));
        adapter.delegateUserDecryption(address(adapter), contractContext, expirationDate);
    }

    function test_DelegateUserDecryption_RevertsWhenDelegateEqualsContract(address contractContext) public {
        vm.assume(contractContext != address(adapter));
        uint64 expirationDate = uint64(block.timestamp + 2 hours);

        vm.expectRevert(abi.encodeWithSelector(ACL.DelegateCannotBeContractAddress.selector, contractContext));
        adapter.delegateUserDecryption(contractContext, contractContext, expirationDate);
    }

    function testFuzz_DelegateUserDecryption_RevertsOnSameBlockReplay(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        uint256 blockNumber = block.number;
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);

        vm.expectRevert(
            abi.encodeWithSelector(
                ACL.AlreadyDelegatedOrRevokedInSameBlock.selector,
                address(adapter),
                delegate,
                contractContext,
                blockNumber
            )
        );
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);
    }

    function testFuzz_DelegateUserDecryption_RevertsWhenSettingSameExpiry(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);

        vm.roll(block.number + 1);

        vm.expectRevert(
            abi.encodeWithSelector(
                ACL.ExpirationDateAlreadySetToSameValue.selector,
                address(adapter),
                delegate,
                contractContext,
                boundedExpiry
            )
        );
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);
    }

    function testFuzz_DelegateUserDecryptionWithoutExpiration_SetsMaxExpiry(
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        adapter.delegateUserDecryptionWithoutExpiration(delegate, contractContext);

        _expectActiveDelegation(delegate, contractContext, type(uint64).max);
    }

    function testFuzz_DelegateUserDecryptions_BatchAssignsEachContext(
        uint256 expirationDate,
        address delegate,
        address contractA,
        address contractB
    ) public {
        _assumeDelegateAndContext(delegate, contractA);
        _assumeDelegateAndContext(delegate, contractB);
        vm.assume(contractA != contractB);
        address[] memory contracts = new address[](2);
        contracts[0] = contractA;
        contracts[1] = contractB;
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);

        adapter.delegateUserDecryptions(delegate, contracts, boundedExpiry);

        uint64 first = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contracts[0]);
        uint64 second = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contracts[1]);
        assertEq(first, boundedExpiry, "first contract expiry mismatch");
        assertEq(second, boundedExpiry, "second contract expiry mismatch");
    }

    function test_DelegateUserDecryptions_ReturnsEarlyWhenNoContracts(
        address delegate,
        address contractContext
    ) public {
        address[] memory contracts = new address[](0);

        adapter.delegateUserDecryptions(delegate, contracts, uint64(block.timestamp + 2 hours));

        uint64 stored = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contractContext);
        assertEq(stored, 0, "no delegation should be recorded");
    }

    function testFuzz_DelegateUserDecryptions_SingleEntrySetsExpiry(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        address[] memory contracts = new address[](1);
        contracts[0] = contractContext;
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);

        adapter.delegateUserDecryptions(delegate, contracts, boundedExpiry);

        _expectActiveDelegation(delegate, contractContext, boundedExpiry);
    }

    function testFuzz_DelegateUserDecryptionsWithoutExpiration_BatchUsesMaxExpiry(
        address delegate,
        address contractA,
        address contractB
    ) public {
        _assumeDelegateAndContext(delegate, contractA);
        _assumeDelegateAndContext(delegate, contractB);
        vm.assume(contractA != contractB);
        address[] memory contracts = new address[](2);
        contracts[0] = contractA;
        contracts[1] = contractB;

        adapter.delegateUserDecryptionsWithoutExpiration(delegate, contracts);

        uint64 first = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contracts[0]);
        uint64 second = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contracts[1]);
        assertEq(first, type(uint64).max, "first contract max expiry mismatch");
        assertEq(second, type(uint64).max, "second contract max expiry mismatch");
    }

    function test_DelegateUserDecryptionsWithoutExpiration_ReturnsEarlyWhenNoContracts(
        address delegate,
        address contractContext
    ) public {
        address[] memory contracts = new address[](0);

        adapter.delegateUserDecryptionsWithoutExpiration(delegate, contracts);

        uint64 stored = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contractContext);
        assertEq(stored, 0, "no delegation should be recorded");
    }

    function testFuzz_DelegateUserDecryptionsWithoutExpiration_SingleEntrySetsMaxExpiry(
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        address[] memory contracts = new address[](1);
        contracts[0] = contractContext;

        adapter.delegateUserDecryptionsWithoutExpiration(delegate, contracts);

        _expectActiveDelegation(delegate, contractContext, type(uint64).max);
    }

    function testFuzz_RevokeUserDecryptionDelegation_ResetsExpiry(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);

        vm.roll(block.number + 1);

        adapter.revokeUserDecryptionDelegation(delegate, contractContext);

        uint64 stored = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contractContext);
        assertEq(stored, 0, "revocation should clear expiry");
    }

    function testFuzz_RevokeUserDecryptionDelegations_BatchClearsEach(
        uint256 expirationDate,
        address delegate,
        address contractA,
        address contractB
    ) public {
        _assumeDelegateAndContext(delegate, contractA);
        _assumeDelegateAndContext(delegate, contractB);
        vm.assume(contractA != contractB);
        address[] memory contracts = new address[](2);
        contracts[0] = contractA;
        contracts[1] = contractB;

        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        adapter.delegateUserDecryptions(delegate, contracts, boundedExpiry);

        vm.roll(block.number + 1);

        adapter.revokeUserDecryptionDelegations(delegate, contracts);

        uint64 first = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contracts[0]);
        uint64 second = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contracts[1]);
        assertEq(first, 0, "first contract should be cleared");
        assertEq(second, 0, "second contract should be cleared");
    }

    function test_RevokeUserDecryptionDelegation_RevertsWhenNotDelegated(
        address delegate,
        address contractContext
    ) public {
        vm.expectRevert(
            abi.encodeWithSelector(ACL.NotDelegatedYet.selector, address(adapter), delegate, contractContext)
        );
        adapter.revokeUserDecryptionDelegation(delegate, contractContext);
    }

    function testFuzz_RevokeUserDecryptionDelegation_RevertsOnSameBlockReplay(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);
        uint256 blockNumber = block.number;

        vm.expectRevert(
            abi.encodeWithSelector(
                ACL.AlreadyDelegatedOrRevokedInSameBlock.selector,
                address(adapter),
                delegate,
                contractContext,
                blockNumber
            )
        );
        adapter.revokeUserDecryptionDelegation(delegate, contractContext);
    }

    function test_RevokeUserDecryptionDelegations_ReturnsEarlyWhenNoContracts(
        address delegate,
        address contractContext
    ) public {
        address[] memory contracts = new address[](0);
        adapter.revokeUserDecryptionDelegations(delegate, contracts);

        uint64 stored = acl.getUserDecryptionDelegationExpirationDate(address(adapter), delegate, contractContext);
        assertEq(stored, 0, "no revocation should be recorded");
    }

    function testFuzz_RevokeUserDecryptionDelegations_SingleEntryClearsExpiry(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        address[] memory contracts = new address[](1);
        contracts[0] = contractContext;

        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        adapter.delegateUserDecryptions(delegate, contracts, boundedExpiry);

        vm.roll(block.number + 1);

        adapter.revokeUserDecryptionDelegations(delegate, contracts);

        _expectActiveDelegation(delegate, contractContext, 0);
    }

    function testFuzz_GetDelegatedUserDecryptionExpirationDate_ReturnsStoredValue(
        uint256 expirationDate,
        address delegate,
        address contractContext
    ) public {
        _assumeDelegateAndContext(delegate, contractContext);
        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);
        adapter.delegateUserDecryption(delegate, contractContext, boundedExpiry);

        uint64 fetched = adapter.getDelegatedUserDecryptionExpirationDate(address(adapter), delegate, contractContext);
        assertEq(fetched, boundedExpiry, "library expiry getter mismatch");
    }

    function testFuzz_AclGetDelegatedUserDecryptionExpirationDate_ReturnsStoredValue(
        uint256 expirationDate,
        address contractContext
    ) public {
        vm.assume(contractContext != address(adapter));

        address userA = vm.randomAddress();
        address userB = vm.randomAddress();

        uint64 boundedExpiry = _boundValidFutureExpiry(expirationDate);

        vm.prank(userA);
        acl.delegateForUserDecryption(userB, contractContext, boundedExpiry);

        uint64 fetched = adapter.getDelegatedUserDecryptionExpirationDate(userA, userB, contractContext);
        assertEq(fetched, boundedExpiry, "library expiry getter mismatch");
    }
}

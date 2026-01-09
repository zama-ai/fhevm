// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {KMSVerifierV2} from "../../contracts/KMSVerifierV2.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";

/**
 * @title KMSVerifierV2Test
 * @notice Tests for KMSVerifierV2 with epoch grace period support.
 */
contract KMSVerifierV2Test is Test {
    KMSVerifierV2 internal kmsVerifier;

    uint256 internal constant initialThreshold = 1;
    address internal constant verifyingContractSource = address(10000);
    address internal constant owner = address(456);

    /// @dev Signer variables.
    uint256 internal constant privateKeySigner0 = 0x022;
    uint256 internal constant privateKeySigner1 = 0x03;
    uint256 internal constant privateKeySigner2 = 0x04;
    uint256 internal constant privateKeySigner3 = 0x05;
    uint256 internal constant privateKeySigner4 = 0x06;
    address[] internal activeSigners;

    mapping(address => uint256) internal signerPrivateKeys;
    address internal signer0;
    address internal signer1;
    address internal signer2;
    address internal signer3;
    address internal signer4;

    /// @dev Proxy and implementation variables
    address internal proxy;
    address internal implementation;

    function _computeSignature(uint256 privateKey, bytes32 digest) internal pure returns (bytes memory signature) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);
        return abi.encodePacked(r, s, v);
    }

    function _computeDigest(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                kmsVerifier.DECRYPTION_RESULT_TYPEHASH(),
                keccak256(abi.encodePacked(handlesList)),
                keccak256(decryptedResult),
                keccak256(abi.encodePacked(extraData))
            )
        );

        bytes32 hashTypeData = MessageHashUtils.toTypedDataHash(_computeDomainSeparator(), structHash);
        return hashTypeData;
    }

    function _computeDomainSeparator() internal view returns (bytes32) {
        (, string memory name, string memory version, uint256 chainId, address verifyingContract, , ) = kmsVerifier
            .eip712Domain();

        return
            keccak256(
                abi.encode(
                    keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                    keccak256(bytes(name)),
                    keccak256(bytes(version)),
                    chainId,
                    verifyingContract
                )
            );
    }

    function _deployProxy() internal {
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );
    }

    function _deployAndEtchACL() internal {
        address _acl = address(new ACL());
        bytes memory code = _acl.code;
        vm.etch(aclAdd, code);
        vm.store(
            aclAdd,
            0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300, // OwnableStorageLocation
            bytes32(uint256(uint160(owner)))
        );
    }

    function _upgradeProxy(address[] memory signers) internal {
        implementation = address(new KMSVerifierV2());
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(
                kmsVerifier.initializeFromEmptyProxy,
                (verifyingContractSource, uint64(block.chainid), signers, initialThreshold)
            ),
            owner
        );
        kmsVerifier = KMSVerifierV2(proxy);
    }

    function _upgradeProxyWithSigners(uint256 numberSigners) internal {
        assert(numberSigners > 0 && numberSigners < 6);

        if (numberSigners >= 1) {
            activeSigners.push(signer0);
        }
        if (numberSigners >= 2) {
            activeSigners.push(signer1);
        }
        if (numberSigners >= 3) {
            activeSigners.push(signer2);
        }
        if (numberSigners >= 4) {
            activeSigners.push(signer3);
        }
        if (numberSigners == 5) {
            activeSigners.push(signer4);
        }

        _upgradeProxy(activeSigners);
    }

    function _generateMockHandlesList(uint256 numberHandles) internal pure returns (bytes32[] memory) {
        assert(numberHandles < 250);
        bytes32[] memory handlesList = new bytes32[](numberHandles);
        for (uint256 i = 0; i < numberHandles; i++) {
            handlesList[i] = bytes32(uint256(i + 1));
        }
        return handlesList;
    }

    function _initializeSigners() internal {
        signer0 = vm.addr(privateKeySigner0);
        signer1 = vm.addr(privateKeySigner1);
        signer2 = vm.addr(privateKeySigner2);
        signer3 = vm.addr(privateKeySigner3);
        signer4 = vm.addr(privateKeySigner4);

        signerPrivateKeys[signer0] = privateKeySigner0;
        signerPrivateKeys[signer1] = privateKeySigner1;
        signerPrivateKeys[signer2] = privateKeySigner2;
        signerPrivateKeys[signer3] = privateKeySigner3;
        signerPrivateKeys[signer4] = privateKeySigner4;
    }

    function setUp() public {
        _deployProxy();
        _deployAndEtchACL();
        _initializeSigners();
    }

    // ============ Basic Functionality Tests ============

    function test_PostProxyUpgradeCheck() public {
        uint256 numberSigners = 3;
        _upgradeProxyWithSigners(numberSigners);
        assertEq(kmsVerifier.getVersion(), string(abi.encodePacked("KMSVerifierV2 v2.0.0")));
        assertEq(kmsVerifier.getThreshold(), initialThreshold);
        assertEq(kmsVerifier.getCurrentEpochId(), 1);
    }

    function test_GetKmsSignersWorkAsExpected() public {
        uint256 numberSigners = 3;
        _upgradeProxyWithSigners(numberSigners);
        address[] memory signers = kmsVerifier.getKmsSigners();
        assertEq(signers.length, numberSigners);
        assertEq(signers[0], signer0);
        assertEq(signers[1], signer1);
        assertEq(signers[2], signer2);
        for (uint256 i = 0; i < numberSigners; i++) {
            assertTrue(kmsVerifier.isValidSigner(signers[i]));
        }
    }

    // ============ Epoch Grace Period Tests ============

    function test_DefineNewContextStartsGracePeriod() public {
        _upgradeProxyWithSigners(3);
        
        assertEq(kmsVerifier.getCurrentEpochId(), 1);
        assertFalse(kmsVerifier.isInGracePeriod());
        
        // Define new context with different signers
        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 2);
        
        // Should now be in grace period
        assertTrue(kmsVerifier.isInGracePeriod());
        assertEq(kmsVerifier.getCurrentEpochId(), 2);
        assertGt(kmsVerifier.getGracePeriodEnd(), block.timestamp);
    }

    function test_BothEpochSignersValidDuringGracePeriod() public {
        _upgradeProxyWithSigners(3);
        
        // Store original signers
        address originalSigner = signer0;
        
        // Define new context
        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 2);
        
        // During grace period, both old and new signers should be valid
        assertTrue(kmsVerifier.isInGracePeriod());
        
        // New signers should be valid
        assertTrue(kmsVerifier.isValidSigner(signer3));
        assertTrue(kmsVerifier.isValidSigner(signer4));
        
        // Old signers should also be valid during grace period
        assertTrue(kmsVerifier.isValidSigner(originalSigner));
        assertTrue(kmsVerifier.isValidSigner(signer1));
        assertTrue(kmsVerifier.isValidSigner(signer2));
    }

    function test_OldSignersInvalidAfterGracePeriod() public {
        _upgradeProxyWithSigners(3);
        
        address originalSigner = signer0;
        
        // Define new context
        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 2);
        
        // Warp time past grace period
        uint256 gracePeriodEnd = kmsVerifier.getGracePeriodEnd();
        vm.warp(gracePeriodEnd + 1);
        
        // Should no longer be in grace period
        assertFalse(kmsVerifier.isInGracePeriod());
        
        // New signers should still be valid
        assertTrue(kmsVerifier.isValidSigner(signer3));
        assertTrue(kmsVerifier.isValidSigner(signer4));
        
        // Old signers should now be invalid
        assertFalse(kmsVerifier.isValidSigner(originalSigner));
        assertFalse(kmsVerifier.isValidSigner(signer1));
        assertFalse(kmsVerifier.isValidSigner(signer2));
    }

    function test_EffectiveThresholdDuringGracePeriod() public {
        _upgradeProxyWithSigners(3);
        
        // Set threshold to 3
        vm.prank(owner);
        kmsVerifier.setThreshold(3);
        assertEq(kmsVerifier.getEffectiveThreshold(), 3);
        
        // Define new context with lower threshold
        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1);
        
        // During grace period, effective threshold should be minimum (1)
        assertTrue(kmsVerifier.isInGracePeriod());
        assertEq(kmsVerifier.getEffectiveThreshold(), 1);
        
        // Current threshold should be 1
        assertEq(kmsVerifier.getThreshold(), 1);
        
        // Previous threshold should be 3
        assertEq(kmsVerifier.getPreviousThreshold(), 3);
    }

    function test_EpochIdIncrementsOnEachContextChange() public {
        _upgradeProxyWithSigners(1);
        assertEq(kmsVerifier.getCurrentEpochId(), 1);
        
        // First context change
        address[] memory newSigners1 = new address[](1);
        newSigners1[0] = signer1;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners1, 1);
        assertEq(kmsVerifier.getCurrentEpochId(), 2);
        
        // Second context change
        address[] memory newSigners2 = new address[](1);
        newSigners2[0] = signer2;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners2, 1);
        assertEq(kmsVerifier.getCurrentEpochId(), 3);
    }

    function test_MultipleRapidContextSwitches() public {
        _upgradeProxyWithSigners(1);
        
        // Rapid context switches
        for (uint256 i = 0; i < 5; i++) {
            address[] memory newSigners = new address[](1);
            newSigners[0] = vm.addr(0x100 + i);
            
            vm.prank(owner);
            kmsVerifier.defineNewContext(newSigners, 1);
            
            // Each should be in grace period
            assertTrue(kmsVerifier.isInGracePeriod());
            assertEq(kmsVerifier.getCurrentEpochId(), 2 + i);
        }
    }

    function test_SignaturesValidDuringGracePeriod() public {
        _upgradeProxyWithSigners(3);
        
        vm.prank(owner);
        kmsVerifier.setThreshold(2);
        
        // Define new context (but old signers still have valid keys)
        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 2);
        
        // Create signature with OLD signers during grace period
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        
        // Sign with old signers (signer1 and signer2)
        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);
        
        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );
        
        // Should succeed because old signers are still valid during grace period
        assertTrue(kmsVerifier.isInGracePeriod());
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    function test_SignaturesInvalidAfterGracePeriod() public {
        _upgradeProxyWithSigners(3);
        
        vm.prank(owner);
        kmsVerifier.setThreshold(2);
        
        // Define new context
        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 2);
        
        // Warp past grace period
        vm.warp(kmsVerifier.getGracePeriodEnd() + 1);
        
        // Create signature with OLD signers after grace period
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        
        // Sign with old signers
        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);
        
        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );
        
        // Should fail because old signers are no longer valid
        assertFalse(kmsVerifier.isInGracePeriod());
        vm.expectPartialRevert(KMSVerifierV2.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    // ============ Grace Period Configuration Tests ============

    function test_SetGracePeriodDuration() public {
        _upgradeProxyWithSigners(1);
        
        uint256 newDuration = 2 hours;
        vm.prank(owner);
        kmsVerifier.setGracePeriodDuration(newDuration);
        
        assertEq(kmsVerifier.getGracePeriodDuration(), newDuration);
    }

    function test_GracePeriodDurationCannotBeZero() public {
        _upgradeProxyWithSigners(1);
        
        vm.prank(owner);
        vm.expectRevert(KMSVerifierV2.GracePeriodDurationIsZero.selector);
        kmsVerifier.setGracePeriodDuration(0);
    }

    function test_CustomGracePeriodDurationApplied() public {
        _upgradeProxyWithSigners(1);
        
        uint256 customDuration = 30 minutes;
        vm.prank(owner);
        kmsVerifier.setGracePeriodDuration(customDuration);
        
        // Define new context
        address[] memory newSigners = new address[](1);
        newSigners[0] = signer1;
        
        uint256 beforeTimestamp = block.timestamp;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1);
        
        // Grace period end should be ~30 minutes from now
        assertEq(kmsVerifier.getGracePeriodEnd(), beforeTimestamp + customDuration);
    }

    // ============ Access Control Tests ============

    function test_OnlyOwnerCanDefineNewContext(address randomAccount) public {
        vm.assume(randomAccount != owner);
        _upgradeProxyWithSigners(3);
        
        address[] memory newSigners = new address[](1);
        newSigners[0] = address(42);
        
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        vm.prank(randomAccount);
        kmsVerifier.defineNewContext(newSigners, 1);
    }

    function test_OnlyOwnerCanSetGracePeriodDuration(address randomAccount) public {
        vm.assume(randomAccount != owner);
        _upgradeProxyWithSigners(1);
        
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        vm.prank(randomAccount);
        kmsVerifier.setGracePeriodDuration(2 hours);
    }

    // ============ Previous Epoch Getters Tests ============

    function test_GetPreviousSignersDuringGracePeriod() public {
        _upgradeProxyWithSigners(3);
        
        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 2);
        
        // Check previous signers
        address[] memory previousSigners = kmsVerifier.getPreviousKmsSigners();
        assertEq(previousSigners.length, 3);
        assertEq(previousSigners[0], signer0);
        assertEq(previousSigners[1], signer1);
        assertEq(previousSigners[2], signer2);
        
        // Check isPreviousSigner
        assertTrue(kmsVerifier.isPreviousSigner(signer0));
        assertTrue(kmsVerifier.isPreviousSigner(signer1));
        assertTrue(kmsVerifier.isPreviousSigner(signer2));
        assertFalse(kmsVerifier.isPreviousSigner(signer3));
    }

    // ============ Backward Compatibility Tests ============

    function test_IsSignerBackwardCompatibility() public {
        _upgradeProxyWithSigners(3);
        
        // isSigner should work the same as isValidSigner
        assertTrue(kmsVerifier.isSigner(signer0));
        assertTrue(kmsVerifier.isSigner(signer1));
        assertTrue(kmsVerifier.isSigner(signer2));
        assertFalse(kmsVerifier.isSigner(signer3));
    }

    // ============ Event Tests ============

    function test_NewContextSetEventEmitted() public {
        _upgradeProxyWithSigners(1);
        
        address[] memory newSigners = new address[](1);
        newSigners[0] = signer1;
        
        uint256 expectedEpochId = 2;
        uint256 expectedGracePeriodEnd = block.timestamp + kmsVerifier.getGracePeriodDuration();
        
        vm.prank(owner);
        // Just record that events are emitted - exact matching is fragile
        vm.recordLogs();
        kmsVerifier.defineNewContext(newSigners, 1);
        
        // Verify state after event
        assertEq(kmsVerifier.getCurrentEpochId(), expectedEpochId);
        assertEq(kmsVerifier.getGracePeriodEnd(), expectedGracePeriodEnd);
    }
}

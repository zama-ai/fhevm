// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {KMSVerifier} from "../../contracts/KMSVerifier.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract KMSVerifierTest is Test {
    KMSVerifier internal kmsVerifier;

    uint256 internal constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;
    bytes32 internal constant KMS_VERIFIER_STORAGE_SLOT =
        0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900;
    bytes32 internal constant OZ_INITIALIZABLE_STORAGE_SLOT =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
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

    /**
     * @dev Computes the signature for a given digest using the provided private key.
     * @param privateKey The private key used to sign the digest.
     * @param digest The hash of the data to be signed.
     * @return signature The computed signature as a byte array, encoded as {r}{s}{v}.
     */
    function _computeSignature(uint256 privateKey, bytes32 digest) internal pure returns (bytes memory signature) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);
        return abi.encodePacked(r, s, v);
    }

    /**
     * @dev Computes the digest of the given handles list and decrypted result.
     * This function uses the EIP-712 encoding scheme to hash the data.
     *
     * @param handlesList An array of bytes32 representing the handles.
     * @param decryptedResult A bytes array containing the decrypted result.
     * @return A bytes32 hash representing the computed digest.
     */
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

    /**
     * @dev Computes the EIP-712 domain separator.
     * This function retrieves the domain parameters from the `kmsVerifier` contract,
     * including the name, version, chain ID, and verifying contract address.
     * It then encodes these parameters and hashes them using the keccak256 algorithm
     * to produce the domain separator.
     *
     * @return bytes32 The computed domain separator.
     */
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

    /**
     * @dev Internal function to deploy a UUPS proxy contract.
     * The proxy is deployed using the UnsafeUpgrades library and initialized with the owner address.
     */
    function _deployProxy() internal {
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );
    }

    /**
     * @dev Internal function to deploy and etch ACL contract at expected constant address.
     * Also stores `owner` as ACL's owner, this is needed for ownership of core contracts.
     */
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

    /**
     * @dev Internal function to upgrade the deployed proxy to a new implementation.
     * The new implementation is an instance of the KMSVerifier contract.
     * The proxy is upgraded using the UnsafeUpgrades library and the owner address.
     */
    function _upgradeProxy(address[] memory signers) internal {
        implementation = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(
                kmsVerifier.initializeFromEmptyProxy,
                (verifyingContractSource, uint64(block.chainid), signers, initialThreshold)
            ),
            owner
        );
        kmsVerifier = KMSVerifier(proxy);
    }

    /**
     * @dev Upgrades the proxy with a specified number of signers (1-5).
     * Adds signers (signer0 to signer4) to `activeSigners` based on `numberSigners`.
     * Calls `_upgradeProxy` with the updated `activeSigners`.
     *
     * @param numberSigners Number of signers (1-5).
     */
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

    /**
     * @dev Builds a complete decryption proof from signer private keys and extra data.
     * @param signerKeys Array of private keys to sign with.
     * @param extraData The extra data to include in the proof.
     * @param handlesList The handles list for digest computation.
     * @param decryptedResult The decrypted result for digest computation.
     * @return decryptionProof The assembled decryption proof bytes.
     */
    function _buildDecryptionProof(
        uint256[] memory signerKeys,
        bytes memory extraData,
        bytes32[] memory handlesList,
        bytes memory decryptedResult
    ) internal view returns (bytes memory) {
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes memory proof = abi.encodePacked(uint8(signerKeys.length));
        for (uint256 i = 0; i < signerKeys.length; i++) {
            proof = abi.encodePacked(proof, _computeSignature(signerKeys[i], digest));
        }
        return abi.encodePacked(proof, extraData);
    }

    /**
     * @dev Initializes signer addresses and maps them to their private keys.
     */
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

    /**
     * @dev Sets up the testing environment by deploying a proxy contract and initializing signers.
     * This function is executed before each test to ensure a consistent and isolated state.
     */
    function setUp() public {
        _deployProxy();
        _deployAndEtchACL();
        _initializeSigners();
    }

    /**
     * @dev Tests that the post-upgrade check for the proxy contract works as expected.
     * It verifies that the version and threshold are set correctly after the upgrade.
     */
    function test_PostProxyUpgradeCheck() public {
        uint256 numberSigners = 3;
        _upgradeProxyWithSigners(numberSigners);
        assertEq(kmsVerifier.getVersion(), string(abi.encodePacked("KMSVerifier v0.2.0")));
        assertEq(kmsVerifier.getThreshold(), initialThreshold);
    }

    /**
     * @dev Tests that getKmsSigners view function works as expected.
     */
    function test_GetKmsSignersWorkAsExpected() public {
        uint256 numberSigners = 3;
        _upgradeProxyWithSigners(numberSigners);
        address[] memory signers = kmsVerifier.getKmsSigners();
        assertEq(signers.length, numberSigners);
        assertEq(signers[0], signer0);
        assertEq(signers[1], signer1);
        assertEq(signers[2], signer2);
        for (uint256 i = 0; i < numberSigners; i++) {
            assertTrue(kmsVerifier.isSigner(signers[i]));
        }
    }

    /**
     * @dev Tests that only the contract owner can add a signer.
     */
    function test_OnlyOwnerCanDefineNewContext(address randomAccount) public {
        vm.assume(randomAccount != owner);
        _upgradeProxyWithSigners(3);
        address randomSigner = address(42);
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        vm.prank(randomAccount);
        address[] memory newSigners = new address[](1);
        newSigners[0] = randomSigner;
        kmsVerifier.defineNewContext(newSigners, 1);
    }

    /**
     * @dev Tests that the contract owner cannot add a null address as a signer.
     */
    function test_OwnerCannotAddNullAddressAsSigner() public {
        _upgradeProxyWithSigners(3);
        address nullSigner = address(0);
        address[] memory newSigners = new address[](1);
        newSigners[0] = nullSigner;
        vm.expectPartialRevert(KMSVerifier.KMSSignerNull.selector);
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1);
    }

    /**
     * @dev Tests that the owner of the contract can successfully add a new signer.
     */
    function test_OwnerCanAddNewSigner() public {
        _upgradeProxyWithSigners(3);
        address randomSigner = address(42);
        address[] memory newSigners = new address[](1);
        newSigners[0] = randomSigner;
        vm.prank(owner);
        vm.expectEmit(true, true, true, true);
        emit KMSVerifier.NewContextSet(KMS_CONTEXT_COUNTER_BASE + 2, newSigners, 1);
        kmsVerifier.defineNewContext(newSigners, 1);
        assertEq(kmsVerifier.getKmsSigners()[0], randomSigner);
        assertTrue(kmsVerifier.isSigner(randomSigner));
    }

    /**
     * @dev Tests that the contract owner cannot add the same signer twice.
     */
    function test_OwnerCannotAddSameSignerTwice() public {
        test_OwnerCanAddNewSigner();
        address randomSigner = kmsVerifier.getKmsSigners()[0];
        address[] memory newSigners = new address[](2);
        newSigners[0] = randomSigner;
        newSigners[1] = randomSigner;
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.KMSAlreadySigner.selector);
        kmsVerifier.defineNewContext(newSigners, 1);
    }

    /**
     * @dev Tests that the owner can successfully remove a signer.
     */
    function test_OwnerCanRemoveSigner() public {
        /// @dev We call the other test to avoid repeating the same code.
        test_OwnerCanAddNewSigner();

        address randomSigner = address(43);
        vm.startPrank(owner);

        address[] memory newSigners = new address[](2);
        newSigners[0] = address(42);
        newSigners[1] = randomSigner;
        kmsVerifier.defineNewContext(newSigners, 2);
        assertEq(kmsVerifier.getKmsSigners().length, 2);

        address[] memory newSigners2 = new address[](1);
        newSigners2[0] = address(42);
        kmsVerifier.defineNewContext(newSigners2, 1);
        assertFalse(kmsVerifier.isSigner(randomSigner));
        assertEq(kmsVerifier.getKmsSigners().length, 1);
    }

    /**
     * @dev Test to ensure that the contract owner cannot remove the last signer.
     * This function verifies that the contract logic prevents the removal of the
     * final signer, maintaining at least one signer at all times.
     */
    function test_OwnerCannotRemoveTheLastSigner() public {
        /// @dev We call the other test to avoid repeating the same code.
        test_OwnerCanAddNewSigner();
        address[] memory emptyAddress = new address[](0);
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.SignersSetIsEmpty.selector);
        kmsVerifier.defineNewContext(emptyAddress, 0);
    }

    /**
     * @dev Tests that only the owner can set the threshold.
     * @param randomAccount An address that is not the owner.
     */
    function test_OnlyOwnerCanSetThreshold(address randomAccount) public {
        vm.assume(randomAccount != owner);
        _upgradeProxyWithSigners(3);
        vm.prank(randomAccount);
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        kmsVerifier.setThreshold(2);
    }

    /**
     * @dev Tests that the threshold value must not be set to 0.
     */
    function test_ThresholdMustBeNotSetToZero() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.ThresholdIsNull.selector);
        kmsVerifier.setThreshold(0);
    }

    /**
     * @dev Tests that the threshold cannot be set if it is above the number of signers.
     */
    function test_ThresholdCannotBeSetIfAboveNumberOfSigners() public {
        _upgradeProxyWithSigners(3);
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.ThresholdIsAboveNumberOfSigners.selector);
        kmsVerifier.setThreshold(4);
    }

    /// @dev This function exists for the test below to call it externally.
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        _upgradeProxyWithSigners(3);
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        this.upgrade(randomAccount);
    }

    /**
     * @dev Tests that the contract owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        _upgradeProxyWithSigners(3);
        /// @dev It does not revert since it called by the owner.
        this.upgrade(owner);
    }

    /**
     * @dev Tests that the EIP-712 KMS signatures verification works correctly
     *      by setting up three signers, creating a list of handles, generating a
     *      decrypted result, computing the digest, and verifying the signatures.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesWork() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    /**
     * @dev Tests that verifyDecryptionEIP712KMSSignatures fails as expected if the digest is invalid.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfDigestIsInvalid() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes[] memory signatures = new bytes[](2);

        bytes32 invalidDigest = bytes32("420");

        signatures[0] = _computeSignature(privateKeySigner1, invalidDigest);
        signatures[1] = _computeSignature(privateKeySigner2, invalidDigest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected when no signer is added.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNoSignerAdded() public {
        _upgradeProxyWithSigners(1);
        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected when no signature is provided.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNoSignatureProvided() public {
        _upgradeProxyWithSigners(3);

        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes[] memory signatures = new bytes[](0);

        bytes memory decryptionProof = abi.encodePacked(uint8(signatures.length), extraData);

        vm.expectPartialRevert(KMSVerifier.KMSZeroSignature.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected
     *      if the number of signatures is less than the defined threshold.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNumberOfSignaturesIsInferiorToThreshold() public {
        _upgradeProxyWithSigners(3);

        vm.prank(owner);
        kmsVerifier.setThreshold(2);
        assertEq(kmsVerifier.getThreshold(), 2);

        /// @dev Mock data for testing purposes.
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](1);
        signatures[0] = _computeSignature(privateKeySigner1, digest);

        bytes memory decryptionProof = abi.encodePacked(uint8(signatures.length), signatures[0], extraData);

        vm.expectPartialRevert(KMSVerifier.KMSSignatureThresholdNotReached.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected if the same signer is used twice.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfSameSignerIsUsedTwice() public {
        _upgradeProxyWithSigners(3);

        /// @dev The threshold is set to 2, so we need at least 2 signatures from different signers.
        vm.prank(owner);
        kmsVerifier.setThreshold(2);
        assertEq(kmsVerifier.getThreshold(), 2);

        /// @dev Mock data for testing purposes.
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner1, digest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        assertFalse(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    /**
     * @dev Tests that the verifyDecryptionEIP712KMSSignatures function fails if the decryptionProof is empty.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesFailsIfEmptyDecryptionProof() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory decryptionProof = new bytes(0);

        vm.expectRevert(KMSVerifier.EmptyDecryptionProof.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /**
     * @dev Tests that the verifyDecryptionEIP712KMSSignatures function fails if the length of the decryption proof is invalid.
     */
    function test_VerifyDecryptionEIP712KMSSignaturesFailsIfDeserializingDecryptionProofFail(
        uint256 randomValue
    ) public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes memory decryptionProof = abi.encodePacked(uint8(3), randomValue);

        vm.expectRevert(KMSVerifier.DeserializingDecryptionProofFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /// @dev This function exists for the test below to call it externally.
    function emptyUpgrade() public {
        address[] memory emptySigners = new address[](0);
        implementation = address(new KMSVerifier());

        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(
                KMSVerifier.initializeFromEmptyProxy,
                (verifyingContractSource, uint64(block.chainid), emptySigners, initialThreshold)
            ),
            owner
        );
    }

    /**
     * @dev Tests that the contract cannot be reinitialized if the initial signers set is empty.
     */
    function test_CannotReinitializeIfInitialSignersSetIsEmpty() public {
        vm.expectPartialRevert(KMSVerifier.SignersSetIsEmpty.selector);
        this.emptyUpgrade();
    }

    // =========================================================================
    //  Context management tests
    // =========================================================================

    /**
     * @dev Tests that defineNewContext increments the context ID and preserves signers for both old and new contexts.
     */
    function test_DefineNewContextIncrementsContextIdAndPreservesSigners() public {
        _upgradeProxyWithSigners(3); // context 1 with signer0, signer1, signer2
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();
        assertEq(ctx1, KMS_CONTEXT_COUNTER_BASE + 1);

        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2

        uint256 ctx2 = kmsVerifier.getCurrentKmsContextId();
        assertEq(ctx2, KMS_CONTEXT_COUNTER_BASE + 2);

        // Old context still has old signers
        address[] memory ctx1Signers = kmsVerifier.getSignersForKmsContext(ctx1);
        assertEq(ctx1Signers.length, 3);
        assertEq(ctx1Signers[0], signer0);
        assertEq(ctx1Signers[1], signer1);
        assertEq(ctx1Signers[2], signer2);

        // New context has new signers
        address[] memory ctx2Signers = kmsVerifier.getSignersForKmsContext(ctx2);
        assertEq(ctx2Signers.length, 2);
        assertEq(ctx2Signers[0], signer3);
        assertEq(ctx2Signers[1], signer4);
    }

    /**
     * @dev Tests that getSignersForKmsContext returns an empty array for destroyed and non-existent contexts.
     */
    function test_GetSignersForKmsContextReturnsEmptyForInvalidContexts() public {
        _upgradeProxyWithSigners(3); // context 1
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.startPrank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2
        kmsVerifier.destroyKmsContext(ctx1);
        vm.stopPrank();

        // Destroyed context returns empty
        assertEq(kmsVerifier.getSignersForKmsContext(ctx1).length, 0);
        // Non-existent context returns empty
        assertEq(kmsVerifier.getSignersForKmsContext(KMS_CONTEXT_COUNTER_BASE + 999).length, 0);
    }

    /**
     * @dev Tests that isValidKmsContext returns true for a valid (non-destroyed, existing) context.
     */
    function test_ValidateKmsContextReturnsTrueForValidContext() public {
        _upgradeProxyWithSigners(3); // context 1
        assertTrue(kmsVerifier.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 1));
    }

    /**
     * @dev Tests that isValidKmsContext returns false for a non-existent context ID.
     */
    function test_ValidateKmsContextReturnsFalseForNonExistentContext() public {
        _upgradeProxyWithSigners(3);
        assertFalse(kmsVerifier.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 999));
        assertFalse(kmsVerifier.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE));
        assertFalse(kmsVerifier.isValidKmsContext(0));
    }

    /**
     * @dev Tests that destroyKmsContext can only be called by the governance (ACL owner).
     */
    function test_DestroyKmsContextOnlyCallableByGovernance() public {
        _upgradeProxyWithSigners(3); // context 1

        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2

        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        vm.prank(address(0xdead));
        kmsVerifier.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 1);
    }

    /**
     * @dev Tests that destroyKmsContext marks the context as destroyed and emits KMSContextDestroyed.
     */
    function test_DestroyKmsContextMarksAsDestroyed() public {
        _upgradeProxyWithSigners(3); // context 1
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.startPrank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2

        vm.expectEmit(true, true, true, true);
        emit KMSVerifier.KMSContextDestroyed(ctx1);
        kmsVerifier.destroyKmsContext(ctx1);
        vm.stopPrank();

        assertFalse(kmsVerifier.isValidKmsContext(ctx1));
    }

    /**
     * @dev Tests that the current (active) context cannot be destroyed.
     */
    function test_CannotDestroyCurrentContext() public {
        _upgradeProxyWithSigners(3); // context 1
        uint256 currentCtx = kmsVerifier.getCurrentKmsContextId();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.CurrentKMSContextCannotBeDestroyed.selector, currentCtx));
        kmsVerifier.destroyKmsContext(currentCtx);
    }

    /**
     * @dev Tests that destroying a non-existent context reverts with InvalidKMSContext.
     */
    function test_DestroyNonExistentContextReverts() public {
        _upgradeProxyWithSigners(3);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, 0));
        kmsVerifier.destroyKmsContext(0);
    }

    // =========================================================================
    //  Verification with context tests
    // =========================================================================

    /**
     * @dev Tests that verification succeeds for an old context using its original signers
     *      with v1 extraData pointing to that context.
     */
    function test_VerificationSucceedsForOldContextWithOldSigners() public {
        _upgradeProxyWithSigners(3); // context 1 with signer0, signer1, signer2
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        // Switch to new context
        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2

        // Verify with old context's signers using v1 extraData
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);

        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner0;
        bytes memory decryptionProof = _buildDecryptionProof(keys, extraData, handlesList, decryptedResult);

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    /**
     * @dev Tests that verification reverts with InvalidKMSContext when targeting a destroyed context.
     */
    function test_VerificationFailsForDestroyedContext() public {
        _upgradeProxyWithSigners(3); // context 1
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.startPrank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2
        kmsVerifier.destroyKmsContext(ctx1);
        vm.stopPrank();

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);

        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner0;
        bytes memory decryptionProof = _buildDecryptionProof(keys, extraData, handlesList, decryptedResult);

        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, ctx1));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /**
     * @dev Tests that verification reverts with UnsupportedExtraDataVersion for unknown version bytes.
     */
    function test_VerificationFailsWithUnsupportedExtraDataVersion() public {
        _upgradeProxyWithSigners(3);

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"));
        bytes memory extraData = abi.encodePacked(uint8(0x02)); // unsupported version

        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner0;
        bytes memory decryptionProof = _buildDecryptionProof(keys, extraData, handlesList, decryptedResult);

        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.UnsupportedExtraDataVersion.selector, uint8(0x02)));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    /**
     * @dev Tests that verification reverts with DeserializingDecryptionProofFail for malformed v1 extraData
     *      (version byte 0x01 but missing the required 32-byte context ID).
     */
    function test_VerificationFailsWithMalformedV1ExtraData() public {
        _upgradeProxyWithSigners(3);

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"));
        // v1 but missing the 32-byte context ID
        bytes memory extraData = abi.encodePacked(uint8(0x01), uint8(0x42));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes memory sig = _computeSignature(privateKeySigner0, digest);
        bytes memory decryptionProof = abi.encodePacked(uint8(1), sig, extraData);

        vm.expectRevert(KMSVerifier.DeserializingDecryptionProofFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    // =========================================================================
    //  Additional edge case tests
    // =========================================================================

    /**
     * @dev Tests that destroying an already-destroyed context reverts with InvalidKMSContext.
     */
    function test_CannotDestroyAlreadyDestroyedContext() public {
        _upgradeProxyWithSigners(3); // context 1
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.startPrank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2
        kmsVerifier.destroyKmsContext(ctx1);

        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, ctx1));
        kmsVerifier.destroyKmsContext(ctx1);
        vm.stopPrank();
    }

    /**
     * @dev Tests that verification fails with InvalidKMSContext when v1 extraData
     *      points to an invalid context ID (above range or off-by-one below range).
     */
    function test_VerificationFailsForInvalidContextWithV1ExtraData() public {
        _upgradeProxyWithSigners(3); // context 1

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner0;

        // Above range: context ID that was never created
        uint256 nonExistentCtx = KMS_CONTEXT_COUNTER_BASE + 999;
        bytes memory extraData1 = abi.encodePacked(uint8(0x01), nonExistentCtx);
        bytes memory decryptedResult1 = abi.encodePacked(keccak256("test"));
        bytes memory proof1 = _buildDecryptionProof(keys, extraData1, handlesList, decryptedResult1);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, nonExistentCtx));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult1, proof1);

        // Off-by-one below range: KMS_CONTEXT_COUNTER_BASE (first valid is BASE + 1)
        bytes memory extraData2 = abi.encodePacked(uint8(0x01), KMS_CONTEXT_COUNTER_BASE);
        bytes memory decryptedResult2 = abi.encodePacked(keccak256("test-base"));
        bytes memory proof2 = _buildDecryptionProof(keys, extraData2, handlesList, decryptedResult2);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, KMS_CONTEXT_COUNTER_BASE));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult2, proof2);

        // Zero context ID: default/uninitialized value guard
        bytes memory extraData3 = abi.encodePacked(uint8(0x01), uint256(0));
        bytes memory decryptedResult3 = abi.encodePacked(keccak256("test-zero"));
        bytes memory proof3 = _buildDecryptionProof(keys, extraData3, handlesList, decryptedResult3);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, uint256(0)));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult3, proof3);
    }

    /**
     * @dev Tests that verification succeeds with empty (zero-length) extraData,
     *      which should fall back to the current context.
     */
    function test_VerificationSucceedsWithEmptyExtraData() public {
        _upgradeProxyWithSigners(3); // context 1

        bytes32[] memory handlesList = _generateMockHandlesList(2);
        bytes memory decryptedResult = abi.encodePacked(keccak256("empty"));
        bytes memory extraData = new bytes(0);

        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner0;
        bytes memory decryptionProof = _buildDecryptionProof(keys, extraData, handlesList, decryptedResult);

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    /**
     * @dev Tests the reinitializeV2 migration path by simulating a V2 deployment
     *      (legacy signers in flat storage, initialized version = 2) and then
     *      calling reinitializeV2 to migrate into context-aware storage.
     */
    function test_ReinitializeV2MigratesExistingSigners() public {
        // Deploy a fresh empty proxy
        address proxy2 = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        // Upgrade to KMSVerifier without calling any initializer
        address impl = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(proxy2, impl, "", owner);
        KMSVerifier kv = KMSVerifier(proxy2);

        // Simulate V2 state by manually populating legacy storage.
        // KMSVerifierStorage layout: +0: isSigner mapping, +1: signers array, +2: threshold, +3: currentKmsContextId
        bytes32 signersLenSlot = bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 1);
        vm.store(proxy2, signersLenSlot, bytes32(uint256(2)));

        bytes32 signersDataSlot = keccak256(abi.encode(signersLenSlot));
        vm.store(proxy2, signersDataSlot, bytes32(uint256(uint160(signer0))));
        vm.store(proxy2, bytes32(uint256(signersDataSlot) + 1), bytes32(uint256(uint160(signer1))));

        vm.store(proxy2, bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 2), bytes32(uint256(1)));

        // Set OZ Initializable _initialized = 2 (simulating completed V2 init)
        vm.store(proxy2, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));

        // Call reinitializeV2 — should migrate legacy signers into context 1
        vm.prank(owner);
        kv.reinitializeV2();

        // Verify migration
        assertEq(kv.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);
        address[] memory ctxSigners = kv.getSignersForKmsContext(KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(ctxSigners.length, 2);
        assertEq(ctxSigners[0], signer0);
        assertEq(ctxSigners[1], signer1);
        assertTrue(kv.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 1));
    }

    /**
     * @dev Tests that setThreshold correctly updates the context threshold so that
     *      verification respects the new threshold.
     */
    function test_SetThresholdAffectsVerification() public {
        _upgradeProxyWithSigners(3); // context 1 with threshold 1
        uint256 ctx = kmsVerifier.getCurrentKmsContextId();

        vm.prank(owner);
        kmsVerifier.setThreshold(2);

        assertEq(kmsVerifier.getThreshold(), 2);
        assertTrue(kmsVerifier.isValidKmsContext(ctx));

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("threshold-test"));
        bytes memory extraData = abi.encodePacked(uint8(0x00));

        // Single signature should now fail
        uint256[] memory singleKey = new uint256[](1);
        singleKey[0] = privateKeySigner0;
        bytes memory singleProof = _buildDecryptionProof(singleKey, extraData, handlesList, decryptedResult);
        vm.expectPartialRevert(KMSVerifier.KMSSignatureThresholdNotReached.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, singleProof);

        // Two signatures should succeed
        uint256[] memory dualKeys = new uint256[](2);
        dualKeys[0] = privateKeySigner0;
        dualKeys[1] = privateKeySigner1;
        bytes memory dualProof = _buildDecryptionProof(dualKeys, extraData, handlesList, decryptedResult);
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, dualProof));
    }

    /**
     * @dev Tests that a signer from context 2 cannot sign for context 1 where they are not registered.
     *      This is a critical security property: context isolation must prevent cross-context signing.
     */
    function test_CrossContextSignerRejection() public {
        _upgradeProxyWithSigners(3); // context 1 with signer0, signer1, signer2
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1); // context 2 with signer3

        // Attempt to verify against context 1 using signer3 (only in context 2)
        bytes32[] memory handlesList = _generateMockHandlesList(2);
        bytes memory decryptedResult = abi.encodePacked(keccak256("cross-ctx"));
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);

        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner3;
        bytes memory proof = _buildDecryptionProof(keys, extraData, handlesList, decryptedResult);

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    /**
     * @dev Tests that reinitializeV2 cannot be called twice (reinitializer guard).
     */
    function test_ReinitializeV2CannotBeCalledTwice() public {
        // Deploy a fresh empty proxy
        address proxy2 = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        address impl = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(proxy2, impl, "", owner);
        KMSVerifier kv = KMSVerifier(proxy2);

        // Populate legacy storage so reinitializeV2 succeeds the first time
        bytes32 signersLenSlot = bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 1);
        vm.store(proxy2, signersLenSlot, bytes32(uint256(1)));
        bytes32 signersDataSlot = keccak256(abi.encode(signersLenSlot));
        vm.store(proxy2, signersDataSlot, bytes32(uint256(uint160(signer0))));
        vm.store(proxy2, bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 2), bytes32(uint256(1)));

        // Set OZ Initializable _initialized = 2
        vm.store(proxy2, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));

        vm.prank(owner);
        kv.reinitializeV2();

        // Second call must revert with OZ's InvalidInitialization
        vm.prank(owner);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        kv.reinitializeV2();
    }

    /**
     * @dev Tests that calling setThreshold on the current context does not retroactively
     *      affect a previous context's threshold. Specifically: context 1 has threshold 1,
     *      then setThreshold(2) is called, then context 2 is created with threshold 1.
     *      Context 1 must still require 2 signatures.
     */
    function test_SetThresholdRetainedOnPreviousContextAfterNewContext() public {
        _upgradeProxyWithSigners(3); // context 1, threshold 1
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        vm.startPrank(owner);
        kmsVerifier.setThreshold(2); // context 1 now requires 2 signatures

        address[] memory newSigners = new address[](2);
        newSigners[0] = signer3;
        newSigners[1] = signer4;
        kmsVerifier.defineNewContext(newSigners, 1); // context 2, threshold 1
        vm.stopPrank();

        // Context 1 should still require 2 signatures — 1 signature must fail
        bytes32[] memory handlesList = _generateMockHandlesList(2);
        bytes memory decryptedResult = abi.encodePacked(keccak256("threshold-isolation"));
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);

        uint256[] memory singleKey = new uint256[](1);
        singleKey[0] = privateKeySigner0;
        bytes memory singleProof = _buildDecryptionProof(singleKey, extraData, handlesList, decryptedResult);
        vm.expectPartialRevert(KMSVerifier.KMSSignatureThresholdNotReached.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, singleProof);

        // 2 signatures should succeed for context 1
        uint256[] memory dualKeys = new uint256[](2);
        dualKeys[0] = privateKeySigner0;
        dualKeys[1] = privateKeySigner1;
        bytes memory dualProof = _buildDecryptionProof(dualKeys, extraData, handlesList, decryptedResult);
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, dualProof));
    }

    /**
     * @dev Tests that setThreshold emits NewContextSet with the correct parameters.
     */
    function test_SetThresholdEmitsNewContextSetEvent() public {
        _upgradeProxyWithSigners(3); // context 1, threshold 1
        uint256 ctx = kmsVerifier.getCurrentKmsContextId();
        address[] memory ctxSigners = kmsVerifier.getSignersForKmsContext(ctx);

        vm.prank(owner);
        vm.expectEmit(true, true, true, true);
        emit KMSVerifier.NewContextSet(ctx, ctxSigners, 2);
        kmsVerifier.setThreshold(2);
    }

    /**
     * @dev Tests that verification succeeds with v1 extraData pointing to the current context ID.
     *      All existing current-context tests use v0 extraData; this exercises the v1 path.
     */
    function test_VerificationSucceedsWithV1ExtraDataForCurrentContext() public {
        _upgradeProxyWithSigners(3); // context 1 with signer0, signer1, signer2
        uint256 currentCtx = kmsVerifier.getCurrentKmsContextId();

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("v1-current"));
        // Trailing bytes after the 33-byte minimum are ignored for context resolution
        // but included in the EIP-712 digest, so they exercise the full roundtrip.
        bytes memory extraData = abi.encodePacked(uint8(0x01), currentCtx, uint256(12345));

        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner0;
        bytes memory decryptionProof = _buildDecryptionProof(keys, extraData, handlesList, decryptedResult);

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    /**
     * @dev Tests that v0 extraData with trailing bytes still routes to the current context.
     *      Exercises the documented forward-compat behavior: `0x00 || arbitrary` → current context.
     */
    function test_V0ExtraDataWithTrailingBytesUsesCurrentContext() public {
        _upgradeProxyWithSigners(3); // context 1

        bytes32[] memory handlesList = _generateMockHandlesList(2);
        bytes memory decryptedResult = abi.encodePacked(keccak256("trailing"));
        // v0 prefix with arbitrary trailing bytes (uint256(12345))
        bytes memory extraData = abi.encodePacked(uint8(0x00), uint256(12345));

        uint256[] memory keys = new uint256[](1);
        keys[0] = privateKeySigner0;
        bytes memory decryptionProof = _buildDecryptionProof(keys, extraData, handlesList, decryptedResult);

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    /**
     * @dev Tests getContextSignersAndThresholdFromExtraData across a context lifecycle:
     *      v0 resolves to current context, v1 reaches back to an old context after rotation,
     *      and v1 reverts once that context is destroyed.
     */
    function test_GetContextSignersAndThresholdFromExtraData() public {
        _upgradeProxyWithSigners(3); // context 1 with signer0, signer1, signer2; threshold 1
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        // v0: resolves to current context
        (address[] memory v0Signers, uint256 v0Threshold) =
            kmsVerifier.getContextSignersAndThresholdFromExtraData(hex"00");
        assertEq(v0Signers.length, 3);
        assertEq(v0Signers[0], signer0);
        assertEq(v0Threshold, 1);

        // Rotate to context 2
        address[] memory newSigners = new address[](1);
        newSigners[0] = signer3;
        vm.prank(owner);
        kmsVerifier.defineNewContext(newSigners, 1);

        // v1 pointing at old context 1 still returns ctx1's signers
        bytes memory v1ExtraData = abi.encodePacked(uint8(0x01), ctx1);
        (address[] memory oldCtxSigners,) =
            kmsVerifier.getContextSignersAndThresholdFromExtraData(v1ExtraData);
        assertEq(oldCtxSigners.length, 3);
        assertEq(oldCtxSigners[0], signer0);

        // Destroy ctx1 → v1 now reverts
        vm.prank(owner);
        kmsVerifier.destroyKmsContext(ctx1);
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.InvalidKMSContext.selector, ctx1));
        kmsVerifier.getContextSignersAndThresholdFromExtraData(v1ExtraData);
    }

    /**
     * @dev Tests that reinitializeV2 reverts with SignersSetIsEmpty when legacy $.signers is empty.
     */
    function test_ReinitializeV2RevertsWithEmptyLegacySigners() public {
        // Deploy a fresh empty proxy
        address proxy2 = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        address impl = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(proxy2, impl, "", owner);
        KMSVerifier kv = KMSVerifier(proxy2);

        // Set OZ Initializable _initialized = 2 (simulating completed V2 init)
        // but leave legacy signers empty (default)
        vm.store(proxy2, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));

        vm.prank(owner);
        vm.expectRevert(KMSVerifier.SignersSetIsEmpty.selector);
        kv.reinitializeV2();
    }

    /**
     * @dev Tests that reinitializeV2 reverts with ThresholdIsNull when legacy $.threshold is 0.
     *      Exercises the _setContextThreshold validation path.
     */
    function test_ReinitializeV2RevertsWithZeroThreshold() public {
        // Deploy a fresh empty proxy
        address proxy2 = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        address impl = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(proxy2, impl, "", owner);
        KMSVerifier kv = KMSVerifier(proxy2);

        // Populate legacy signers but leave threshold at 0
        bytes32 signersLenSlot = bytes32(uint256(KMS_VERIFIER_STORAGE_SLOT) + 1);
        vm.store(proxy2, signersLenSlot, bytes32(uint256(1)));
        bytes32 signersDataSlot = keccak256(abi.encode(signersLenSlot));
        vm.store(proxy2, signersDataSlot, bytes32(uint256(uint160(signer0))));
        // threshold slot (+2) is left at 0 (default)

        // Set OZ Initializable _initialized = 2
        vm.store(proxy2, OZ_INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(2)));

        vm.prank(owner);
        vm.expectRevert(KMSVerifier.ThresholdIsNull.selector);
        kv.reinitializeV2();
    }
}

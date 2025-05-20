// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {KMSVerifier} from "../../contracts/KMSVerifier.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMExecutorAddress.sol";

contract KMSVerifierTest is Test {
    KMSVerifier internal kmsVerifier;

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
        bytes memory decryptedResult
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                kmsVerifier.DECRYPTION_RESULT_TYPEHASH(),
                keccak256(abi.encodePacked(handlesList)),
                keccak256(decryptedResult)
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
            abi.encodeCall(EmptyUUPSProxy.initialize, owner)
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
                kmsVerifier.reinitialize,
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
        _initializeSigners();
    }

    /**
     * @dev Tests that the post-upgrade check for the proxy contract works as expected.
     * It verifies that the version and threshold are set correctly after the upgrade.
     */
    function test_PostProxyUpgradeCheck() public {
        uint256 numberSigners = 3;
        _upgradeProxyWithSigners(numberSigners);
        assertEq(kmsVerifier.getVersion(), string(abi.encodePacked("KMSVerifier v0.1.0")));
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
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
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
        vm.expectEmit();
        emit KMSVerifier.NewContextSet(newSigners, 1);
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
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
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
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
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
    function test_VerifyInputEIP712KMSSignaturesWork() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](2);

        bytes32 digest = _computeDigest(handlesList, decryptedResult);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures));
    }

    /**
     * @dev Tests that verifyInputEIP712KMSSignatures fails as expected if the digest is invalid.
     */
    function test_VerifyInputEIP712KMSSignaturesFailAsExpectedIfDigestIsInvalid() public {
        _upgradeProxyWithSigners(3);
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](3);

        bytes32 invalidDigest = bytes32("420");

        signatures[0] = _computeSignature(privateKeySigner1, invalidDigest);
        signatures[1] = _computeSignature(privateKeySigner2, invalidDigest);

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected when no signer is added.
     */
    function test_VerifyInputEIP712KMSSignaturesFailAsExpectedIfNoSignerAdded() public {
        _upgradeProxyWithSigners(1);
        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](3);

        bytes32 digest = _computeDigest(handlesList, decryptedResult);

        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected when no signature is provided.
     */
    function test_VerifyInputEIP712KMSSignaturesFailAsExpectedIfNoSignatureProvided() public {
        _upgradeProxyWithSigners(3);

        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](0);

        vm.expectPartialRevert(KMSVerifier.KMSZeroSignature.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected
     *      if the number of signatures is less than the defined threshold.
     */
    function test_VerifyInputEIP712KMSSignaturesFailAsExpectedIfNumberOfSignaturesIsInferiorToThreshold() public {
        _upgradeProxyWithSigners(3);

        vm.prank(owner);
        kmsVerifier.setThreshold(2);
        assertEq(kmsVerifier.getThreshold(), 2);

        /// @dev Mock data for testing purposes.
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](1);

        bytes32 digest = _computeDigest(handlesList, decryptedResult);
        signatures[0] = _computeSignature(privateKeySigner1, digest);

        vm.expectPartialRevert(KMSVerifier.KMSSignatureThresholdNotReached.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected if the same signer is used twice.
     */
    function test_VerifyInputEIP712KMSSignaturesFailAsExpectedIfSameSignerIsUsedTwice() public {
        _upgradeProxyWithSigners(3);

        /// @dev The threshold is set to 2, so we need at least 2 signatures from different signers.
        vm.prank(owner);
        kmsVerifier.setThreshold(2);
        assertEq(kmsVerifier.getThreshold(), 2);

        /// @dev Mock data for testing purposes.
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](2);

        bytes32 digest = _computeDigest(handlesList, decryptedResult);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner1, digest);

        assertFalse(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures));
    }
}

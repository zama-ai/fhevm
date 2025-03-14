// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {KMSVerifier} from "../../contracts/KMSVerifier.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {tfheExecutorAdd} from "../../addresses/TFHEExecutorAddress.sol";

contract KMSVerifierTest is Test {
    KMSVerifier internal kmsVerifier;

    address internal constant owner = address(456);
    uint256 internal constant privateKeySigner1 = 0x022;
    uint256 internal constant privateKeySigner2 = 0x03;
    uint256 internal constant privateKeySigner3 = 0x04;

    address internal proxy;
    address internal implementation;
    address internal kmsSigner1;
    address internal kmsSigner2;
    address internal kmsSigner3;

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

    /// @dev Adds three predefined signers to the KMSVerifier contract.
    function _setThreeSigners() internal {
        vm.startPrank(owner);
        kmsVerifier.addSigner(kmsSigner1);
        kmsVerifier.addSigner(kmsSigner2);
        kmsVerifier.addSigner(kmsSigner3);
        vm.stopPrank();
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
    function _upgradeProxy() internal {
        implementation = address(new KMSVerifier());
        UnsafeUpgrades.upgradeProxy(proxy, implementation, "", owner);
        kmsVerifier = KMSVerifier(proxy);
    }

    /**
     * @dev Internal function to initialize the signers.
     * The signers are initialized using their respective private keys.
     */
    function _initializeSigners() internal {
        kmsSigner1 = vm.addr(privateKeySigner1);
        kmsSigner2 = vm.addr(privateKeySigner2);
        kmsSigner3 = vm.addr(privateKeySigner3);
    }

    /**
     * @dev Public function to set up the test environment.
     * This function deploys the proxy, upgrades it to the KMSVerifier implementation, and initializes the signers.
     */
    function setUp() public {
        _deployProxy();
        _upgradeProxy();
        _initializeSigners();
    }

    /// @dev Tests that the version returned by getVersion is "KMSVerifier v0.1.0"
    function test_getVersion() public view {
        assertEq(kmsVerifier.getVersion(), string(abi.encodePacked("KMSVerifier v0.1.0")));
    }

    /// @dev Tests that the initial threshold, owner, and signers list are correctly set after deployment
    function test_postDeployment() public view {
        assertEq(kmsVerifier.getThreshold(), 0);
        assertEq(kmsVerifier.getSigners().length, 0);
        assertEq(kmsVerifier.owner(), owner);
    }

    /**
     * @dev Tests that only the contract owner can add a signer.
     * @param randomAccount An address that is not the owner of the contract.
     */
    function test_OnlyOwnerCanAddSigner(address randomAccount) public {
        vm.assume(randomAccount != owner);
        address randomSigner = address(42);
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        vm.prank(randomAccount);
        kmsVerifier.addSigner(randomSigner);
    }

    /**
     * @dev Tests that only the owner can remove a signer.
     * @param randomAccount An address that is not the owner.
     */
    function test_OnlyOwnerCanRemoveSigner(address randomAccount) public {
        vm.assume(randomAccount != owner);
        address randomSigner = address(42);
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        vm.prank(randomAccount);
        kmsVerifier.removeSigner(randomSigner);
    }

    /**
     * @dev Tests that the contract owner cannot add a null address as a signer.
     */
    function test_OwnerCannotAddNullAddressAsSigner() public {
        address nullSigner = address(0);
        vm.expectPartialRevert(KMSVerifier.KMSSignerNull.selector);
        vm.prank(owner);
        kmsVerifier.addSigner(nullSigner);
    }

    /**
     * @dev Tests that the owner of the contract can successfully add a new signer.
     */
    function test_OwnerCanAddNewSigner() public {
        address randomSigner = address(42);
        vm.prank(owner);
        vm.expectEmit();
        emit KMSVerifier.SignerAdded(randomSigner);
        kmsVerifier.addSigner(randomSigner);
        assertEq(kmsVerifier.getSigners()[0], randomSigner);
        assertTrue(kmsVerifier.isSigner(randomSigner));
    }

    /**
     * @dev Tests that the contract owner cannot add the same signer twice.
     */
    function test_OwnerCannotAddSameSignerTwice() public {
        /// @dev We call the other test to avoid repeating the same code.
        test_OwnerCanAddNewSigner();
        address randomSigner = kmsVerifier.getSigners()[0];
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.KMSAlreadySigner.selector);
        kmsVerifier.addSigner(randomSigner);
    }

    /**
     * @dev Tests that the owner cannot remove a signer if the owner is not a signer.
     */
    function test_OwnerCannotRemoveSignerIfNotSigner() public {
        address randomSigner = address(42);
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.KMSNotASigner.selector);
        kmsVerifier.removeSigner(randomSigner);
    }

    /**
     * @dev Tests that the owner can successfully remove a signer.
     */
    function test_OwnerCanRemoveSigner() public {
        /// @dev We call the other test to avoid repeating the same code.
        test_OwnerCanAddNewSigner();

        address randomSigner2 = address(43);
        vm.prank(owner);
        kmsVerifier.addSigner(randomSigner2);
        assertEq(kmsVerifier.getSigners().length, 2);

        address randomSigner1 = kmsVerifier.getSigners()[0];
        vm.prank(owner);
        vm.expectEmit(true, false, false, true);
        emit KMSVerifier.SignerRemoved(randomSigner1);
        kmsVerifier.removeSigner(randomSigner1);

        assertFalse(kmsVerifier.isSigner(randomSigner1));
        assertEq(kmsVerifier.getSigners().length, 1);
    }

    /**
     * @dev Test to ensure that the contract owner cannot remove the last signer.
     * This function verifies that the contract logic prevents the removal of the
     * final signer, maintaining at least one signer at all times.
     */
    function test_OwnerCannotRemoveTheLastSigner() public {
        /// @dev We call the other test to avoid repeating the same code.
        test_OwnerCanAddNewSigner();

        address randomSigner = kmsVerifier.getSigners()[0];
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.AtLeastOneSignerIsRequired.selector);
        kmsVerifier.removeSigner(randomSigner);
    }

    /**
     * @dev Tests that only the owner can set the threshold.
     * @param randomAccount An address that is not the owner.
     */
    function test_OnlyOwnerCanSetThreshold(address randomAccount) public {
        vm.assume(randomAccount != owner);
        vm.prank(randomAccount);
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        kmsVerifier.setThreshold(2);
    }

    /**
     * @dev Tests that the threshold value must not be set to 0.
     */
    function test_ThresholdMustBeNotSetToZero() public {
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.ThresholdIsNull.selector);
        kmsVerifier.setThreshold(0);
    }

    /**
     * @dev Tests that the threshold cannot be set if it is above the number of signers.
     */
    function test_ThresholdCannotBeSetIfAboveNumberOfSigners() public {
        _setThreeSigners();
        vm.prank(owner);
        vm.expectRevert(KMSVerifier.ThresholdIsAboveNumberOfSigners.selector);
        kmsVerifier.setThreshold(4);
    }

    /**
     * @param randomAccount The address of the random account to be used for the upgrade.
     * @dev This function is used to test that only the owner can authorize an upgrade.
     *      It attempts to upgrade the proxy contract to a new implementation using a random account.
     *      The upgrade should fail if the random account is not the owner.
     */
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     * @param randomAccount An address that is not the owner, used to test unauthorized upgrade attempts.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
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
        /// @dev It does not revert since it called by the owner.
        this.upgrade(owner);
    }

    /**
     * @dev Tests that the EIP-712 KMS signatures verification works correctly
     *         by setting up three signers, creating a list of handles, generating a
     *         decrypted result, computing the digest, and verifying the signatures.
     */
    function test_verifyInputEIP712KMSSignaturesWork() public {
        _setThreeSigners();
        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](3);

        bytes32 digest = _computeDigest(handlesList, decryptedResult);

        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);
        signatures[2] = _computeSignature(privateKeySigner3, digest);

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures));
    }

    /**
     * @dev Tests that verifyInputEIP712KMSSignatures fails as expected if the digest is invalid.
     */
    function test_verifyInputEIP712KMSSignaturesFailAsExpectedIfDigestIsInvalid() public {
        _setThreeSigners();
        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](3);

        bytes32 invalidDigest = bytes32("420");

        signatures[0] = _computeSignature(privateKeySigner1, invalidDigest);
        signatures[1] = _computeSignature(privateKeySigner2, invalidDigest);
        signatures[2] = _computeSignature(privateKeySigner3, invalidDigest);

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected when no signer is added.
     */
    function test_verifyInputEIP712KMSSignaturesFailAsExpectedIfNoSignerAdded() public {
        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](3);

        bytes32 digest = _computeDigest(handlesList, decryptedResult);

        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);
        signatures[2] = _computeSignature(privateKeySigner3, digest);

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected when no signature is provided.
     */
    function test_verifyInputEIP712KMSSignaturesFailAsExpectedIfNoSignatureProvided() public {
        _setThreeSigners();

        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));

        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](0);

        vm.expectPartialRevert(KMSVerifier.KMSZeroSignature.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }

    /**
     * @dev Tests that the verification of EIP-712 KMS signatures fails as expected
     *      if the number of signatures is less than the defined threshold.
     */
    function test_verifyInputEIP712KMSSignaturesFailAsExpectedIfNumberOfSignaturesIsInferiorToThreshold() public {
        _setThreeSigners();

        vm.prank(owner);
        kmsVerifier.setThreshold(2);
        assertEq(kmsVerifier.getThreshold(), 2);

        /// @dev Mock data for testing purposes.
        bytes32[] memory handlesList = new bytes32[](3);
        handlesList[0] = bytes32(uint256(4));
        handlesList[1] = bytes32(uint256(5));
        handlesList[2] = bytes32(uint256(323));
        bytes memory decryptedResult = abi.encodePacked(keccak256("test"), keccak256("test"), keccak256("test"));
        bytes[] memory signatures = new bytes[](1);

        bytes32 digest = _computeDigest(handlesList, decryptedResult);
        signatures[0] = _computeSignature(privateKeySigner1, digest);

        vm.expectPartialRevert(KMSVerifier.KMSSignatureThresholdNotReached.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, signatures);
    }
}

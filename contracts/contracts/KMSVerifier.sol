// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {EIP712UpgradeableCrossChain} from "./EIP712UpgradeableCrossChain.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title   KMSVerifier.
 * @notice  KMSVerifier (Key Management System Verifier) is a contract that allows the management of signers and provides
 *          signature verification functions.
 * @dev     The contract uses EIP712UpgradeableCrossChain for cryptographic operations and is deployed using an UUPS proxy.
 */
contract KMSVerifier is UUPSUpgradeable, Ownable2StepUpgradeable, EIP712UpgradeableCrossChain {
    /// @notice Returned if the KMS signer to add is already a signer.
    error KMSAlreadySigner();

    /// @notice Returned if the owner tries to remove all the KMS signers.
    error AtLeastOneSignerIsRequired();

    /// @notice Returned if the recovered KMS signer is not a valid KMS signer.
    /// @param invalidSigner Address of the invalid signer.
    error KMSInvalidSigner(address invalidSigner);

    /// @notice Returned if the KMS signer to remove is not a signer.
    error KMSNotASigner();

    /// @notice Returned if the KMS signer to add is the null address.
    error KMSSignerNull();

    /// @notice                 Returned if the number of signatures is inferior to the threshold.
    /// @param numSignatures    Number of signatures.
    error KMSSignatureThresholdNotReached(uint256 numSignatures);

    /// @notice Returned if the number of signatures is equal to 0.
    error KMSZeroSignature();

    /// @notice Returned if the initial signers set is empty.
    error InitialSignersSetIsEmpty();

    /// @notice Returned if the chosen threshold is null.
    error ThresholdIsNull();

    /// @notice Threshold is above number of signers.
    error ThresholdIsAboveNumberOfSigners();

    /// @notice         Emitted when a signer is added.
    /// @param signer   The address of the signer that was added.
    event SignerAdded(address indexed signer);

    /// @notice         Emitted when a signer is removed.
    /// @param signer   The address of the signer that was removed.
    event SignerRemoved(address indexed signer);

    /// @notice         Emitted when a threshold is set.
    /// @param threshold   The new threshold set by the owner.
    event KMSThresholdSet(uint256 threshold);

    /**
     * @param handlesList       List of handles.
     * @param decryptedResult   Decrypted result.
     */
    struct PublicDecryptionResult {
        uint256[] handlesList;
        bytes decryptedResult;
    }

    /**
     * @param aclAddress        ACL address.
     * @param hashOfCiphertext  Hash of ciphertext.
     * @param userAddress       Address of the user.
     * @param contractAddress   Contract address.
     */
    struct CiphertextVerificationForKMS {
        address aclAddress;
        bytes32 hashOfCiphertext;
        address userAddress;
        address contractAddress;
    }

    /// @notice Decryption result type.
    string public constant DECRYPTION_RESULT_TYPE =
        "PublicDecryptionResult(uint256[] handlesList,bytes decryptedResult)";

    /// @notice Decryption result typehash.
    bytes32 public constant DECRYPTION_RESULT_TYPEHASH = keccak256(bytes(DECRYPTION_RESULT_TYPE));

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "KMSVerifier";

    /// @notice Name of the source contract for which original EIP712 was destinated.
    string private constant CONTRACT_NAME_SOURCE = "DecryptionManager";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @custom:storage-location erc7201:fhevm.storage.KMSVerifier
    struct KMSVerifierStorage {
        mapping(address => bool) isSigner; /// @notice Mapping to keep track of addresses that are signers
        address[] signers; /// @notice Array to keep track of all signers
        uint256 threshold; /// @notice The threshold for the number of signers required for a signature to be valid
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.KMSVerifier")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant KMSVerifierStorageLocation =
        0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Re-initializes the contract.
     * @param verifyingContractSource The DecryptionManager contract address from the Gateway chain.
     * @param chainIDSource The chain id of the Gateway chain.
     * @param initialSigners The list of initial KMS signers, should be non-empty and contain unique addresses, otherwise initialization will fail.
     * @param initialThreshold Initial threshold, should be non-null and less or equal to the initialSigners length.
     */
    function reinitialize(
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] calldata initialSigners,
        uint256 initialThreshold
    ) public reinitializer(2) {
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);
        uint256 initialSignersLen = initialSigners.length;
        if (initialSignersLen == 0) {
            revert InitialSignersSetIsEmpty();
        }
        for (uint256 i = 0; i < initialSignersLen; i++) {
            addSigner(initialSigners[i]);
        }
        setThreshold(initialThreshold);
    }

    /**
     * @notice          Adds a new signer.
     * @dev             Only the owner can add a signer.
     * @param signer    The address to be added as a signer.
     */
    function addSigner(address signer) public virtual onlyOwner {
        if (signer == address(0)) {
            revert KMSSignerNull();
        }

        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if ($.isSigner[signer]) {
            revert KMSAlreadySigner();
        }

        $.isSigner[signer] = true;
        $.signers.push(signer);
        emit SignerAdded(signer);
    }

    /**
     * @notice          Sets a threshold (i.e. the minimum number of valid signatures required to accept a transaction).
     * @dev             Only the owner can set a threshold.
     * @param threshold    The threshold to be set. Threshold should be non-null and less than the number of signers.
     */
    function setThreshold(uint256 threshold) public virtual onlyOwner {
        if (threshold == 0) {
            revert ThresholdIsNull();
        }
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if (threshold > $.signers.length) {
            revert ThresholdIsAboveNumberOfSigners();
        }
        $.threshold = threshold;
        emit KMSThresholdSet(threshold);
    }

    /**
     * @notice          Removes an existing signer.
     * @dev             Only the owner can remove a signer.
     * @param signer    The signer address to remove.
     */
    function removeSigner(address signer) public virtual onlyOwner {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if (!$.isSigner[signer]) {
            revert KMSNotASigner();
        }

        /// @dev Remove signer from the mapping.
        $.isSigner[signer] = false;

        /// @dev Find the index of the signer and remove it from the array.
        for (uint i = 0; i < $.signers.length; i++) {
            if ($.signers[i] == signer) {
                $.signers[i] = $.signers[$.signers.length - 1]; /// @dev Move the last element into the place to delete.
                $.signers.pop(); /// @dev Remove the last element.

                if ($.signers.length == 0) {
                    revert AtLeastOneSignerIsRequired();
                }

                emit SignerRemoved(signer);
                return;
            }
        }
    }

    /**
     * @notice                  Verifies multiple signatures for a given handlesList and a given decryptedResult.
     * @dev                     Calls verifySignaturesDigest internally.
     * @param handlesList       The list of handles, which where requested to be decrypted.
     * @param decryptedResult   A bytes array representing the abi-encoding of all requested decrypted values.
     * @param signatures        An array of signatures to verify.
     * @return isVerified       true if enough provided signatures are valid, false otherwise.
     */
    function verifyDecryptionEIP712KMSSignatures(
        uint256[] memory handlesList,
        bytes memory decryptedResult,
        bytes[] memory signatures
    ) public virtual returns (bool) {
        PublicDecryptionResult memory decRes;
        decRes.handlesList = handlesList;
        decRes.decryptedResult = decryptedResult;
        bytes32 digest = _hashDecryptionResult(decRes);
        return _verifySignaturesDigest(digest, signatures);
    }

    /**
     * @notice          Returns the list of KMS signers.
     * @dev             If there are too many signers, it could be out-of-gas.
     * @return signers  List of signers.
     */
    function getSigners() public view virtual returns (address[] memory) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.signers;
    }

    /**
     * @notice              Get the threshold for signature.
     * @return threshold    Threshold for signature verification.
     */
    function getThreshold() public view virtual returns (uint256) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.threshold;
    }

    /**
     * @notice              Returns whether the account address is a valid KMS signer.
     * @param account       Account address.
     * @return isSigner     Whether the account is a valid KMS signer.
     */
    function isSigner(address account) public view virtual returns (bool) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.isSigner[account];
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
     */
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }

    /**
     * @notice          Cleans a hashmap in transient storage.
     * @dev             This is important to keep composability in the context of account abstraction.
     * @param keys      An array of keys to cleanup from transient storage.
     * @param maxIndex  The biggest index to take into account from the array - assumed to be less or equal to keys.length.
     */
    function _cleanTransientHashMap(address[] memory keys, uint256 maxIndex) internal virtual {
        for (uint256 j = 0; j < maxIndex; j++) {
            _tstore(keys[j], 0);
        }
    }

    /**
     * @notice          Writes to transient storage.
     * @dev             Uses inline assembly to access the Transient Storage's _tstore operation.
     * @param location  The address used as key where transient storage of the contract is written at.
     * @param value     An uint256 stored at location key in transient storage of the contract.
     */
    function _tstore(address location, uint256 value) internal virtual {
        assembly {
            tstore(location, value)
        }
    }

    /**
     * @notice              Verifies multiple signatures for a given message at a certain threshold.
     * @dev                 Calls verifySignature internally.
     * @param digest        The hash of the message that was signed by all signers.
     * @param signatures    An array of signatures to verify.
     * @return isVerified   true if enough provided signatures are valid, false otherwise.
     */
    function _verifySignaturesDigest(bytes32 digest, bytes[] memory signatures) internal virtual returns (bool) {
        uint256 numSignatures = signatures.length;

        if (numSignatures == 0) {
            revert KMSZeroSignature();
        }

        uint256 threshold = getThreshold();

        if (numSignatures < threshold) {
            revert KMSSignatureThresholdNotReached(numSignatures);
        }

        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = _recoverSigner(digest, signatures[i]);
            if (!isSigner(signerRecovered)) {
                revert KMSInvalidSigner(signerRecovered);
            }
            if (!_tload(signerRecovered)) {
                recoveredSigners[uniqueValidCount] = signerRecovered;
                uniqueValidCount++;
                _tstore(signerRecovered, 1);
            }
            if (uniqueValidCount >= threshold) {
                _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
                return true;
            }
        }
        _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
        return false;
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /**
     * @notice                  Hashes the decryption result.
     * @param decRes            Decryption result.
     * @return hashTypedData    Hash typed data.
     */
    function _hashDecryptionResult(PublicDecryptionResult memory decRes) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        DECRYPTION_RESULT_TYPEHASH,
                        keccak256(abi.encodePacked(decRes.handlesList)),
                        keccak256(decRes.decryptedResult)
                    )
                )
            );
    }

    /**
     * @notice           Reads transient storage.
     * @dev              Uses inline assembly to access the Transient Storage's tload operation.
     * @param location   The address used as key where transient storage of the contract is read at.
     * @return value     true if value stored at the given location is non-null, false otherwise.
     */
    function _tload(address location) internal view virtual returns (bool value) {
        assembly {
            value := tload(location)
        }
    }

    /**
     * @dev Returns the KMSVerifier storage location.
     */
    function _getKMSVerifierStorage() internal pure returns (KMSVerifierStorage storage $) {
        assembly {
            $.slot := KMSVerifierStorageLocation
        }
    }

    /**
     * @notice          Recovers the signer's address from a `signature` and a `message` digest.
     * @dev             It utilizes ECDSA for actual address recovery. It does not support contract signature (EIP-1271).
     * @param message   The hash of the message that was signed.
     * @param signature The signature to verify.
     * @return signer   The address that supposedly signed the message.
     */
    function _recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }
}

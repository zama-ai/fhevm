// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Importing OpenZeppelin contracts for cryptographic signature verification and access control.
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/// @title KMS Verifier for signature verification and signers management
/// @author The developer
/// @notice This contract allows for the management of signers and provides methods to verify signatures
/// @dev The contract uses OpenZeppelin's EIP712Upgradeable for cryptographic operations
contract KMSVerifier is UUPSUpgradeable, Ownable2StepUpgradeable, EIP712Upgradeable {
    struct DecryptionResult {
        address aclAddress;
        uint256[] handlesList;
        bytes decryptedResult;
    }

    struct CiphertextVerificationForKMS {
        address aclAddress;
        bytes32 hashOfCiphertext;
        address userAddress;
        address contractAddress;
    }

    string private constant DECRYPTIONRESULT_TYPE =
        "DecryptionResult(address aclAddress,uint256[] handlesList,bytes decryptedResult)";
    bytes32 private constant DECRYPTIONRESULT_TYPE_HASH = keccak256(bytes(DECRYPTIONRESULT_TYPE));

    string public constant CIPHERTEXTVERIFICATION_KMS_TYPE =
        "CiphertextVerificationForKMS(address aclAddress,bytes32 hashOfCiphertext,address userAddress,address contractAddress)";
    bytes32 private constant CIPHERTEXTVERIFICATION_KMS_TYPE_HASH = keccak256(bytes(CIPHERTEXTVERIFICATION_KMS_TYPE));

    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "KMSVerifier";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @custom:storage-location erc7201:fhevm.storage.KMSVerifier
    struct KMSVerifierStorage {
        mapping(address => bool) isSigner; /// @notice Mapping to keep track of addresses that are signers
        address[] signers; /// @notice Array to keep track of all signers
        uint256 threshold; /// @notice The threshold for the number of signers required for a signature to be valid
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm.storage.KMSVerifier")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant KMSVerifierStorageLocation =
        0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900;

    function _getKMSVerifierStorage() internal pure returns (KMSVerifierStorage storage $) {
        assembly {
            $.slot := KMSVerifierStorageLocation
        }
    }

    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    function isSigner(address account) public virtual returns (bool) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.isSigner[account];
    }

    function getSigners() public view virtual returns (address[] memory) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.signers;
    }

    function getThreshold() public view virtual returns (uint256) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.threshold;
    }

    function get_DECRYPTIONRESULT_TYPE() public view virtual returns (string memory) {
        return DECRYPTIONRESULT_TYPE;
    }

    function get_CIPHERTEXTVERIFICATION_KMS_TYPE() public view virtual returns (string memory) {
        return CIPHERTEXTVERIFICATION_KMS_TYPE;
    }

    /// @notice Emitted when a signer is added
    /// @param signer The address of the signer that was added
    event SignerAdded(address indexed signer);

    /// @notice Emitted when a signer is removed
    /// @param signer The address of the signer that was removed
    event SignerRemoved(address indexed signer);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract setting `initialOwner` as the initial owner
    function initialize(address initialOwner) external initializer {
        __Ownable_init(initialOwner);
        __EIP712_init(CONTRACT_NAME, "1");
    }

    /// @notice Sets the threshold for the number of signers required for a signature to be valid
    function applyThreshold() internal virtual {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        $.threshold = ($.signers.length - 1) / 3 + 1;
    }

    /// @notice Adds a new signer
    /// @dev Only the owner can add a signer
    /// @param signer The address to be added as a signer
    function addSigner(address signer) public virtual onlyOwner {
        require(signer != address(0), "KMSVerifier: Address is null");
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        require(!$.isSigner[signer], "KMSVerifier: Address is already a signer");
        $.isSigner[signer] = true;
        $.signers.push(signer);
        applyThreshold();
        emit SignerAdded(signer);
    }

    function hashDecryptionResult(DecryptionResult memory decRes) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        DECRYPTIONRESULT_TYPE_HASH,
                        decRes.aclAddress,
                        keccak256(abi.encodePacked(decRes.handlesList)),
                        keccak256(decRes.decryptedResult)
                    )
                )
            );
    }

    function hashCiphertextVerificationForKMS(
        CiphertextVerificationForKMS memory CVkms
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        CIPHERTEXTVERIFICATION_KMS_TYPE_HASH,
                        CVkms.aclAddress,
                        CVkms.hashOfCiphertext,
                        CVkms.userAddress,
                        CVkms.contractAddress
                    )
                )
            );
    }

    /// @notice Removes an existing signer
    /// @dev Only the owner can remove a signer
    /// @param signer The address to be removed from signers
    function removeSigner(address signer) public virtual onlyOwner {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        require($.isSigner[signer], "KMSVerifier: Address is not a signer");

        // Remove signer from the mapping
        $.isSigner[signer] = false;

        // Find the index of the signer and remove it from the array
        for (uint i = 0; i < $.signers.length; i++) {
            if ($.signers[i] == signer) {
                $.signers[i] = $.signers[$.signers.length - 1]; // Move the last element into the place to delete
                $.signers.pop(); // Remove the last element
                applyThreshold();
                emit SignerRemoved(signer);
                return;
            }
        }
    }

    /// @notice recovers the signer's address from a `signature` and a `message` digest
    /// @dev Utilizes ECDSA for actual address recovery
    /// @param message The hash of the message that was signed
    /// @param signature The signature to verify
    /// @return signer The address that supposedly signed the message
    function recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }

    /// @notice Verifies multiple signatures for a given handlesList and a given decryptedResult
    /// @dev Calls verifySignaturesDigest internally;
    /// @param handlesList The list of handles which where requested to be decrypted
    /// @param decryptedResult A bytes array representing the abi-encoding of all requested decrypted values
    /// @param signatures An array of signatures to verify
    /// @return true if enough provided signatures are valid, false otherwise
    function verifyDecryptionEIP712KMSSignatures(
        address aclAddress,
        uint256[] memory handlesList,
        bytes memory decryptedResult,
        bytes[] memory signatures
    ) public virtual returns (bool) {
        DecryptionResult memory decRes;
        decRes.aclAddress = aclAddress;
        decRes.handlesList = handlesList;
        decRes.decryptedResult = decryptedResult;
        bytes32 digest = hashDecryptionResult(decRes);
        return verifySignaturesDigest(digest, signatures);
    }

    /// @notice Verifies multiple signatures for a given CiphertextVerificationForKMS (user inputs)
    /// @dev Calls verifySignaturesDigest internally;
    /// @param cv The CiphertextVerificationForKMS struct for encrypted user inputs
    /// @param signatures An array of signatures to verify
    /// @return true if enough provided signatures are valid, false otherwise
    function verifyInputEIP712KMSSignatures(
        CiphertextVerificationForKMS memory cv,
        bytes[] memory signatures
    ) public virtual returns (bool) {
        bytes32 digest = hashCiphertextVerificationForKMS(cv);
        return verifySignaturesDigest(digest, signatures);
    }

    /// @notice Verifies multiple signatures for a given message at a certain threshold
    /// @dev Calls verifySignature internally;
    /// @param digest The hash of the message that was signed by all signers
    /// @param signatures An array of signatures to verify
    /// @return true if enough provided signatures are valid, false otherwise
    function verifySignaturesDigest(bytes32 digest, bytes[] memory signatures) internal virtual returns (bool) {
        uint256 numSignatures = signatures.length;
        require(numSignatures > 0, "KmsVerifier: no signatures provided");
        uint256 threshold = getThreshold();
        require(numSignatures >= threshold, "KmsVerifier: at least threshold number of signatures required");
        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = recoverSigner(digest, signatures[i]);
            require(isSigner(signerRecovered), "KmsVerifier: Invalid KMS signer");
            if (!tload(signerRecovered)) {
                recoveredSigners[uniqueValidCount] = signerRecovered;
                uniqueValidCount++;
                tstore(signerRecovered, 1);
            }
            if (uniqueValidCount >= threshold) {
                cleanTransientStorage(recoveredSigners, uniqueValidCount);
                return true;
            }
        }
        cleanTransientStorage(recoveredSigners, uniqueValidCount);
        return false;
    }

    /// @notice Writes to transient storage
    /// @dev Uses inline assembly to access the Transient Storage's tstore operation.
    /// @param location The address used as key where transient storage of the contract is written at
    /// @param value An uint256 stored at location key in transient storage of the contract
    function tstore(address location, uint256 value) internal virtual {
        assembly {
            tstore(location, value)
        }
    }

    /// @notice Reads transient storage
    /// @dev Uses inline assembly to access the Transient Storage's tload operation.
    /// @param location The address used as key where transient storage of the contract is read at
    /// @return value true if value stored at the given location is non-null, false otherwise.
    function tload(address location) internal view virtual returns (bool value) {
        assembly {
            value := tload(location)
        }
    }

    /// @notice Cleans transient storage
    /// @dev Important to keep composability in the context of account abstraction
    /// @param keys An array of keys to cleanup from transient storage
    /// @param maxIndex The biggest index to take into account from the array - assumed to be less or equal to keys.length
    function cleanTransientStorage(address[] memory keys, uint256 maxIndex) internal virtual {
        for (uint256 j = 0; j < maxIndex; j++) {
            /// @note : clearing transient storage for composability
            tstore(keys[j], 0);
        }
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
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
}

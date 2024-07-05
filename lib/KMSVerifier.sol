// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Importing OpenZeppelin contracts for cryptographic signature verification and access control.
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/// @title KMS Verifier for signature verification and verifier management
/// @author The developer
/// @notice This contract allows for the management of verifiers and provides methods to verify signatures
/// @dev The contract uses OpenZeppelin's SignatureChecker for cryptographic operations
contract KMSVerifier is Ownable2Step, EIP712 {
    struct DecryptionResult {
        uint256[] handlesList;
        bytes decryptedResult;
    }

    string public constant DECRYPTIONRESULT_TYPE = "DecryptionResult(uint256[] handlesList,bytes decryptedResult)";
    bytes32 private constant DECRYPTIONRESULT_TYPE_HASH = keccak256(bytes(DECRYPTIONRESULT_TYPE));

    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "KMSVerifier";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice Emitted when a verifier is added
    /// @param verifier The address of the verifier that was added
    event VerifierAdded(address indexed verifier);

    /// @notice Emitted when a verifier is removed
    /// @param verifier The address of the verifier that was removed
    event VerifierRemoved(address indexed verifier);

    /// @notice Mapping to keep track of addresses that are verifiers
    mapping(address => bool) public isVerifier;

    /// @notice Array to keep track of all verifiers
    address[] public verifiers;

    /// @notice The threshold for the number of verifiers required for a signature to be valid
    uint256 public threshold;

    /// @notice Initializes the contract setting the deployer as the initial owner
    constructor() Ownable(msg.sender) EIP712(CONTRACT_NAME, "1") {}

    /// @notice Sets the threshold for the number of verifiers required for a signature to be valid
    function applyThreshold() internal {
        threshold = (verifiers.length - 1) / 3 + 1;
    }

    /// @notice Adds a new verifier
    /// @dev Only the owner can add a verifier
    /// @param verifier The address to be added as a verifier
    function addVerifier(address verifier) public onlyOwner {
        require(verifier != address(0), "KMSVerifier: Address is null");
        require(!isVerifier[verifier], "KMSVerifier: Address is already a verifier");
        isVerifier[verifier] = true;
        verifiers.push(verifier);
        applyThreshold();
        emit VerifierAdded(verifier);
    }

    function hashDecryptionResult(DecryptionResult memory decRes) internal view returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        DECRYPTIONRESULT_TYPE_HASH,
                        keccak256(abi.encodePacked(decRes.handlesList)),
                        keccak256(decRes.decryptedResult)
                    )
                )
            );
    }

    /// @notice Removes an existing verifier
    /// @dev Only the owner can remove a verifier
    /// @param verifier The address to be removed from verifiers
    function removeVerifier(address verifier) public onlyOwner {
        require(isVerifier[verifier], "KMSVerifier: Address is not a verifier");

        // Remove verifier from the mapping
        isVerifier[verifier] = false;

        // Find the index of the verifier and remove it from the array
        for (uint i = 0; i < verifiers.length; i++) {
            if (verifiers[i] == verifier) {
                verifiers[i] = verifiers[verifiers.length - 1]; // Move the last element into the place to delete
                verifiers.pop(); // Remove the last element
                applyThreshold();
                emit VerifierRemoved(verifier);
                return;
            }
        }
    }

    /// @notice recovers the verifier's address from a `signature` and a `message` digest
    /// @dev Utilizes ECDSA for actual address recovery
    /// @param message The hash of the message that was signed
    /// @param signature The signature to verify
    /// @return signer The address that supposedly signed the message
    function recoverSigner(bytes32 message, bytes memory signature) internal pure returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }

    /// @notice Verifies multiple signatures for a given handlesList and a given decryptedResult
    /// @dev Calls verifySignaturesDigest internally;
    /// @param handlesList The list of handles which where requested to be decrypted
    /// @param decryptedResult A bytes array representing the abi-encoding of all requested decrypted values
    /// @param signatures An array of signatures to verify
    /// @return true if enough provided signatures are valid, false otherwise
    function verifySignatures(
        uint256[] memory handlesList,
        bytes memory decryptedResult,
        bytes[] memory signatures
    ) public returns (bool) {
        DecryptionResult memory decRes;
        decRes.handlesList = handlesList;
        decRes.decryptedResult = decryptedResult;
        bytes32 message = hashDecryptionResult(decRes);
        return verifySignaturesDigest(message, signatures);
    }

    /// @notice Verifies multiple signatures for a given message at a certain threshold
    /// @dev Calls verifySignature internally;
    /// @param message The hash of the message that was signed by all signers
    /// @param signatures An array of signatures to verify
    /// @return true if enough provided signatures are valid, false otherwise
    function verifySignaturesDigest(bytes32 message, bytes[] memory signatures) internal returns (bool) {
        uint256 numSignatures = signatures.length;
        require(numSignatures > 0, "KmsVerifier: no signatures provided");
        require(numSignatures >= threshold, "KmsVerifier: at least threshold number of signatures required");
        address[] memory recoveredVerifiers = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = recoverSigner(message, signatures[i]);
            if (isVerifier[signerRecovered]) {
                if (!tload(signerRecovered)) {
                    recoveredVerifiers[uniqueValidCount] = signerRecovered;
                    uniqueValidCount++;
                    tstore(signerRecovered, 1);
                }
            }
            if (uniqueValidCount >= threshold) {
                for (uint256 j = 0; i < uniqueValidCount; i++) {
                    /// @note : clearing transient storage for composability
                    tstore(recoveredVerifiers[j], 0);
                }
                return true;
            }
        }
        return false;
    }

    /// @notice Writes to transient storage
    /// @dev Uses inline assembly to access the Transient Storage's tstore operation.
    /// @param location The address used as key where transient storage of the contract is written at
    /// @param value An uint256 stored at location key in transient storage of the contract
    function tstore(address location, uint256 value) private {
        assembly {
            tstore(location, value)
        }
    }

    /// @notice Reads transient storage
    /// @dev Uses inline assembly to access the Transient Storage's tload operation.
    /// @param location The address used as key where transient storage of the contract is read at
    /// @return value true if value stored at the given location is non-null, false otherwise.
    function tload(address location) private view returns (bool value) {
        assembly {
            value := tload(location)
        }
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
    function getVersion() external pure returns (string memory) {
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

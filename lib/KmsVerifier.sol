// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.25;

// Importing OpenZeppelin contracts for cryptographic signature verification and access control.
import {SignatureChecker} from "@openzeppelin/contracts/utils/cryptography/SignatureChecker.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

/// @title KMS Verifier for signature verification and verifier management
/// @author The developer
/// @notice This contract allows for the management of verifiers and provides methods to verify signatures
/// @dev The contract uses OpenZeppelin's SignatureChecker for cryptographic operations
contract KmsVerifier is Ownable2Step {
    /// @notice Version of the contract
    string public VERSION = "1.0.0";

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
    constructor() Ownable(msg.sender) {}

    /// @notice Sets the threshold for the number of verifiers required for a signature to be valid
    function applyThreshold() internal {
        threshold = verifiers.length / 2 + 1;
    }

    /// @notice Adds a new verifier
    /// @dev Only the owner can add a verifier
    /// @param verifier The address to be added as a verifier
    function addVerifier(address verifier) public onlyOwner {
        require(!isVerifier[verifier], "KmsVerifier: address is already a verifier");
        isVerifier[verifier] = true;
        verifiers.push(verifier);
        applyThreshold();
        emit VerifierAdded(verifier);
    }

    /// @notice Removes an existing verifier
    /// @dev Only the owner can remove a verifier
    /// @param verifier The address to be removed from verifiers
    function removeVerifier(address verifier) public onlyOwner {
        require(isVerifier[verifier], "Address is not a verifier");

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

    /// @notice Verifies if a signature is valid for a given message and signer
    /// @dev Utilizes SignatureChecker for actual signature verification
    /// @param message The hash of the message that was signed
    /// @param signature The signature to verify
    /// @param signer The address that supposedly signed the message
    /// @return isValid True if the signature is valid, false otherwise
    function verifySignature(
        bytes32 message,
        bytes memory signature,
        address signer
    ) public view returns (bool isValid) {
        return SignatureChecker.isValidSignatureNow(signer, message, signature);
    }

    /// @notice Verifies multiple signatures for a given message at a certain threshold
    /// @dev Calls verifySignature internally; ensures signatures array and signers array are of the same length
    /// @param message The hash of the message that was signed by all signers
    /// @param signatures An array of signatures to verify
    /// @return allValid True if all provided signatures are valid, false if any one is invalid
    function verifySignatures(bytes32 message, bytes[] memory signatures) public view returns (bool allValid) {
        require(signatures.length > 0, "KmsVerifier: no signatures provided");
        require(signatures.length >= threshold, "KmsVerifier: at least threshold number of signatures required");
        uint256 validCount = 0;
        for (uint256 i = 0; i < signatures.length; i++) {
            if (verifySignature(message, signatures[i], verifiers[i])) {
                validCount++;
                if (validCount >= threshold) {
                    return true;
                }
            }
        }
        return false;
    }
}

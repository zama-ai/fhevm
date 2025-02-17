// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import { IDecryptionManager } from "./interfaces/IDecryptionManager.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./interfaces/IHTTPZ.sol";
import "./interfaces/IACLManager.sol";

/// @title DecryptionManager contract
/// @dev See {IDecryptionManager}.
contract DecryptionManager is Ownable2Step, EIP712, IDecryptionManager {
    /// @notice The typed data structure for the EIP712 signature to validate in public decryption responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_PUBLIC_DECRYPT_TYPE is, but we keep it the same for clarity.
    struct EIP712PublicDecrypt {
        /// @notice The handles of the ciphertexts that have been decrypted.
        uint256[] ctHandles;
        /// @notice The decrypted result of the public decryption.
        bytes decryptedResult;
    }

    /// @notice The address of the HTTPZ contract for checking if a signer is valid
    IHTTPZ internal immutable _HTTPZ;

    /// @notice The address of the ACLManager contract for getting the 128-PBS ciphertexts, if they are
    /// @notice allowed for decryption
    IACLManager internal immutable _ACL_MANAGER;

    // TODO: Use a reference to the PaymentManager contract
    /// @notice The address of the Payment Manager contract for service fees, burn and distribution
    address internal immutable _PAYMENT_MANAGER;

    /// @notice Handles of the ciphertexts requested for a public decryption
    mapping(uint256 publicDecryptionId => uint256[] ctHandles) internal publicCtHandles;

    /// @notice Whether a public decryption has been done
    mapping(uint256 publicDecryptionId => bool publicDecryptionDone) internal publicDecryptionDone;

    /// @notice Whether a public decryption has been signed (and verified)by a signer
    /// @dev There is an edge case which is hard to deal with if we change the signers set in
    /// @dev the middle of pending signatures being sent, but maybe we can ignore it for the moment
    mapping(uint256 publicDecryptionId => mapping(address kmsSigner => bool alreadySigned))
        internal alreadySignedPublic;

    /// @notice Pending verified signatures for a public decryption
    mapping(uint256 publicDecryptionId => mapping(bytes32 digest => bytes[] verifiedSignatures))
        internal verifiedSignaturesPublic;

    /// @notice The number of public decryptions requested, used to generate the publicDecryptionIds
    uint256 internal counterPublicDecryption;

    /// @notice The definition of the EIP712PublicDecrypt structure typed data.
    string public constant EIP712_PUBLIC_DECRYPT_TYPE =
        "EIP712PublicDecrypt(uint256[] ctHandles,bytes decryptedResult)";

    /// @notice The hash of the EIP712PublicDecrypt structure typed data definition used for signature validation.
    bytes32 public constant EIP712_PUBLIC_DECRYPT_TYPE_HASH = keccak256(bytes(EIP712_PUBLIC_DECRYPT_TYPE));

    string private constant CONTRACT_NAME = "DecryptionManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Contract name and version for EIP712 signature validation are defined here
    constructor(
        IHTTPZ httpz,
        address paymentManager,
        IACLManager aclManager
    ) Ownable(msg.sender) EIP712(CONTRACT_NAME, "1") {
        _HTTPZ = httpz;
        _ACL_MANAGER = aclManager;
        _PAYMENT_MANAGER = paymentManager;
    }

    /// @dev See {IDecryptionManager-publicDecryptionRequest}.
    function publicDecryptionRequest(uint256[] calldata ctHandles) public virtual {
        counterPublicDecryption++;
        uint256 publicDecryptionId = counterPublicDecryption;

        /// @dev The handles are used during response calls for the EIP712 signature validation.
        publicCtHandles[publicDecryptionId] = ctHandles;

        // TODO: Get the 128-PBS ciphertexts from the ACLManager contract instead
        // bytes[] memory ciphertext128s = _ACL_MANAGER.getCiphertexts(ctHandles);
        IACLManager.CtHandleCiphertext128Pair[]
            memory ctHandleCiphertext128Pairs = new IACLManager.CtHandleCiphertext128Pair[](ctHandles.length);

        // TODO: Implement sending service fees to PaymentManager contract

        emit PublicDecryptionRequest(publicDecryptionId, ctHandleCiphertext128Pairs);
    }

    /// @dev See {IDecryptionManager-publicDecryptionResponse}.
    function publicDecryptionResponse(
        uint256 publicDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) public virtual {
        /// @dev Initialize the EIP712PublicDecrypt structure for the signature validation.
        EIP712PublicDecrypt memory eip712PublicDecrypt = EIP712PublicDecrypt(
            publicCtHandles[publicDecryptionId],
            decryptedResult
        );

        /// @dev Compute the digest of the EIP712PublicDecrypt structure.
        bytes32 digest = _hashEIP712PublicDecrypt(eip712PublicDecrypt);

        /// @dev Recover the signer address from the signature and validate that is a KMS node that
        /// @dev has not already signed.
        _validateEIP712Signature(publicDecryptionId, digest, signature);

        verifiedSignaturesPublic[publicDecryptionId][digest].push(signature);

        bytes[] memory verifiedSignaturesArray = verifiedSignaturesPublic[publicDecryptionId][digest];

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!publicDecryptionDone[publicDecryptionId] && _isConsensusReachedPublic(verifiedSignaturesArray.length)) {
            publicDecryptionDone[publicDecryptionId] = true;

            // TODO: Implement sending service fees to PaymentManager contract

            emit PublicDecryptionResponse(publicDecryptionId, decryptedResult, verifiedSignaturesArray);
        }
    }

    /// @dev See {IDecryptionManager-userDecryptionRequest}.
    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        address userAddress,
        bytes calldata publicKey,
        uint256 eip712ChainId,
        address[] calldata eip712Contracts,
        bytes calldata eip712Signature
    ) public virtual {
        // TODO: Implement the logic for the user decryption request
        emit UserDecryptionRequest(0, ctHandleContractPairs, userAddress);
    }

    /// @dev See {IDecryptionManager-userDecryptionResponse}.
    function userDecryptionResponse(
        uint256 userDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) public virtual {
        // TODO: Implement the logic for the user decryption response
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = signature;
        emit UserDecryptionResponse(userDecryptionId, decryptedResult, signatures);
    }

    /// @dev See {IDecryptionManager-isPublicDecryptionDone}.
    function isPublicDecryptionDone(uint256 publicDecryptionId) public view virtual returns (bool) {
        return publicDecryptionDone[publicDecryptionId];
    }

    function getVersion() public pure virtual returns (string memory) {
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

    /// @notice Validates the EIP712 signature for a given public decryption
    /// @dev This function checks that the signer address is a KMS Connector.
    /// @dev It also checks that the signer has not already signed the public decryption.
    /// @param publicDecryptionId The ID of the public decryption
    /// @param digest The hash of the EIP712ResponsePublicDecrypt structure
    /// @param signature The signature to be validated
    function _validateEIP712Signature(
        uint256 publicDecryptionId,
        bytes32 digest,
        bytes calldata signature
    ) internal virtual {
        address signer = ECDSA.recover(digest, signature);

        if (!_isKmsNode(signer)) {
            revert InvalidKmsSigner(signer);
        }

        if (alreadySignedPublic[publicDecryptionId][signer]) {
            revert KmsSignerAlreadySigned(publicDecryptionId, signer);
        }

        alreadySignedPublic[publicDecryptionId][signer] = true;
    }

    /// @notice Computes the hash of a given EIP712PublicDecrypt structured data
    /// @param eip712PublicDecrypt The EIP712PublicDecrypt structure
    /// @return The hash of the EIP712PublicDecrypt structure
    function _hashEIP712PublicDecrypt(
        EIP712PublicDecrypt memory eip712PublicDecrypt
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_PUBLIC_DECRYPT_TYPE_HASH,
                        keccak256(abi.encodePacked(eip712PublicDecrypt.ctHandles)),
                        keccak256(eip712PublicDecrypt.decryptedResult)
                    )
                )
            );
    }

    /// @notice Indicates if a given address is a valid KMS node.
    /// @dev This function calls the HTTPZ contract to check if the address is a KMS node.
    /// @param signer The address to be checked
    /// @return Whether the address is a valid KMS node
    function _isKmsNode(address signer) internal view virtual returns (bool) {
        return _HTTPZ.isKmsNode(signer);
    }

    /// @notice Checks if the consensus for public decryption is reached among the KMS signers.
    /// @dev This function calls the HTTPZ contract to retrieve the current number of KMS nodes.
    /// @dev The consensus threshold is based on the fact that KMS nodes are expected to return the same
    /// @dev result message.
    /// @param verifiedSignaturesCount The number of signatures that have been verified for a public decryption
    /// @return Whether the consensus for public decryption is reached
    function _isConsensusReachedPublic(uint256 verifiedSignaturesCount) internal view virtual returns (bool) {
        // TODO: Change this to use threshold value instead:
        // https://github.com/zama-ai/gateway-l2/issues/63
        uint256 consensusThreshold = (_HTTPZ.getKmsNodesCount() - 1) / 3 + 1;
        return verifiedSignaturesCount >= consensusThreshold;
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IZKPoKManager.sol";
import "./interfaces/IHTTPZ.sol";

/// @title ZKPoKManager smart contract
/// @dev See {IZKPoKManager}
contract ZKPoKManager is IZKPoKManager, EIP712 {
    /// @notice The typed data structure for the EIP712 signature to validate in ZK Proof verification responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_ZKPOK_TYPE is, but we keep it the same for clarity.
    struct EIP712ZKPoK {
        /// @notice The Coprocessor's computed handles.
        bytes32[] handles;
        /// @notice The address of the user that has provided the input in the ZK Proof verification request.
        address userAddress;
        /// @notice The address of the dapp requiring the ZK Proof verification.
        address contractAddress;
        /// @notice The chainId of the contract requiring the ZK Proof verification.
        uint256 contractChainId;
    }

    /// @notice The stored structure for the received ZK Proof verification request inputs.
    struct ZKProofInput {
        /// @notice The chain ID of the contract address.
        uint256 contractChainId;
        /// @notice The contract address that verification is requested for.
        address contractAddress;
        /// @notice The user address that requested the verification.
        address userAddress;
    }

    /// @notice The address of the HTTPZ contract for protocol state calls.
    IHTTPZ internal immutable _HTTPZ;

    /// @notice The address of the Payment Manager contract for service fees, burn and distribution.
    address internal immutable _PAYMENT_MANAGER;

    /// @notice The counter used for the ZK Proof IDs returned in verification request events.
    uint256 internal zkProofIdCounter;

    /// @notice The mapping of ZK Proof IDs to their verification status.
    mapping(uint256 zkProofId => bool isVerified) internal verifiedZKProofs;

    /// @notice The mapping of ZK Proof IDs to their validated signatures.
    mapping(uint256 zkProofId => mapping(bytes32 digest => bytes[] signatures)) internal zkProofSignatures;

    /// @notice The mapping of ZK Proof IDs to their signers and their signing status.
    mapping(uint256 zkProofId => mapping(address signer => bool hasSigned)) internal zkProofSigners;

    /// @notice The mapping of ZK Proof IDs to their inputs received on verification requests.
    mapping(uint256 zkProofId => ZKProofInput zkProofInput) internal _zkProofInputs;

    /// @notice The definition of the EIP712ZKPoK structure typed data.
    string private constant EIP712_ZKPOK_TYPE =
        "EIP712ZKPoK(bytes32[] handles,address userAddress,address contractAddress,uint256 contractChainId)";

    /// @notice The hash of the EIP712ZKPoK structure typed data definition used for signature validation.
    bytes32 private constant EIP712_ZKPOK_TYPE_HASH = keccak256(bytes(EIP712_ZKPOK_TYPE));

    string private constant CONTRACT_NAME = "ZKPoKManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor(IHTTPZ httpz, address paymentManager) EIP712(CONTRACT_NAME, "1") {
        _HTTPZ = httpz;
        _PAYMENT_MANAGER = paymentManager;
    }

    /// @dev See {IZKPoKManager-verifyProofRequest}.
    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) public virtual {
        _HTTPZ.checkNetworkIsRegistered(contractChainId);

        // TODO(#52): Implement sending service fees to PaymentManager contract

        uint256 zkProofId = zkProofIdCounter++;

        /// @dev The following stored inputs are used during response calls for the EIP712 signature validation.
        _zkProofInputs[zkProofId] = ZKProofInput(contractChainId, contractAddress, userAddress);

        emit VerifyProofRequest(zkProofId, contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    }

    /// @dev See {IZKPoKManager-verifyProofResponse}.
    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata handles,
        bytes calldata signature
    ) public virtual {
        /// @dev Retrieve stored ZK Proof verification request inputs.
        ZKProofInput memory zkProofInput = _zkProofInputs[zkProofId];

        /// @dev Initialize the EIP712ZKPoK structure for the signature validation.
        EIP712ZKPoK memory eip712ZKPoK = EIP712ZKPoK(
            handles,
            zkProofInput.userAddress,
            zkProofInput.contractAddress,
            zkProofInput.contractChainId
        );

        /// @dev Compute the digest of the EIP712ZKPoK structure.
        bytes32 digest = _hashEIP712ZKPoK(eip712ZKPoK);

        /// @dev Recover the signer address from the signature and validate that is a Coprocessor.
        _validateEIP712Signature(zkProofId, digest, signature);

        bytes[] storage currentSignatures = zkProofSignatures[zkProofId][digest];
        currentSignatures.push(signature);

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!isProofVerified(zkProofId) && _isConsensusReached(currentSignatures.length)) {
            // TODO(#52): Implement calling PaymentManager contract to burn and distribute fees
            verifiedZKProofs[zkProofId] = true;

            emit VerifyProofResponse(zkProofId, handles, currentSignatures);
        }
    }

    /// @dev See {IZKPoKManager-isProofVerified}.
    function isProofVerified(uint256 zkProofId) public view virtual returns (bool) {
        return verifiedZKProofs[zkProofId];
    }

    /// @notice Returns the versions of the ZKPoKManager contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
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

    /// @notice Validates the EIP712 signature for a given ZK Proof
    /// @dev This function calls the HTTPZ contract to check that the signer address is a Coprocessor.
    /// @dev It also checks that the signer has not already signed the ZK Proof.
    /// @param zkProofId The ID of the ZK Proof
    /// @param digest The hash of the EIP712ZKPoK structure
    /// @param signature The signature to be validated
    function _validateEIP712Signature(uint256 zkProofId, bytes32 digest, bytes calldata signature) internal virtual {
        address signer = ECDSA.recover(digest, signature);

        _HTTPZ.checkIsCoprocessor(signer);

        if (zkProofSigners[zkProofId][signer]) {
            revert CoprocessorHasAlreadySigned(zkProofId, signer);
        }

        zkProofSigners[zkProofId][signer] = true;
    }

    /// @notice Computes the hash of a given EIP712ZKPoK structured data
    /// @param ctVerification The EIP712ZKPoK structure
    /// @return The hash of the EIP712ZKPoK structure
    function _hashEIP712ZKPoK(EIP712ZKPoK memory ctVerification) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_ZKPOK_TYPE_HASH,
                        keccak256(abi.encodePacked(ctVerification.handles)),
                        ctVerification.userAddress,
                        ctVerification.contractAddress,
                        ctVerification.contractChainId
                    )
                )
            );
    }

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @dev This function calls the HTTPZ contract to retrieve the consensus threshold.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }
}

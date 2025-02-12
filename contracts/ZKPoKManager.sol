// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IZKPoKManager.sol";
import "./HTTPZ.sol";

/// @title ZKPoKManager smart contract
/// @dev See {IZKPoKManager}
contract ZKPoKManager is IZKPoKManager, EIP712 {
    /// @notice The typed data structure for the EIP712 signature to validate in ZK Proof verification responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev ZK_PROOF_VERIFICATION_RESULT_TYPE is, but we keep it the same for clarity.
    struct EIP712ZKPoK {
        /// @notice The Coprocessor's computed handles.
        bytes32[] ctHandles;
        /// @notice The address of the user that has provided the input in the ZK Proof verification request.
        address userAddress;
        /// @notice The address of the dapp requiring the ZK Proof verification.
        address contractAddress;
        /// @notice The chainId of the contract requiring the ZK Proof verification.
        uint256 contractChainId;
    }

    /// @notice The address of the HTTPZ contract for protocol state calls.
    address internal immutable _HTTPZ;

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

    /// @notice The mapping of ZK Proof IDs to their user address that requested the verification.
    mapping(uint256 zkProofId => address userAddress) internal zkProofUserAddresses;

    /// @notice The mapping of ZK Proof IDs to their contract address requested by the user.
    mapping(uint256 zkProofId => address contractAddress) internal zkProofContractAddresses;

    /// @notice The mapping of ZK Proof IDs to their contract chain ID requested by the user.
    mapping(uint256 zkProofId => uint256 contractChainId) internal zkProofContractChainIds;

    /// @notice The definition of the EIP712ZKPoK structure typed data.
    string private constant EIP712_ZKPOK_TYPE =
        "EIP712ZKPoK(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId)";

    /// @notice The hash of the EIP712ZKPoK structure typed data definition used for signature validation.
    bytes32 private constant EIP712_ZKPOK_TYPE_HASH = keccak256(bytes(EIP712_ZKPOK_TYPE));

    string private constant CONTRACT_NAME = "ZKPoKManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor(address httpz, address paymentManager) EIP712(CONTRACT_NAME, "1") {
        _HTTPZ = httpz;
        _PAYMENT_MANAGER = paymentManager;
    }

    /// @dev See {IZKPoKManager-verifyProofRequest}.
    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextProof
    ) public virtual {
        // TODO: Enable the following HTTPZ contract call
        // bool isNetworkRegistered = HTTPZ.isNetwork(contractChainId);
        bool isNetworkRegistered = true;
        if (!isNetworkRegistered) {
            revert NetworkNotRegistered();
        }

        // TODO(#52): Implement sending service fees to PaymentManager contract

        uint256 zkProofId = zkProofIdCounter++;

        /// @dev The following stored inputs are used during response calls for the EIP712 signature validation.
        zkProofUserAddresses[zkProofId] = userAddress;
        zkProofContractAddresses[zkProofId] = contractAddress;
        zkProofContractChainIds[zkProofId] = contractChainId;

        emit VerifyProofRequest(zkProofId, contractChainId, contractAddress, userAddress, ciphertextProof);
    }

    /// @dev See {IZKPoKManager-verifyProofResponse}.
    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata handles,
        bytes calldata signature
    ) public virtual {
        /// @dev Initialize the EIP712ZKPoK structure for the signature validation.
        EIP712ZKPoK memory eip712ZKPoK = EIP712ZKPoK(
            handles,
            zkProofUserAddresses[zkProofId],
            zkProofContractAddresses[zkProofId],
            zkProofContractChainIds[zkProofId]
        );

        /// @dev Compute the digest of the EIP712ZKPoK structure.
        bytes32 digest = _hashEIP712ZKPoK(eip712ZKPoK);

        /// @dev Recover the signer address from the signature and validate that is a Coprocessor.
        _validateEIP712Signature(zkProofId, digest, signature);

        zkProofSignatures[zkProofId][digest].push(signature);

        bytes[] memory currentSignatures = zkProofSignatures[zkProofId][digest];

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
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

        // TODO: Enable the following HTTPZ contract call
        // bool isCoprocessor = HTTPZ.isCoprocessor(signer);
        bool isCoprocessor = true;
        if (!isCoprocessor) {
            revert InvalidCoprocessorSigner(signer);
        }

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
                        keccak256(abi.encodePacked(ctVerification.ctHandles)),
                        ctVerification.userAddress,
                        ctVerification.contractAddress,
                        ctVerification.contractChainId
                    )
                )
            );
    }

    /// @notice Checks if the ZK Proof verification consensus is reached among the Coprocessors.
    /// @dev This function calls the HTTPZ contract to retrieve the current Coprocessors.
    /// @dev The consensus threshold is calculated as the simple majority of the total Coprocessors.
    /// @param verifiedSignaturesCount The number of signatures that have been verified for a ZK Proof
    /// @return Whether the consensus for ZK Proof verification is reached
    function _isConsensusReached(uint256 verifiedSignaturesCount) internal pure virtual returns (bool) {
        // TODO: Enable the following HTTPZ contract call
        // uint256 consensusThreshold = _HTTPZ.getCoprocessorsCount() / 2 + 1;
        uint256 consensusThreshold = 4 / 2 + 1;
        return verifiedSignaturesCount >= consensusThreshold;
    }
}

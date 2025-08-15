// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IInputVerification.sol";
import "./interfaces/IGatewayConfig.sol";
import "./shared/UUPSUpgradeableEmptyProxy.sol";
import "./shared/GatewayConfigChecks.sol";
import "./shared/Pausable.sol";

/**
 * @title InputVerification smart contract
 * @dev See {IInputVerification}
 */
contract InputVerification is
    IInputVerification,
    EIP712Upgradeable,
    Ownable2StepUpgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayConfigChecks,
    Pausable
{
    /**
     * @notice The typed data structure for the EIP712 signature to validate in ZK Proof verification responses.
     * @dev The name of this struct is not relevant for the signature validation, only the one defined
     * @dev EIP712_ZKPOK_TYPE is, but we keep it the same for clarity.
     */
    struct CiphertextVerification {
        /// @notice The coprocessor's computed ciphertext handles.
        bytes32[] ctHandles;
        /// @notice The address of the user that has provided the input in the ZK Proof verification request.
        address userAddress;
        /// @notice The address of the dapp requiring the ZK Proof verification.
        address contractAddress;
        /// @notice The host chain's chain ID of the contract requiring the ZK Proof verification.
        uint256 contractChainId;
        /// @notice Generic bytes metadata for versioned payloads. First byte is for the version.
        bytes extraData;
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

    /// @notice The address of the GatewayConfig contract for protocol state calls.
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /// @notice The definition of the CiphertextVerification structure typed data.
    string private constant EIP712_ZKPOK_TYPE =
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)";

    /// @notice The hash of the CiphertextVerification structure typed data definition used for signature validation.
    bytes32 private constant EIP712_ZKPOK_TYPE_HASH = keccak256(bytes(EIP712_ZKPOK_TYPE));

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "InputVerification";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number using in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 3;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.InputVerification
    struct InputVerificationStorage {
        /// @notice The counter used for the ZKPoK IDs returned in verification request events.
        uint256 zkProofIdCounter;
        /// @notice The ZKPoK request IDs that have been verified.
        mapping(uint256 zkProofId => bool isVerified) verifiedZKProofs;
        /// @notice The ZKPoK request IDs that have been rejected.
        mapping(uint256 zkProofId => bool isRejected) rejectedZKProofs;
        /// @notice The validated signatures associated to a verified ZKPoK with the given ID.
        mapping(uint256 zkProofId => mapping(bytes32 digest => bytes[] signatures)) zkProofSignatures;
        /// @notice The number of coprocessors that have rejected a ZKPoK with the given ID.
        mapping(uint256 zkProofId => uint256 responseCounter) rejectedProofResponseCounter;
        /// @notice Whether a coprocessor has signed a ZKPoK verification.
        mapping(uint256 zkProofId => mapping(address coprocessorSigner => bool hasVerified)) signerVerifiedZKPoK;
        /// @notice Whether a coprocessor has signed a ZKPoK rejection.
        mapping(uint256 zkProofId => mapping(address coprocessorSigner => bool hasRejected)) signerRejectedZKPoK;
        /// @notice The ZKPoK request inputs to be used for signature validation in response calls.
        mapping(uint256 zkProofId => ZKProofInput zkProofInput) _zkProofInputs;
        // ----------------------------------------------------------------------------------------------
        // Transaction sender addresses from consensus state variables:
        // ----------------------------------------------------------------------------------------------
        // prettier-ignore
        /// @notice The coprocessor transaction senders involved in a consensus for a proof verification.
        mapping(uint256 zkProofId =>
            mapping(bytes32 digest => address[] coprocessorTxSenderAddresses))
               verifyProofConsensusTxSenders;
        /// @notice The digest of the proof verification response that reached consensus for a proof verification request.
        mapping(uint256 zkProofId => bytes32 verifyProofConsensusDigest) verifyProofConsensusDigest;
        /// @notice The coprocessor transaction senders involved in a consensus for a proof rejection.
        mapping(uint256 zkProofId => address[] coprocessorTxSenderAddresses) rejectProofConsensusTxSenders;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.InputVerification")) - 1))
    /// @dev & ~bytes32(uint256(0xff))
    bytes32 private constant INPUT_VERIFICATION_STORAGE_LOCATION =
        0x4544165ce1653264fdcb09b029891e3d4c8d8583486821172f882e19a149a800;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract.
    /// @dev Contract name and version for EIP712 signature validation are defined here
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME, "1");
        __Ownable_init(owner());
        __Pausable_init();
    }

    /**
     * @notice Re-initializes the contract from V1.
     */
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /// @dev See {IInputVerification-verifyProofRequest}.
    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof,
        bytes calldata extraData
    ) external virtual onlyRegisteredHostChain(contractChainId) whenNotPaused {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        $.zkProofIdCounter++;
        uint256 zkProofId = $.zkProofIdCounter;

        /// @dev The following stored inputs are used during response calls for the EIP712 signature validation.
        $._zkProofInputs[zkProofId] = ZKProofInput(contractChainId, contractAddress, userAddress);

        emit VerifyProofRequest(
            zkProofId,
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithZKProof,
            extraData
        );
    }

    /**
     * @dev See {IInputVerification-verifyProofResponse}.
     * We restrict this call to coprocessor transaction senders because, in case of reorgs, we need to
     * prevent anyone else from copying the signature and sending it to trigger a consensus.
     */
    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata ctHandles,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual onlyCoprocessorTxSender {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        // Make sure the zkProofId corresponds to a generated ZK Proof verification request.
        if (zkProofId > $.zkProofIdCounter || zkProofId == 0) {
            revert VerifyProofNotRequested(zkProofId);
        }

        /// @dev Retrieve stored ZK Proof verification request inputs.
        ZKProofInput memory zkProofInput = $._zkProofInputs[zkProofId];

        /// @dev Initialize the CiphertextVerification structure for the signature validation.
        CiphertextVerification memory ciphertextVerification = CiphertextVerification(
            ctHandles,
            zkProofInput.userAddress,
            zkProofInput.contractAddress,
            zkProofInput.contractChainId,
            extraData
        );

        /// @dev Compute the digest of the CiphertextVerification structure.
        bytes32 digest = _hashCiphertextVerification(ciphertextVerification);

        /// @dev Recover the signer address from the signature,
        address signerAddress = ECDSA.recover(digest, signature);

        GATEWAY_CONFIG.checkIsCoprocessorSigner(signerAddress);

        /// @dev Check that the coprocessor has not already responded to the ZKPoK verification request.
        _checkCoprocessorAlreadyResponded(zkProofId, msg.sender, signerAddress);

        bytes[] storage currentSignatures = $.zkProofSignatures[zkProofId][digest];
        currentSignatures.push(signature);
        $.signerVerifiedZKPoK[zkProofId][signerAddress] = true;

        // Store the coprocessor transaction sender address for the proof verification response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.verifyProofConsensusTxSenders[zkProofId][digest].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.verifiedZKProofs[zkProofId] && _isConsensusReached(currentSignatures.length)) {
            $.verifiedZKProofs[zkProofId] = true;

            // A "late" valid coprocessor could still see its transaction sender address be added to
            // the list after consensus. This storage variable is here to be able to retrieve this list
            // later by only knowing the zkProofId, since a consensus can only happen once per proof
            // verification request.
            $.verifyProofConsensusDigest[zkProofId] = digest;

            emit VerifyProofResponse(zkProofId, ctHandles, currentSignatures);
        }
    }

    /// @dev See {IInputVerification-rejectProofResponse}.
    function rejectProofResponse(
        uint256 zkProofId,
        bytes calldata /* extraData */
    ) external virtual onlyCoprocessorTxSender {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        // Make sure the zkProofId corresponds to a generated ZK Proof verification request.
        if (zkProofId > $.zkProofIdCounter || zkProofId == 0) {
            revert VerifyProofNotRequested(zkProofId);
        }

        /**
         * @dev Retrieve the coprocessor signer address from the GatewayConfig contract using the
         * coprocessor transaction sender address.
         * Extracting the signer address is important in order to prevent potential issues with re-org, as this could
         * lead to situations where a coprocessor can both verify and reject a proof, which is forbidden. This check
         * is directly done within `_checkCoprocessorAlreadyResponded` below.
         */
        Coprocessor memory coprocessor = GATEWAY_CONFIG.getCoprocessor(msg.sender);
        address coprocessorSignerAddress = coprocessor.signerAddress;

        /// @dev Check that the coprocessor has not already responded to the ZKPoK verification request.
        _checkCoprocessorAlreadyResponded(zkProofId, msg.sender, coprocessorSignerAddress);

        $.rejectedProofResponseCounter[zkProofId]++;
        $.signerRejectedZKPoK[zkProofId][coprocessorSignerAddress] = true;

        // Store the coprocessor transaction sender address for the proof rejection response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.rejectProofConsensusTxSenders[zkProofId].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.rejectedZKProofs[zkProofId] && _isConsensusReached($.rejectedProofResponseCounter[zkProofId])) {
            $.rejectedZKProofs[zkProofId] = true;

            emit RejectProofResponse(zkProofId);
        }
    }

    /// @dev See {IInputVerification-checkProofVerified}.
    function checkProofVerified(uint256 zkProofId) external view virtual {
        InputVerificationStorage storage $ = _getInputVerificationStorage();
        if (!$.verifiedZKProofs[zkProofId]) {
            revert ProofNotVerified(zkProofId);
        }
    }

    /// @dev See {IInputVerification-checkProofRejected}.
    function checkProofRejected(uint256 zkProofId) external view virtual {
        InputVerificationStorage storage $ = _getInputVerificationStorage();
        if (!$.rejectedZKProofs[zkProofId]) {
            revert ProofNotRejected(zkProofId);
        }
    }

    /**
     * @dev See {IInputVerification-getVerifyProofConsensusTxSenders}.
     * The list remains empty until the consensus is reached.
     */
    function getVerifyProofConsensusTxSenders(uint256 zkProofId) external view virtual returns (address[] memory) {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        // Get the unique digest associated to the ZK Proof verification request in order to retrieve the
        // list of coprocessor transaction sender address that were involved in the consensus for a
        // proof verification.
        // This digest remains the default value (0x0) until the consensus is reached.
        bytes32 consensusDigest = $.verifyProofConsensusDigest[zkProofId];

        return $.verifyProofConsensusTxSenders[zkProofId][consensusDigest];
    }

    /// @dev See {IDecryption-getRejectProofConsensusTxSenders}.
    function getRejectProofConsensusTxSenders(uint256 zkProofId) external view virtual returns (address[] memory) {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        return $.rejectProofConsensusTxSenders[zkProofId];
    }

    /// @dev See {IInputVerification-getVersion}.
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
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Checks if the coprocessor has already verified or rejected a ZKPoK verification request.
     * @param zkProofId The ID of the ZK Proof.
     * @param txSenderAddress The transaction sender address of the coprocessor.
     * @param signerAddress The signer address of the coprocessor.
     */
    function _checkCoprocessorAlreadyResponded(
        uint256 zkProofId,
        address txSenderAddress,
        address signerAddress
    ) internal virtual {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        if ($.signerVerifiedZKPoK[zkProofId][signerAddress]) {
            revert CoprocessorAlreadyVerified(zkProofId, txSenderAddress, signerAddress);
        }

        if ($.signerRejectedZKPoK[zkProofId][signerAddress]) {
            revert CoprocessorAlreadyRejected(zkProofId, txSenderAddress, signerAddress);
        }
    }

    /**
     * @notice Computes the hash of a given CiphertextVerification structured data
     * @param ctVerification The CiphertextVerification structure
     * @return The hash of the CiphertextVerification structure
     */
    function _hashCiphertextVerification(
        CiphertextVerification memory ctVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_ZKPOK_TYPE_HASH,
                        keccak256(abi.encodePacked(ctVerification.ctHandles)),
                        ctVerification.userAddress,
                        ctVerification.contractAddress,
                        ctVerification.contractChainId,
                        keccak256(abi.encodePacked(ctVerification.extraData))
                    )
                )
            );
    }

    /**
     * @notice Checks if the consensus is reached among the coprocessors.
     * @param coprocessorCounter The number of coprocessors that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the InputVerification storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getInputVerificationStorage() internal pure returns (InputVerificationStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := INPUT_VERIFICATION_STORAGE_LOCATION
        }
    }
}

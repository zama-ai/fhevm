// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IInputVerification } from "./interfaces/IInputVerification.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { Pausable } from "./shared/Pausable.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { ProtocolPaymentUtils } from "./shared/ProtocolPaymentUtils.sol";
import { Coprocessor } from "./shared/Structs.sol";

/**
 * @title InputVerification smart contract
 * @notice See {IInputVerification}
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract InputVerification is
    IInputVerification,
    EIP712Upgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayOwnable,
    GatewayConfigChecks,
    ProtocolPaymentUtils,
    Pausable
{
    /**
     * @notice The typed data structure for the EIP712 signature to validate in ZK Proof verification responses.
     * @dev The name of this struct is not relevant for the signature validation, only the one defined
     * EIP712_ZKPOK_TYPE is, but we keep it the same for clarity.
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

    /**
     * @notice The stored structure for the received ZK Proof verification request inputs.
     */
    struct ZKProofInput {
        /// @notice The chain ID of the contract address.
        uint256 contractChainId;
        /// @notice The contract address that verification is requested for.
        address contractAddress;
        /// @notice The user address that requested the verification.
        address userAddress;
    }

    /**
     * @notice RFC-021 (Solana host) counterpart of {@link CiphertextVerification}. The dapp and
     * user identities are 32-byte host addresses (Solana program ids / pubkeys) rather than 20-byte
     * EVM addresses. The EVM `CiphertextVerification` is left untouched; this parallel struct lets
     * the gateway verify the same bytes32 coprocessor attestation that the zama-host program checks.
     */
    struct SolanaCiphertextVerification {
        bytes32[] ctHandles;
        bytes32 userAddress;
        bytes32 contractAddress;
        uint256 contractChainId;
        bytes extraData;
    }

    /**
     * @notice Solana counterpart of {@link ZKProofInput}: stores the bytes32 host identities of a
     * ZK Proof verification request for later EIP712 signature validation in the response call.
     */
    struct SolanaZKProofInput {
        uint256 contractChainId;
        bytes32 contractAddress;
        bytes32 userAddress;
    }

    /**
     * @notice The address of the GatewayConfig contract for protocol state calls.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice The definition of the CiphertextVerification structure typed data.
     */
    string private constant EIP712_ZKPOK_TYPE =
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)";

    /**
     * @notice The hash of the CiphertextVerification structure typed data definition used for signature validation.
     */
    bytes32 private constant EIP712_ZKPOK_TYPE_HASH = keccak256(bytes(EIP712_ZKPOK_TYPE));

    /**
     * @notice The Solana (RFC-021) CiphertextVerification typed data definition. Identical to
     * EIP712_ZKPOK_TYPE except the user/dapp identities are bytes32. This string MUST match the
     * zama-host program's CiphertextVerification type byte-for-byte so a single coprocessor
     * signature validates both on the gateway and on-chain on Solana.
     */
    string private constant EIP712_SOLANA_ZKPOK_TYPE =
        "CiphertextVerification(bytes32[] ctHandles,bytes32 userAddress,bytes32 contractAddress,uint256 contractChainId,bytes extraData)";

    /**
     * @notice The hash of the Solana CiphertextVerification typed data definition.
     */
    bytes32 private constant EIP712_SOLANA_ZKPOK_TYPE_HASH = keccak256(bytes(EIP712_SOLANA_ZKPOK_TYPE));

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "InputVerification";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 5;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 6;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.InputVerification
    struct InputVerificationStorage {
        // ----------------------------------------------------------------------------------------------
        // Common state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The counter used for the ZKPoK IDs returned in verification request events.
        uint256 zkProofIdCounter;
        /// @notice The ZKPoK request inputs to be used for signature validation in response calls.
        mapping(uint256 zkProofId => ZKProofInput zkProofInput) zkProofInputs;
        /// @notice The validated signatures associated to a verified ZKPoK with the given ID.
        mapping(uint256 zkProofId => mapping(bytes32 digest => bytes[] signatures)) zkProofSignatures;
        // prettier-ignore
        /// @notice The coprocessor transaction senders involved in a consensus for a proof verification.
        mapping(uint256 zkProofId =>
            mapping(bytes32 digest => address[] coprocessorTxSenderAddresses))
               verifyProofConsensusTxSenders;
        // ----------------------------------------------------------------------------------------------
        // Proof verification state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The ZKPoK request IDs that have been verified.
        mapping(uint256 zkProofId => bool isVerified) verifiedZKProofs;
        /// @notice Whether a coprocessor has signed a ZKPoK verification.
        mapping(uint256 zkProofId => mapping(address coprocessorSigner => bool hasVerified)) signerVerifiedZKPoK;
        /// @notice The digest of the proof verification response that reached consensus for a proof verification request.
        mapping(uint256 zkProofId => bytes32 verifyProofConsensusDigest) verifyProofConsensusDigest;
        // ----------------------------------------------------------------------------------------------
        // Proof rejection state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The ZKPoK request IDs that have been rejected.
        mapping(uint256 zkProofId => bool isRejected) rejectedZKProofs;
        /// @notice The number of coprocessors that have rejected a ZKPoK with the given ID.
        mapping(uint256 zkProofId => uint256 responseCounter) rejectedProofResponseCounter;
        /// @notice Whether a coprocessor has signed a ZKPoK rejection.
        mapping(uint256 zkProofId => mapping(address coprocessorSigner => bool hasRejected)) signerRejectedZKPoK;
        /// @notice The coprocessor transaction senders involved in a consensus for a proof rejection.
        mapping(uint256 zkProofId => address[] coprocessorTxSenderAddresses) rejectProofConsensusTxSenders;
        // ----------------------------------------------------------------------------------------------
        // Context state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessor context ID associated to the input verification request
        mapping(uint256 zkProofId => uint256 contextId) inputVerificationContextId;
        // ----------------------------------------------------------------------------------------------
        // Solana (RFC-021) host state variables (appended; preserves the ERC-7201 storage layout):
        // ----------------------------------------------------------------------------------------------
        /// @notice The bytes32-identity request inputs for Solana host ZK Proof verifications.
        mapping(uint256 zkProofId => SolanaZKProofInput solanaZkProofInput) solanaZkProofInputs;
        /// @notice The priority coprocessor transaction sender that finalized proof verification.
        /// @dev Raw responder arrays keep every valid sender, including non-priority senders that
        ///      submitted before or after finalization. When set, this freezes the exposed
        ///      consensus result to a singleton priority sender, even if the global priority
        ///      configuration later changes.
        mapping(uint256 zkProofId => address coprocessorTxSenderAddress) priorityVerifyProofConsensusTxSender;
        /// @notice The priority coprocessor transaction sender that finalized proof rejection.
        /// @dev Raw responder arrays keep every valid sender, including non-priority senders that
        ///      submitted before or after finalization. When set, this freezes the exposed
        ///      consensus result to a singleton priority sender, even if the global priority
        ///      configuration later changes.
        mapping(uint256 zkProofId => address coprocessorTxSenderAddress) priorityRejectProofConsensusTxSender;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.InputVerification")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant INPUT_VERIFICATION_STORAGE_LOCATION =
        0x4544165ce1653264fdcb09b029891e3d4c8d8583486821172f882e19a149a800;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract.
     * @dev Contract name and version for EIP712 signature validation are defined here
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME, "1");
        __Pausable_init();
    }

    /**
     * @notice Re-initializes the contract from V4 (Solana RFC-021 input-verification path).
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV5() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice See {IInputVerification-verifyProofRequest}.
     */
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

        // The following stored inputs are used during response calls for the EIP712 signature validation.
        $.zkProofInputs[zkProofId] = ZKProofInput(contractChainId, contractAddress, userAddress);

        // Associate the request to coprocessor context ID 1 to anticipate their introduction in V2.
        $.inputVerificationContextId[zkProofId] = 1;

        // Collect the fee from the transaction sender for this input verification request.
        _collectInputVerificationFee(msg.sender);

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
     * @notice See {IInputVerification-verifyProofRequestSolana}.
     * @dev RFC-021 (Solana host) counterpart of {verifyProofRequest}. Identical lifecycle and fee
     * collection; only the dapp/user identities are 32-byte host addresses. Shares the zkProofId
     * counter and all consensus state with the EVM path; only the request inputs are stored in the
     * parallel `solanaZkProofInputs` mapping for bytes32 EIP712 signature validation on response.
     */
    function verifyProofRequestSolana(
        uint256 contractChainId,
        bytes32 contractAddress,
        bytes32 userAddress,
        bytes calldata ciphertextWithZKProof,
        bytes calldata extraData
    ) external virtual onlyRegisteredHostChain(contractChainId) whenNotPaused {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        $.zkProofIdCounter++;
        uint256 zkProofId = $.zkProofIdCounter;

        // Stored for bytes32 EIP712 signature validation during the response call.
        $.solanaZkProofInputs[zkProofId] = SolanaZKProofInput(contractChainId, contractAddress, userAddress);

        // Associate the request to coprocessor context ID 1 to anticipate their introduction in V2.
        $.inputVerificationContextId[zkProofId] = 1;

        // Collect the fee from the transaction sender for this input verification request.
        _collectInputVerificationFee(msg.sender);

        emit VerifyProofRequestSolana(
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
     * Also, a user is currently allowed to send an empty list of packed ciphertexts for the input
     * proof request, meaning the coprocessors will respond with an empty list of handles. We thus
     * don't revert if the `ctHandles` list is empty on purpose.
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

        bytes32 digest;
        address signerAddress;
        {
            // Retrieve stored ZK Proof verification request inputs.
            ZKProofInput memory zkProofInput = $.zkProofInputs[zkProofId];

            // Compute the digest of the CiphertextVerification structure.
            digest = _hashCiphertextVerification(
                CiphertextVerification(
                    ctHandles,
                    zkProofInput.userAddress,
                    zkProofInput.contractAddress,
                    zkProofInput.contractChainId,
                    extraData
                )
            );

            // Recover the signer address from the signature,
            signerAddress = ECDSA.recover(digest, signature);
        }

        // Check that the signer is a coprocessor signer, and that it corresponds to the transaction
        // sender of the same coprocessor.
        _checkCoprocessorSignerMatchesTxSender(signerAddress, msg.sender);

        // Check that the coprocessor has not already responded to the ZKPoK verification request.
        _checkCoprocessorAlreadyResponded(zkProofId, signerAddress);

        bytes[] storage currentSignatures = $.zkProofSignatures[zkProofId][digest];
        currentSignatures.push(signature);
        $.signerVerifiedZKPoK[zkProofId][signerAddress] = true;

        // Store the coprocessor transaction sender address for the proof verification response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.verifyProofConsensusTxSenders[zkProofId][digest].push(msg.sender);

        // Emit the event at each call for monitoring purposes.
        emit VerifyProofResponseCall(zkProofId, ctHandles, signature, msg.sender, extraData);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        // Make sure the proof has neither been verified nor rejected yet: this prevents "lazy"
        // coprocessors to be able to send both a verification and a rejection response by waiting for
        // a coprocessor threshold decrement before sending some responses.
        if (!$.verifiedZKProofs[zkProofId] && !$.rejectedZKProofs[zkProofId]) {
            (bool canFinalize, bool finalizedByPriority) = _canFinalizeCoprocessorConsensus(
                msg.sender,
                currentSignatures.length
            );
            if (canFinalize) {
                $.verifiedZKProofs[zkProofId] = true;

                // A "late" valid coprocessor could still see its transaction sender address be added to
                // the list after consensus. This storage variable is here to be able to retrieve this list
                // later by only knowing the zkProofId, since a consensus can only happen once per proof
                // verification request.
                $.verifyProofConsensusDigest[zkProofId] = digest;
                if (finalizedByPriority) {
                    $.priorityVerifyProofConsensusTxSender[zkProofId] = msg.sender;

                    bytes[] memory prioritySignatures = new bytes[](1);
                    prioritySignatures[0] = signature;
                    emit VerifyProofResponse(zkProofId, ctHandles, prioritySignatures);
                } else {
                    emit VerifyProofResponse(zkProofId, ctHandles, currentSignatures);
                }
            }
        }
    }

    /**
     * @notice See {IInputVerification-verifyProofResponseSolana}.
     * @dev RFC-021 (Solana host) counterpart of {verifyProofResponse}. The only difference is the
     * EIP712 digest, computed over bytes32 host identities via {_hashSolanaCiphertextVerification};
     * all consensus, signer/tx-sender and verification state is shared with the EVM path (keyed by
     * zkProofId), so a Solana request verified here is indistinguishable downstream from an EVM one.
     */
    function verifyProofResponseSolana(
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

        // Retrieve stored Solana ZK Proof verification request inputs (bytes32 identities).
        SolanaZKProofInput memory solanaInput = $.solanaZkProofInputs[zkProofId];

        // Initialize the bytes32 CiphertextVerification structure for the signature validation.
        SolanaCiphertextVerification memory ciphertextVerification = SolanaCiphertextVerification(
            ctHandles,
            solanaInput.userAddress,
            solanaInput.contractAddress,
            solanaInput.contractChainId,
            extraData
        );

        // Compute the digest of the Solana CiphertextVerification structure.
        bytes32 digest = _hashSolanaCiphertextVerification(ciphertextVerification);

        // Recover the signer address from the signature.
        address signerAddress = ECDSA.recover(digest, signature);

        // Check that the signer is a coprocessor signer matching the transaction sender.
        _checkCoprocessorSignerMatchesTxSender(signerAddress, msg.sender);

        // Check that the coprocessor has not already responded to the ZKPoK verification request.
        _checkCoprocessorAlreadyResponded(zkProofId, signerAddress);

        bytes[] storage currentSignatures = $.zkProofSignatures[zkProofId][digest];
        currentSignatures.push(signature);
        $.signerVerifiedZKPoK[zkProofId][signerAddress] = true;

        // Store the coprocessor transaction sender address for the proof verification response.
        $.verifyProofConsensusTxSenders[zkProofId][digest].push(msg.sender);

        // Emit the event at each call for monitoring purposes.
        emit VerifyProofResponseCall(zkProofId, ctHandles, signature, msg.sender, extraData);

        // Send the event if and only if the consensus is reached in the current response call.
        if (
            !$.verifiedZKProofs[zkProofId] &&
            !$.rejectedZKProofs[zkProofId] &&
            _isConsensusReached(currentSignatures.length)
        ) {
            $.verifiedZKProofs[zkProofId] = true;
            $.verifyProofConsensusDigest[zkProofId] = digest;

            emit VerifyProofResponse(zkProofId, ctHandles, currentSignatures);
        }
    }

    /// @notice See {IInputVerification-rejectProofResponse}.
    function rejectProofResponse(uint256 zkProofId, bytes calldata extraData) external virtual onlyCoprocessorTxSender {
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

        // Check that the coprocessor has not already responded to the ZKPoK verification request.
        _checkCoprocessorAlreadyResponded(zkProofId, coprocessorSignerAddress);

        $.rejectedProofResponseCounter[zkProofId]++;
        $.signerRejectedZKPoK[zkProofId][coprocessorSignerAddress] = true;

        // Store the coprocessor transaction sender address for the proof rejection response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.rejectProofConsensusTxSenders[zkProofId].push(msg.sender);

        // Emit the event at each call for monitoring purposes.
        emit RejectProofResponseCall(zkProofId, extraData);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        // Make sure the proof has neither been verified nor rejected yet: this prevents "lazy"
        // coprocessors to be able to send both a verification and a rejection response by waiting for
        // a coprocessor threshold decrement before sending some responses.
        if (!$.verifiedZKProofs[zkProofId] && !$.rejectedZKProofs[zkProofId]) {
            (bool canFinalize, bool finalizedByPriority) = _canFinalizeCoprocessorConsensus(
                msg.sender,
                $.rejectedProofResponseCounter[zkProofId]
            );
            if (canFinalize) {
                $.rejectedZKProofs[zkProofId] = true;
                if (finalizedByPriority) {
                    $.priorityRejectProofConsensusTxSender[zkProofId] = msg.sender;
                }

                emit RejectProofResponse(zkProofId);
            }
        }
    }

    /**
     * @notice See {IInputVerification-isProofVerified}.
     */
    function isProofVerified(uint256 zkProofId) external view virtual returns (bool) {
        InputVerificationStorage storage $ = _getInputVerificationStorage();
        return $.verifiedZKProofs[zkProofId];
    }

    /**
     * @notice See {IInputVerification-isProofRejected}.
     */
    function isProofRejected(uint256 zkProofId) external view virtual returns (bool) {
        InputVerificationStorage storage $ = _getInputVerificationStorage();
        return $.rejectedZKProofs[zkProofId];
    }

    /**
     * @notice See {IInputVerification-getVerifyProofConsensusTxSenders}.
     * The list remains empty until the consensus is reached.
     */
    function getVerifyProofConsensusTxSenders(uint256 zkProofId) external view virtual returns (address[] memory) {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        address priorityConsensusTxSender = $.priorityVerifyProofConsensusTxSender[zkProofId];
        if (priorityConsensusTxSender != address(0)) {
            // Keep the getter aligned with the singleton event payload emitted in priority mode.
            address[] memory txSenders = new address[](1);
            txSenders[0] = priorityConsensusTxSender;
            return txSenders;
        }

        bytes32 consensusDigest = $.verifyProofConsensusDigest[zkProofId];

        return $.verifyProofConsensusTxSenders[zkProofId][consensusDigest];
    }

    /**
     * @notice See {IInputVerification-getRejectProofConsensusTxSenders}.
     */
    function getRejectProofConsensusTxSenders(uint256 zkProofId) external view virtual returns (address[] memory) {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        address priorityConsensusTxSender = $.priorityRejectProofConsensusTxSender[zkProofId];
        if (priorityConsensusTxSender != address(0)) {
            // Keep the getter aligned with the singleton consensus result exposed in priority mode.
            address[] memory txSenders = new address[](1);
            txSenders[0] = priorityConsensusTxSender;
            return txSenders;
        }

        return $.rejectProofConsensusTxSenders[zkProofId];
    }

    /**
     * @notice See {IInputVerification-getVersion}.
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
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Checks if the coprocessor has already verified or rejected a ZKPoK verification request.
     * @param zkProofId The ID of the ZK Proof.
     * @param signerAddress The signer address of the coprocessor.
     */
    function _checkCoprocessorAlreadyResponded(uint256 zkProofId, address signerAddress) internal virtual {
        InputVerificationStorage storage $ = _getInputVerificationStorage();

        if ($.signerVerifiedZKPoK[zkProofId][signerAddress]) {
            revert CoprocessorAlreadyVerified(zkProofId, msg.sender, signerAddress);
        }

        if ($.signerRejectedZKPoK[zkProofId][signerAddress]) {
            revert CoprocessorAlreadyRejected(zkProofId, msg.sender, signerAddress);
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
     * @notice Computes the EIP712 digest of a {SolanaCiphertextVerification} (bytes32 identities).
     * @dev Mirrors {_hashCiphertextVerification} with the bytes32 type hash. Because the EIP712
     * domain (name "InputVerification", version "1", the gateway chain id and this contract address)
     * matches the zama-host program's verifier domain, a coprocessor signature over this digest
     * validates both here and on-chain on Solana.
     */
    function _hashSolanaCiphertextVerification(
        SolanaCiphertextVerification memory ctVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_SOLANA_ZKPOK_TYPE_HASH,
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
     * @notice Returns whether a coprocessor response can finalize consensus.
     */
    function _canFinalizeCoprocessorConsensus(
        address coprocessorTxSender,
        uint256 coprocessorCounter
    ) internal view virtual returns (bool canFinalize, bool finalizedByPriority) {
        address priorityCoprocessorTxSender = GATEWAY_CONFIG.getPriorityCoprocessorTxSender();
        if (priorityCoprocessorTxSender != address(0)) {
            finalizedByPriority = coprocessorTxSender == priorityCoprocessorTxSender;
            return (finalizedByPriority, finalizedByPriority);
        }
        return (_isConsensusReached(coprocessorCounter), false);
    }

    /**
     * @notice Returns the InputVerification storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
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

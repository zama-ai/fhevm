// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { httpzAddress } from "../addresses/HttpzAddress.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IZKPoKManager.sol";
import "./interfaces/IHTTPZ.sol";

/**
 * @title ZKPoKManager smart contract
 * @dev See {IZKPoKManager}
 */
contract ZKPoKManager is IZKPoKManager, EIP712Upgradeable, Ownable2StepUpgradeable, UUPSUpgradeable {
    /**
     * @notice The typed data structure for the EIP712 signature to validate in ZK Proof verification responses.
     * @dev The name of this struct is not relevant for the signature validation, only the one defined
     * @dev EIP712_ZKPOK_TYPE is, but we keep it the same for clarity.
     */
    struct CiphertextVerification {
        /// @notice The Coprocessor's computed ciphertext handles.
        bytes32[] ctHandles;
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
    IHTTPZ private constant _HTTPZ = IHTTPZ(httpzAddress);

    /// @notice The definition of the CiphertextVerification structure typed data.
    string private constant EIP712_ZKPOK_TYPE =
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId)";

    /// @notice The hash of the CiphertextVerification structure typed data definition used for signature validation.
    bytes32 private constant EIP712_ZKPOK_TYPE_HASH = keccak256(bytes(EIP712_ZKPOK_TYPE));

    string private constant CONTRACT_NAME = "ZKPoKManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:httpz_gateway.storage.ZKPoKManager
    struct ZKPoKManagerStorage {
        /// @notice The counter used for the ZK Proof IDs returned in verification request events.
        uint256 zkProofIdCounter;
        /// @notice The ZK Proof request IDs that have been verified.
        mapping(uint256 zkProofId => bool isVerified) verifiedZKProofs;
        /// @notice The ZK Proof request IDs that have been rejected.
        mapping(uint256 zkProofId => bool isRejected) rejectedZKProofs;
        /// @notice The validated signatures associated to a verified proof for a given ZK Proof ID.
        mapping(uint256 zkProofId => mapping(bytes32 digest => bytes[] signatures)) zkProofSignatures;
        /// @notice The number of coprocessors that have responded with a proof rejection for a given ZK Proof ID.
        mapping(uint256 zkProofId => uint256 responseCounter) rejectedProofResponseCounter;
        /// @notice Whether a coprocessor signer has already responded to a ZK Proof verification request.
        mapping(uint256 zkProofId => mapping(address coprocessorSigner => bool alreadyResponded)) alreadyResponded;
        /// @notice The ZK Proof request inputs to be used for signature validation in response calls.
        mapping(uint256 zkProofId => ZKProofInput zkProofInput) _zkProofInputs;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("httpz_gateway.storage.ZKPoKManager")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant ZKPOK_MANAGER_STORAGE_LOCATION =
        0xfc4275d554bf606fe8341b5d487ea117d8e849caa41b49b189053b59696e2700;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract.
    /// @dev Contract name and version for EIP712 signature validation are defined here
    function initialize() public reinitializer(2) {
        __EIP712_init(CONTRACT_NAME, "1");
        __Ownable_init(msg.sender);
    }

    /// @dev See {IZKPoKManager-verifyProofRequest}.
    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) public virtual {
        ZKPoKManagerStorage storage $ = _getZKPoKManagerStorage();
        _HTTPZ.checkNetworkIsRegistered(contractChainId);

        // TODO(#52): Implement sending service fees to PaymentManager contract

        $.zkProofIdCounter++;
        uint256 zkProofId = $.zkProofIdCounter;

        /// @dev The following stored inputs are used during response calls for the EIP712 signature validation.
        $._zkProofInputs[zkProofId] = ZKProofInput(contractChainId, contractAddress, userAddress);

        emit VerifyProofRequest(zkProofId, contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    }

    /// @dev See {IZKPoKManager-verifyProofResponse}.
    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata ctHandles,
        bytes calldata signature
    ) public virtual {
        /**
         * @dev Check that the transaction sender is a Coprocessor. In case of reorgs, this prevents
         * someone else from copying the signature and sending it to trigger a consensus.
         */
        _HTTPZ.checkIsCoprocessor(msg.sender);

        ZKPoKManagerStorage storage $ = _getZKPoKManagerStorage();

        /// @dev Retrieve stored ZK Proof verification request inputs.
        ZKProofInput memory zkProofInput = $._zkProofInputs[zkProofId];

        /// @dev Initialize the CiphertextVerification structure for the signature validation.
        CiphertextVerification memory ciphertextVerification = CiphertextVerification(
            ctHandles,
            zkProofInput.userAddress,
            zkProofInput.contractAddress,
            zkProofInput.contractChainId
        );

        /// @dev Compute the digest of the CiphertextVerification structure.
        bytes32 digest = _hashCiphertextVerification(ciphertextVerification);

        /// @dev Recover the signer address from the signature and validate that it is a coprocessor
        /// @dev that has not already responded (with either a proof verification or rejection).
        _validateEIP712Signature(zkProofId, digest, signature);

        bytes[] storage currentSignatures = $.zkProofSignatures[zkProofId][digest];
        currentSignatures.push(signature);

        /// @dev Send the event if and only if the consensus is reached in the current response call
        /// @dev for a proof verification.
        /// @dev This means a "late" response will not be reverted, just ignored
        /// @dev Note that this considers that the consensus is reached with at least N/2 + 1
        /// @dev coprocessors. If the threshold is updated to below this number, we should also
        /// @dev check that the ZK proof request has not been rejected yet.
        if (!isProofVerified(zkProofId) && _isConsensusReached(currentSignatures.length)) {
            // TODO(#52): Implement calling PaymentManager contract to burn and distribute fees
            $.verifiedZKProofs[zkProofId] = true;

            emit VerifyProofResponse(zkProofId, ctHandles, currentSignatures);
        }
    }

    /// @dev See {IZKPoKManager-rejectProofResponse}.
    function rejectProofResponse(uint256 zkProofId) public virtual {
        ZKPoKManagerStorage storage $ = _getZKPoKManagerStorage();

        /// @dev Validate that the caller is a coprocessor that has not already responded.
        /// @dev More info on why we do not need to validate the signature in this case is in the
        /// @dev functions's description from the interface.
        _checkCoprocessorAddress(msg.sender, zkProofId);

        $.rejectedProofResponseCounter[zkProofId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call
        /// @dev for a proof rejection.
        /// @dev This means a "late" response will not be reverted, just ignored
        /// @dev Note that this considers that the consensus is reached with at least N/2 + 1
        /// @dev coprocessors. If the threshold is updated to below this number, we should also
        /// @dev check that the ZK proof request has not been verified yet.
        if (!isProofRejected(zkProofId) && _isConsensusReached($.rejectedProofResponseCounter[zkProofId])) {
            // TODO(#52): Implement calling PaymentManager contract to burn and distribute fees
            $.rejectedZKProofs[zkProofId] = true;

            emit RejectProofResponse(zkProofId);
        }
    }

    /// @dev See {IZKPoKManager-isProofVerified}.
    function isProofVerified(uint256 zkProofId) public view virtual returns (bool) {
        ZKPoKManagerStorage storage $ = _getZKPoKManagerStorage();
        return $.verifiedZKProofs[zkProofId];
    }

    /// @dev See {IZKPoKManager-isProofRejected}.
    function isProofRejected(uint256 zkProofId) public view virtual returns (bool) {
        ZKPoKManagerStorage storage $ = _getZKPoKManagerStorage();
        return $.rejectedZKProofs[zkProofId];
    }

    /**
     * @notice Returns the versions of the ZKPoKManager contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
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

    /**
     * @notice Check that the given address is a registered coprocessor that has not already responded.
     * @param coprocessorAddress The address of the potential coprocessor
     * @param zkProofId The ID of the ZK Proof
     */
    function _checkCoprocessorAddress(address coprocessorAddress, uint256 zkProofId) internal virtual {
        ZKPoKManagerStorage storage $ = _getZKPoKManagerStorage();
        _HTTPZ.checkIsCoprocessor(coprocessorAddress);

        if ($.alreadyResponded[zkProofId][coprocessorAddress]) {
            revert CoprocessorSignerAlreadyResponded(zkProofId, coprocessorAddress);
        }

        $.alreadyResponded[zkProofId][coprocessorAddress] = true;
    }

    /**
     * @notice Validates the EIP712 signature for a given ZK Proof
     * @param zkProofId The ID of the ZK Proof
     * @param digest The hash of the CiphertextVerification structure
     * @param signature The signature to be validated
     */
    function _validateEIP712Signature(uint256 zkProofId, bytes32 digest, bytes calldata signature) internal virtual {
        address signerAddress = ECDSA.recover(digest, signature);

        _checkCoprocessorAddress(signerAddress, zkProofId);
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
                        ctVerification.contractChainId
                    )
                )
            );
    }

    /**
     * @notice Computes the hash of ctHandles
     * @param ctHandles The ctHandles
     * @return The hash of the ctHandles
     */
    function _hashCtHandles(bytes32[] calldata ctHandles) internal view virtual returns (bytes32) {
        return keccak256(abi.encodePacked(ctHandles));
    }

    /**
     * @notice Checks if the consensus is reached among the Coprocessors.
     * @dev This function calls the HTTPZ contract to retrieve the consensus threshold.
     * @param coprocessorCounter The number of coprocessors that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the ZKPoKManager storage location.
     */
    function _getZKPoKManagerStorage() internal pure returns (ZKPoKManagerStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := ZKPOK_MANAGER_STORAGE_LOCATION
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}

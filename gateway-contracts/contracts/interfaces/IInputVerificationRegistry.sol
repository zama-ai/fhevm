// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the InputVerificationRegistry contract.
 * @notice The InputVerificationRegistry contract handles input verification request registration.
 * @dev V2 Design: Unlike V1 InputVerification.sol, this contract:
 * - Does NOT receive full ciphertext payloads (only commitment hash)
 * - Does NOT handle response processing (done off-chain via Coprocessor HTTP API)
 * - Relayer computes commitment = keccak256(ciphertext || ZKPoK), posts commitment only
 * - Relayer broadcasts full payload directly to Coprocessors via HTTP
 * - Coprocessors verify commitment matches before processing
 */
interface IInputVerificationRegistry {
    // ============================================
    // Events
    // ============================================

    /**
     * @notice Emitted when an input verification request is registered.
     * @param requestId The unique identifier for the verification request.
     * @param commitment Hash of the ciphertext + ZKPoK payload (keccak256).
     * @param userAddress The address of the user who owns the input.
     * @param contractChainId The target host chain ID where handles will be used.
     * @param contractAddress The target contract address on the host chain.
     * @param userSignature EIP-712 signature from user binding the request parameters.
     * @param timestamp The block timestamp when the request was registered.
     */
    event InputVerificationRegistered(
        uint256 indexed requestId,
        bytes32 commitment,
        address indexed userAddress,
        uint256 contractChainId,
        address contractAddress,
        bytes userSignature,
        uint256 timestamp
    );

    // ============================================
    // Errors
    // ============================================

    /**
     * @notice Error indicating that the commitment is empty (zero).
     */
    error EmptyCommitment();

    /**
     * @notice Error indicating that the user address is zero.
     */
    error EmptyUserAddress();

    /**
     * @notice Error indicating that the contract address is zero.
     */
    error EmptyContractAddress();

    /**
     * @notice Error indicating that the user signature is empty.
     */
    error EmptyUserSignature();

    /**
     * @notice Error indicating that the chain ID is zero.
     */
    error InvalidChainId();

    // ============================================
    // Functions
    // ============================================

    /**
     * @notice Register an input verification request with a commitment.
     * @dev The Relayer calls this with commitment = keccak256(ciphertext || ZKPoK).
     * The full payload is sent directly to Coprocessors via HTTP API.
     * userSignature MUST bind (commitment, contractChainId, contractAddress, userAddress)
     * to prevent Relayer from registering with arbitrary userAddress.
     * @param commitment Hash of the ciphertext + ZKPoK payload.
     * @param contractChainId Target host chain ID where handles will be used.
     * @param contractAddress Target contract address on the host chain.
     * @param userAddress The user who owns the input (NOT msg.sender, which is Relayer).
     * @param userSignature EIP-712 signature from user binding the request parameters.
     * @return requestId The unique identifier for the verification request.
     */
    function registerInputVerification(
        bytes32 commitment,
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata userSignature
    ) external payable returns (uint256 requestId);

    /**
     * @notice Get the details of a registered input verification request.
     * @param requestId The unique identifier for the verification request.
     * @return commitment The commitment hash.
     * @return userAddress The user who owns the input.
     * @return contractChainId The target host chain ID.
     * @return contractAddress The target contract address.
     * @return fee The fee paid for the request.
     * @return timestamp The block timestamp when the request was registered.
     */
    function getRequest(uint256 requestId) external view returns (
        bytes32 commitment,
        address userAddress,
        uint256 contractChainId,
        address contractAddress,
        uint256 fee,
        uint256 timestamp
    );

    /**
     * @notice Returns the version of the InputVerificationRegistry contract in SemVer format.
     */
    function getVersion() external pure returns (string memory);
}

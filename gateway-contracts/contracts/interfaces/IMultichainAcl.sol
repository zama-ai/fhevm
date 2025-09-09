// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/Structs.sol";

/**
 * @title Interface for the MultichainAcl contract.
 * @notice The MultichainAcl contract aggregates ACLs from all host chains.
 */
interface IMultichainAcl {
    /**
     * @notice Emitted when an account is allowed to use a ciphertext handle.
     * @param ctHandle The ciphertext handle that the account is allowed to use.
     * @param accountAddress The address of the account allowed to use the ciphertext handle.
     */
    event AllowAccount(bytes32 indexed ctHandle, address accountAddress);

    /**
     * @notice Emitted when a public decryption is allowed for a ciphertext handle.
     * @param ctHandle The ciphertext handle that is allowed for public decryption.
     */
    event AllowPublicDecrypt(bytes32 indexed ctHandle);

    /**
     * @notice Emitted when an account delegates its access to another account.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegationAccounts The delegator and the delegated addresses.
     * @param contractAddresses The addresses of the delegated contracts.
     */
    event DelegateAccount(uint256 indexed chainId, DelegationAccounts delegationAccounts, address[] contractAddresses);

    /**
     * @notice Error indicating that the coprocessor has already allowed public decryption to the ciphertext.
     * @param ctHandle The ciphertext handle that the coprocessor has already allowed access to.
     * @param txSender The transaction sender address of the coprocessor that has already allowed access.
     */
    error CoprocessorAlreadyAllowedPublicDecrypt(bytes32 ctHandle, address txSender);

    /**
     * @notice Error indicating that the coprocessor has already allowed the account to use the ciphertext handle.
     * @param ctHandle The ciphertext handle that the coprocessor has already allowed access to.
     * @param account The address of the account that has already been allowed access.
     * @param txSender The transaction sender address of the coprocessor that has already allowed access.
     */
    error CoprocessorAlreadyAllowedAccount(bytes32 ctHandle, address account, address txSender);

    /**
     * @notice Error indicating that the coprocessor has already delegated access to another account.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegationAccounts The delegator and the delegated addresses.
     * @param contractAddresses The addresses of the contracts that the coprocessor has already delegated.
     * @param txSender The transaction sender address of the coprocessor that has already confirmed delegation.
     */
    error CoprocessorAlreadyDelegated(
        uint256 chainId,
        DelegationAccounts delegationAccounts,
        address[] contractAddresses,
        address txSender
    );

    /**
     * @notice Error indicating that the account address is not allowed to use the ciphertext handle.
     * @param ctHandle The ciphertext handle that the account is not allowed to use.
     * @param accountAddress The address of the account that is not allowed to use the ciphertext handle.
     */
    error AccountNotAllowedToUseCiphertext(bytes32 ctHandle, address accountAddress);

    /**
     * @notice Error indicating that the ciphertext handle is not allowed for public decryption.
     * @param ctHandle The ciphertext handle that is not allowed for public decryption.
     */
    error PublicDecryptNotAllowed(bytes32 ctHandle);

    /// @notice Error indicating that the contract addresses list is empty.
    error EmptyContractAddresses();

    /**
     * @notice Error indicating that the number of delegation contracts requested exceeds the maximum allowed.
     * @param maxLength The maximum number of contracts allowed.
     * @param actualLength The actual number of contracts requested.
     */
    error ContractsMaxLengthExceeded(uint8 maxLength, uint256 actualLength);

    /**
     * @notice Error indicating that the account has not been fully delegated.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegationAccounts The delegator and the delegated addresses.
     * @param contractAddress The address of the delegated contract.
     */
    error AccountNotDelegated(uint256 chainId, DelegationAccounts delegationAccounts, address contractAddress);

    /**
     * @notice Allows access to the ciphertext handle for public decryption.
     * @param ctHandle The ciphertext handle.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function allowPublicDecrypt(bytes32 ctHandle, bytes calldata extraData) external;

    /**
     * @notice Allows an account to access a ciphertext handle.
     * @param ctHandle The handle of the ciphertext.
     * @param accountAddress The address of the account to allow.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function allowAccount(bytes32 ctHandle, address accountAddress, bytes calldata extraData) external;

    /**
     * @notice Delegates the access to the delegated and contract addresses.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegationAccounts The delegator and the delegated addresses.
     * @param contractAddresses The contract addresses to delegate access to.
     */
    function delegateAccount(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external;

    /**
     * @notice Checks that the ciphertext handle is allowed for public decryption.
     * @param ctHandle The handle of the ciphertext.
     */
    function checkPublicDecryptAllowed(bytes32 ctHandle) external view;

    /**
     * @notice Checks that the account is allowed to use the ciphertext handle.
     * @param ctHandle The handle of the ciphertext.
     * @param accountAddress The address of the account.
     */
    function checkAccountAllowed(bytes32 ctHandle, address accountAddress) external view;

    /**
     * @notice Checks that the delegator has delegated access to the delegate and contracts addresses.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegationAccounts The delegator and the delegated addresses.
     * @param contractAddresses The delegated contract addresses.
     */
    function checkAccountDelegated(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external view;

    /**
     * @notice Returns the coprocessor transaction sender addresses that were involved in the consensus for an allow public decrypt.
     * @param ctHandle The ciphertext handle.
     */
    function getAllowPublicDecryptConsensusTxSenders(bytes32 ctHandle) external view returns (address[] memory);

    /**
     * @notice Returns the coprocessor transaction sender addresses that were involved in the consensus for an allow account.
     * @param ctHandle The ciphertext handle.
     * @param accountAddress The address of the account.
     */
    function getAllowAccountConsensusTxSenders(
        bytes32 ctHandle,
        address accountAddress
    ) external view returns (address[] memory);

    /**
     * @notice Returns the coprocessor transaction sender addresses that were involved in the consensus for a delegate account.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegationAccounts The delegator and the delegated addresses.
     * @param contractAddresses The delegated contract addresses.
     */
    function getDelegateAccountConsensusTxSenders(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external view returns (address[] memory);

    /**
     * @notice Returns the versions of the MultichainACL contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

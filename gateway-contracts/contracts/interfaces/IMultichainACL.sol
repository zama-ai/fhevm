// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/Structs.sol";

/**
 * @title Interface for the MultichainACL contract.
 * @notice The MultichainACL contract aggregates ACLs from all host chains.
 */
interface IMultichainACL {
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
     * @notice Error indicating that the contract addresses list is empty.
     */
    error EmptyContractAddresses();

    /**
     * @notice Error indicating that the number of delegation contracts requested exceeds the maximum allowed.
     * @param maxLength The maximum number of contracts allowed.
     * @param actualLength The actual number of contracts requested.
     */
    error ContractsMaxLengthExceeded(uint256 maxLength, uint256 actualLength);

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
     * @notice Indicates if the ciphertext handle is allowed for public decryption.
     * @param ctHandle The handle of the ciphertext.
     */
    function isPublicDecryptAllowed(bytes32 ctHandle) external view returns (bool);

    /**
     * @notice Indicates if the account is allowed to use the ciphertext handle.
     * @param ctHandle The handle of the ciphertext.
     * @param accountAddress The address of the account.
     */
    function isAccountAllowed(bytes32 ctHandle, address accountAddress) external view returns (bool);

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
     * @notice Returns the versions of the MultichainACL contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

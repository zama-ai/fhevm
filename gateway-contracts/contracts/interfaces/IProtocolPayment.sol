// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the ProtocolPayment contract.
 * @notice The ProtocolPayment contract manages protocol payment and fees.
 * @dev All prices are in $ZAMA base units (using 18 decimals).
 */
interface IProtocolPayment {
    /**
     * @notice Emitted when the protocol payment is initialized.
     * @param inputVerificationPrice The input verification price in $ZAMA base units (using 18 decimals).
     * @param publicDecryptionPrice The public decryption price in $ZAMA base units (using 18 decimals).
     * @param userDecryptionPrice The user decryption price in $ZAMA base units (using 18 decimals).
     */
    event InitializeProtocolPayment(
        uint256 inputVerificationPrice,
        uint256 publicDecryptionPrice,
        uint256 userDecryptionPrice
    );

    /**
     * @notice Emitted when the input verification price is set.
     * @param price The new input verification price in $ZAMA base units (using 18 decimals).
     */
    event NewInputVerificationPrice(uint256 price);

    /**
     * @notice Emitted when the public decryption price is set.
     * @param price The new public decryption price in $ZAMA base units (using 18 decimals).
     */
    event NewPublicDecryptionPrice(uint256 price);

    /**
     * @notice Emitted when the user decryption price is set.
     * @param price The new user decryption price in $ZAMA base units (using 18 decimals).
     */
    event NewUserDecryptionPrice(uint256 price);

    /**
     * @notice Emitted when the sender is not the Decryption contract.
     * @param sender The address of the sender.
     */
    error SenderNotDecryptionContract(address sender);

    /**
     * @notice Emitted when the sender is not the InputVerification contract.
     * @param sender The address of the sender.
     */
    error SenderNotInputVerificationContract(address sender);

    /**
     * @notice Get the price in $ZAMA for an input verification.
     * @return The price of the input verification in $ZAMA base units (using 18 decimals).
     */
    function getInputVerificationPrice() external view returns (uint256);

    /**
     * @notice Get the price in $ZAMA for a public decryption.
     * @return The price of the public decryption in $ZAMA base units (using 18 decimals).
     */
    function getPublicDecryptionPrice() external view returns (uint256);

    /**
     * @notice Get the price in $ZAMA for a user decryption (including delegated user decryption).
     * @return The price of the user decryption in $ZAMA base units (using 18 decimals).
     */
    function getUserDecryptionPrice() external view returns (uint256);

    /**
     * @notice Sets the price in $ZAMA for an input verification.
     * @param price The price of the input verification in $ZAMA base units (using 18 decimals).
     */
    function setInputVerificationPrice(uint256 price) external;

    /**
     * @notice Sets the price in $ZAMA for a public decryption.
     * @param price The price of the public decryption in $ZAMA base units (using 18 decimals).
     */
    function setPublicDecryptionPrice(uint256 price) external;

    /**
     * @notice Sets the price in $ZAMA for a user decryption (including delegated user decryption).
     * @param price The price of the user decryption in $ZAMA base units (using 18 decimals).
     */
    function setUserDecryptionPrice(uint256 price) external;

    /**
     * @notice Collects the $ZAMA fees from the transaction sender for an input verification.
     * @param txSender The address of the transaction sender.
     */
    function collectInputVerificationFee(address txSender) external;

    /**
     * @notice Collects the $ZAMA fees from the transaction sender for a public decryption.
     * @param txSender The address of the transaction sender.
     */
    function collectPublicDecryptionFee(address txSender) external;

    /**
     * @notice Collects the $ZAMA fees from the transaction sender for a user decryption (including
     * delegated user decryption).
     * @param txSender The address of the transaction sender.
     */
    function collectUserDecryptionFee(address txSender) external;

    /**
     * @notice Returns the versions of the ProtocolPayment contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

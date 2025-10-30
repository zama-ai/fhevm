// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { decryptionAddress, inputVerificationAddress } from "../addresses/GatewayAddresses.sol";
import { feesSenderToBurnerAddress } from "../addresses/PaymentBridgingAddresses.sol";

import { zamaOFTAddress } from "../addresses/PaymentBridgingAddresses.sol";
import { IProtocolPayment } from "./interfaces/IProtocolPayment.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";

/**
 * @title Payment smart contract
 * @notice See {IProtocolPayment}
 */
contract ProtocolPayment is IProtocolPayment, UUPSUpgradeableEmptyProxy, GatewayOwnable {
    /**
     * @notice The address of the FeesSenderToBurner contract to send the fees to.
     */
    address private constant FEES_SENDER_TO_BURNER_ADDRESS = address(feesSenderToBurnerAddress);

    /**
     * @notice The address of the Decryption contract from which some fees are collected.
     */
    address private constant DECRYPTION_ADDRESS = address(decryptionAddress);

    /**
     * @notice The address of the InputVerification contract from which some fees are collected.
     */
    address private constant INPUT_VERIFICATION_ADDRESS = address(inputVerificationAddress);

    /**
     * @notice The interface of the $ZAMA OFT contract as an ERC20 to transfer fees.
     */
    IERC20 private constant ZAMA_OFT = IERC20(zamaOFTAddress);

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "ProtocolPayment";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 2;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     * @dev All prices are in $ZAMA base units (using 18 decimals).
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.ProtocolPayment
    struct ProtocolPaymentStorage {
        uint256 inputVerificationPrice;
        uint256 publicDecryptionPrice;
        uint256 userDecryptionPrice;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.ProtocolPayment")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant PROTOCOL_PAYMENT_STORAGE_LOCATION =
        0x395ab2663a9dbb96f058af7a33668e31bc69e0016b9978f85369489bcee86800;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract.
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint256 initialInputVerificationPrice,
        uint256 initialPublicDecryptionPrice,
        uint256 initialUserDecryptionPrice
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();

        $.inputVerificationPrice = initialInputVerificationPrice;
        $.publicDecryptionPrice = initialPublicDecryptionPrice;
        $.userDecryptionPrice = initialUserDecryptionPrice;

        emit InitializeProtocolPayment(
            initialInputVerificationPrice,
            initialPublicDecryptionPrice,
            initialUserDecryptionPrice
        );
    }

    /**
     * @notice Re-initializes the contract from V1.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    // function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    modifier onlyDecryptionContract() {
        if (msg.sender != DECRYPTION_ADDRESS) {
            revert SenderNotDecryptionContract(msg.sender);
        }
        _;
    }

    modifier onlyInputVerificationContract() {
        if (msg.sender != INPUT_VERIFICATION_ADDRESS) {
            revert SenderNotInputVerificationContract(msg.sender);
        }
        _;
    }

    /**
     * @notice See {IProtocolPayment-getInputVerificationPrice}.
     */
    function getInputVerificationPrice() external view virtual returns (uint256) {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        return $.inputVerificationPrice;
    }

    /**
     * @notice See {IProtocolPayment-getPublicDecryptionPrice}.
     */
    function getPublicDecryptionPrice() external view virtual returns (uint256) {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        return $.publicDecryptionPrice;
    }

    /**
     * @notice See {IProtocolPayment-getUserDecryptionPrice}.
     */
    function getUserDecryptionPrice() external view virtual returns (uint256) {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        return $.userDecryptionPrice;
    }

    /**
     * @notice See {IProtocolPayment-setInputVerificationPrice}.
     */
    function setInputVerificationPrice(uint256 price) external virtual onlyGatewayOwner {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        $.inputVerificationPrice = price;
        emit NewInputVerificationPrice(price);
    }

    /**
     * @notice See {IProtocolPayment-setPublicDecryptionPrice}.
     */
    function setPublicDecryptionPrice(uint256 price) external virtual onlyGatewayOwner {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        $.publicDecryptionPrice = price;
        emit NewPublicDecryptionPrice(price);
    }

    /**
     * @notice See {IProtocolPayment-setUserDecryptionPrice}.
     */
    function setUserDecryptionPrice(uint256 price) external virtual onlyGatewayOwner {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        $.userDecryptionPrice = price;
        emit NewUserDecryptionPrice(price);
    }

    /**
     * @notice See {IProtocolPayment-collectInputVerificationFee}.
     */
    function collectInputVerificationFee(address txSender) external virtual onlyInputVerificationContract {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        _transferFromToFeesSenderToBurner(txSender, $.inputVerificationPrice);
    }

    /**
     * @notice See {IProtocolPayment-collectPublicDecryptionFee}.
     */
    function collectPublicDecryptionFee(address txSender) external virtual onlyDecryptionContract {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        _transferFromToFeesSenderToBurner(txSender, $.publicDecryptionPrice);
    }

    /**
     * @notice See {IProtocolPayment-collectUserDecryptionFee}.
     */
    function collectUserDecryptionFee(address txSender) external virtual onlyDecryptionContract {
        ProtocolPaymentStorage storage $ = _getProtocolPaymentStorage();
        _transferFromToFeesSenderToBurner(txSender, $.userDecryptionPrice);
    }

    /**
     * @notice See {IProtocolPayment-getVersion}.
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
     * @notice Transfers the $ZAMA from the sender to the FeesSenderToBurner contract.
     * @param txSender The address of the transaction sender.
     * @param price The price of the input verification in $ZAMA base units (using 18 decimals).
     */
    function _transferFromToFeesSenderToBurner(address txSender, uint256 price) internal virtual {
        ZAMA_OFT.transferFrom(txSender, FEES_SENDER_TO_BURNER_ADDRESS, price);
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Returns the ProtocolPayment storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getProtocolPaymentStorage() internal pure returns (ProtocolPaymentStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := PROTOCOL_PAYMENT_STORAGE_LOCATION
        }
    }
}

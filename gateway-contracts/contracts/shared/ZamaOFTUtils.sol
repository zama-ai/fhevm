// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { zamaOFTAddress } from "../../addresses/PaymentBridgingAddresses.sol";
import { protocolPaymentAddress } from "../../addresses/GatewayAddresses.sol";
import { IProtocolPayment } from "../interfaces/IProtocolPayment.sol";

/**
 * @title ZamaOFT utils
 */
abstract contract ZamaOFTUtils {
    /**
     * @notice The interface of the $ZAMA token contract as an ERC20.
     */
    IERC20 private constant ZAMA_OFT = IERC20(zamaOFTAddress);

    /**
     * @notice The interface of the ProtocolPayment contract.
     */
    IProtocolPayment private constant PROTOCOL_PAYMENT = IProtocolPayment(protocolPaymentAddress);

    /**
     * @notice Collects the fee in $ZAMA for an input verification.
     * @return bool Indicates the success of the transfer.
     */
    function _collectInputVerificationFee() internal returns (bool) {
        uint256 inputVerificationFee = PROTOCOL_PAYMENT.getInputVerificationPrice();
        return _transferFrom(inputVerificationFee);
    }

    /**
     * @notice Collects the fee in $ZAMA for a public decryption.
     * @return bool Indicates the success of the transfer.
     */
    function _collectPublicDecryptionFee() internal returns (bool) {
        uint256 publicDecryptionFee = PROTOCOL_PAYMENT.getPublicDecryptionPrice();
        return _transferFrom(publicDecryptionFee);
    }

    /**
     * @notice Collects the fee in $ZAMA for a user decryption.
     * @return bool Indicates the success of the transfer.
     */
    function _collectUserDecryptionFee() internal returns (bool) {
        uint256 userDecryptionFee = PROTOCOL_PAYMENT.getUserDecryptionPrice();
        return _transferFrom(userDecryptionFee);
    }

    /**
     * @notice Calls the transferFrom function on the ZamaOFT contract from the sender to the
     * ProtocolPayment contract.
     * @param value The amount of $ZAMA to transfer, in base units (using 18 decimals).
     * @return bool Indicates the success of the transfer.
     */
    function _transferFrom(uint256 value) internal returns (bool) {
        return ZAMA_OFT.transferFrom(msg.sender, protocolPaymentAddress, value);
    }
}

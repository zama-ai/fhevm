// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { protocolPaymentAddress } from "../../addresses/GatewayAddresses.sol";
import { IProtocolPayment } from "../interfaces/IProtocolPayment.sol";

/**
 * @title ProtocolPayment utils
 */
abstract contract ProtocolPaymentUtils {
    /**
     * @notice The interface of the ProtocolPayment contract.
     */
    IProtocolPayment private constant PROTOCOL_PAYMENT = IProtocolPayment(protocolPaymentAddress);

    /**
     * @notice Collects the fee in $ZAMA from the transaction sender for an input verification.
     * @param txSender The address of the transaction sender.
     */
    function _collectInputVerificationFee(address txSender) internal {
        PROTOCOL_PAYMENT.collectInputVerificationFee(txSender);
    }

    /**
     * @notice Collects the fee in $ZAMA from the transaction sender for a public decryption.
     * @param txSender The address of the transaction sender.
     */
    function _collectPublicDecryptionFee(address txSender) internal {
        PROTOCOL_PAYMENT.collectPublicDecryptionFee(txSender);
    }

    /**
     * @notice Collects the fee in $ZAMA from the transaction sender for a user decryption.
     * @param txSender The address of the transaction sender.
     */
    function _collectUserDecryptionFee(address txSender) internal {
        PROTOCOL_PAYMENT.collectUserDecryptionFee(txSender);
    }
}

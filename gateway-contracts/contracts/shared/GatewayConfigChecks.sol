// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../../addresses/GatewayAddresses.sol";
import { IGatewayConfig } from "../interfaces/IGatewayConfig.sol";
import { HandleOps } from "../libraries/HandleOps.sol";

/**
 * @title GatewayConfig Checks
 * @dev Contract that provides checks on top of the GatewayConfig contract
 */
abstract contract GatewayConfigChecks {
    /**
     * @notice The address of the GatewayConfig contract.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice Error emitted when an address is not a KMS transaction sender.
     * @param txSenderAddress The address that is not a KMS transaction sender.
     */
    error NotKmsTxSender(address txSenderAddress);

    /**
     * @notice Error emitted when an address is not a KMS signer.
     * @param signerAddress The address that is not a KMS signer.
     */
    error NotKmsSigner(address signerAddress);

    /**
     * @notice Error emitted when an address is not a coprocessor transaction sender.
     * @param txSenderAddress The address that is not a coprocessor transaction sender.
     */
    error NotCoprocessorTxSender(address txSenderAddress);

    /**
     * @notice Error emitted when an address is not a coprocessor signer.
     * @param signerAddress The address that is not a coprocessor signer.
     */
    error NotCoprocessorSigner(address signerAddress);

    /**
     * @notice Error emitted when an address is not a custodian transaction sender.
     * @param txSenderAddress The address that is not a custodian transaction sender.
     */
    error NotCustodianTxSender(address txSenderAddress);

    /**
     * @notice Error emitted when an address is not a custodian signer.
     * @param signerAddress The address that is not a custodian signer.
     */
    error NotCustodianSigner(address signerAddress);

    /**
     * @notice Error emitted when a host chain is not registered in the GatewayConfig contract.
     * @param chainId The host chain's chain ID.
     */
    error HostChainNotRegistered(uint256 chainId);

    /**
     * @notice Error emitted when the KMS signer does not correspond to the KMS transaction sender.
     * @param signerAddress The address of the KMS signer.
     * @param txSenderAddress The address of the KMS transaction sender.
     */
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);

    /**
     * @notice Error emitted when the coprocessor signer does not correspond to the coprocessor transaction sender.
     * @param signerAddress The address of the coprocessor signer.
     * @param txSenderAddress The address of the coprocessor transaction sender.
     */
    error CoprocessorSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);

    /**
     * @notice Checks if the sender is a KMS transaction sender.
     */
    modifier onlyKmsTxSender() {
        if (!GATEWAY_CONFIG.isKmsTxSender(msg.sender)) {
            revert NotKmsTxSender(msg.sender);
        }
        _;
    }

    /**
     * @notice Checks if the sender is a coprocessor transaction sender.
     */
    modifier onlyCoprocessorTxSender() {
        if (!GATEWAY_CONFIG.isCoprocessorTxSender(msg.sender)) {
            revert NotCoprocessorTxSender(msg.sender);
        }
        _;
    }

    /**
     * @notice Checks if the chain ID corresponds to a registered host chain.
     */
    modifier onlyRegisteredHostChain(uint256 chainId) {
        if (!GATEWAY_CONFIG.isHostChainRegistered(chainId)) {
            revert HostChainNotRegistered(chainId);
        }
        _;
    }

    /**
     * @notice Checks if the chain ID extracted from the handle corresponds to a registered host chain.
     */
    modifier onlyHandleFromRegisteredHostChain(bytes32 handle) {
        uint256 handleChainId = HandleOps.extractChainId(handle);
        if (!GATEWAY_CONFIG.isHostChainRegistered(handleChainId)) {
            revert HostChainNotRegistered(handleChainId);
        }
        _;
    }

    /**
     * @notice Checks if the address is a KMS signer.
     * @param signerAddress The address to check.
     */
    function _checkIsKmsSigner(address signerAddress) internal view {
        if (!GATEWAY_CONFIG.isKmsSigner(signerAddress)) {
            revert NotKmsSigner(signerAddress);
        }
    }

    /**
     * @notice Checks if the address is a coprocessor signer.
     * @param signerAddress The address to check.
     */
    function _checkIsCoprocessorSigner(address signerAddress) internal view {
        if (!GATEWAY_CONFIG.isCoprocessorSigner(signerAddress)) {
            revert NotCoprocessorSigner(signerAddress);
        }
    }

    /**
     * @notice Checks if the signer is a KMS signer, and that it corresponds to the transaction
     * sender of the same KMS node.
     * @param signerAddress The signer address to check.
     * @param txSenderAddress The address of the KMS transaction sender.
     */
    function _checkKmsSignerMatchesTxSender(address signerAddress, address txSenderAddress) internal view {
        _checkIsKmsSigner(signerAddress);

        if (GATEWAY_CONFIG.getKmsNode(txSenderAddress).signerAddress != signerAddress) {
            revert KmsSignerDoesNotMatchTxSender(signerAddress, txSenderAddress);
        }
    }

    /**
     * @notice Checks if the signer is a KMS signer for a given context, and that it corresponds
     * to the transaction sender of the same KMS node within that context.
     * @param contextId The context ID to check against.
     * @param signerAddress The signer address to check.
     * @param txSenderAddress The address of the KMS transaction sender.
     */
    function _checkKmsContextSignerMatchesTxSender(
        uint256 contextId,
        address signerAddress,
        address txSenderAddress
    ) internal view {
        if (!GATEWAY_CONFIG.isKmsContextSigner(contextId, signerAddress)) {
            revert NotKmsSigner(signerAddress);
        }
        if (GATEWAY_CONFIG.getKmsContextNode(contextId, txSenderAddress).signerAddress != signerAddress) {
            revert KmsSignerDoesNotMatchTxSender(signerAddress, txSenderAddress);
        }
    }

    /**
     * @notice Checks if the signer is a coprocessor signer, and that it corresponds to the
     * transaction sender of the same coprocessor.
     * @param signerAddress The signer address to check.
     * @param txSenderAddress The address of the coprocessor transaction sender.
     */
    function _checkCoprocessorSignerMatchesTxSender(address signerAddress, address txSenderAddress) internal view {
        _checkIsCoprocessorSigner(signerAddress);

        if (GATEWAY_CONFIG.getCoprocessor(txSenderAddress).signerAddress != signerAddress) {
            revert CoprocessorSignerDoesNotMatchTxSender(signerAddress, txSenderAddress);
        }
    }
}

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
     * @notice Checks if the sender is a KMS transaction sender.
     */
    modifier onlyKmsTxSender() {
        if (!GATEWAY_CONFIG.isKmsTxSender(msg.sender)) {
            revert NotKmsTxSender(msg.sender);
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
}

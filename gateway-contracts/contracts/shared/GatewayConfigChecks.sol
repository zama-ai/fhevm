// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { gatewayConfigAddress } from "../../addresses/GatewayAddresses.sol";
import { IGatewayConfig } from "../interfaces/IGatewayConfig.sol";
import { HandleOps } from "../libraries/HandleOps.sol";

/**
 * @title GatewayConfig Checks
 * @dev Base contract that provides modifiers that checks proper registration in the GatewayConfig contract
 */
abstract contract GatewayConfigChecks {
    /// @notice The address of the GatewayConfig contract
    IGatewayConfig private constant _GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice Error emitted when an address is not the owner of the GatewayConfig contract.
     * @param sender The address that is not the owner.
     */
    error NotGatewayOwner(address sender);

    /// @notice Checks if the sender is a coprocessor transaction sender.
    modifier onlyCoprocessorTxSender() {
        _GATEWAY_CONFIG.checkIsCoprocessorTxSender(msg.sender);
        _;
    }

    /// @notice Checks if the sender is a KMS transaction sender.
    modifier onlyKmsTxSender() {
        _GATEWAY_CONFIG.checkIsKmsTxSender(msg.sender);
        _;
    }

    /// @dev Check that the chain ID corresponds to a registered host chain.
    modifier onlyRegisteredHostChain(uint256 chainId) {
        _GATEWAY_CONFIG.checkHostChainIsRegistered(chainId);
        _;
    }

    /// @dev Check that the chain ID extracted from the handle corresponds to a registered host chain.
    modifier onlyHandleFromRegisteredHostChain(bytes32 handle) {
        _GATEWAY_CONFIG.checkHostChainIsRegistered(HandleOps.extractChainId(handle));
        _;
    }

    /// @dev Check that the sender is the owner of the GatewayConfig contract.
    modifier onlyGatewayOwner() {
        /**
         * @dev We cast to Ownable2StepUpgradeable instead of importing GatewayConfig
         * to avoid a circular dependency. Solidity requires that base contracts be defined
         * before derived contracts, which GatewayConfig would violate in this context.
         */
        if (msg.sender != Ownable2StepUpgradeable(gatewayConfigAddress).owner()) {
            revert NotGatewayOwner(msg.sender);
        }
        _;
    }
}

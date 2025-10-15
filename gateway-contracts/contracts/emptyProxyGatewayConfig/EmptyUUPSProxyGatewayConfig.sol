// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

/**
 * @title  EmptyUUPSProxyGatewayConfig
 * @notice EmptyUUPSProxyGatewayConfig is an empty UUPS Proxy containing only upgrade logic to simplify
 * deployment, making it independent from nonce to solve circular dependencies. It is owned by
 * the gateway owner and is only used for deploying the GatewayConfig contract.
 */
contract EmptyUUPSProxyGatewayConfig is UUPSUpgradeable, Ownable2StepUpgradeable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice              Initializes the contract.
     * @param initialOwner  Initial owner address.
     */
    function initialize(address initialOwner) public initializer {
        __Ownable_init(initialOwner);
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}

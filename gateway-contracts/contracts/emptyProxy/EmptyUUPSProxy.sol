// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { GatewayOwnable } from "../shared/GatewayOwnable.sol";
/**
 * @title  EmptyUUPSProxy
 * @notice EmptyUUPSProxy is an empty UUPS Proxy containing only upgrade logic to simplify deployment,
 * making it independent from nonce to solve circular dependencies. It is owned by the Gateway owner,
 * defined as the owner of the GatewayConfig contract.
 */
contract EmptyUUPSProxy is UUPSUpgradeable, GatewayOwnable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice              Initializes the contract.
     */
    function initialize() public initializer {}

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}
}

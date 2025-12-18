// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";
import {IDeploymentCoordinator} from "../interfaces/IDeploymentCoordinator.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @notice Factory contract specialized for deploying RegulatedERC7984Upgradeable tokens
/// @dev Part of the split factory architecture to stay under contract size limits
/// @custom:security-contact contact@zaiffer.org
contract RegulatedERC7984UpgradeableFactory is Ownable2Step {
    struct TokenDeploymentParams {
        address implementation;
        address confidentialToken;
        string name;
        string symbol;
        uint8 decimals;
        address underlying;
    }

    event ConfidentialTokenDeployed(
        address indexed implementation,
        address indexed confidentialToken,
        string name,
        string symbol,
        uint8 decimals,
        address indexed underlying
    );

    constructor() Ownable(msg.sender) {}

    /// @notice Deploy a confidential token for a given original token
    /// @param implementation_ Address of the canonical implementation contract
    /// @param name_ Name of the confidential token
    /// @param symbol_ Symbol of the confidential token
    /// @param decimals_ Decimals of the confidential token
    /// @param rate_ Rate for decimal conversion
    /// @param underlying_ Address of the underlying token
    /// @param deploymentCoordinator_ DeploymentCoordinator address (used to dynamically fetch AdminProvider)
    /// @param admin_ Address that will receive DEFAULT_ADMIN_ROLE
    /// @param wrapperSetter_ Address that will receive WRAPPER_SETTER_ROLE (typically the coordinator)
    /// @return confidentialToken Address of the deployed confidential token
    function deployConfidentialToken(
        address implementation_,
        string calldata name_,
        string calldata symbol_,
        uint8 decimals_,
        uint256 rate_,
        address underlying_,
        IDeploymentCoordinator deploymentCoordinator_,
        address admin_,
        address wrapperSetter_
    ) external onlyOwner returns (RegulatedERC7984Upgradeable) {
        // Use provided canonical implementation instead of deploying new one
        address proxyAddress;
        {
            bytes memory data = abi.encodeCall(
                RegulatedERC7984Upgradeable.initialize,
                (name_, symbol_, decimals_, admin_, rate_, underlying_, deploymentCoordinator_, wrapperSetter_)
            );

            ERC1967Proxy proxy = new ERC1967Proxy(implementation_, data);
            proxyAddress = address(proxy);
        }

        emit ConfidentialTokenDeployed(
            implementation_,
            proxyAddress,
            name_,
            symbol_,
            decimals_,
            underlying_
        );

        return RegulatedERC7984Upgradeable(proxyAddress);
    }
}

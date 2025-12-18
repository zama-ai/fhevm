// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import {WrapperUpgradeable} from "../wrapper/WrapperUpgradeable.sol";
import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";
import {IDeploymentCoordinator} from "../interfaces/IDeploymentCoordinator.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @notice Factory contract specialized for deploying upgradeable Wrapper contracts
/// @dev Part of the split factory architecture to stay under contract size limits
/// @custom:security-contact contact@zaiffer.org
contract WrapperFactory is Ownable2Step {
    error ZeroAddressImplementation();
    error ZeroAddressAdmin();
    error ImplementationNotContract();

    event WrapperDeployed(
        address indexed wrapper,
        address indexed originalToken,
        address indexed confidentialToken,
        address implementation,
        address deploymentCoordinator,
        address admin
    );

    constructor() Ownable(msg.sender) {}

    /// @notice Deploy an upgradeable wrapper for a given original token and confidential token pair
    /// @param implementation_ Address of the WrapperUpgradeable implementation contract
    /// @param originalToken_ Address of the original token (address(0) for ETH)
    /// @param confidentialToken_ Address of the paired confidential token
    /// @param deploymentCoordinator_ Address of the deployment coordinator (used to dynamically fetch AdminProvider)
    /// @param admin_ Address that will be granted DEFAULT_ADMIN_ROLE on the wrapper
    /// @return wrapper Address of the deployed wrapper proxy contract
    function deployWrapper(
        address implementation_,
        address originalToken_,
        RegulatedERC7984Upgradeable confidentialToken_,
        IDeploymentCoordinator deploymentCoordinator_,
        address admin_
    ) external onlyOwner returns (WrapperUpgradeable) {
        // Validate implementation address
        require(implementation_ != address(0), ZeroAddressImplementation());
        require(_isContract(implementation_), ImplementationNotContract());

        // Validate admin address
        require(admin_ != address(0), ZeroAddressAdmin());

        // Note: confidentialToken_ and deploymentCoordinator_ are validated in WrapperUpgradeable.initialize()

        bytes memory data = abi.encodeCall(
            WrapperUpgradeable.initialize,
            (originalToken_, confidentialToken_, deploymentCoordinator_, admin_)
        );

        ERC1967Proxy proxy = new ERC1967Proxy(implementation_, data);

        WrapperUpgradeable wrapper = WrapperUpgradeable(payable(address(proxy)));

        emit WrapperDeployed(
            address(wrapper),
            originalToken_,
            address(confidentialToken_),
            implementation_,
            address(deploymentCoordinator_),
            admin_
        );

        return wrapper;
    }

    /// @notice Check if an address is a contract
    /// @param account Address to check
    /// @return True if the address contains code, false otherwise
    function _isContract(address account) private view returns (bool) {
        return account.code.length > 0;
    }
}

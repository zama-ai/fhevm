// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.27;

import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {SanctionsList} from "./SanctionsList.sol";
import {FeeManager} from "./FeeManager.sol";

/// @title IAdminProvider
/// @notice Interface for AdminProvider configuration management
interface IAdminProvider {
    function sanctionsList() external view returns (SanctionsList);
    function feeManager() external view returns (FeeManager);
    function setSanctionsList(SanctionsList sanctionsList_) external;
    function setFeeManager(FeeManager feeManager_) external;

    error ZeroAddressFeeManager();
    error ZeroAddressSanctionsList();
    error ZeroAddressRegulator();

    event SanctionsListUpdated(address indexed oldSanctionsList, address indexed newSanctionsList);
    event FeeManagerUpdated(address indexed oldFeeManager, address indexed newFeeManager);
    event RegulatorUpdated(address indexed oldRegulator, address indexed newRegulator);
}

/// @title AdminProvider
/// @notice Centralized configuration provider for confidential token system
/// @dev Provides shared access to sanctions list, fee manager, and regulator address
/// @dev Uses Ownable2Step for secure ownership transfers
/// @dev All setter functions are restricted to owner and validate against zero addresses
/// @custom:security-contact contact@zaiffer.org
contract AdminProvider is IAdminProvider, Ownable2Step {
    /// @dev Current sanctions list contract
    SanctionsList public sanctionsList;

    /// @dev Current fee manager contract
    FeeManager public feeManager;

    /// @dev Address authorized to decrypt user handles
    address public regulator;

    /// @notice Constructs AdminProvider with initial configuration
    /// @param feeManager_ Initial fee manager contract address
    /// @param sanctionsList_ Initial sanctions list contract address
    /// @param regulator_ Initial regulator address for regulatory actions
    /// @dev Reverts if any address is zero
    /// @dev Sets msg.sender as initial owner via Ownable2Step
    constructor(FeeManager feeManager_, SanctionsList sanctionsList_, address regulator_) Ownable(msg.sender) {
        require(address(feeManager_) != address(0), ZeroAddressFeeManager());
        require(address(sanctionsList_) != address(0), ZeroAddressSanctionsList());
        require(regulator_ != address(0), ZeroAddressRegulator());
        feeManager = feeManager_;
        sanctionsList = sanctionsList_;
        regulator = regulator_;
    }

    /// @notice Updates the sanctions list contract
    /// @param sanctionsList_ New sanctions list contract address
    /// @dev Only callable by owner
    /// @dev Reverts if sanctionsList_ is zero address
    /// @dev Emits SanctionsListUpdated event with old and new addresses
    function setSanctionsList(SanctionsList sanctionsList_) external onlyOwner {
        require(address(sanctionsList_) != address(0), ZeroAddressSanctionsList());
        address oldSanctionsList = address(sanctionsList);
        sanctionsList = sanctionsList_;
        emit SanctionsListUpdated(oldSanctionsList, address(sanctionsList_));
    }

    /// @notice Updates the fee manager contract
    /// @param feeManager_ New fee manager contract address
    /// @dev Only callable by owner
    /// @dev Reverts if feeManager_ is zero address
    /// @dev Emits FeeManagerUpdated event with old and new addresses
    function setFeeManager(FeeManager feeManager_) external onlyOwner {
        require(address(feeManager_) != address(0), ZeroAddressFeeManager());
        address oldFeeManager = address(feeManager);
        feeManager = feeManager_;
        emit FeeManagerUpdated(oldFeeManager, address(feeManager_));
    }

    /// @notice Updates the regulator address
    /// @param regulator_ New regulator address
    /// @dev Only callable by owner
    /// @dev Reverts if regulator_ is zero address
    /// @dev Emits RegulatorUpdated event with old and new addresses
    function setRegulator(address regulator_) external onlyOwner {
        require(regulator_ != address(0), ZeroAddressRegulator());
        address oldRegulator = regulator;
        regulator = regulator_;
        emit RegulatorUpdated(oldRegulator, regulator_);
    }
}

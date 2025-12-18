// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;


import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {FHE, ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

/// @title IFeeManager
/// @notice Interface for fee management in the confidential token system
interface IFeeManager {
    function getWrapFee(uint256 amount, address wrapFrom, address wrapTo) external view returns (uint256);
    function getUnwrapFee(uint64 amount, address unwrapFrom, address unwrapTo) external view returns (uint64);
    function getDeployFee(address deployer) external view returns (uint64);
    function getBatchTransferFee() external view returns (uint64);
    function getFeeRecipient() external view returns (address);
    function setWrapFeeBasisPoints(uint64 feeBasisPoints) external;
    function setUnwrapFeeBasisPoints(uint64 feeBasisPoints) external;
    function setSwapperWrapFeeBasisPoints(uint64 feeBasisPoints) external;
    function setSwapperUnwrapFeeBasisPoints(uint64 feeBasisPoints) external;
    function setDeployFee(uint64 deployFee) external;
    function setBatchTransferFee(uint64 batchTransferFee) external;
    function setFeeRecipient(address recipient) external;
    function setSwapperFeeWaiverActive(bool active) external;

    event WrapFeeBasisPointsUpdated(uint64 oldFeeBasisPoints, uint64 newFeeBasisPoints);
    event UnwrapFeeBasisPointsUpdated(uint64 oldFeeBasisPoints, uint64 newFeeBasisPoints);
    event SwapperWrapFeeBasisPointsUpdated(uint64 oldFeeBasisPoints, uint64 newFeeBasisPoints);
    event SwapperUnwrapFeeBasisPointsUpdated(uint64 oldFeeBasisPoints, uint64 newFeeBasisPoints);
    event DeployFeeUpdated(uint64 oldDeployFee, uint64 newDeployFee);
    event BatchTransferFeeUpdated(uint64 oldBatchTransferFee, uint64 newBatchTransferFee);
    event FeeRecipientUpdated(address indexed oldRecipient, address indexed newRecipient);
    event SwapperFeeWaiverUpdated(bool active);
}

/// @title FeeManager
/// @notice Manages all fee configuration for the confidential token system
/// @dev Uses AccessControl with FEE_MANAGER_ROLE for fee parameter updates
/// @dev Supports both basis point fees (wrap/unwrap) and fixed fees (deploy/batch transfer)
/// @dev Fee ranges:
///      - Wrap/Unwrap: 0-10,000 basis points (0%-100%)
///      - Deploy: 0-type(uint64).max wei
///      - Batch Transfer: 0-type(uint64).max wei
/// @dev Special feature: SWAPPER_ROLE fee waiver for authorized swapper contracts
/// @custom:security-contact contact@zaiffer.org
contract FeeManager is IFeeManager, AccessControl, ZamaEthereumConfig {
    /// @dev Role identifier for swapper contracts eligible for fee waivers
    bytes32 public constant SWAPPER_ROLE = keccak256("SWAPPER_ROLE");

    /// @dev Role identifier for accounts authorized to modify fee parameters
    bytes32 public constant FEE_MANAGER_ROLE = keccak256("FEE_MANAGER_ROLE");

    /// @dev Maximum allowed basis points (10,000 = 100%)
    uint64 public constant MAX_BASIS_POINTS = 10_000;

    /// @dev Wrap fee in basis points (1 = 0.01%, 100 = 1%, 10000 = 100%)
    uint64 public wrapFeeBasisPoints;

    /// @dev Unwrap fee in basis points (1 = 0.01%, 100 = 1%, 10000 = 100%)
    uint64 public unwrapFeeBasisPoints;

    /// @dev Wrap fee for swappers in basis points when swapperFeeWaiverActive is true
    uint64 public swapperWrapFeeBasisPoints;

    /// @dev Unwrap fee for swappers in basis points when swapperFeeWaiverActive is true
    uint64 public swapperUnwrapFeeBasisPoints;

    /// @dev Fixed fee in wei for deploying new wrapper pairs
    uint64 public deployFee;

    /// @dev Fixed fee in wei for batch confidential transfers
    uint64 public batchTransferFee;

    /// @dev Address that receives all collected protocol fees
    address public feeRecipient;

    /// @dev When true, addresses with SWAPPER_ROLE pay swapperWrapFeeBasisPoints/swapperUnwrapFeeBasisPoints instead of standard fees
    bool public swapperFeeWaiverActive;

    error ZeroAddressFeeRecipient();
    error FeeExceedsMaximum();

    /// @notice Constructs FeeManager with initial fee configuration
    /// @param wrapFeeBasisPoints_ Initial wrap fee in basis points (0-10,000)
    /// @param unwrapFeeBasisPoints_ Initial unwrap fee in basis points (0-10,000)
    /// @param deployFee_ Initial deployment fee in wei
    /// @param batchTransferFee_ Initial batch transfer fee in wei
    /// @param feeRecipient_ Address that will receive protocol fees
    /// @dev Grants DEFAULT_ADMIN_ROLE to msg.sender
    /// @dev Reverts if feeRecipient_ is zero address
    /// @dev Reverts if wrap or unwrap fee basis points exceed MAX_BASIS_POINTS
    /// @dev Initializes swapper fees to 0 by default
    constructor(
        uint64 wrapFeeBasisPoints_,
        uint64 unwrapFeeBasisPoints_,
        uint64 deployFee_,
        uint64 batchTransferFee_,
        address feeRecipient_
    ) {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        require(feeRecipient_ != address(0), ZeroAddressFeeRecipient());
        require(wrapFeeBasisPoints_ <= MAX_BASIS_POINTS, FeeExceedsMaximum());
        require(unwrapFeeBasisPoints_ <= MAX_BASIS_POINTS, FeeExceedsMaximum());
        wrapFeeBasisPoints = wrapFeeBasisPoints_;
        unwrapFeeBasisPoints = unwrapFeeBasisPoints_;
        swapperWrapFeeBasisPoints = 0;
        swapperUnwrapFeeBasisPoints = 0;
        deployFee = deployFee_;
        batchTransferFee = batchTransferFee_;
        feeRecipient = feeRecipient_;
    }

    /// @notice Calculates the wrap fee for a given amount and addresses
    /// @param amount The total wrap amount in original token units
    /// @param wrapFrom The address performing the wrap operation
    /// @param wrapTo The recipient address (unused, reserved for future logic)
    /// @return The calculated fee in original token units (rounded up)
    /// @dev Uses ceiling division to prevent fee leakage through transaction splitting
    /// @dev Returns ceiling of (amount * swapperWrapFeeBasisPoints) / MAX_BASIS_POINTS if swapperFeeWaiverActive is true and wrapFrom has SWAPPER_ROLE
    /// @dev Otherwise returns ceiling of (amount * wrapFeeBasisPoints) / MAX_BASIS_POINTS
    function getWrapFee(uint256 amount, address wrapFrom, address wrapTo) external view returns (uint256) {
        uint64 feeBasisPoints = (swapperFeeWaiverActive && hasRole(SWAPPER_ROLE, wrapFrom))
            ? swapperWrapFeeBasisPoints
            : wrapFeeBasisPoints;

        // Ceiling division: (a + b - 1) / b
        // This prevents fee leakage from rounding down
        uint256 fee = amount * feeBasisPoints;
        return (fee + MAX_BASIS_POINTS - 1) / MAX_BASIS_POINTS;
    }

    /// @notice Calculates the unwrap fee for a given amount and addresses
    /// @param amount The unwrap amount in confidential token units
    /// @param unwrapFrom The address initiating unwrap (unused, reserved for future logic)
    /// @param unwrapTo The recipient address receiving original tokens
    /// @return The calculated fee in confidential token units
    /// @dev Returns (amount * swapperUnwrapFeeBasisPoints) / MAX_BASIS_POINTS if swapperFeeWaiverActive is true and unwrapTo has SWAPPER_ROLE
    /// @dev Otherwise returns (amount * unwrapFeeBasisPoints) / MAX_BASIS_POINTS
    function getUnwrapFee(uint64 amount, address unwrapFrom, address unwrapTo) external view returns (uint64) {
        return getFee(amount, getUnwrapFeeBasisPoints(unwrapFrom, unwrapTo));
    }

    /// @notice Returns the unwrap fee basis points for given addresses
    /// @param unwrapFrom The address initiating unwrap (unused, reserved for future logic)
    /// @param unwrapTo The recipient address receiving original tokens
    /// @return The unwrap fee basis points (out of MAX_BASIS_POINTS)
    /// @dev Returns swapperUnwrapFeeBasisPoints if swapperFeeWaiverActive is true and unwrapTo has SWAPPER_ROLE
    /// @dev Otherwise returns unwrapFeeBasisPoints
    /// @dev This method is used by WrapperUpgradeable to commit fee basis points at unwrap initiation,
    ///      ensuring that fee changes during the unwrap process don't affect in-flight transactions
    function getUnwrapFeeBasisPoints(address unwrapFrom, address unwrapTo) public view returns (uint64) {
        if (swapperFeeWaiverActive && hasRole(SWAPPER_ROLE, unwrapTo)) {
            return swapperUnwrapFeeBasisPoints;
        }
        return unwrapFeeBasisPoints;
    }

    /// @notice Calculates a fee given an amount and basis points
    /// @param amount The amount in confidential token units
    /// @param basisPoints The fee rate in basis points (e.g., 100 = 1%)
    /// @return The calculated fee amount (rounded up)
    /// @dev This is a utility function that applies committed basis points to calculate final fees
    /// @dev Used by WrapperUpgradeable.finalizeUnwrap to apply committed fee rates
    /// @dev Uses ceiling division to prevent fee leakage through transaction splitting
    function getFee(uint64 amount, uint64 basisPoints) public view returns (uint64) {
        // Ceiling division: (a + b - 1) / b
        // This prevents fee leakage from rounding down
        uint256 fee = uint256(amount) * uint256(basisPoints);
        return uint64((fee + MAX_BASIS_POINTS - 1) / MAX_BASIS_POINTS);
    }

    /// @notice Returns the fixed deployment fee
    /// @param deployer The address deploying (unused, reserved for future per-deployer logic)
    /// @return The deployment fee in wei
    function getDeployFee(address deployer) external view returns (uint64) {
        return deployFee;
    }

    /// @notice Returns the fixed batch transfer fee
    /// @return The batch transfer fee in wei
    function getBatchTransferFee() external view returns (uint64) {
        return batchTransferFee;
    }

    /// @notice Returns the current fee recipient address
    /// @return The address receiving protocol fees
    function getFeeRecipient() external view returns (address) {
        return feeRecipient;
    }

    /// @notice Updates the wrap fee basis points
    /// @param feeBasisPoints New wrap fee in basis points (0-10,000)
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev Reverts if feeBasisPoints exceeds MAX_BASIS_POINTS
    /// @dev Emits WrapFeeBasisPointsUpdated event
    function setWrapFeeBasisPoints(uint64 feeBasisPoints) external onlyRole(FEE_MANAGER_ROLE) {
        require(feeBasisPoints <= MAX_BASIS_POINTS, FeeExceedsMaximum());
        uint64 oldFeeBasisPoints = wrapFeeBasisPoints;
        wrapFeeBasisPoints = feeBasisPoints;
        emit WrapFeeBasisPointsUpdated(oldFeeBasisPoints, feeBasisPoints);
    }

    /// @notice Updates the unwrap fee basis points
    /// @param feeBasisPoints New unwrap fee in basis points (0-10,000)
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev Reverts if feeBasisPoints exceeds MAX_BASIS_POINTS
    /// @dev Emits UnwrapFeeBasisPointsUpdated event
    function setUnwrapFeeBasisPoints(uint64 feeBasisPoints) external onlyRole(FEE_MANAGER_ROLE) {
        require(feeBasisPoints <= MAX_BASIS_POINTS, FeeExceedsMaximum());
        uint64 oldFeeBasisPoints = unwrapFeeBasisPoints;
        unwrapFeeBasisPoints = feeBasisPoints;
        emit UnwrapFeeBasisPointsUpdated(oldFeeBasisPoints, feeBasisPoints);
    }

    /// @notice Updates the swapper wrap fee basis points
    /// @param feeBasisPoints New swapper wrap fee in basis points (0-10,000)
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev Reverts if feeBasisPoints exceeds MAX_BASIS_POINTS
    /// @dev Emits SwapperWrapFeeBasisPointsUpdated event
    function setSwapperWrapFeeBasisPoints(uint64 feeBasisPoints) external onlyRole(FEE_MANAGER_ROLE) {
        require(feeBasisPoints <= MAX_BASIS_POINTS, FeeExceedsMaximum());
        uint64 oldFeeBasisPoints = swapperWrapFeeBasisPoints;
        swapperWrapFeeBasisPoints = feeBasisPoints;
        emit SwapperWrapFeeBasisPointsUpdated(oldFeeBasisPoints, feeBasisPoints);
    }

    /// @notice Updates the swapper unwrap fee basis points
    /// @param feeBasisPoints New swapper unwrap fee in basis points (0-10,000)
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev Reverts if feeBasisPoints exceeds MAX_BASIS_POINTS
    /// @dev Emits SwapperUnwrapFeeBasisPointsUpdated event
    function setSwapperUnwrapFeeBasisPoints(uint64 feeBasisPoints) external onlyRole(FEE_MANAGER_ROLE) {
        require(feeBasisPoints <= MAX_BASIS_POINTS, FeeExceedsMaximum());
        uint64 oldFeeBasisPoints = swapperUnwrapFeeBasisPoints;
        swapperUnwrapFeeBasisPoints = feeBasisPoints;
        emit SwapperUnwrapFeeBasisPointsUpdated(oldFeeBasisPoints, feeBasisPoints);
    }

    /// @notice Updates the deployment fee
    /// @param newDeployFee New deployment fee in wei
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev Emits DeployFeeUpdated event
    function setDeployFee(uint64 newDeployFee) external onlyRole(FEE_MANAGER_ROLE) {
        uint64 oldDeployFee = deployFee;
        deployFee = newDeployFee;
        emit DeployFeeUpdated(oldDeployFee, newDeployFee);
    }

    /// @notice Updates the batch transfer fee
    /// @param newBatchTransferFee New batch transfer fee in wei
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev Emits BatchTransferFeeUpdated event
    function setBatchTransferFee(uint64 newBatchTransferFee) external onlyRole(FEE_MANAGER_ROLE) {
        uint64 oldBatchTransferFee = batchTransferFee;
        batchTransferFee = newBatchTransferFee;
        emit BatchTransferFeeUpdated(oldBatchTransferFee, newBatchTransferFee);
    }

    /// @notice Updates the fee recipient address
    /// @param recipient New fee recipient address
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev Reverts if recipient is zero address
    /// @dev Emits FeeRecipientUpdated event
    function setFeeRecipient(address recipient) external onlyRole(FEE_MANAGER_ROLE) {
        require(recipient != address(0), ZeroAddressFeeRecipient());
        address oldRecipient = feeRecipient;
        feeRecipient = recipient;
        emit FeeRecipientUpdated(oldRecipient, recipient);
    }

    /// @notice Enables or disables fee waiver for SWAPPER_ROLE addresses
    /// @param active True to enable fee waiver, false to disable
    /// @dev Only callable by accounts with FEE_MANAGER_ROLE
    /// @dev When active, addresses with SWAPPER_ROLE use swapper{Wrap/Unwrap}FeeBasisPoints to calculate fees
    /// @dev Emits SwapperFeeWaiverUpdated event
    function setSwapperFeeWaiverActive(bool active) external onlyRole(FEE_MANAGER_ROLE) {
        swapperFeeWaiverActive = active;
        emit SwapperFeeWaiverUpdated(active);
    }
}

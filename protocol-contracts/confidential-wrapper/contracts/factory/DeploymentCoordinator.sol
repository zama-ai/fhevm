// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.27;

import {WrapperUpgradeable} from "../wrapper/WrapperUpgradeable.sol";
import {WrapperFactory} from "./WrapperFactory.sol";
import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";
import {RegulatedERC7984UpgradeableFactory} from "./RegulatedERC7984UpgradeableFactory.sol";
import {AdminProvider} from "../admin/AdminProvider.sol";
import {FeeManager} from "../admin/FeeManager.sol";
import {IDeploymentCoordinator} from "../interfaces/IDeploymentCoordinator.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/interfaces/IERC20Metadata.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/// @notice Coordinator that orchestrates deployment using specialized factories
/// @dev Coordinates WrapperFactory and RegulatedERC7984UpgradeableFactory to deploy wrapper pairs
/// @custom:security-contact contact@zaiffer.org
contract DeploymentCoordinator is Ownable2Step {
    AdminProvider public adminProvider;
    WrapperFactory public wrapperFactory;
    RegulatedERC7984UpgradeableFactory public confidentialTokenFactory;

    /// @notice Canonical implementation address for all RegulatedERC7984Upgradeable proxies
    address public confidentialTokenImplementation;

    /// @notice Canonical implementation address for all WrapperUpgradeable proxies
    address public wrapperImplementation;

    /// @notice Mapping from original token address to deployed wrapper address (for compatibility)
    mapping(address originalToken => WrapperUpgradeable wrapper) public deployedWrappers;

    /// @notice Mapping from original token address to deployed confidential token address (for compatibility)
    mapping(address originalToken => RegulatedERC7984Upgradeable confidentialToken) public deployedConfidentialTokens;

    error ZeroAddressAdminProvider();
    error ZeroAddressWrapperFactory();
    error ZeroAddressConfidentialTokenFactory();
    error ZeroAddressImplementation();
    error IncorrectDeployFee();
    error WrapperAlreadyExists();
    error FeeTransferFailed();
    error ImplementationNotSet();
    error TokenMustExist();

    event WrapperPairDeployed(
        address indexed originalToken,
        address indexed wrapper,
        address indexed confidentialToken,
        string originalName,
        string originalSymbol,
        uint8 originalDecimals,
        address deployer
   );

    event AdminProviderUpdated(address indexed oldAdminProvider, address indexed newAdminProvider);
    event WrapperFactoryUpdated(address indexed oldWrapperFactory, address indexed newWrapperFactory);
    event ConfidentialTokenFactoryUpdated(address indexed oldConfidentialTokenFactory, address indexed newConfidentialTokenFactory);
    event ConfidentialTokenImplementationUpdated(address indexed oldImplementation, address indexed newImplementation);
    event WrapperImplementationUpdated(address indexed oldImplementation, address indexed newImplementation);

    constructor(
        AdminProvider adminProvider_,
        WrapperFactory wrapperFactory_,
        RegulatedERC7984UpgradeableFactory confidentialTokenFactory_
    ) Ownable(msg.sender) {
        require(address(adminProvider_) != address(0), ZeroAddressAdminProvider());
        require(address(wrapperFactory_) != address(0), ZeroAddressWrapperFactory());
        require(address(confidentialTokenFactory_) != address(0), ZeroAddressConfidentialTokenFactory());

        adminProvider = adminProvider_;
        wrapperFactory = wrapperFactory_;
        confidentialTokenFactory = confidentialTokenFactory_;
    }

    /// @notice Deploy a wrapper/cToken pair for a given token (maintains original interface)
    /// @param originalToken_ Address of the token to wrap (address(0) for ETH)
    /// @return wrapper Address of the deployed wrapper contract
    /// @return confidentialToken Address of the deployed confidential token contract
    function deploy(address originalToken_)
        external
        payable
        returns (WrapperUpgradeable wrapper,  RegulatedERC7984Upgradeable confidentialToken)
    {
        // Get deploy fee from FeeManager
        uint64 requiredFee = _getDeployFee();
        require(msg.value == requiredFee, IncorrectDeployFee());
        require(address(deployedWrappers[originalToken_]) == address(0), WrapperAlreadyExists());
        // Prevent griefing attack: ensure token exists (has code) unless it's ETH (address(0))
        require(originalToken_ == address(0) || originalToken_.code.length > 0, TokenMustExist());

        // Ensure canonical implementation is deployed
        if (confidentialTokenImplementation == address(0)) {
            confidentialTokenImplementation = address(new RegulatedERC7984Upgradeable());
        }
        require(wrapperImplementation != address(0), ImplementationNotSet());

        // Deploy confidential token first using factory
        string memory originalName;
        string memory originalSymbol;
        uint8 originalDecimals;
        (
            confidentialToken,
            originalName,
            originalSymbol,
            originalDecimals
        ) = _deployConfidentialTokenForAsset(originalToken_);

        // Deploy wrapper using factory
        wrapper = wrapperFactory.deployWrapper(
            wrapperImplementation,
            originalToken_,
            confidentialToken,
            IDeploymentCoordinator(address(this)),
            adminProvider.owner()
        );

        // Set wrapper using WRAPPER_SETTER_ROLE (one-time only)
        confidentialToken.setWrapper(address(wrapper));

        // Store mappings for compatibility
        deployedWrappers[originalToken_] = wrapper;
        deployedConfidentialTokens[originalToken_] = confidentialToken;

        // Transfer deployment fee to fee recipient from FeeManager
        if (msg.value > 0) {
            address feeRecipient = _getFeeRecipient();
            (bool success, ) = feeRecipient.call{value: msg.value}("");
            require(success, FeeTransferFailed());
        }

        emit WrapperPairDeployed(
            originalToken_,
            address(wrapper),
            address(confidentialToken),
            originalName,
            originalSymbol,
            originalDecimals,
            msg.sender
        );
    }

    /// @notice Get wrapper address for a token
    /// @param originalToken_ Token address (address(0) for ETH)
    /// @return Address of the wrapper, or address(0) if not deployed
    function getWrapper(address originalToken_) external view returns (address) {
        return address(deployedWrappers[originalToken_]);
    }

    /// @notice Get confidential token address for a token
    /// @param originalToken_ Token address (address(0) for ETH)
    /// @return Address of the confidential token, or address(0) if not deployed
    function getConfidentialToken(address originalToken_) external view returns (address) {
        return address(deployedConfidentialTokens[originalToken_]);
    }

    /// @notice Check if wrapper exists for a token
    /// @param originalToken_ Token address (address(0) for ETH)
    /// @return True if wrapper exists
    function wrapperExists(address originalToken_) external view returns (bool) {
        return address(deployedWrappers[originalToken_]) != address(0);
    }

    /// @notice Deploy confidential token for a given asset
    /// @param originalToken_ Address of the original token
    /// @return confidentialToken The deployed confidential token
    function _deployConfidentialTokenForAsset(address originalToken_)
        internal
        returns (RegulatedERC7984Upgradeable confidentialToken, string memory originalName, string memory originalSymbol, uint8 originalDecimals)
    {
        if (originalToken_ != address(0)) {
            originalName = _tryGetAssetName(originalToken_);
            originalSymbol = _tryGetAssetSymbol(originalToken_);
            originalDecimals = _tryGetAssetDecimals(IERC20Metadata(originalToken_));
        } else {
            originalName = "Ethereum";
            originalSymbol = "ETH";
            originalDecimals = 18;
        }

        uint8 maxDecimals = _maxDecimals();
        uint8 tokenDecimals;
        uint256 rate;

        if (originalDecimals > maxDecimals) {
            tokenDecimals = maxDecimals;
            rate = 10 ** (originalDecimals - maxDecimals);
        } else {
            tokenDecimals = originalDecimals;
            rate = 1;
        }

        confidentialToken = confidentialTokenFactory.deployConfidentialToken(
            confidentialTokenImplementation,
            string.concat("confidential ", originalName),
            string.concat("c", originalSymbol),
            tokenDecimals,
            rate,
            originalToken_,
            IDeploymentCoordinator(address(this)),
            adminProvider.owner(), // admin role goes directly to adminProvider owner
            address(this) // coordinator gets WRAPPER_SETTER_ROLE for one-time setWrapper call
        );
    }

    /// @notice Get deploy fee from AdminProvider's FeeManager
    function _getDeployFee() private view returns (uint64) {
        FeeManager feeManager = adminProvider.feeManager();
        return feeManager.getDeployFee(msg.sender);
    }

    /// @notice Get fee recipient from AdminProvider's FeeManager
    function _getFeeRecipient() private view returns (address) {
        FeeManager feeManager = adminProvider.feeManager();
        return feeManager.getFeeRecipient();
    }

    function _fallbackUnderlyingDecimals() private pure returns (uint8) {
        return 18;
    }

    function _maxDecimals() private pure returns (uint8) {
        return 6;
    }

    function _tryGetAssetDecimals(IERC20Metadata asset_) private view returns (uint8 assetDecimals) {
        (bool success, bytes memory encodedDecimals) = address(asset_).staticcall(
            abi.encodeCall(IERC20Metadata.decimals, ())
        );
        if (success && encodedDecimals.length == 32) {
            return abi.decode(encodedDecimals, (uint8));
        }
        return _fallbackUnderlyingDecimals();
    }

    /// @notice Try to get the token name, handling both string and bytes32 returns
    /// @param token_ The token address
    /// @return The token name, or address-based fallback (40 chars) if unavailable
    function _tryGetAssetName(address token_) private view returns (string memory) {
        (bool success, bytes memory result) = token_.staticcall(abi.encodeWithSignature("name()"));
        if (success) {
            string memory parsed = _parseStringOrBytes32(result);
            if (bytes(parsed).length > 0) {
                return parsed;
            }
        }
        // Fallback: full address as string (40 hex chars)
        return Strings.toHexString(token_);
    }

    /// @notice Try to get the token symbol, handling both string and bytes32 returns
    /// @param token_ The token address
    /// @return The token symbol, or address-based fallback (6 chars) if unavailable
    function _tryGetAssetSymbol(address token_) private view returns (string memory) {
        (bool success, bytes memory result) = token_.staticcall(abi.encodeWithSignature("symbol()"));
        if (success) {
            string memory parsed = _parseStringOrBytes32(result);
            if (bytes(parsed).length > 0) {
                return parsed;
            }
        }
        // Fallback: first 6 chars of address hex string
        string memory fullAddress = Strings.toHexString(token_);
        return _substring(fullAddress, 0, 8); // "0x" + 6 chars
    }

    /// @notice Parse encoded data as either string or bytes32
    /// @param data The encoded return data from name() or symbol()
    /// @return The parsed string, or empty string if parsing fails
    function _parseStringOrBytes32(bytes memory data) private view returns (string memory) {
        // Try to decode as string (dynamic type, length >= 64)
        if (data.length >= 64) {
            // Attempt string decode - if it's valid UTF-8 string data
            try this._externalDecodeString(data) returns (string memory str) {
                return str;
            } catch {
                // Not a valid string encoding
            }
        }

        // Try to decode as bytes32 (fixed type, length == 32)
        if (data.length == 32) {
            bytes32 b32 = abi.decode(data, (bytes32));
            return _bytes32ToString(b32);
        }

        return "";
    }

    /// @notice External helper to decode string (used with try/catch for safety)
    /// @param data The encoded data
    /// @return The decoded string
    function _externalDecodeString(bytes memory data) external pure returns (string memory) {
        return abi.decode(data, (string));
    }

    /// @notice Convert bytes32 to string, removing trailing null bytes
    /// @param data The bytes32 data
    /// @return The string representation without trailing nulls
    function _bytes32ToString(bytes32 data) private pure returns (string memory) {
        // Find the actual length (last non-zero byte)
        uint256 length = 0;
        for (uint256 i = 0; i < 32; i++) {
            if (data[i] != 0) {
                length = i + 1;
            }
        }

        bytes memory result = new bytes(length);
        for (uint256 i = 0; i < length; i++) {
            result[i] = data[i];
        }
        return string(result);
    }

    /// @notice Extract a substring from a string
    /// @param str The source string
    /// @param startIndex The starting index (inclusive)
    /// @param endIndex The ending index (exclusive)
    /// @return The substring
    function _substring(string memory str, uint256 startIndex, uint256 endIndex) private pure returns (string memory) {
        bytes memory strBytes = bytes(str);
        require(endIndex <= strBytes.length && startIndex < endIndex, "Invalid substring indices");

        bytes memory result = new bytes(endIndex - startIndex);
        for (uint256 i = startIndex; i < endIndex; i++) {
            result[i - startIndex] = strBytes[i];
        }
        return string(result);
    }

    /// @notice Accept ownership of the confidential token factory
    function acceptConfidentialTokenFactoryOwnership() external onlyOwner {
        confidentialTokenFactory.acceptOwnership();
    }

    /// @notice Accept ownership of the wrapper factory
    function acceptWrapperFactoryOwnership() external onlyOwner {
        wrapperFactory.acceptOwnership();
    }

    /// @notice Set the canonical implementation address for confidential tokens
    /// @param implementation_ New implementation address
    /// @dev Allows owner to set an externally deployed implementation
    function setConfidentialTokenImplementation(address implementation_) external onlyOwner {
        require(implementation_ != address(0), ZeroAddressImplementation());
        address oldImplementation = confidentialTokenImplementation;
        confidentialTokenImplementation = implementation_;
        emit ConfidentialTokenImplementationUpdated(oldImplementation, implementation_);
    }

    /// @notice Set the canonical implementation address for wrappers
    /// @param implementation_ New implementation address
    /// @dev Allows owner to set an externally deployed implementation
    function setWrapperImplementation(address implementation_) external onlyOwner {
        require(implementation_ != address(0), ZeroAddressImplementation());
        address oldImplementation = wrapperImplementation;
        wrapperImplementation = implementation_;
        emit WrapperImplementationUpdated(oldImplementation, implementation_);
    }

    /// @notice Set the admin provider
    /// @param adminProvider_ New admin provider address
    function setAdminProvider(AdminProvider adminProvider_) external onlyOwner {
        require(address(adminProvider_) != address(0), ZeroAddressAdminProvider());
        address oldAdminProvider = address(adminProvider);
        adminProvider = adminProvider_;
        emit AdminProviderUpdated(oldAdminProvider, address(adminProvider_));
    }

    /// @notice Set the wrapper factory
    /// @param wrapperFactory_ New wrapper factory address
    function setWrapperFactory(WrapperFactory wrapperFactory_) external onlyOwner {
        require(address(wrapperFactory_) != address(0), ZeroAddressWrapperFactory());
        address oldWrapperFactory = address(wrapperFactory);
        wrapperFactory = wrapperFactory_;
        emit WrapperFactoryUpdated(oldWrapperFactory, address(wrapperFactory_));
    }

    /// @notice Set the confidential token factory
    /// @param confidentialTokenFactory_ New confidential token factory address
    function setConfidentialTokenFactory(RegulatedERC7984UpgradeableFactory confidentialTokenFactory_) external onlyOwner {
        require(address(confidentialTokenFactory_) != address(0), ZeroAddressConfidentialTokenFactory());
        address oldConfidentialTokenFactory = address(confidentialTokenFactory);
        confidentialTokenFactory = confidentialTokenFactory_;
        emit ConfidentialTokenFactoryUpdated(oldConfidentialTokenFactory, address(confidentialTokenFactory_));
    }
}

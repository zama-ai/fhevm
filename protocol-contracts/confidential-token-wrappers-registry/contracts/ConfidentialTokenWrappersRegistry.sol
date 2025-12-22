// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

/**
 * @title ConfidentialTokenWrappersRegistry
 * @notice  registry contract mapping ERC-20 tokens to their corresponding confidential FHEVM wrapper
 * extensions of ERC-7984.
 * @dev This contract allows an owner to register new entries and flag revoked ones.
 */
contract ConfidentialTokenWrappersRegistry is Ownable2StepUpgradeable, UUPSUpgradeable {
    /// @notice Struct to represent a (token, confidential token, is revoked) tuple.
    struct ConfidentialTokenPair {
        /// @notice The address of the token.
        address tokenAddress;
        /// @notice The address of the confidential token.
        address confidentialTokenAddress;
        /// @notice If the confidential token has been revoked.
        bool isRevoked;
    }

    /// @custom:storage-location erc7201:fhevm_protocol.storage.ConfidentialTokenWrappersRegistry
    struct ConfidentialTokenWrappersRegistryStorage {
        /// @notice Mapping from token address to confidential token address.
        mapping(address tokenAddress => address confidentialTokenAddress) _tokensToConfidentialTokens;
        /// @notice Mapping from confidential token address to token address.
        mapping(address confidentialTokenAddress => address tokenAddress) _confidentialTokensToTokens;
        /// @notice If a confidential token has been revoked.
        mapping(address confidentialTokenAddress => bool isRevoked) _revokedConfidentialTokens;
        /// @notice Index of registered tokens.
        mapping(address tokenAddress => uint256 index) _tokenIndex;
        /// @notice Registered token and confidential token pairs.
        ConfidentialTokenPair[] _tokenConfidentialTokenPairs;
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm_protocol.storage.ConfidentialTokenWrappersRegistry")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_STORAGE_LOCATION =
        0xc361bd0b1d7584416623b46edb98317525b8de8e557ab49cee21f14d6752da00;

    /// @notice Error thrown when the token address is zero.
    error TokenZeroAddress();

    /// @notice Error thrown when the confidential token address is zero.
    error ConfidentialTokenZeroAddress();

    /// @notice Error thrown when a confidential token is not a valid ERC7984 confidential token
    /// because it does not support the ERC7984 interface (0x4958f2a4).
    error NotERC7984(address confidentialTokenAddress);

    /// @notice Error thrown when a confidential token does not support ERC165's `supportsInterface`.
    error ConfidentialTokenDoesNotSupportERC165(address confidentialTokenAddress);

    /// @notice Error thrown when a confidential token is already associated with a token.
    error ConfidentialTokenAlreadyAssociatedWithToken(address tokenAddress, address existingConfidentialTokenAddress);

    /// @notice Error thrown when a token is already associated with a confidential token.
    error TokenAlreadyAssociatedWithConfidentialToken(address tokenAddress, address existingConfidentialTokenAddress);

    /// @notice Error thrown when a confidential token is revoked.
    error RevokedConfidentialToken(address confidentialTokenAddress);

    /// @notice Error thrown when no token is associated with a confidential token.
    error NoTokenAssociatedWithConfidentialToken(address confidentialTokenAddress);

    /// @notice Error thrown when a confidential token is not revoked.
    error ConfidentialTokenNotRevoked(address confidentialTokenAddress);

    /// @notice Emitted when a token is registered and associated with a confidential token.
    event ConfidentialTokenRegistered(address indexed tokenAddress, address indexed confidentialTokenAddress);

    /// @notice Emitted when a confidential token is revoked and the association with a token is removed.
    event ConfidentialTokenRevoked(address indexed tokenAddress, address indexed confidentialTokenAddress);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initialize the ConfidentialTokenWrappersRegistry contract.
     * @param initialOwner The initial owner of the contract.
     */
    function initialize(address initialOwner) public initializer {
        __Ownable_init(initialOwner);
    }

    /**
     * @notice Register a new ERC20 token and associate it with a validated corresponding ERC7984 confidential token.
     * @param tokenAddress The address of the ERC20 token contract to register.
     */
    function registerConfidentialToken(address tokenAddress, address confidentialTokenAddress) external onlyOwner {
        if (tokenAddress == address(0)) {
            revert TokenZeroAddress();
        }
        if (confidentialTokenAddress == address(0)) {
            revert ConfidentialTokenZeroAddress();
        }

        // The confidential token must support the ERC7984 interface (0x4958f2a4) via
        // `supportsInterface` from ERC165.
        // See https://eips.ethereum.org/EIPS/eip-7984.
        _validateERC7984(confidentialTokenAddress);

        // The confidential token must not be already associated with a token.
        (, address existingTokenAddress) = getTokenAddress(confidentialTokenAddress);
        if (existingTokenAddress != address(0)) {
            revert ConfidentialTokenAlreadyAssociatedWithToken(confidentialTokenAddress, existingTokenAddress);
        }

        // The token must not be already associated with a confidential token.
        (, address existingConfidentialTokenAddress) = getConfidentialTokenAddress(tokenAddress);
        if (existingConfidentialTokenAddress != address(0)) {
            revert TokenAlreadyAssociatedWithConfidentialToken(tokenAddress, existingConfidentialTokenAddress);
        }

        ConfidentialTokenWrappersRegistryStorage storage $ = _getConfidentialTokenWrappersRegistryStorage();

        // Register the token and confidential token mappings.
        $._tokensToConfidentialTokens[tokenAddress] = confidentialTokenAddress;
        $._confidentialTokensToTokens[confidentialTokenAddress] = tokenAddress;

        // Register the token and confidential token pairs in the array and keep track of their indexes.
        $._tokenConfidentialTokenPairs.push(
            ConfidentialTokenPair({
                tokenAddress: tokenAddress,
                confidentialTokenAddress: confidentialTokenAddress,
                isRevoked: false
            })
        );
        $._tokenIndex[tokenAddress] = $._tokenConfidentialTokenPairs.length - 1;

        emit ConfidentialTokenRegistered(tokenAddress, confidentialTokenAddress);
    }

    /**
     * @notice Revoke an ERC7984 confidential token.
     * @param confidentialTokenAddress The address of the ERC7984 confidential token to revoke.
     */
    function revokeConfidentialToken(address confidentialTokenAddress) external onlyOwner {
        if (confidentialTokenAddress == address(0)) {
            revert ConfidentialTokenZeroAddress();
        }

        // The confidential token must not be already revoked.
        if (isConfidentialTokenRevoked(confidentialTokenAddress)) {
            revert RevokedConfidentialToken(confidentialTokenAddress);
        }

        // The confidential token must be associated with a token.
        (, address tokenAddress) = getTokenAddress(confidentialTokenAddress);
        if (tokenAddress == address(0)) {
            revert NoTokenAssociatedWithConfidentialToken(confidentialTokenAddress);
        }

        ConfidentialTokenWrappersRegistryStorage storage $ = _getConfidentialTokenWrappersRegistryStorage();

        $._revokedConfidentialTokens[confidentialTokenAddress] = true;

        // Set token's confidential token address to zero to indicate that it has been revoked.
        uint256 index = $._tokenIndex[tokenAddress];
        $._tokenConfidentialTokenPairs[index].isRevoked = true;

        emit ConfidentialTokenRevoked(tokenAddress, confidentialTokenAddress);
    }
    /**
     * @notice Returns the address of the confidential token associated with a token. A null address
     * is returned if no confidential token has been registered for the token.
     * @param tokenAddress The address of the token.
     * @return True if the confidential token has been revoked, false otherwise.
     * @return The address of the confidential token.
     */
    function getConfidentialTokenAddress(address tokenAddress) public view returns (bool, address) {
        address confidentialTokenAddress = _getConfidentialTokenWrappersRegistryStorage()._tokensToConfidentialTokens[
            tokenAddress
        ];
        bool isRevoked = isConfidentialTokenRevoked(confidentialTokenAddress);
        return (isRevoked, confidentialTokenAddress);
    }

    /**
     * @notice Returns the address of the token associated with a confidential token.
     * A null address is returned if the confidential token has not been registered for any token.
     * @param confidentialTokenAddress The address of the confidential token.
     * @return True if the confidential token has been revoked, false otherwise.
     * @return The address of the token.
     */
    function getTokenAddress(address confidentialTokenAddress) public view returns (bool, address) {
        bool isRevoked = isConfidentialTokenRevoked(confidentialTokenAddress);
        address tokenAddress = _getConfidentialTokenWrappersRegistryStorage()._confidentialTokensToTokens[
            confidentialTokenAddress
        ];
        return (isRevoked, tokenAddress);
    }

    /**
     * @notice Returns the array of (token address, confidential token address, is revoked) tuples.
     * A tuple containing a revoked confidential token is kept in the array and addresses are not
     * affected, only the isRevoked flag is set to true.
     * @return The array of (token address, confidential token address, is revoked) tuples.
     */
    function getTokenConfidentialTokenPairs() public view returns (ConfidentialTokenPair[] memory) {
        return _getConfidentialTokenWrappersRegistryStorage()._tokenConfidentialTokenPairs;
    }

    /**
     * @notice Returns true if a confidential token has been revoked, false otherwise.
     * @param confidentialTokenAddress The address of the confidential token.
     * @return True if the confidential token has been revoked, false otherwise.
     */
    function isConfidentialTokenRevoked(address confidentialTokenAddress) public view returns (bool) {
        return _getConfidentialTokenWrappersRegistryStorage()._revokedConfidentialTokens[confidentialTokenAddress];
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    /**
     * @notice Validates that a confidential token supports the ERC7984 interface (0x4958f2a4) via
     * `supportsInterface` from ERC165.
     * See https://eips.ethereum.org/EIPS/eip-7984.
     * @param confidentialTokenAddress The address of the confidential token to validate.
     */
    function _validateERC7984(address confidentialTokenAddress) internal view {
        (bool success, bytes memory returnData) = confidentialTokenAddress.staticcall(
            abi.encodeWithSelector(IERC165.supportsInterface.selector, bytes4(0x4958f2a4))
        );

        // Check if the address supports the `supportsInterface` function
        if (!success || returnData.length < 32) {
            revert ConfidentialTokenDoesNotSupportERC165(confidentialTokenAddress);
        }

        // Check if the confidential token supports the ERC7984 interface (0x4958f2a4).
        bool isERC7984InterfaceSupported = abi.decode(returnData, (bool));
        if (!isERC7984InterfaceSupported) {
            revert NotERC7984(confidentialTokenAddress);
        }
    }

    function _getConfidentialTokenWrappersRegistryStorage()
        private
        pure
        returns (ConfidentialTokenWrappersRegistryStorage storage $)
    {
        assembly {
            $.slot := CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_STORAGE_LOCATION
        }
    }
}

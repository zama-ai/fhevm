// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";

/**
 * @title ConfidentialTokensRegistry
 * @notice A registry contract to map ERC20 token addresses to their ERC7984 confidential token addresses.
 * @dev This contract allows an owner to register new entries and flag revoked ones.
 */
contract ConfidentialTokensRegistry is Ownable2StepUpgradeable, UUPSUpgradeable {
    /// @custom:storage-location erc7201:fhevm_protocol.storage.ConfidentialTokensRegistry
    struct ConfidentialTokensRegistryStorage {
        mapping(address tokenAddress => address confidentialTokenAddress) _tokensToConfidentialTokens;
        mapping(address confidentialTokenAddress => address tokenAddress) _confidentialTokensToTokens;
        mapping(address confidentialTokenAddress => bool isRevoked) _revokedConfidentialTokens;
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm_protocol.storage.ConfidentialTokensRegistry")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CONFIDENTIAL_TOKENS_REGISTRY_STORAGE_LOCATION =
        0x25094496394205337c7da64eaca0c35bf780125467d04de96cd0ee9b701d6c00;

    /// @notice Error thrown when the token address is zero.
    error TokenZeroAddress();

    /// @notice Error thrown when the confidential token address is zero.
    error ConfidentialTokenZeroAddress();

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

    /// @notice Emitted when a confidential token no longer revoked and thus is allowed to be
    /// associated with a token again.
    event ConfidentialTokenReinstated(address indexed confidentialTokenAddress);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initialize the ConfidentialTokensRegistry contract.
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

        // The confidential token must not be revoked.
        if (isConfidentialTokenRevoked(confidentialTokenAddress)) {
            revert RevokedConfidentialToken(confidentialTokenAddress);
        }

        // The confidential token must not be already associated with a token.
        address existingTokenAddress = getTokenAddress(confidentialTokenAddress);
        if (existingTokenAddress != address(0)) {
            revert ConfidentialTokenAlreadyAssociatedWithToken(confidentialTokenAddress, existingTokenAddress);
        }

        // The token must not be already associated with a confidential token.
        address existingConfidentialTokenAddress = getConfidentialTokenAddress(tokenAddress);
        if (existingConfidentialTokenAddress != address(0)) {
            revert TokenAlreadyAssociatedWithConfidentialToken(tokenAddress, existingConfidentialTokenAddress);
        }

        ConfidentialTokensRegistryStorage storage $ = _getConfidentialTokensRegistryStorage();

        $._tokensToConfidentialTokens[tokenAddress] = confidentialTokenAddress;
        $._confidentialTokensToTokens[confidentialTokenAddress] = tokenAddress;

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
        address tokenAddress = getTokenAddress(confidentialTokenAddress);
        if (tokenAddress == address(0)) {
            revert NoTokenAssociatedWithConfidentialToken(confidentialTokenAddress);
        }

        ConfidentialTokensRegistryStorage storage $ = _getConfidentialTokensRegistryStorage();

        delete $._confidentialTokensToTokens[confidentialTokenAddress];
        delete $._tokensToConfidentialTokens[tokenAddress];
        $._revokedConfidentialTokens[confidentialTokenAddress] = true;

        emit ConfidentialTokenRevoked(tokenAddress, confidentialTokenAddress);
    }

    /**
     * @notice Reinstate an ERC7984 confidential token in case it was revoked. It does not restore
     * the association with a token, it only allows it to be associated with one again.
     * @param confidentialTokenAddress The address of the ERC7984 confidential token to reinstate.
     */
    function reinstateConfidentialToken(address confidentialTokenAddress) external onlyOwner {
        ConfidentialTokensRegistryStorage storage $ = _getConfidentialTokensRegistryStorage();
        if (confidentialTokenAddress == address(0)) {
            revert ConfidentialTokenZeroAddress();
        }

        // The confidential token must be revoked.
        if (!isConfidentialTokenRevoked(confidentialTokenAddress)) {
            revert ConfidentialTokenNotRevoked(confidentialTokenAddress);
        }

        $._revokedConfidentialTokens[confidentialTokenAddress] = false;
        emit ConfidentialTokenReinstated(confidentialTokenAddress);
    }

    /**
     * @notice Returns the address of the confidential token associated with a token.
     * @param tokenAddress The address of the token.
     * @return The address of the confidential token.
     */
    function getConfidentialTokenAddress(address tokenAddress) public view returns (address) {
        return _getConfidentialTokensRegistryStorage()._tokensToConfidentialTokens[tokenAddress];
    }

    /**
     * @notice Returns the address of the token associated with a confidential token.
     * @param confidentialTokenAddress The address of the confidential token.
     * @return The address of the token.
     */
    function getTokenAddress(address confidentialTokenAddress) public view returns (address) {
        return _getConfidentialTokensRegistryStorage()._confidentialTokensToTokens[confidentialTokenAddress];
    }

    /**
     * @notice Returns true if a confidential token is revoked, false otherwise. A revoked confidential token
     * is not allowed to be associated with a token again.
     * @param confidentialTokenAddress The address of the confidential token.
     * @return True if the confidential token is revoked, false otherwise.
     */
    function isConfidentialTokenRevoked(address confidentialTokenAddress) public view returns (bool) {
        return _getConfidentialTokensRegistryStorage()._revokedConfidentialTokens[confidentialTokenAddress];
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    function _getConfidentialTokensRegistryStorage()
        private
        pure
        returns (ConfidentialTokensRegistryStorage storage $)
    {
        assembly {
            $.slot := CONFIDENTIAL_TOKENS_REGISTRY_STORAGE_LOCATION
        }
    }
}

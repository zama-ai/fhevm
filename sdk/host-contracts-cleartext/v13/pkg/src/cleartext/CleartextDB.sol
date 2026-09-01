// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ICleartextDB} from "./ICleartextDB.sol";
import {UUPSUpgradeableEmptyProxy} from "../contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable, aclAdd} from "../contracts/shared/ACLOwnable.sol";

/**
 * @title CleartextDB
 * @notice Shared handle → cleartext-value store for the cleartext execution mocks. Extracted from
 *         `CleartextFHEVMExecutor` so that several executors (e.g. once the base executor must be
 *         split across contracts for size) can share a single cleartext database.
 * @dev Upgradeable (UUPS). Writes are restricted to registered writers (the `CleartextArithmetic`
 *      layer), administered by the ACL owner — the same shape as `PauserSet`.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CleartextDB is ICleartextDB, UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @dev Name of the contract, used in `getVersion`.
    string private constant CONTRACT_NAME = "CleartextDB";

    /// @dev Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @dev Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @dev Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number used in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and any future `reinitializeVX` method.
     */
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.CleartextDB")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CLEARTEXT_DB_STORAGE_LOCATION =
        0xe4f42c2725f815cf40b6d15eed8801860e2cc12665a1b8bde61616462018ef00;

    /// @custom:storage-location erc7201:fhevm.storage.CleartextDB
    struct CleartextDBStorage {
        mapping(bytes32 handle => uint256 value) plaintexts;
        mapping(address account => bool isWriter) writers;
    }

    /// @dev Restricts a function to registered writers.
    modifier onlyWriter() {
        if (!_getCleartextDBStorage().writers[msg.sender]) {
            revert NotWriter(msg.sender);
        }
        _;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the store from an empty proxy and seeds the first writer.
     * @param initialWriter The address granted initial write access (the CleartextArithmetic proxy).
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(address initialWriter)
        public
        virtual
        onlyFromEmptyProxy
        reinitializer(REINITIALIZER_VERSION)
    {
        if (initialWriter == address(0)) {
            revert InvalidNullWriter();
        }
        _getCleartextDBStorage().writers[initialWriter] = true;
        emit AddWriter(initialWriter);
    }

    /// @notice Getter for the name and version of the contract.
    /// @return string Name and the version of the contract.
    function getVersion() external pure virtual returns (string memory) {
        return string(
            abi.encodePacked(
                CONTRACT_NAME,
                " v",
                Strings.toString(MAJOR_VERSION),
                ".",
                Strings.toString(MINOR_VERSION),
                ".",
                Strings.toString(PATCH_VERSION)
            )
        );
    }

    /// @inheritdoc ICleartextDB
    function get(bytes32 handle) external view override returns (uint256) {
        return _getCleartextDBStorage().plaintexts[handle];
    }

    /// @inheritdoc ICleartextDB
    function set(bytes32 handle, uint256 value) external override onlyWriter {
        _getCleartextDBStorage().plaintexts[handle] = value;
    }

    /// @inheritdoc ICleartextDB
    function addWriter(address account) external override onlyACLOwner {
        if (account == address(0)) revert InvalidNullWriter();
        CleartextDBStorage storage $ = _getCleartextDBStorage();
        if ($.writers[account]) revert AccountAlreadyWriter(account);
        $.writers[account] = true;
        emit AddWriter(account);
    }

    /// @inheritdoc ICleartextDB
    function removeWriter(address account) external override onlyACLOwner {
        if (account == address(0)) revert InvalidNullWriter();
        CleartextDBStorage storage $ = _getCleartextDBStorage();
        if (!$.writers[account]) revert AccountNotWriter(account);
        $.writers[account] = false;
        emit RemoveWriter(account);
    }

    /// @inheritdoc ICleartextDB
    function isWriter(address account) external view override returns (bool) {
        return _getCleartextDBStorage().writers[account];
    }

    /// @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    function _getCleartextDBStorage() private pure returns (CleartextDBStorage storage $) {
        assembly {
            $.slot := CLEARTEXT_DB_STORAGE_LOCATION
        }
    }

    /**
     * @notice Getter function for the ACL contract address.
     */
    function getACLAddress() public view virtual returns (address) {
        return aclAdd;
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {MulticallUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/MulticallUpgradeable.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {fhevmExecutorAdd, pauserSetAdd} from "../addresses/FHEVMHostAddresses.sol";
import {IPauserSet} from "./interfaces/IPauserSet.sol";

import {ACLEvents} from "./ACLEvents.sol";

/**
 * @title ACL.
 * @notice The ACL (Access Control List) is a permission management system designed to control who can access, compute on,
 * or decrypt encrypted values in fhEVM. By defining and enforcing these permissions, the ACL ensures that encrypted data remains
 * secure while still being usable within authorized contexts.
 */
contract ACL is
    UUPSUpgradeableEmptyProxy,
    Ownable2StepUpgradeable,
    PausableUpgradeable,
    ACLEvents,
    MulticallUpgradeable
{
    /**
     * @notice Returned if a delegation or revoke has already been done in a same block.
     * @param delegator The address of the account that delegates access to its handles.
     * @param delegate The address of the account that receives the delegation.
     * @param contractAddress The contract address to delegate access to.
     * @param blockNumber The block number.
     */
    error AlreadyDelegatedOrRevokedInSameBlock(
        address delegator,
        address delegate,
        address contractAddress,
        uint256 blockNumber
    );

    /**
     * @notice Returned if the delegate address is the same as the contract address.
     * @param contractAddress The contract address to delegate access to.
     */
    error DelegateCannotBeContractAddress(address contractAddress);

    /// @notice Returned if the requested expiry date for delegation is after the next year.
    error ExpiryDateAfterOneYear();

    /**
     * @notice Returned if the requested expiry date was already set to same expiry for (delegate,contractAddress).
     * @param delegator The address of the account that delegates access to its handles.
     * @param delegate The address of the account that receives the delegation.
     * @param contractAddress The contract address to delegate access to.
     * @param expiryDate The expiration date for the intended delegation.
     */
    error ExpiryDateAlreadySetToSameValue(
        address delegator,
        address delegate,
        address contractAddress,
        uint256 expiryDate
    );

    /// @notice Returned if the requested expiry date array is before the next hour.
    error ExpiryDateBeforeOneHour();

    /// @notice Returned if the handlesList array is empty.
    error HandlesListIsEmpty();

    /**
     * @notice Returned if the the delegate contract is not already delegate for sender & delegator addresses.
     * @param delegator The address of the account that delegates access to its handles.
     * @param delegate The address of the account that receives the delegation.
     * @param contractAddress The contract address to delegate access to.
     */
    error NotDelegatedYet(address delegator, address delegate, address contractAddress);

    /// @notice Returned if the sender address is not allowed to pause the contract.
    error NotPauser(address sender);

    /**
     * @notice Returned if the sender address is the same as the contract address.
     * @param contractAddress The contract address to delegate access to.
     */
    error SenderCannotBeContractAddress(address contractAddress);

    /**
     * @notice Returned if the sender address is the same as the delegate address.
     * @param delegate The address of the account that receives the delegation.
     */
    error SenderCannotBeDelegate(address delegate);

    /**
     * @notice Returned if the sender address is not allowed for allow operations.
     * @param sender The address of the account that is not allowed.
     */
    error SenderNotAllowed(address sender);

    /**
     * @notice Struct that represents a delegation.
     * @dev The `delegationCounter` is incremented at each delegation or revocation
     *      to allow off-chain clients to track changes.
     */
    struct Delegation {
        /// @notice Date when the delegation expires.
        uint64 expiryDate;
        /// @notice The last block number when a delegation or revocation happened.
        uint64 lastBlockDelegateOrRevoke;
        /// @notice Counter that tracks the order of each delegation or revocation.
        uint64 delegationCounter;
    }

    /// @custom:storage-location erc7201:fhevm.storage.ACL
    struct ACLStorage {
        mapping(bytes32 handle => mapping(address account => bool isAllowed)) persistedAllowedPairs;
        mapping(bytes32 handle => bool isAllowedForDecryption) allowedForDecryption;
        mapping(address account => mapping(address delegate => mapping(address contractAddress => Delegation delegation))) delegations;
    }

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "ACL";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 2;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice FHEVMExecutor address.
    address private constant fhevmExecutorAddress = fhevmExecutorAdd;

    /// @notice PauserSet contract.
    IPauserSet private constant PAUSER_SET = IPauserSet(pauserSetAdd);

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 3;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.ACL")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant ACLStorageLocation = 0xa688f31953c2015baaf8c0a488ee1ee22eb0e05273cc1fd31ea4cbee42febc00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());
        __Pausable_init();
    }

    /**
     * @notice Re-initializes the contract from V1.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Allows the use of `handle` for the address `account`.
     * @dev The caller must be allowed to use `handle` for allow() to succeed. If not, allow() reverts.
     * @param handle Handle.
     * @param account Address of the account.
     */
    function allow(bytes32 handle, address account) public virtual whenNotPaused {
        ACLStorage storage $ = _getACLStorage();
        if (!isAllowed(handle, msg.sender)) {
            revert SenderNotAllowed(msg.sender);
        }
        $.persistedAllowedPairs[handle][account] = true;
        emit Allowed(msg.sender, account, handle);
    }

    /**
     * @notice Allows a list of handles to be decrypted.
     * @param handlesList List of handles.
     */
    function allowForDecryption(bytes32[] memory handlesList) public virtual whenNotPaused {
        uint256 lenHandlesList = handlesList.length;
        if (lenHandlesList == 0) {
            revert HandlesListIsEmpty();
        }

        ACLStorage storage $ = _getACLStorage();
        for (uint256 k = 0; k < lenHandlesList; k++) {
            bytes32 handle = handlesList[k];
            if (!isAllowed(handle, msg.sender)) {
                revert SenderNotAllowed(msg.sender);
            }
            $.allowedForDecryption[handle] = true;
        }
        emit AllowedForDecryption(msg.sender, handlesList);
    }

    /**
     * @notice Allows the use of `handle` by address `account` for this transaction.
     * @dev The caller must be allowed to use `handle` for allowTransient() to succeed.
     * If not, allowTransient() reverts. The Coprocessor contract can always `allowTransient`, contrarily to `allow`.
     * @param handle Handle.
     * @param account Address of the account.
     */
    function allowTransient(bytes32 handle, address account) public virtual whenNotPaused {
        if (msg.sender != fhevmExecutorAddress) {
            if (!isAllowed(handle, msg.sender)) {
                revert SenderNotAllowed(msg.sender);
            }
        }
        bytes32 key = keccak256(abi.encodePacked(handle, account));
        assembly {
            tstore(key, 1)
            let length := tload(0)
            let lengthPlusOne := add(length, 1)
            tstore(lengthPlusOne, key)
            tstore(0, lengthPlusOne)
        }
    }

    /**
     * @notice Delegates an account the access to handles for user decryption, for instance, in the context of account
     * abstraction for issuing user decryption requests from a smart contract account.
     * @param delegate The address of the account that receives the delegation.
     * @param contractAddress The contract address to delegate access to.
     * @param expiryDate Expiry date in seconds, between 1 hour and 1 year in the future.
     */
    function delegateForUserDecryption(
        address delegate,
        address contractAddress,
        uint64 expiryDate
    ) public virtual whenNotPaused {
        if (expiryDate < block.timestamp + 1 hours) {
            revert ExpiryDateBeforeOneHour();
        }
        if (expiryDate > block.timestamp + 365 days) {
            revert ExpiryDateAfterOneYear();
        }

        ACLStorage storage $ = _getACLStorage();
        Delegation storage delegation = $.delegations[msg.sender][delegate][contractAddress];
        uint256 blockNumber = block.number;

        if (delegation.lastBlockDelegateOrRevoke == blockNumber) {
            revert AlreadyDelegatedOrRevokedInSameBlock(msg.sender, delegate, contractAddress, blockNumber);
        }

        // Set the last block where the delegation happened.
        delegation.lastBlockDelegateOrRevoke = uint64(blockNumber);

        if (contractAddress == msg.sender) {
            revert SenderCannotBeContractAddress(contractAddress);
        }
        if (delegate == msg.sender) {
            revert SenderCannotBeDelegate(delegate);
        }
        if (delegate == contractAddress) {
            revert DelegateCannotBeContractAddress(contractAddress);
        }

        uint64 newExpiryDate = expiryDate;
        uint64 oldExpiryDate = delegation.expiryDate;
        if (oldExpiryDate == newExpiryDate) {
            revert ExpiryDateAlreadySetToSameValue(msg.sender, delegate, contractAddress, oldExpiryDate);
        }

        // Set the delegation expiry date.
        delegation.expiryDate = newExpiryDate;

        emit DelegatedForUserDecryption(
            msg.sender,
            delegate,
            contractAddress,
            delegation.delegationCounter++,
            oldExpiryDate,
            newExpiryDate
        );
    }

    /**
     * @notice Revokes the access to handles for user decryption delegated to an account.
     * @param delegate The address of the account that receives the delegation.
     * @param contractAddress The contract address to delegate access to.
     */
    function revokeDelegationForUserDecryption(address delegate, address contractAddress) public virtual whenNotPaused {
        ACLStorage storage $ = _getACLStorage();
        Delegation storage delegation = $.delegations[msg.sender][delegate][contractAddress];
        uint256 blockNumber = block.number;

        if (delegation.lastBlockDelegateOrRevoke == blockNumber) {
            revert AlreadyDelegatedOrRevokedInSameBlock(msg.sender, delegate, contractAddress, blockNumber);
        }

        // Set the last block where the revocation happened.
        delegation.lastBlockDelegateOrRevoke = uint64(blockNumber);

        uint64 oldExpiryDate = delegation.expiryDate;
        if (oldExpiryDate == 0) {
            revert NotDelegatedYet(msg.sender, delegate, contractAddress);
        }

        // Reset the delegation expiry date.
        delegation.expiryDate = 0;

        emit RevokedDelegationForUserDecryption(
            msg.sender,
            delegate,
            contractAddress,
            delegation.delegationCounter++,
            oldExpiryDate
        );
    }

    /**
     * @dev Triggers stopped state.
     * Only a pauser address can pause.
     * The contract must not be paused.
     */
    function pause() external virtual {
        if (!PAUSER_SET.isPauser(msg.sender)) {
            revert NotPauser(msg.sender);
        }
        _pause();
    }

    /**
     * @dev Returns to normal state.
     * Only owner can unpause.
     * The contract must be paused.
     */
    function unpause() external virtual onlyOwner {
        _unpause();
    }

    /**
     * @notice Get the expiry date of a delegation for user decryption.
     * @param delegate The address of the account that receives the delegation.
     * @param delegator The address of the account that delegates access to its handles.
     * @param contractAddress The contract address to delegate access to.
     * @return expiryDate the expiryDate (0 means delegation is inactive).
     */
    function getUserDecryptionDelegationExpiryDate(
        address delegate,
        address delegator,
        address contractAddress
    ) public view virtual returns (uint64) {
        ACLStorage storage $ = _getACLStorage();
        Delegation storage delegation = $.delegations[delegator][delegate][contractAddress];
        return delegation.expiryDate;
    }

    /**
     * @notice Checks whether the account is allowed to use the handle in the
     * same transaction (transient).
     * @param handle Handle.
     * @param account Address of the account.
     * @return isAllowedTransient Whether the account can access transiently the handle.
     */
    function allowedTransient(bytes32 handle, address account) public view virtual returns (bool) {
        bool isAllowedTransient;
        bytes32 key = keccak256(abi.encodePacked(handle, account));
        assembly {
            isAllowedTransient := tload(key)
        }
        return isAllowedTransient;
    }

    /**
     * @notice Getter function for the FHEVMExecutor contract address.
     * @return fhevmExecutorAddress Address of the FHEVMExecutor.
     */
    function getFHEVMExecutorAddress() public view virtual returns (address) {
        return fhevmExecutorAddress;
    }

    /**
     * @notice Getter function for the PauserSet contract address.
     * @return pauserSetAddress Address of the PauserSet contract.
     */
    function getPauserSetAddress() public view virtual returns (address) {
        return address(PAUSER_SET);
    }

    /**
     * @notice Returns whether the account is allowed to use the `handle`, either due to
     * allowTransient() or allow().
     * @param handle Handle.
     * @param account Address of the account.
     * @return isAllowed Whether the account can access the handle.
     */
    function isAllowed(bytes32 handle, address account) public view virtual returns (bool) {
        return allowedTransient(handle, account) || persistAllowed(handle, account);
    }

    /**
     * @notice Checks whether a handle is allowed for decryption.
     * @param handle Handle.
     * @return isAllowed Whether the handle is allowed for decryption.
     */
    function isAllowedForDecryption(bytes32 handle) public view virtual returns (bool) {
        ACLStorage storage $ = _getACLStorage();
        return $.allowedForDecryption[handle];
    }

    /**
     * @notice Returns whether an account is delegated to access the handle for user decryption.
     * @param delegate The address of the account that receives the delegation.
     * @param delegator The address of the account that delegates access to its handles.
     * @param handle The handle to check for delegated user decryption.
     * @param contractAddress The contract address to delegate access to.
     * @return isDelegatedForUserDecryption Whether the handle can be accessed for delegated user decryption.
     */
    function isHandleDelegatedForUserDecryption(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle
    ) public view virtual returns (bool) {
        ACLStorage storage $ = _getACLStorage();
        Delegation storage delegation = $.delegations[delegator][delegate][contractAddress];
        return
            $.persistedAllowedPairs[handle][delegator] &&
            $.persistedAllowedPairs[handle][contractAddress] &&
            delegation.expiryDate >= block.timestamp;
    }

    /**
     * @notice Returns `true` if address `a` is allowed to use `c` and `false` otherwise.
     * @param handle Handle.
     * @param account Address of the account.
     * @return isAllowed Whether the account can access the handle.
     */
    function persistAllowed(bytes32 handle, address account) public view virtual returns (bool) {
        ACLStorage storage $ = _getACLStorage();
        return $.persistedAllowedPairs[handle][account];
    }

    /**
     * @notice Returns wether specified account is in the set of pausers.
     * @param account The address of the account.
     */
    function isPauser(address account) external view virtual returns (bool) {
        return PAUSER_SET.isPauser(account);
    }

    /**
     * @dev This function removes the transient allowances, which could be useful for integration with
     * Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
     */
    function cleanTransientStorage() external virtual {
        assembly {
            let length := tload(0)
            tstore(0, 0)
            let lengthPlusOne := add(length, 1)
            for {
                let i := 1
            } lt(i, lengthPlusOne) {
                i := add(i, 1)
            } {
                let handle := tload(i)
                tstore(i, 0)
                tstore(handle, 0)
            }
        }
    }

    /**
     * @notice Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
     */
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
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

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /**
     * @dev Returns the ACL storage location.
     */
    function _getACLStorage() internal pure returns (ACLStorage storage $) {
        assembly {
            $.slot := ACLStorageLocation
        }
    }
}

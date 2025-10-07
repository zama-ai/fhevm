// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Multicall} from "@openzeppelin/contracts/utils/Multicall.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {fhevmExecutorAdd, pauserSetAdd} from "../addresses/FHEVMHostAddresses.sol";
import {IPauserSet} from "./interfaces/IPauserSet.sol";

import {ACLEvents} from "./ACLEvents.sol";

/**
 * @title  ACL
 * @notice The ACL (Access Control List) is a permission management system designed to control who can access, compute on,
 * or decrypt encrypted values in fhEVM. By defining and enforcing these permissions, the ACL ensures that encrypted data remains
 * secure while still being usable within authorized contexts.
 */
contract ACL is UUPSUpgradeableEmptyProxy, Ownable2StepUpgradeable, PausableUpgradeable, ACLEvents, Multicall {
    /// @notice Returned if the delegatee contract is already delegatee for sender & delegator addresses.
    /// @param delegator delegator address.
    /// @param delegatee delegatee address.
    /// @param contractAddress contract address.
    error AlreadyDelegatedOrRevokedInSameBlock(address delegator, address delegatee, address contractAddress);

    /// @notice Returned if the delegatee is the contract address.
    /// @param contractAddress contract address.
    error DelegateeCannotBeContractAddress(address contractAddress);

    /// @notice Returned if the requested expiry date array is after the next year.
    error ExpiryDateAfterOneYear();

    /// @notice Returned if the requested expiry date was already set to same expiry for (delegatee,contractAddress).
    /// @param delegator the delegator address.
    /// @param delegatee the delegatee address.
    /// @param contractAddress contract address.
    /// @param expiryDate the expiry date.
    error ExpiryDateAlreadySetToSameValue(
        address delegator,
        address delegatee,
        address contractAddress,
        uint256 expiryDate
    );

    /// @notice Returned if the requested expiry date array is before the next hour.
    error ExpiryDateBeforeOneHour();

    /// @notice Returned if the handlesList array is empty.
    error HandlesListIsEmpty();

    /// @notice Returned if the the delegatee contract is not already delegatee for sender & delegator addresses.
    /// @param delegator delegator address.
    /// @param delegatee delegatee address.
    /// @param contractAddress contract address.
    error NotDelegatedYet(address delegator, address delegatee, address contractAddress);

    /// @notice Returned if the sender address is not allowed to pause the contract.
    error NotPauser(address sender);

    /// @notice Returned if the sender is the contract address.
    /// @param contractAddress contract address.
    error SenderCannotBeContractAddress(address contractAddress);

    /// @notice Returned if the sender is the delegatee address.
    /// @param delegatee delegatee address.
    error SenderCannotBeDelegatee(address delegatee);

    /// @notice Returned if the sender address is not allowed for allow operations.
    /// @param sender Sender address.
    error SenderNotAllowed(address sender);

    /// @custom:storage-location erc7201:fhevm.storage.ACL
    struct ACLStorage {
        mapping(bytes32 handle => mapping(address account => bool isAllowed)) persistedAllowedPairs;
        mapping(bytes32 handle => bool isAllowedForDecryption) allowedForDecryption;
        /// @notice: TODO deprecate delegates mapping for mainnet
        mapping(address account => mapping(address delegatee => mapping(address contractAddress => bool isDelegate))) delegates;
        /// @notice: TODO: pack following two mappings (expiryDates and lastBlockDelegateOrRevoke) to save gas
        mapping(address account => mapping(address delegatee => mapping(address contractAddress => uint256 expiryDate))) expiryDates;
        mapping(address account => mapping(address delegatee => mapping(address contractAddress => uint256 expiryDate))) lastBlockDelegateOrRevoke;
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
     * @notice Delegates the access of handles, for instance, in the context of account
     *  abstraction for issuing user decryption requests from a smart contract account.
     * @param delegatee Delegatee address.
     * @param contractAddress Contract address.
     * @param expiryDate Expiry date in seconds, between 1 hour and 1 year in the future.
     */
    function delegateAccount(
        address delegatee,
        address contractAddress,
        uint256 expiryDate
    ) public virtual whenNotPaused {
        if (expiryDate < block.timestamp + 1 hours) revert ExpiryDateBeforeOneHour();
        if (expiryDate > block.timestamp + 365 days) revert ExpiryDateAfterOneYear();

        ACLStorage storage $ = _getACLStorage();

        if ($.lastBlockDelegateOrRevoke[msg.sender][delegatee][contractAddress] == block.number) {
            revert AlreadyDelegatedOrRevokedInSameBlock(msg.sender, delegatee, contractAddress);
        }
        $.lastBlockDelegateOrRevoke[msg.sender][delegatee][contractAddress] = block.number;

        if (contractAddress == msg.sender) {
            revert SenderCannotBeContractAddress(contractAddress);
        }
        if (delegatee == msg.sender) {
            revert SenderCannotBeDelegatee(delegatee);
        }
        if (delegatee == contractAddress) {
            revert DelegateeCannotBeContractAddress(contractAddress);
        }

        uint256 newExpiryDate = block.timestamp + expiryDate;
        uint256 oldExpiryData = $.expiryDates[msg.sender][delegatee][contractAddress];
        if (oldExpiryData == newExpiryDate) {
            revert ExpiryDateAlreadySetToSameValue(msg.sender, delegatee, contractAddress, oldExpiryData);
        }
        $.expiryDates[msg.sender][delegatee][contractAddress] = newExpiryDate;

        emit NewDelegation(msg.sender, delegatee, contractAddress, oldExpiryData, newExpiryDate);
    }

    /**
     * @notice Revokes delegated access of handles
     * @param delegatee Delegatee address.
     * @param contractAddress Contract address.
     */
    function revokeDelegation(address delegatee, address contractAddress) public virtual whenNotPaused {
        ACLStorage storage $ = _getACLStorage();

        if ($.lastBlockDelegateOrRevoke[msg.sender][delegatee][contractAddress] == block.number) {
            revert AlreadyDelegatedOrRevokedInSameBlock(msg.sender, delegatee, contractAddress);
        }
        $.lastBlockDelegateOrRevoke[msg.sender][delegatee][contractAddress] = block.number;

        uint256 oldExpiryData = $.expiryDates[msg.sender][delegatee][contractAddress];
        if (oldExpiryData == 0) {
            revert NotDelegatedYet(msg.sender, delegatee, contractAddress);
        }
        $.expiryDates[msg.sender][delegatee][contractAddress] = 0;

        emit RevokedDelegation(msg.sender, delegatee, contractAddress, oldExpiryData);
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
     * @notice Returns whether the delegatee is allowed to access the handle.
     * @param delegatee Delegatee address.
     * @param handle Handle.
     * @param contractAddress Contract address.
     * @param account Address of the account.
     * @return isAllowed Whether the handle can be accessed.
     */
    function allowedOnBehalf(
        address delegatee,
        bytes32 handle,
        address contractAddress,
        address account
    ) public view virtual returns (bool) {
        ACLStorage storage $ = _getACLStorage();
        return
            $.persistedAllowedPairs[handle][account] &&
            $.persistedAllowedPairs[handle][contractAddress] &&
            $.expiryDates[account][delegatee][contractAddress] >= block.timestamp;
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

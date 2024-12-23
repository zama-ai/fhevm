// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import "../addresses/TFHEExecutorAddress.sol";

contract ACL is UUPSUpgradeable, Ownable2StepUpgradeable {
    /// @notice Returned if the delegatee contract is already delegatee for sender & delegator addresses.
    error AlreadyDelegated();

    /// @notice Returned if the sender is the delegatee address.
    error SenderCannotBeDelegateeAddress();

    /// @notice Returned if the sender address is not allowed for allow operations.
    error SenderNotAllowed(address sender);

    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "ACL";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    address private constant tfheExecutorAddress = tfheExecutorAdd;

    /// @custom:storage-location erc7201:fhevm.storage.ACL
    struct ACLStorage {
        mapping(uint256 handle => mapping(address account => bool isAllowed)) persistedAllowedPairs;
        mapping(uint256 => bool) allowedForDecryption;
        mapping(address account => mapping(address delegatee => mapping(address contractAddress => bool isDelegate))) delegates;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.ACL")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant ACLStorageLocation = 0xa688f31953c2015baaf8c0a488ee1ee22eb0e05273cc1fd31ea4cbee42febc00;

    function _getACLStorage() internal pure returns (ACLStorage storage $) {
        assembly {
            $.slot := ACLStorageLocation
        }
    }

    /// @notice Getter function for the TFHEExecutor contract address
    function getTFHEExecutorAddress() public view virtual returns (address) {
        return tfheExecutorAddress;
    }

    event NewDelegation(address indexed sender, address indexed delegatee, address indexed contractAddress);
    event AllowedForDecryption(uint256[] handlesList);

    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract setting `initialOwner` as the initial owner
    function initialize(address initialOwner) external initializer {
        __Ownable_init(initialOwner);
    }

    // allowTransient use of `handle` for address `account`.
    // The caller must be allowed to use `handle` for allowTransient() to succeed. If not, allowTransient() reverts.
    // @note: The Coprocessor contract can always `allowTransient`, contrarily to `allow`
    function allowTransient(uint256 handle, address account) public virtual {
        if (msg.sender != tfheExecutorAddress) {
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

    function allowedTransient(uint256 handle, address account) public view virtual returns (bool) {
        bool isAllowedTransient;
        bytes32 key = keccak256(abi.encodePacked(handle, account));
        assembly {
            isAllowedTransient := tload(key)
        }
        return isAllowedTransient;
    }

    /**
     * @dev This function removes the transient allowances, which could be useful for integration with
     *      Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
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

    // Allow use of `handle` for address `account`.
    // The caller must be allowed to use `handle` for allow() to succeed. If not, allow() reverts.
    function allow(uint256 handle, address account) external virtual {
        ACLStorage storage $ = _getACLStorage();
        if (!isAllowed(handle, msg.sender)) {
            revert SenderNotAllowed(msg.sender);
        }
        $.persistedAllowedPairs[handle][account] = true;
    }

    // Returns true if address `a` is allowed to use `c` and false otherwise.
    function persistAllowed(uint256 handle, address account) public view virtual returns (bool) {
        ACLStorage storage $ = _getACLStorage();
        return $.persistedAllowedPairs[handle][account];
    }

    // Useful in the context of account abstraction for issuing reencryption requests from a smart contract account
    function isAllowed(uint256 handle, address account) public view virtual returns (bool) {
        return allowedTransient(handle, account) || persistAllowed(handle, account);
    }

    function delegateAccountForContract(address delegatee, address delegateeContract) external virtual {
        if (delegateeContract == msg.sender) {
            revert SenderCannotBeDelegateeAddress();
        }

        ACLStorage storage $ = _getACLStorage();
        if ($.delegates[msg.sender][delegatee][delegateeContract]) {
            revert AlreadyDelegated();
        }
        $.delegates[msg.sender][delegatee][delegateeContract] = true;
        emit NewDelegation(msg.sender, delegatee, delegateeContract);
    }

    function allowedOnBehalf(
        address delegatee,
        uint256 handle,
        address contractAddress,
        address account
    ) external view virtual returns (bool) {
        ACLStorage storage $ = _getACLStorage();
        return
            $.persistedAllowedPairs[handle][account] &&
            $.persistedAllowedPairs[handle][contractAddress] &&
            $.delegates[account][delegatee][contractAddress];
    }

    function allowForDecryption(uint256[] memory handlesList) external virtual {
        uint256 len = handlesList.length;
        ACLStorage storage $ = _getACLStorage();
        for (uint256 k = 0; k < len; k++) {
            uint256 handle = handlesList[k];
            if (!isAllowed(handle, msg.sender)) {
                revert SenderNotAllowed(msg.sender);
            }
            $.allowedForDecryption[handle] = true;
        }
        emit AllowedForDecryption(handlesList);
    }

    function isAllowedForDecryption(uint256 handle) public view virtual returns (bool) {
        ACLStorage storage $ = _getACLStorage();
        return $.allowedForDecryption[handle];
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
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
}

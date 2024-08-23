// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@openzeppelin/contracts/utils/Strings.sol";
import "./TFHEExecutorAddress.sol";

contract ACL {
    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "ACL";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    address public immutable tfheExecutorAddress = tfheExecutorAdd;

    mapping(uint256 => bool) public allowedForDecryption;

    // A set of (handle, address) pairs.
    // If address A is in the set for handle H, full access is granted to H for A.
    mapping(uint256 handle => mapping(address account => bool isAllowed)) public persistedAllowedPairs;

    mapping(address account => mapping(address delegatee => mapping(address contractAddress => bool isDelegate)))
        public delegates;

    event NewDelegation(address indexed sender, address indexed delegatee, address indexed contractAddress);
    event RevokedDelegation(address indexed sender, address indexed delegatee, address indexed contractAddress);
    event AllowedForDecryption(uint256[] handlesList);

    // allowTransient use of `handle` for address `account`.
    // The caller must be allowed to use `handle` for allowTransient() to succeed. If not, allowTransient() reverts.
    // @note: The Coprocessor contract can always `allowTransient`, contrarily to `allow`
    function allowTransient(uint256 handle, address account) public {
        if (msg.sender != tfheExecutorAddress) {
            require(isAllowed(handle, msg.sender), "sender isn't allowed");
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

    function allowedTransient(uint256 handle, address account) public view returns (bool) {
        bool isAllowedTransient;
        bytes32 key = keccak256(abi.encodePacked(handle, account));
        assembly {
            isAllowedTransient := tload(key)
        }
        return isAllowedTransient;
    }

    function cleanTransientStorage() external {
        // this function removes the transient allowances, could be useful for integration with Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor
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
    function allow(uint256 handle, address account) external {
        require(isAllowed(handle, msg.sender), "sender isn't allowed");
        persistedAllowedPairs[handle][account] = true;
    }

    // Returns true if address `a` is allowed to use `c` and false otherwise.
    function persistAllowed(uint256 handle, address account) public view returns (bool) {
        return persistedAllowedPairs[handle][account];
    }

    // Useful in the context of account abstraction for issuing reencryption requests from a smart contract account
    function isAllowed(uint256 handle, address account) public view returns (bool) {
        return allowedTransient(handle, account) || persistAllowed(handle, account);
    }

    function delegateAccountForContract(address delegatee, address contractAddress) external {
        require(contractAddress != msg.sender, "contractAddress should be different from msg.sender");
        require(!delegates[msg.sender][delegatee][contractAddress], "already delegated");
        delegates[msg.sender][delegatee][contractAddress] = true;
        emit NewDelegation(msg.sender, delegatee, contractAddress);
    }

    function removeDelegationForContract(address delegatee, address contractAddress) external {
        require(delegates[msg.sender][delegatee][contractAddress], "not delegated yet");
        delegates[msg.sender][delegatee][contractAddress] = false;
        emit RevokedDelegation(msg.sender, delegatee, contractAddress);
    }

    function allowedOnBehalf(
        address delegatee,
        uint256 handle,
        address contractAddress,
        address account
    ) external view returns (bool) {
        return
            persistedAllowedPairs[handle][account] &&
            persistedAllowedPairs[handle][contractAddress] &&
            delegates[account][delegatee][contractAddress];
    }

    function allowForDecryption(uint256[] memory handlesList) external {
        uint256 len = handlesList.length;
        for (uint256 k = 0; k < len; k++) {
            uint256 handle = handlesList[k];
            require(isAllowed(handle, msg.sender), "sender isn't allowed");
            allowedForDecryption[handle] = true;
        }
        emit AllowedForDecryption(handlesList);
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
    function getVersion() external pure returns (string memory) {
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

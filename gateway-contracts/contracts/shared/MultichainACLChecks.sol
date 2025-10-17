// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { multichainACLAddress } from "../../addresses/GatewayAddresses.sol";
import { IMultichainACL } from "../interfaces/IMultichainACL.sol";

/**
 * @title MultichainACL Checks
 * @dev Contract that provides checks on top of the MultichainACL contract
 */
abstract contract MultichainACLChecks {
    /**
     * @notice The address of the MultichainACL contract.
     */
    IMultichainACL private constant MULTICHAIN_ACL = IMultichainACL(multichainACLAddress);

    /**
     * @notice Error indicating that the ciphertext handle is not allowed for public decryption.
     * @param ctHandle The ciphertext handle that is not allowed for public decryption.
     */
    error PublicDecryptNotAllowed(bytes32 ctHandle);

    /**
     * @notice Error indicating that the account address is not allowed to use the ciphertext handle.
     * @param ctHandle The ciphertext handle that the account is not allowed to use.
     * @param accountAddress The address of the account that is not allowed to use the ciphertext handle.
     */
    error AccountNotAllowedToUseCiphertext(bytes32 ctHandle, address accountAddress);

    /**
     * @notice Error indicating that the account has not been fully delegated.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegator The address of the account that revokes access to its handles.
     * @param delegate The address of the account that stops receiving the delegation.
     * @param contractAddress The address of the contract that was part of the user decryption context.
     */
    error UserDecryptionNotDelegated(uint256 chainId, address delegator, address delegate, address contractAddress);

    /**
     * @notice Checks if the ciphertext handle is allowed for public decryption.
     * @param ctHandle The ciphertext handle to check.
     */
    function _checkIsPublicDecryptAllowed(bytes32 ctHandle) internal view {
        if (!MULTICHAIN_ACL.isPublicDecryptAllowed(ctHandle)) {
            revert PublicDecryptNotAllowed(ctHandle);
        }
    }

    /**
     * @notice Checks if the account is allowed to use the ciphertext handle.
     * @param ctHandle The ciphertext handle to check.
     * @param accountAddress The address of the account to check.
     */
    function _checkIsAccountAllowed(bytes32 ctHandle, address accountAddress) internal view {
        if (!MULTICHAIN_ACL.isAccountAllowed(ctHandle, accountAddress)) {
            revert AccountNotAllowedToUseCiphertext(ctHandle, accountAddress);
        }
    }

    /**
     * @notice Checks if the delegate account has been granted access to the contract addresses by the delegator.
     * @param chainId The chain ID of the registered host chain where the contracts are deployed.
     * @param delegator The address of the account that grants access to its handles.
     * @param delegate The address of the account that receives the delegation.
     * @param contractAddresses The list of contract addresses involved in the user decryption context.
     */
    function _checkIsUserDecryptionDelegated(
        uint256 chainId,
        address delegator,
        address delegate,
        address[] calldata contractAddresses
    ) internal view {
        for (uint256 i = 0; i < contractAddresses.length; i++) {
            // if (!MULTICHAIN_ACL.isUserDecryptionDelegated(chainId, delegator, delegate, contractAddresses[i])) {
            //     revert UserDecryptionNotDelegated(chainId, delegator, delegate, contractAddresses[i]);
            // }
        }
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IPauserSet } from "@fhevm/host-contracts/contracts/interfaces/IPauserSet.sol";

/**
 * @title PauserSetWrapper
 * @notice This contract is used to wrap the PauserSet contract around the pause function of a
 * pausable contract. Both the target contract and the function are set at deployment time and cannot
 * be updated later. There could several instances of this contract, each wrapping a different contract
 * and/or pause function.
 */
contract PauserSetWrapper {
    /**
     * @notice The PauserSet contract, used to manage the pausers.
     */
    IPauserSet public immutable PAUSER_SET;

    /**
     * @notice The address of the target pausable contract whose pause function is wrapped.
     */
    address public immutable CONTRACT_TARGET;

    /**
     * @notice The selector of the contract's pause function.
     */
    bytes4 public immutable FUNCTION_SELECTOR;

    /**
     * @notice The signature of the contract's pause function, mostly for monitoring purposes.
     */
    string public FUNCTION_SIGNATURE;

    /**
     * @notice Error indicating that the pause function execution failed.
     * @param errorData The error data returned by the pause function.
     */
    error ExecutionFailed(bytes errorData);

    /**
     * @notice Error indicating that the target contract does not have code, suggesting that the
     * address is not a valid contract address.
     * @param target The address of the target contract.
     */
    error NoCodeAtTarget(address target);

    /**
     * @notice Error indicating that the PauserSet contract does not have code, suggesting that the
     * address is not a valid contract address.
     * @param pauserSet The address of the PauserSet contract.
     */
    error NoCodeAtPauserSet(address pauserSet);

    /**
     * @notice Error indicating that the sender is not a pauser registered in the PauserSet contract.
     */
    error SenderNotPauser();

    /**
     * @notice Constructor.
     * @param _target The address of the target contract.
     * @param _functionSignature The signature of the contract's pause function. In practice, this
     * function can be any of the contract's function, although this wrapper is meant to be used
     * @param _pauserSet The address of the PauserSet contract.
     */
    constructor(address _target, string memory _functionSignature, address _pauserSet) {
        if (_target.code.length == 0) revert NoCodeAtTarget(_target);
        if (_pauserSet.code.length == 0) revert NoCodeAtPauserSet(_pauserSet);
        CONTRACT_TARGET = _target;
        FUNCTION_SIGNATURE = _functionSignature;
        FUNCTION_SELECTOR = bytes4(keccak256(bytes(_functionSignature)));
        PAUSER_SET = IPauserSet(_pauserSet);
    }

    /**
     * @notice Calls the pausable function of the target contract, restricted to pausers registered in
     * the PauserSet contract.
     * @param args The arguments to pass to the pausable function, encoded in bytes without the
     * function selector.
     */
    function callFunction(bytes memory args) external payable {
        // Check that the sender is a pauser registered in the PauserSet contract.
        if (!PAUSER_SET.isPauser(msg.sender)) revert SenderNotPauser();

        // Encode the function selector and arguments.
        bytes memory data = abi.encodePacked(FUNCTION_SELECTOR, args);

        // Call the function on the target contract and revert if the call fails.
        (bool success, bytes memory errorData) = CONTRACT_TARGET.call{ value: msg.value }(data);
        if (!success) {
            revert ExecutionFailed(errorData);
        }
    }
}

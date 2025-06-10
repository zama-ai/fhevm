// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Coprocessor, CoprocessorContextBlockPeriods, CoprocessorContext } from "../shared/Structs.sol";
import { ContextStatus } from "../shared/Enums.sol";

/**
 * @title Interface for the CoprocessorContexts contract.
 * @notice The CoprocessorContexts contract is responsible for being a point of truth for all contracts
 * regarding coprocessors to consider in the protocol.
 *
 * The CoprocessorContexts contract has an owner.
 * The owner can call some restricted functions, such as adding coprocessor contexts.
 * Some view functions are accessible to everyone (ex: getting the active coprocessors).
 */
interface ICoprocessorContexts {
    /**
     * @notice Emitted when the CoprocessorContexts initialization is completed.
     * @param featureSet The feature set.
     * @param coprocessors The coprocessors.
     */
    event InitializeCoprocessorContexts(uint256 featureSet, Coprocessor[] coprocessors);

    /**
     * @notice Emitted when a new coprocessor context has been suggested to be added.
     * This does not mean that the new coprocessor context is already active, it is mostly to better
     * track coprocessor context updates.
     * @param activeCoprocessorContext The current active coprocessor context.
     * @param newCoprocessorContext The new coprocessor context.
     * @param blockPeriods The block periods for the new coprocessor context.
     */
    event NewCoprocessorContext(
        CoprocessorContext activeCoprocessorContext,
        CoprocessorContext newCoprocessorContext,
        CoprocessorContextBlockPeriods blockPeriods
    );

    /**
     * @notice Emitted when a new coprocessor context gets pre-activated.
     * @param newCoprocessorContext The new coprocessor context.
     * @param activationBlockNumber The block number at which the coprocessor context will be activated.
     */
    event PreActivateCoprocessorContext(CoprocessorContext newCoprocessorContext, uint256 activationBlockNumber);

    /**
     * @notice Emitted when a coprocessor context gets suspended.
     * @param contextId The ID of the coprocessor context.
     * @param deactivatedBlockNumber The block number at which the coprocessor context will be deactivated.
     */
    event SuspendCoprocessorContext(uint256 contextId, uint256 deactivatedBlockNumber);

    /**
     * @notice Emitted when a coprocessor context gets activated.
     * @param contextId The ID of the coprocessor context.
     */
    event ActivateCoprocessorContext(uint256 contextId);

    /**
     * @notice Emitted when a coprocessor context gets deactivated.
     * @param contextId The ID of the coprocessor context.
     */
    event DeactivateCoprocessorContext(uint256 contextId);

    /**
     * @notice Emitted when a coprocessor context gets compromised.
     * @param contextId The ID of the coprocessor context.
     */
    event CompromiseCoprocessorContext(uint256 contextId);

    /**
     * @notice Emitted when a coprocessor context gets destroyed.
     * @param contextId The ID of the coprocessor context.
     */
    event DestroyCoprocessorContext(uint256 contextId);

    /**
     * @notice Error indicating that the coprocessor context is not initialized.
     * @param contextId The ID of the coprocessor context.
     */
    error CoprocessorContextNotInitialized(uint256 contextId);

    /**
     * @notice Error indicating that the coprocessors list is empty.
     */
    error EmptyCoprocessors();

    /**
     * @notice Error indicating that a coprocessor has a null transaction sender address.
     * @param contextId The ID of the coprocessor context.
     * @param coprocessorIndex The index of the coprocessor in the coprocessors list.
     */
    error NullCoprocessorTxSenderAddress(uint256 contextId, uint256 coprocessorIndex);

    /**
     * @notice Error indicating that a coprocessor has a null signer address.
     * @param contextId The ID of the coprocessor context.
     * @param coprocessorIndex The index of the coprocessor in the coprocessors list.
     */
    error NullCoprocessorSignerAddress(uint256 contextId, uint256 coprocessorIndex);

    /**
     * @notice Error indicating that there is no pre-activation coprocessor context.
     */
    error NoPreActivationCoprocessorContext();

    /**
     * @notice Error indicating that there is no suspended coprocessor context.
     */
    error NoSuspendedCoprocessorContext();

    /**
     * @notice Error indicating that there is no active coprocessor context.
     * There should always be a single active coprocessor context defined in the gateway, as we do not
     * allow manually setting active coprocessor contexts to `Compromised` or `Destroyed` states.
     * We still consider some reverts to prevent any unexpected behaviors that could cause the protocol
     * to behave in an unexpected manner (ex: by considering a null contextId).
     */
    error NoActiveCoprocessorContext();

    /**
     * @notice Error indicating that a transaction sender address is not associated with a registered
     * coprocessor within the context.
     * @param contextId The coprocessor context ID.
     * @param coprocessorTxSenderAddress The transaction sender address that is not associated with a registered coprocessor.
     */
    error NotCoprocessorFromContext(uint256 contextId, address coprocessorTxSenderAddress);

    /**
     * @notice Error indicating that an address is not a coprocessor transaction sender from a context.
     * @param contextId The coprocessor context ID.
     * @param txSenderAddress The address to check.
     */
    error NotCoprocessorTxSenderFromContext(uint256 contextId, address txSenderAddress);

    /**
     * @notice Error indicating that an address is not a coprocessor signer from a context.
     * @param contextId The coprocessor context ID.
     * @param signerAddress The address to check.
     */
    error NotCoprocessorSignerFromContext(uint256 contextId, address signerAddress);

    /**
     * @notice Get the ID of the pre-activation coprocessor context.
     * This function reverts if there is no pre-activation coprocessor context.
     * @return The ID of the pre-activation coprocessor context.
     */
    function getPreActivationCoprocessorContextId() external view returns (uint256);

    /**
     * @notice Get the ID of the suspended coprocessor context.
     * This function reverts if there is no suspended coprocessor context.
     * @return The ID of the suspended coprocessor context.
     */
    function getSuspendedCoprocessorContextId() external view returns (uint256);

    /**
     * @notice Get the ID of the active coprocessor context.
     * This function reverts if there is no active coprocessor context.
     * @return The ID of the active coprocessor context.
     */
    function getActiveCoprocessorContextId() external view returns (uint256);

    /**
     * @notice Get the metadata of the active coprocessor context.
     * This function reverts if there is no active coprocessor context.
     * There should always be a single active coprocessor context defined in the gateway, as we do not
     * allow manually setting active coprocessor contexts to `Compromised` or `Destroyed` states.
     * We still consider some reverts to prevent any unexpected behaviors that could cause the protocol
     * to behave in an unexpected manner (ex: by considering a null contextId).
     * @return The metadata of the active coprocessor context.
     */
    function getActiveCoprocessorContext() external view returns (CoprocessorContext memory);

    /**
     * @notice Get the metadata of the coprocessor associated to the transaction sender within the active coprocessor context.
     * This function reverts if there is no active coprocessor context.
     * There should always be a single active coprocessor context defined in the gateway, as we do not
     * allow manually setting active coprocessor contexts to `Compromised` or `Destroyed` states.
     * We still consider some reverts to prevent any unexpected behaviors that could cause the protocol
     * to behave in an unexpected manner (ex: by considering a null contextId).
     * @param contextId The coprocessor context ID.
     * @param coprocessorTxSenderAddress The signer address of the coprocessor to get.
     * @return The coprocessor's metadata.
     */
    function getCoprocessorFromContext(
        uint256 contextId,
        address coprocessorTxSenderAddress
    ) external view returns (Coprocessor memory);

    /**
     * @notice Add a new coprocessor context.
     * @param featureSet The feature set.
     * @param blockPeriods The block periods.
     * @param coprocessors The set of coprocessors representing the coprocessor context.
     */
    function addCoprocessorContext(
        uint256 featureSet,
        CoprocessorContextBlockPeriods calldata blockPeriods,
        Coprocessor[] calldata coprocessors
    ) external;

    /**
     * @notice Refresh the statuses of all coprocessor contexts.
     * More precisely, this function:
     * - checks if there is a pre-activation coprocessor context that should be activated (by
     * checking if the pre-activation period has ended)
     * - checks if there is a suspended coprocessor context that should be deactivated (by checking
     * if the suspension period has ended)
     */
    function refreshCoprocessorContextStatuses() external;

    /**
     * @notice Compromise a coprocessor context.
     * @param contextId The ID of the coprocessor context to compromise.
     */
    function compromiseCoprocessorContext(uint256 contextId) external;

    /**
     * @notice Destroy a coprocessor context.
     * @param contextId The ID of the coprocessor context to destroy.
     */
    function destroyCoprocessorContext(uint256 contextId) external;

    /**
     * @notice Move a suspended coprocessor context to active.
     * @dev This function is provided in case of emergency (ex: if a software update failed)
     */
    function moveSuspendedCoprocessorContextToActive() external;

    /**
     * @notice Check if an address is a registered coprocessor transaction sender from a context.
     * @param contextId The coprocessor context ID.
     * @param txSenderAddress The address to check.
     */
    function checkIsCoprocessorTxSenderFromContext(uint256 contextId, address txSenderAddress) external view;

    /**
     * @notice Check if an address is a registered coprocessor signer from a context.
     * @param contextId The coprocessor context ID.
     * @param signerAddress The address to check.
     */
    function checkIsCoprocessorSignerFromContext(uint256 contextId, address signerAddress) external view;

    /**
     * @notice Indicates if a coprocessor context is active or suspended.
     * @param contextId The coprocessor context ID.
     * @return True if the coprocessor context is active or suspended, false otherwise.
     */
    function isCoprocessorContextActiveOrSuspended(uint256 contextId) external view returns (bool);

    /**
     * @notice Get the block number at which the coprocessor context is activated.
     * @param contextId The coprocessor context ID.
     * @return The activation block number.
     */
    function getCoprocessorContextActivationBlockNumber(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the block number at which the coprocessor context is deactivated.
     * @param contextId The coprocessor context ID.
     * @return The deactivation block number.
     */
    function getCoprocessorContextDeactivatedBlockNumber(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the coprocessor majority threshold for a coprocessor context.
     * @param contextId The coprocessor context ID.
     * @return The coprocessor majority threshold.
     */
    function getCoprocessorMajorityThresholdFromContext(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the metadata of the coprocessor associated to the transaction sender within the
     * active coprocessor context.
     * @param coprocessorTxSenderAddress The signer address of the coprocessor to get.
     * @return The coprocessor's metadata.
     */
    function getCoprocessor(address coprocessorTxSenderAddress) external view returns (Coprocessor memory);

    /**
     * @notice Get the list of all active coprocessors' transaction sender addresses.
     * @return The list of active coprocessors' transaction sender addresses.
     */
    function getCoprocessorTxSenders() external view returns (address[] memory);

    /**
     * @notice Get the list of all active coprocessors' signer addresses.
     * @return The list of active coprocessors' signer addresses.
     */
    function getCoprocessorSigners() external view returns (address[] memory);

    /**
     * @notice Get the context status of a coprocessor context.
     * @param contextId The coprocessor context ID.
     * @return The status of the coprocessor context.
     */
    function getCoprocessorContextStatus(uint256 contextId) external view returns (ContextStatus);

    /**
     * @notice Returns the versions of the CoprocessorContexts contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

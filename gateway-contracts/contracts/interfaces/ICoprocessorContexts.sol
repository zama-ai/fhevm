// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { CoprocessorV2, CoprocessorContextTimePeriods, CoprocessorContext } from "../shared/Structs.sol";
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
     * @param blob The blob.
     * @param coprocessors The coprocessors.
     */
    event InitializeCoprocessorContexts(bytes blob, CoprocessorV2[] coprocessors);

    /**
     * @notice Emitted when a new coprocessor context has been suggested to be added.
     * This does not mean that the new coprocessor context is already active, it is mostly to better
     * track coprocessor context updates.
     * @param activeCoprocessorContext The current active coprocessor context.
     * @param newCoprocessorContext The new coprocessor context.
     * @param timePeriods The time periods for the new coprocessor context.
     */
    event NewCoprocessorContext(
        CoprocessorContext activeCoprocessorContext,
        CoprocessorContext newCoprocessorContext,
        CoprocessorContextTimePeriods timePeriods
    );

    /**
     * @notice Emitted when a new coprocessor context gets pre-activated.
     * @param newCoprocessorContext The new coprocessor context.
     * @param activationBlockTimestamp The block timestamp at which the coprocessor context will be activated.
     */
    event PreActivateCoprocessorContext(CoprocessorContext newCoprocessorContext, uint256 activationBlockTimestamp);

    /**
     * @notice Emitted when a coprocessor context gets suspended.
     * @param contextId The ID of the coprocessor context.
     * @param deactivatedBlockTimestamp The block timestamp at which the coprocessor context will be deactivated.
     */
    event SuspendCoprocessorContext(uint256 contextId, uint256 deactivatedBlockTimestamp);

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
     * @notice Error indicating that the list of coprocessors to register in the context is empty.
     */
    error EmptyCoprocessors();

    /**
     * @notice Error indicating that the list of coprocessors in the context to register has at
     * least one coprocessor with a null transaction sender address.
     * @param coprocessorIndex The index of the coprocessor in the coprocessors list.
     * @param coprocessors The list of coprocessors to register in the context.
     */
    error NullCoprocessorTxSenderAddress(uint256 coprocessorIndex, CoprocessorV2[] coprocessors);

    /**
     * @notice Error indicating that the list of coprocessors in the context to register has at
     * least one coprocessor with a null signer address.
     * @param coprocessorIndex The index of the coprocessor in the coprocessors list.
     * @param coprocessors The list of coprocessors to register in the context.
     */
    error NullCoprocessorSignerAddress(uint256 coprocessorIndex, CoprocessorV2[] coprocessors);

    /**
     * @notice Error indicating that the list of coprocessors in the context to register has at
     * least two coprocessors with the same transaction sender address.
     * @param txSenderAddress The first transaction sender address that is not unique.
     * @param coprocessorIndex The index of the first coprocessor with the same transaction sender address as another one.
     * @param coprocessors The list of coprocessors to register in the context.
     */
    error CoprocessorTxSenderAddressesNotUnique(
        address txSenderAddress,
        uint256 coprocessorIndex,
        CoprocessorV2[] coprocessors
    );

    /**
     * @notice Error indicating that the list of coprocessors in the context to register has at
     * least two coprocessors with the same signer address.
     * @param signerAddress The first signer address that is not unique.
     * @param coprocessorIndex The index of the first coprocessor with the same signer address as another one.
     * @param coprocessors The list of coprocessors to register in the context.
     */
    error CoprocessorSignerAddressesNotUnique(
        address signerAddress,
        uint256 coprocessorIndex,
        CoprocessorV2[] coprocessors
    );

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
     * @notice Emitted when a coprocessor context status targeted for a forced update is invalid.
     * This means that the status does not reflect that the context has already been added
     * (ex: `NotInitialized`, `Generating`, `PreActivation`).
     * @param contextId The ID of the coprocessor context.
     * @param status The status that was attempted to be updated.
     */
    error InvalidContextStatusForceUpdate(uint256 contextId, ContextStatus status);

    /**
     * @notice Error indicating that a transaction sender address is not associated with a registered
     * coprocessor within the context.
     * @param contextId The coprocessor context ID.
     * @param coprocessorTxSenderAddress The transaction sender address that is not associated with a registered coprocessor.
     */
    error NotCoprocessorFromContext(uint256 contextId, address coprocessorTxSenderAddress);

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
     * @notice Add a new coprocessor context.
     * @param blob The blob.
     * @param coprocessors The set of coprocessors representing the coprocessor context.
     * @param timePeriods The time periods.
     */
    function addCoprocessorContext(
        bytes calldata blob,
        CoprocessorV2[] calldata coprocessors,
        CoprocessorContextTimePeriods calldata timePeriods
    ) external;

    /**
     * @notice Refresh the statuses of all coprocessor contexts.
     * More precisely, this function:
     * - checks if there is a pre-activation coprocessor context that should be activated (by
     * checking if the pre-activation period has ended)
     * - checks if there is a suspended coprocessor context that should be deactivated (by checking
     * if the suspended period has ended)
     */
    function refreshCoprocessorContextStatuses() external;

    /**
     * @notice Manually force the status update of a coprocessor context.
     * This function reverts if
     * - the status update is against any of the lifecycle's rules
     * - the context has not been added yet
     * The following context status updates are only possible through this function:
     * - Compromised
     * - Destroyed
     * Additionally, if a status update needs to be associated to a block timestamp, the current
     * block timestamp is used (i.e., the status update is immediate)
     * @param contextId The ID of the coprocessor context to update.
     * @param status The status to update the coprocessor context to.
     */
    function forceUpdateCoprocessorContextToStatus(uint256 contextId, ContextStatus status) external;

    /**
     * @notice Swap a suspended coprocessor context with the current active one.
     * This function is provided in case of emergency (ex: if a software update failed)
     * @param suspendedTimePeriod The suspended time period for the active coprocessor context
     */
    function swapSuspendedCoprocessorContextWithActive(uint256 suspendedTimePeriod) external;

    /**
     * @notice Indicates if an address is a registered coprocessor transaction sender from a context.
     * @param contextId The coprocessor context ID.
     * @param txSenderAddress The address to check.
     */
    function isCoprocessorTxSender(uint256 contextId, address txSenderAddress) external view returns (bool);

    /**
     * @notice Indicates if an address is a registered coprocessor signer from a context.
     * @param contextId The coprocessor context ID.
     * @param signerAddress The address to check.
     */
    function isCoprocessorSigner(uint256 contextId, address signerAddress) external view returns (bool);

    /**
     * @notice Indicates if a coprocessor context is operating, i.e. it is either active or suspended.
     * @param contextId The coprocessor context ID.
     * @return True if the coprocessor context is active or suspended, false otherwise.
     */
    function isCoprocessorContextOperating(uint256 contextId) external view returns (bool);

    /**
     * @notice Get the block timestamp at which the coprocessor context is activated.
     * @param contextId The coprocessor context ID.
     * @return The activation block timestamp.
     */
    function getCoprocessorActivationBlockTimestamp(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the block timestamp at which the coprocessor context is deactivated.
     * @param contextId The coprocessor context ID.
     * @return The deactivation block timestamp.
     */
    function getCoprocessorDeactivatedBlockTimestamp(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the coprocessor majority threshold for a coprocessor context.
     * @param contextId The coprocessor context ID.
     * @return The coprocessor majority threshold.
     */
    function getCoprocessorMajorityThreshold(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the metadata of the coprocessor associated to the transaction sender within a coprocessor context.
     * @param contextId The coprocessor context ID.
     * @param coprocessorTxSenderAddress The signer address of the coprocessor to get.
     * @return The coprocessor's metadata.
     */
    function getCoprocessor(
        uint256 contextId,
        address coprocessorTxSenderAddress
    ) external view returns (CoprocessorV2 memory);

    /**
     * @notice Get the list of all coprocessors' transaction sender addresses from a context.
     * @param contextId The coprocessor context ID.
     * @return The list of coprocessors' transaction sender addresses from a context.
     */
    function getCoprocessorTxSenders(uint256 contextId) external view returns (address[] memory);

    /**
     * @notice Get the list of all coprocessors' signer addresses from a context.
     * @param contextId The coprocessor context ID.
     * @return The list of coprocessors' signer addresses from a context.
     */
    function getCoprocessorSigners(uint256 contextId) external view returns (address[] memory);

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

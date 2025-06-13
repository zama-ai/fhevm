// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Coprocessor, CoprocessorContextBlockPeriods, CoprocessorContext } from "../shared/Structs.sol";
import { ContextStatus } from "../shared/Enums.sol";

/**
 * @title Interface for the CoprocessorContexts contract.
 * @notice The CoprocessorContexts contract is responsible for being a point of truth for all contracts
 * regarding the coprocessors to consider in the protocol.
 *
 * The CoprocessorContexts contract has an owner.
 * The owner can call some restricted functions, such as adding or removing coprocessors.
 * Some view functions are accessible to everyone (ex: getting the number of coprocessors).
 */
interface ICoprocessorContexts {
    /**
     * @notice Emitted when the CoprocessorContexts initialization is completed.
     * @param featureSet The feature set.
     * @param coprocessors The coprocessors.
     */
    event InitCoprocessorContext(uint256 featureSet, Coprocessor[] coprocessors);

    /**
     * @notice Emitted when a new coprocessor context has been suggested to be added.
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
     * @notice Emitted when a new coprocessor context is being pre-activated.
     * @param newCoprocessorContext The new coprocessor context.
     * @param preActivationBlockNumber The block number at which the coprocessor context will be activated.
     */
    event PreActivateCoprocessorContext(CoprocessorContext newCoprocessorContext, uint256 preActivationBlockNumber);

    event ActivateCoprocessorContext(uint256 contextId);

    event SuspendCoprocessorContext(uint256 contextId, uint256 suspendedBlockNumber);

    event CompromiseCoprocessorContext(uint256 contextId);

    event DeactivateCoprocessorContext(uint256 contextId);

    event DestroyCoprocessorContext(uint256 contextId);

    error CoprocessorContextNotInitialized(uint256 contextId);

    /// @notice Error emitted when the coprocessors list is empty.
    error EmptyCoprocessors();

    /**
     * @notice Error emitted when an address is not a coprocessor transaction sender from a context.
     * @param contextId The coprocessor context ID.
     * @param txSenderAddress The address to check.
     */
    error NotCoprocessorTxSenderFromContext(uint256 contextId, address txSenderAddress);

    /**
     * @notice Error emitted when an address is not a coprocessor signer from a context.
     * @param contextId The coprocessor context ID.
     * @param signerAddress The address to check.
     */
    error NotCoprocessorSignerFromContext(uint256 contextId, address signerAddress);

    error SuspendedCoprocessorContextOngoing(uint256 suspendedContextId);

    error CompromiseActiveCoprocessorContextNotAllowed(uint256 contextId);

    error DestroyActiveCoprocessorContextNotAllowed(uint256 contextId);

    error CoprocessorContextNotGenerating(uint256 contextId);

    error NoPreActivationCoprocessorContext();

    error NoSuspendedCoprocessorContext();

    error NoActiveCoprocessorContext();

    /**
     * @notice Error emitted when an transaction sender address is not associated with a registered coprocessor within the context.
     * @param contextId The coprocessor context ID.
     * @param coprocessorTxSenderAddress The transaction sender address that is not associated with a registered coprocessor.
     */
    error NotCoprocessorFromContext(uint256 contextId, address coprocessorTxSenderAddress);

    function getPreActivationCoprocessorContextId() external view returns (uint256);

    function getSuspendedCoprocessorContextId() external view returns (uint256);

    function getActiveCoprocessorContextId() external view returns (uint256);

    function getActiveCoprocessorContext() external view returns (CoprocessorContext memory);

    /**
     * @notice Get the infos of the coprocessor associated to the transaction sender within the active coprocessor context.
     * @param contextId The coprocessor context ID.
     * @param coprocessorTxSenderAddress The signer address of the coprocessor to get.
     * @return The coprocessor's metadata.
     */
    function getCoprocessorFromContext(
        uint256 contextId,
        address coprocessorTxSenderAddress
    ) external view returns (Coprocessor memory);

    /**
     * @notice Add a new coprocessor context to the CoprocessorContexts contract.
     * @param featureSet The feature set index.
     * @param blockPeriods The block periods.
     * @param coprocessors The set of coprocessors representing the coprocessor context.
     */
    function addCoprocessorContext(
        uint256 featureSet,
        CoprocessorContextBlockPeriods calldata blockPeriods,
        Coprocessor[] calldata coprocessors
    ) external;

    function refreshCoprocessorContextStatuses() external;

    function compromiseCoprocessorContext(uint256 contextId) external;

    function destroyCoprocessorContext(uint256 contextId) external;

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

    function isCoprocessorContextActiveOrSuspended(uint256 contextId) external view returns (bool);

    function getCoprocessorContextPreActivationBlockNumber(uint256 contextId) external view returns (uint256);

    function getCoprocessorContextSuspendedBlockNumber(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the coprocessor majority threshold.
     * @param contextId The coprocessor context ID.
     * @return The coprocessor majority threshold.
     */
    function getCoprocessorMajorityThresholdFromContext(uint256 contextId) external view returns (uint256);

    /**
     * @notice Get the infos of the coprocessor associated to the transaction sender within the active coprocessor context.
     * @param coprocessorTxSenderAddress The signer address of the coprocessor to get.
     * @return The coprocessor's metadata.
     */
    function getCoprocessor(address coprocessorTxSenderAddress) external view returns (Coprocessor memory);

    function getCoprocessors() external view returns (Coprocessor[] memory);

    /**
     * @notice Get the list of all coprocessors' transaction sender addresses currently registered.
     * @return The list of coprocessors' transaction sender addresses.
     */
    function getCoprocessorTxSenders() external view returns (address[] memory);

    /**
     * @notice Get the list of all coprocessors' signer addresses currently registered.
     * @return The list of coprocessors' signer addresses.
     */
    function getCoprocessorSigners() external view returns (address[] memory);

    function getCoprocessorContextStatus(uint256 contextId) external view returns (ContextStatus);

    /**
     * @notice Returns the versions of the CoprocessorContexts contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

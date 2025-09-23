// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ContextStatus } from "../shared/Enums.sol";

/**
 * @title ContextLifecycle library
 * @notice This library is used to manage the lifecycle of a context (of any kind).
 * It implements the full logic and checks around lifecycle status updates.
 */
library ContextLifecycle {
    /**
     * @notice The storage struct for the context lifecycle.
     * This struct keeps track of all context statuses, as well as some of the IDs per status
     */
    struct ContextLifecycleStorage {
        /// @notice The status of each context
        mapping(uint256 => ContextStatus) contextStatuses;
        /// @notice The ID of the context that is generating (at most one)
        uint256 generatingContextId;
        /// @notice The ID of the context that is pre-activated (at most one)
        uint256 preActivationContextId;
        /// @notice The ID of the context that is active (at most one)
        uint256 activeContextId;
        /// @notice The ID of the context that is suspended (at most one)
        uint256 suspendedContextId;
        /// @notice Whether the context lifecycle is initialized with a first active context
        bool initialized;
    }

    /**
     * @notice Error indicating that the context ID is null.
     */
    error InvalidNullContextId();

    /**
     * @notice Error indicating that the context ID does not exist.
     */
    error ContextDoesNotExist(uint256 contextId);

    /**
     * @notice Error indicating that the context ID already exists.
     */
    error ContextAlreadyExists(uint256 contextId);

    /**
     * @notice Error indicating that a pre-activation context is ongoing.
     */
    error PreActivationContextOngoing(uint256 preActivationContextId);

    /**
     * @notice Error indicating that an active context is ongoing.
     */
    error ActiveContextOngoing(uint256 activeContextId);

    /**
     * @notice Error indicating that a suspended context is ongoing.
     */
    error SuspendedContextOngoing(uint256 suspendedContextId);

    /**
     * @notice Error indicating that the context has not been generated.
     */
    error ContextNotGenerated(uint256 contextId);

    /**
     * @notice Error indicating that the context is not pre-activated or suspended.
     */
    error ContextNotPreActivatedOrSuspended(uint256 contextId);

    /**
     * @notice Error indicating that the context is not active.
     */
    error ContextNotActive(uint256 contextId);

    /**
     * @notice Error indicating that the context is not suspended.
     */
    error ContextNotSuspended(uint256 contextId);

    /**
     * @notice Error indicating that the context is generating.
     */
    error ContextIsGenerating(uint256 contextId);

    /**
     * @notice Error indicating that the context is active.
     */
    error ContextIsActive(uint256 contextId);

    /**
     * @notice Modifier to ensure that the context ID is not null (not initialized).
     * @param contextId The context ID to check.
     */
    modifier onlyNonNullContextId(uint256 contextId) {
        if (contextId == 0) {
            revert InvalidNullContextId();
        }
        _;
    }

    /**
     * @notice Modifier to ensure that the context ID is not null (not initialized).
     * @param contextId The context ID to check.
     */
    modifier onlyExistingContextId(ContextLifecycleStorage storage $, uint256 contextId) {
        if (isNotInitialized($, contextId)) {
            revert ContextDoesNotExist(contextId);
        }
        _;
    }

    /**
     * @notice Sets the context as generating.
     * @dev There can only be one generating context at a time.
     * @param contextId The ID of the context to set as generating.
     */
    function setGenerating(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
        // Only a new context ID can be set as generating
        if (!isNotInitialized($, contextId)) {
            revert ContextAlreadyExists(contextId);
        }

        // It is not possible to generate a new context if there is already one in pre-activation
        // The union of contexts in status Generating and Pre-Activation should have size at most 1
        if ($.preActivationContextId != 0) {
            revert PreActivationContextOngoing($.preActivationContextId);
        }

        // It is not possible to generate a new context if there is already one suspended
        if ($.suspendedContextId != 0) {
            revert SuspendedContextOngoing($.suspendedContextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Generating;

        $.generatingContextId = contextId;
    }

    /**
     * @notice Sets the context as pre-activated.
     * @dev There can only be one pre-activated context at a time.
     * @param contextId The ID of the context to set as pre-activated.
     */
    function setPreActivation(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
        // Only a generating context ID can be set as pre-activated
        if (!isGenerating($, contextId)) {
            revert ContextNotGenerated(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.PreActivation;
        $.preActivationContextId = contextId;

        // Reset the generating context ID as it is now pre-activated
        // The union of contexts in status Generating and Pre-Activation should have size at most 1
        $.generatingContextId = 0;
    }

    /**
     * @notice Initializes the context as active.
     * If the active context is not set, it means that it is the very first context to be initialized.
     * In this very specific case, we allow the context to be set as active directly for convenience.
     * @dev There should only be one active context at a time.
     * @param contextId The ID of the context to set as active.
     */
    function initializeActive(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
        // This function should only be called if there is no active context yet
        if ($.activeContextId != 0) {
            revert ActiveContextOngoing($.activeContextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Active;
        $.activeContextId = contextId;
    }

    /**
     * @notice Sets the context as active.
     * @dev There should only be one active context at a time.
     * @param contextId The ID of the context to set as active.
     */
    function _setActive(ContextLifecycleStorage storage $, uint256 contextId) internal onlyNonNullContextId(contextId) {
        // Only a pre-activated or suspended context can be set as active
        if (!isPreActivation($, contextId) && !isSuspended($, contextId)) {
            revert ContextNotPreActivatedOrSuspended(contextId);
        }

        // There should only be one active context at a time. The old active context must be suspended
        // first before a new active context can be set.
        if ($.activeContextId != 0) {
            revert ActiveContextOngoing($.activeContextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Active;
        $.activeContextId = contextId;

        // Reset the pre-activation context ID if it is the one being activated
        if ($.preActivationContextId == contextId) {
            $.preActivationContextId = 0;
        }

        // Reset the suspended context ID if it is the one being activated
        if ($.suspendedContextId == contextId) {
            $.suspendedContextId = 0;
        }
    }

    /**
     * @notice Sets the context as suspended.
     * ⚠️ This function should be used with caution as it can lead to unexpected behaviors if not
     * used correctly. Use `setActiveAndSuspended` instead. ⚠️
     * A suspended context is expected to always be followed by a context activation in order to
     * avoid having a state where no active context is available, which should never happen.
     * @dev There can only be one suspended context at a time.
     * @param contextId The ID of the context to set as suspended.
     */
    function _setSuspended(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
        // Only an active context ID can be set as suspended
        if (!isActive($, contextId)) {
            revert ContextNotActive(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Suspended;
        $.suspendedContextId = contextId;

        // Reset the active context ID as it is now suspended
        $.activeContextId = 0;
    }

    /**
     * @notice Sets the contexts as active and suspended.
     * @dev This function should be favored over setting the contexts separately as it ensures that
     * there is always a (single) active context.
     * @param contextIdToActivate The ID of the context to set as active.
     * @param contextIdToSuspend The ID of the context to set as suspended.
     */
    function setActiveAndSuspended(
        ContextLifecycleStorage storage $,
        uint256 contextIdToActivate,
        uint256 contextIdToSuspend
    ) internal {
        _setSuspended($, contextIdToSuspend);
        _setActive($, contextIdToActivate);
    }

    /**
     * @notice Sets the context as deactivated.
     * @dev There can be multiple deactivated contexts at the same time.
     * @param contextId The ID of the context to set as deactivated.
     */
    function setDeactivated(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
        // Only a suspended context ID can be deactivated
        if (!isSuspended($, contextId)) {
            revert ContextNotSuspended(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Deactivated;

        // Reset the suspended context ID as it is now deactivated
        $.suspendedContextId = 0;
    }

    /**
     * @notice Sets the context as compromised.
     * @dev There can be multiple compromised contexts at the same time.
     * @param contextId The ID of the context to set as compromised.
     */
    function setCompromised(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) onlyExistingContextId($, contextId) {
        // Generating contexts cannot be set as compromised
        if (isGenerating($, contextId)) {
            revert ContextIsGenerating(contextId);
        }

        // Active contexts cannot be set as compromised
        if (isActive($, contextId)) {
            revert ContextIsActive(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Compromised;

        // Reset the pre-activation context ID if it is the one being compromised
        if ($.preActivationContextId == contextId) {
            $.preActivationContextId = 0;
        }

        // Reset the suspended context ID if it is the one being compromised
        if ($.suspendedContextId == contextId) {
            $.suspendedContextId = 0;
        }
    }

    /**
     * @notice Sets the context as destroyed.
     * @dev There can be multiple destroyed contexts at the same time.
     * @dev Any existing context can be set as destroyed.
     * @param contextId The ID of the context to set as destroyed.
     */
    function setDestroyed(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) onlyExistingContextId($, contextId) {
        // Active contexts cannot be set as destroyed
        if (isActive($, contextId)) {
            revert ContextIsActive(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Destroyed;

        // Reset the generating context ID if it is the one being destroyed
        if ($.generatingContextId == contextId) {
            $.generatingContextId = 0;
        }

        // Reset the pre-activation context ID if it is the one being destroyed
        if ($.preActivationContextId == contextId) {
            $.preActivationContextId = 0;
        }

        // Reset the suspended context ID if it is the one being destroyed
        if ($.suspendedContextId == contextId) {
            $.suspendedContextId = 0;
        }
    }

    /**
     * @notice Returns the status of a context.
     * @param contextId The ID of the context to get the status of.
     * @return The status of the context.
     */
    function getContextStatus(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal view returns (ContextStatus) {
        return $.contextStatuses[contextId];
    }

    /**
     * @notice Indicates whether a context is not initialized (i.e. does not exist).
     * @param contextId The ID of the context to check.
     * @return Whether the context is not initialized.
     */
    function isNotInitialized(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.NotInitialized;
    }

    /**
     * @notice Indicates whether a context is generating.
     * @param contextId The ID of the context to check.
     * @return Whether the context is generating.
     */
    function isGenerating(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.Generating;
    }

    /**
     * @notice Indicates whether a context is pre-activated.
     * @param contextId The ID of the context to check.
     * @return Whether the context is pre-activated.
     */
    function isPreActivation(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.PreActivation;
    }

    /**
     * @notice Indicates whether a context is active.
     * @param contextId The ID of the context to check.
     * @return Whether the context is active.
     */
    function isActive(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.Active;
    }

    /**
     * @notice Indicates whether a context is suspended.
     * @param contextId The ID of the context to check.
     * @return Whether the context is suspended.
     */
    function isSuspended(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.Suspended;
    }
}

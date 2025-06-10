// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ContextStatus } from "../shared/Enums.sol";

/**
 * @title ContextLifecycle library
 * @notice This library is used to manage the lifecycle of a context (of any kind).
 * It implements the full logic and checks around lifecycle state updates.
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
     * @notice Error indicating that the context is not active or suspended.
     */
    error ContextNotActiveOrSuspended(uint256 contextId);

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
        // The union of contexts in state Generating and Pre-Activation should have size at most 1
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
        // The union of contexts in state Generating and Pre-Activation should have size at most 1
        $.generatingContextId = 0;
    }

    /**
     * @notice Sets the context as active.
     * @dev There can only be one active context at a time.
     * @param contextId The ID of the context to set as active.
     */
    function setActive(ContextLifecycleStorage storage $, uint256 contextId) internal onlyNonNullContextId(contextId) {
        // Only a pre-activated can be set as active
        // In a normal situation, a suspended context should not be set back as active. However, we also
        // allow this to happen in case of emergency. This pattern should be used with caution and should
        // remain exceptional.
        // Additionally, if the active context is not set, it means that it is the very first context
        // to be initialized. In this very specific case, we allow the context to be set as active directly.
        if (!isPreActivation($, contextId) && !isSuspended($, contextId) && $.activeContextId != 0) {
            revert ContextNotPreActivatedOrSuspended(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Active;
        $.activeContextId = contextId;

        // Reset the pre-activation context ID as it is now active
        $.preActivationContextId = 0;
    }

    /**
     * @notice Sets the context as suspended.
     * @dev There can only be one suspended context at a time.
     * @param contextId The ID of the context to set as suspended.
     */
    function setSuspended(
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
     * @notice Sets the context as deactivated.
     * @dev There can be multiple deactivated contexts at the same time.
     * @param contextId The ID of the context to set as deactivated.
     */
    function setDeactivated(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
        // Only an active or suspended context ID can be set as deactivated
        if (!isActive($, contextId) && !isSuspended($, contextId)) {
            revert ContextNotActiveOrSuspended(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.Deactivated;

        // Reset the active context ID if it is the one being deactivated
        if ($.activeContextId == contextId) {
            $.activeContextId = 0;
        }

        // Reset the suspended context ID if it is the one being deactivated
        if ($.suspendedContextId == contextId) {
            $.suspendedContextId = 0;
        }
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

        // Reset the active context ID if it is the one being compromised
        if ($.activeContextId == contextId) {
            $.activeContextId = 0;
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

        // Reset the active context ID if it is the one being destroyed
        if ($.activeContextId == contextId) {
            $.activeContextId = 0;
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

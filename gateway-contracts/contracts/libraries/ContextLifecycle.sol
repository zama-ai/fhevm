// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ContextStatus } from "../shared/Enums.sol";

library ContextLifecycle {
    struct ContextLifecycleStorage {
        mapping(uint256 => ContextStatus) contextStatuses;
        uint256 generatingContextId;
        uint256 preActivationContextId;
        uint256 activeContextId;
        uint256 suspendedContextId;
    }

    error InvalidNullContextId();
    error ContextAlreadyExists(uint256 contextId);
    error ContextNotGenerated(uint256 contextId);
    error ContextNotPreActivatedOrSuspended(uint256 contextId);
    error ContextNotActive(uint256 contextId);
    error ContextNotActiveOrSuspended(uint256 contextId);
    error ContextNotInitializedOrIsGenerating(uint256 contextId);
    error ContextIsActive(uint256 contextId);

    /**
     * @notice Make sure the context ID is not null.
     * @param contextId The context ID to check.
     */
    modifier onlyNonNullContextId(uint256 contextId) {
        if (contextId == 0) {
            revert InvalidNullContextId();
        }
        _;
    }

    /**
     * @notice Sets the context as generating.
     * @dev There can only be one generating context at most at a time.
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

        $.contextStatuses[contextId] = ContextStatus.Generating;

        $.generatingContextId = contextId;
    }

    /**
     * @notice Sets the context as pre-activated.
     * @dev There can only be one pre-activated context at most at a time.
     * @param contextId The ID of the context to set as pre-activated.
     */
    function setPreActivation(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
        if (!isGenerating($, contextId)) {
            revert ContextNotGenerated(contextId);
        }

        $.contextStatuses[contextId] = ContextStatus.PreActivation;
        $.preActivationContextId = contextId;

        // Reset the generating context ID as it is now pre-activated
        $.generatingContextId = 0;
    }

    /**
     * @notice Sets the context as active.
     * @dev There can only be one active context at most at a time.
     * In a normal situation, a suspended context should not be set back as active. However, we still
     * allow this to happen in case of emergency. This pattern should be used with caution and should
     * remain exceptional.
     * Additionally, if the active context is not set, it means that the context is not initialized.
     * In this very specific case, we allow the context to be set as active directly.
     * @param contextId The ID of the context to set as active.
     */
    function setActive(ContextLifecycleStorage storage $, uint256 contextId) internal onlyNonNullContextId(contextId) {
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
     * @dev There can only be one suspended context at most at a time.
     * @param contextId The ID of the context to set as suspended.
     */
    function setSuspended(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
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
        if (!isActive($, contextId) || !isSuspended($, contextId)) {
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
    ) internal onlyNonNullContextId(contextId) {
        if (isNotInitialized($, contextId) || isGenerating($, contextId)) {
            revert ContextNotInitializedOrIsGenerating(contextId);
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
     * @param contextId The ID of the context to set as destroyed.
     */
    function setDestroyed(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal onlyNonNullContextId(contextId) {
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

    function getContextStatus(
        ContextLifecycleStorage storage $,
        uint256 contextId
    ) internal view returns (ContextStatus) {
        return $.contextStatuses[contextId];
    }

    function isNotInitialized(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.NotInitialized;
    }

    function isGenerating(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.Generating;
    }

    function isPreActivation(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.PreActivation;
    }

    function isActive(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.Active;
    }

    function isSuspended(ContextLifecycleStorage storage $, uint256 contextId) internal view returns (bool) {
        return $.contextStatuses[contextId] == ContextStatus.Suspended;
    }
}

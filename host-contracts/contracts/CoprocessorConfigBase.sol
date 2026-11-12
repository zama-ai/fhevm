// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ICoprocessorConfigBase} from "./interfaces/ICoprocessorConfigBase.sol";
import {Coprocessor} from "./shared/Structs.sol";

/**
 * @title CoprocessorConfigBase
 * @notice Shared storage, reads and write helpers for CoprocessorConfig and CoprocessorConfigReplica.
 */
abstract contract CoprocessorConfigBase is ICoprocessorConfigBase {
    /// @notice Upper bound on the coprocessor set size.
    /// @dev Driven by the input proof format consumed in `InputVerifier.verifyInput`, which encodes
    ///      the signature count in a single byte (`uint8(inputProof[1])`). A set registered above
    ///      this bound could never be verified in full, so the limit is enforced at registration
    ///      time to reject the misconfiguration loudly.
    uint256 private constant MAX_COPROCESSORS = type(uint8).max;

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @custom:storage-location erc7201:fhevm.storage.CoprocessorConfig
    struct CoprocessorConfigStorage {
        /// @notice The current coprocessor context ID, the latest defined or mirrored one.
        uint256 currentCoprocessorContextId;
        /// @notice The coprocessor context that was current before the current one, 0 if none.
        uint256 previousCoprocessorContextId;
        /// @notice Number of coprocessors per context. Zero means the context was never defined.
        mapping(uint256 contextId => uint256) coprocessorCountForContext;
        /// @notice Coprocessors per context, by position in definition order.
        /// @dev A mapping rather than an array, so that `Coprocessor` can gain fields in place.
        mapping(uint256 contextId => mapping(uint256 index => Coprocessor)) coprocessorsForContext;
        /// @notice Majority threshold per coprocessor context.
        mapping(uint256 contextId => uint256) coprocessorMajorityThresholdForContext;
        /// @notice Coprocessor signer lookup per context.
        mapping(uint256 contextId => mapping(address signer => bool isRegistered)) isCoprocessorSignerForContext;
        /// @notice Coprocessor tx sender lookup per context.
        mapping(uint256 contextId => mapping(address txSender => bool isRegistered)) isCoprocessorTxSenderForContext;
        /// @notice Whether a coprocessor context has been destroyed.
        mapping(uint256 contextId => bool) destroyedCoprocessorContexts;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.CoprocessorConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant COPROCESSOR_CONFIG_STORAGE_LOCATION =
        0x0ecd3baf695d010fc2a14d3a2bf238b1c414d6e381b2e8efb81b7b2c7a9ef000;

    function _getCoprocessorConfigStorage() internal pure returns (CoprocessorConfigStorage storage $) {
        assembly {
            $.slot := COPROCESSOR_CONFIG_STORAGE_LOCATION
        }
    }

    // -----------------------------------------------------------------------------------------
    // Read functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc ICoprocessorConfigBase
    function getCurrentCoprocessorContextId() external view virtual returns (uint256) {
        return _getCoprocessorConfigStorage().currentCoprocessorContextId;
    }

    /// @inheritdoc ICoprocessorConfigBase
    function getPreviousCoprocessorContextId() external view virtual returns (uint256) {
        return _getCoprocessorConfigStorage().previousCoprocessorContextId;
    }

    /// @inheritdoc ICoprocessorConfigBase
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view virtual returns (bool) {
        return _isValidCoprocessorContext(coprocessorContextId);
    }

    /// @inheritdoc ICoprocessorConfigBase
    function getCoprocessorsForContext(
        uint256 coprocessorContextId
    ) external view virtual returns (Coprocessor[] memory coprocessors) {
        CoprocessorConfigStorage storage $ = _getCoprocessorConfigStorage();
        uint256 coprocessorCount = $.coprocessorCountForContext[coprocessorContextId];
        if (coprocessorCount == 0) {
            revert InvalidCoprocessorContext(coprocessorContextId);
        }
        coprocessors = new Coprocessor[](coprocessorCount);
        for (uint256 i = 0; i < coprocessorCount; i++) {
            coprocessors[i] = $.coprocessorsForContext[coprocessorContextId][i];
        }
    }

    /// @inheritdoc ICoprocessorConfigBase
    function getCoprocessorMajorityThresholdForContext(
        uint256 coprocessorContextId
    ) external view virtual returns (uint256) {
        CoprocessorConfigStorage storage $ = _getCoprocessorConfigStorage();
        if ($.coprocessorCountForContext[coprocessorContextId] == 0) {
            revert InvalidCoprocessorContext(coprocessorContextId);
        }
        return $.coprocessorMajorityThresholdForContext[coprocessorContextId];
    }

    /// @inheritdoc ICoprocessorConfigBase
    function isCoprocessorSignerForContext(
        uint256 coprocessorContextId,
        address signer
    ) external view virtual returns (bool) {
        return _getCoprocessorConfigStorage().isCoprocessorSignerForContext[coprocessorContextId][signer];
    }

    /// @inheritdoc ICoprocessorConfigBase
    function isCoprocessorTxSenderForContext(
        uint256 coprocessorContextId,
        address txSender
    ) external view virtual returns (bool) {
        return _getCoprocessorConfigStorage().isCoprocessorTxSenderForContext[coprocessorContextId][txSender];
    }

    // -----------------------------------------------------------------------------------------
    // Internal functions
    // -----------------------------------------------------------------------------------------

    function _isValidCoprocessorContext(uint256 coprocessorContextId) internal view virtual returns (bool) {
        CoprocessorConfigStorage storage $ = _getCoprocessorConfigStorage();
        return
            $.coprocessorCountForContext[coprocessorContextId] != 0 &&
            !$.destroyedCoprocessorContexts[coprocessorContextId];
    }

    /**
     * @dev Checks the set and threshold, stores them under `contextId` and makes it the current
     *      context. Callers pick the ID and emit the event.
     */
    function _storeCoprocessorContext(
        uint256 contextId,
        Coprocessor[] calldata coprocessors,
        uint256 majorityThreshold
    ) internal virtual {
        uint256 coprocessorCount = coprocessors.length;
        if (coprocessorCount == 0) {
            revert EmptyCoprocessors();
        }
        if (coprocessorCount > MAX_COPROCESSORS) {
            revert CoprocessorSetExceedsProofFormatLimit(coprocessorCount, MAX_COPROCESSORS);
        }
        // More than half, so that two conflicting outputs cannot both reach the threshold.
        if (majorityThreshold <= coprocessorCount / 2 || majorityThreshold > coprocessorCount) {
            revert InvalidCoprocessorMajorityThreshold(majorityThreshold, coprocessorCount);
        }

        CoprocessorConfigStorage storage $ = _getCoprocessorConfigStorage();
        for (uint256 i = 0; i < coprocessorCount; i++) {
            Coprocessor calldata coprocessor = coprocessors[i];
            if (coprocessor.txSenderAddress == address(0)) {
                revert CoprocessorNullTxSender();
            }
            if (coprocessor.signerAddress == address(0)) {
                revert CoprocessorNullSigner();
            }
            if (bytes(coprocessor.storageUrl).length == 0) {
                revert CoprocessorEmptyStorageUrl();
            }
            if ($.isCoprocessorTxSenderForContext[contextId][coprocessor.txSenderAddress]) {
                revert CoprocessorTxSenderAlreadyRegistered(coprocessor.txSenderAddress);
            }
            if ($.isCoprocessorSignerForContext[contextId][coprocessor.signerAddress]) {
                revert CoprocessorSignerAlreadyRegistered(coprocessor.signerAddress);
            }
            $.isCoprocessorTxSenderForContext[contextId][coprocessor.txSenderAddress] = true;
            $.isCoprocessorSignerForContext[contextId][coprocessor.signerAddress] = true;
            $.coprocessorsForContext[contextId][i] = coprocessor;
        }
        $.coprocessorCountForContext[contextId] = coprocessorCount;
        $.coprocessorMajorityThresholdForContext[contextId] = majorityThreshold;
        $.previousCoprocessorContextId = $.currentCoprocessorContextId;
        $.currentCoprocessorContextId = contextId;
    }

    /**
     * @dev Destroys a past coprocessor context. Its set and threshold stay readable.
     */
    function _destroyCoprocessorContext(uint256 coprocessorContextId) internal virtual {
        CoprocessorConfigStorage storage $ = _getCoprocessorConfigStorage();
        if (coprocessorContextId == $.currentCoprocessorContextId) {
            revert CurrentCoprocessorContextCannotBeDestroyed(coprocessorContextId);
        }
        if (!_isValidCoprocessorContext(coprocessorContextId)) {
            revert InvalidCoprocessorContext(coprocessorContextId);
        }
        $.destroyedCoprocessorContexts[coprocessorContextId] = true;
        emit CoprocessorContextDestroyed(coprocessorContextId);
    }
}

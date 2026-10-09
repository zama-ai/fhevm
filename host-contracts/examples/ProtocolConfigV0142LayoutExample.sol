// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import {KmsContextAnchor, KmsNode, KmsNodeParams, KmsThresholds} from "../contracts/shared/Structs.sol";
import {EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "../contracts/shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "../contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "../contracts/shared/ACLOwnable.sol";

/// @notice Stand-in for the release v0.14.2 ProtocolConfig proxy: same storage layout and initialized version.
contract ProtocolConfigV0142LayoutExample is UUPSUpgradeableEmptyProxy, ACLOwnable {
    enum ContextState {
        None,
        Pending,
        Created,
        Active
    }

    enum EpochState {
        None,
        Pending,
        Active
    }

    /// @dev Copied verbatim from v0.14.2.
    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfig
    struct ProtocolConfigStorage {
        uint256 currentKmsContextId;
        mapping(uint256 contextId => KmsNode[]) kmsNodesForContext;
        mapping(uint256 contextId => mapping(address txSender => bool isRegistered)) isKmsTxSenderForContext;
        mapping(uint256 contextId => mapping(address signer => bool isRegistered)) isKmsSignerForContext;
        mapping(uint256 contextId => mapping(address txSender => KmsNode node)) kmsNodeByTxSenderForContext;
        mapping(uint256 contextId => address[]) kmsSignerAddressesForContext;
        mapping(uint256 contextId => uint256) publicDecryptionThresholdForContext;
        mapping(uint256 contextId => uint256) userDecryptionThresholdForContext;
        mapping(uint256 contextId => uint256) kmsGenThresholdForContext;
        mapping(uint256 contextId => uint256) mpcThresholdForContext;
        mapping(uint256 contextId => bool) destroyedContexts;
        uint256 latestActiveKmsContextId;
        uint256 epochCounter;
        uint256 latestActiveEpochId;
        mapping(uint256 contextId => ContextState) contextState;
        mapping(uint256 epochId => EpochState) epochState;
        mapping(uint256 epochId => uint256 contextId) contextForEpoch;
        mapping(uint256 contextId => mapping(address txSender => bool confirmed)) contextCreationConfirmedByTxSender;
        mapping(uint256 epochId => mapping(address signer => bool confirmed)) epochActivationConfirmedBySigner;
        mapping(uint256 epochId => mapping(bytes32 dataHash => uint256 confirmations)) epochActivationConfirmationCountForDigest;
        mapping(uint256 contextId => uint256 threshold) contextCreationPreviousTxSenderThreshold;
        mapping(uint256 contextId => uint256 confirmations) contextCreationNewTxSenderConfirmationCount;
        mapping(uint256 contextId => uint256 confirmations) contextCreationPreviousTxSenderConfirmationCount;
        mapping(uint256 contextId => KmsContextAnchor) contextAnchors;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.ProtocolConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_CONFIG_STORAGE_LOCATION =
        0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;

    function _getProtocolConfigStorage() internal pure returns (ProtocolConfigStorage storage $) {
        assembly {
            $.slot := PROTOCOL_CONFIG_STORAGE_LOCATION
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Seeds one active context and epoch at initialized version 3, like v0.14.2.
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds
    ) public virtual onlyFromEmptyProxy reinitializer(3) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 1;
        $.currentKmsContextId = contextId;
        $.latestActiveKmsContextId = contextId;
        $.epochCounter = epochId;
        $.latestActiveEpochId = epochId;
        $.contextState[contextId] = ContextState.Active;
        $.epochState[epochId] = EpochState.Active;
        $.contextForEpoch[epochId] = contextId;
        for (uint256 i = 0; i < kmsNodeParams.length; i++) {
            KmsNode memory node = KmsNode({
                txSenderAddress: kmsNodeParams[i].txSenderAddress,
                signerAddress: kmsNodeParams[i].signerAddress,
                ipAddress: kmsNodeParams[i].ipAddress,
                storageUrl: kmsNodeParams[i].storageUrl
            });
            $.kmsNodesForContext[contextId].push(node);
            $.isKmsTxSenderForContext[contextId][node.txSenderAddress] = true;
            $.isKmsSignerForContext[contextId][node.signerAddress] = true;
            $.kmsNodeByTxSenderForContext[contextId][node.txSenderAddress] = node;
            $.kmsSignerAddressesForContext[contextId].push(node.signerAddress);
        }
        $.publicDecryptionThresholdForContext[contextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[contextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[contextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[contextId] = thresholds.mpc;
    }

    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfigCommon} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigCommon.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";

// Test double for the v0.1.0 storage prefix. It accepts the current KmsNodeParams shape because
// these upgrade tests assert storage continuity, not old ABI compatibility.
contract ProtocolConfigV010TestDouble is UUPSUpgradeableEmptyProxy, ACLOwnable {
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfig
    struct ProtocolConfigV010Storage {
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
    }

    bytes32 private constant PROTOCOL_CONFIG_STORAGE_LOCATION =
        0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;

    function _getProtocolConfigStorage() internal pure returns (ProtocolConfigV010Storage storage $) {
        assembly {
            $.slot := PROTOCOL_CONFIG_STORAGE_LOCATION
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initializeFromEmptyProxy(
        uint256,
        KmsNodeParams[] calldata initialKmsNodeParams,
        IProtocolConfigCommon.KmsThresholds calldata initialThresholds,
        string calldata,
        PcrValues[] calldata
    ) external onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        ProtocolConfigV010Storage storage $ = _getProtocolConfigStorage();
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        _storeInitialContext(initialKmsNodeParams, initialThresholds);
    }

    function getCurrentKmsContextId() external view returns (uint256) {
        return _getProtocolConfigStorage().currentKmsContextId;
    }

    function getKmsSignersForContext(uint256 kmsContextId) external view returns (address[] memory) {
        return _getProtocolConfigStorage().kmsSignerAddressesForContext[kmsContextId];
    }

    function getKmsNodesForContext(uint256 kmsContextId) external view returns (KmsNode[] memory) {
        return _getProtocolConfigStorage().kmsNodesForContext[kmsContextId];
    }

    function getPublicDecryptionThreshold() external view returns (uint256) {
        ProtocolConfigV010Storage storage $ = _getProtocolConfigStorage();
        return $.publicDecryptionThresholdForContext[$.currentKmsContextId];
    }

    function getUserDecryptionThreshold() external view returns (uint256) {
        ProtocolConfigV010Storage storage $ = _getProtocolConfigStorage();
        return $.userDecryptionThresholdForContext[$.currentKmsContextId];
    }

    function getKmsGenThreshold() external view returns (uint256) {
        ProtocolConfigV010Storage storage $ = _getProtocolConfigStorage();
        return $.kmsGenThresholdForContext[$.currentKmsContextId];
    }

    function getMpcThreshold() external view returns (uint256) {
        ProtocolConfigV010Storage storage $ = _getProtocolConfigStorage();
        return $.mpcThresholdForContext[$.currentKmsContextId];
    }

    function getVersion() external pure returns (string memory) {
        return "ProtocolConfig v0.1.0";
    }

    function _storeInitialContext(
        KmsNodeParams[] calldata kmsNodeParams,
        IProtocolConfigCommon.KmsThresholds calldata thresholds
    ) internal {
        ProtocolConfigV010Storage storage $ = _getProtocolConfigStorage();
        uint256 contextId = ++$.currentKmsContextId;

        for (uint256 i = 0; i < kmsNodeParams.length; i++) {
            KmsNodeParams calldata params = kmsNodeParams[i];
            KmsNode memory node = KmsNode({
                txSenderAddress: params.txSenderAddress,
                signerAddress: params.signerAddress,
                ipAddress: params.ipAddress,
                storageUrl: params.storageUrl
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

    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}

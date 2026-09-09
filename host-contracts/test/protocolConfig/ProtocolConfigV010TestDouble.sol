// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode, KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";

// Storage-layout double for the pre-epoch v0.1.0 ProtocolConfig, used to set up the
// `reinitializeV2` migration test. It reproduces only what that test needs: the namespaced
// storage prefix (slots 0-10, a strict prefix of the V2 layout) and an initializer that writes
// the first context so post-upgrade storage-continuity reads have something to resolve against.
// It accepts the current KmsNodeParams shape because the test asserts storage continuity, not old
// ABI compatibility, and v0.1.0 never stored softwareVersion/pcrValues so they are omitted.
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
        KmsNodeParams[] calldata initialKmsNodeParams,
        IProtocolConfig.KmsThresholds calldata initialThresholds
    ) external onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        ProtocolConfigV010Storage storage $ = _getProtocolConfigStorage();
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        uint256 contextId = ++$.currentKmsContextId;

        for (uint256 i = 0; i < initialKmsNodeParams.length; i++) {
            KmsNodeParams calldata params = initialKmsNodeParams[i];
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

        $.publicDecryptionThresholdForContext[contextId] = initialThresholds.publicDecryption;
        $.userDecryptionThresholdForContext[contextId] = initialThresholds.userDecryption;
        $.kmsGenThresholdForContext[contextId] = initialThresholds.kmsGen;
        $.mpcThresholdForContext[contextId] = initialThresholds.mpc;
    }

    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}

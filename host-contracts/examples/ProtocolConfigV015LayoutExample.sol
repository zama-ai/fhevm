// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import {KmsContextAnchor, KmsNode} from "../contracts/shared/Structs.sol";
import {UUPSUpgradeableEmptyProxy} from "../contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "../contracts/shared/ACLOwnable.sol";

/// @notice Copy of the ProtocolConfig storage layout and initialized version at tag v0.15.0-6.
/// @dev Deployed by upgrades.ts as the v0.15 baseline, so the OpenZeppelin upgrades plugin checks that
///      ProtocolConfig and ProtocolConfigReplica keep a compatible storage layout when upgraded in place.
contract ProtocolConfigV015LayoutExample is UUPSUpgradeableEmptyProxy, ACLOwnable {
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

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Sets the initialized version to 4, like v0.15.
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public onlyFromEmptyProxy reinitializer(4) {}

    function _authorizeUpgrade(address _newImplementation) internal override onlyACLOwner {}
}

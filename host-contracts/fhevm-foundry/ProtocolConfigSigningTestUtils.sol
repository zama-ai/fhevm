// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsThresholds, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KEY_COUNTER_BASE, CRS_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {HostContractsDeployerTestUtils} from "./HostContractsDeployerTestUtils.sol";

/**
 * @dev EIP-712 signing helpers for the four ProtocolConfig confirmations, kept out of the deployer so that
 * suites which only deploy the host stack do not inherit them.
 */
abstract contract ProtocolConfigSigningTestUtils is HostContractsDeployerTestUtils {
    bytes32 internal constant EIP712_CONTEXT_CREATION_TYPE_HASH =
        keccak256(
            "ContextCreationConfirmation(uint256 previousContextId,uint256 newContextId,bytes32 nodeConfigHash,bytes extraData)"
        );
    bytes32 internal constant EIP712_EPOCH_ACTIVATION_TYPE_HASH =
        keccak256(
            "EpochActivationConfirmation(uint256 contextId,uint256 previousEpochId,uint256 epochId,bytes32 epochMaterialHash,bytes extraData)"
        );
    bytes32 internal constant EIP712_CONTEXT_DESTRUCTION_TYPE_HASH =
        keccak256("ContextDestructionConfirmation(uint256 destroyedContextId,bytes extraData)");
    bytes32 internal constant EIP712_EPOCH_DESTRUCTION_TYPE_HASH =
        keccak256("EpochDestructionConfirmation(uint256 destroyedEpochId,bytes extraData)");

    /// @dev Shared ProtocolConfig handle bound by each suite's setUp; used by the EIP-712 helpers below.
    ProtocolConfig internal protocolConfig;

    /// @dev nodeConfigHash of each context defined through `_defineNewKmsContextAndEpoch`. A Pending
    ///      context has no getter for its nodes or thresholds, so the helper records what it defined.
    mapping(uint256 contextId => bytes32) internal nodeConfigHashes;

    function _computeProtocolConfigDomainSeparator(
        uint256 chainId,
        address verifyingContract
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPE_HASH,
                    keccak256(bytes("ProtocolConfig")),
                    keccak256(bytes("1")),
                    chainId,
                    verifyingContract
                )
            );
    }

    function _hashProtocolConfigTypedData(bytes32 structHash) internal view virtual returns (bytes32) {
        return
            MessageHashUtils.toTypedDataHash(
                _computeProtocolConfigDomainSeparator(block.chainid, address(protocolConfig)),
                structHash
            );
    }

    function _hashContextCreation(
        uint256 previousContextId,
        uint256 newContextId,
        bytes32 nodeConfigHash,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return
            _hashProtocolConfigTypedData(
                keccak256(
                    abi.encode(
                        EIP712_CONTEXT_CREATION_TYPE_HASH,
                        previousContextId,
                        newContextId,
                        nodeConfigHash,
                        keccak256(extraData)
                    )
                )
            );
    }

    function _hashEpochActivation(
        uint256 contextId,
        uint256 previousEpochId,
        uint256 epochId,
        bytes32 epochMaterialHash,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return
            _hashProtocolConfigTypedData(
                keccak256(
                    abi.encode(
                        EIP712_EPOCH_ACTIVATION_TYPE_HASH,
                        contextId,
                        previousEpochId,
                        epochId,
                        epochMaterialHash,
                        keccak256(extraData)
                    )
                )
            );
    }

    function _hashContextDestruction(
        uint256 destroyedContextId,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return
            _hashProtocolConfigTypedData(
                keccak256(abi.encode(EIP712_CONTEXT_DESTRUCTION_TYPE_HASH, destroyedContextId, keccak256(extraData)))
            );
    }

    function _hashEpochDestruction(uint256 destroyedEpochId, bytes memory extraData) internal view returns (bytes32) {
        return
            _hashProtocolConfigTypedData(
                keccak256(abi.encode(EIP712_EPOCH_DESTRUCTION_TYPE_HASH, destroyedEpochId, keccak256(extraData)))
            );
    }

    /// @dev nodeConfigHash of ContextCreationConfirmation, over the inputs projected onto the stored KmsNode shape.
    function _nodeConfigHash(
        KmsNodeParams[] memory params,
        string[] memory connectorUrls,
        KmsThresholds memory thresholds
    ) internal pure returns (bytes32) {
        return keccak256(abi.encode(_toKmsNodes(params, connectorUrls), thresholds));
    }

    /// @dev epochMaterialHash of EpochActivationConfirmation. CRS entries encode the raw digest bytes.
    function _epochMaterialHash(
        IProtocolConfig.EpochKeyResult[] memory keys,
        IProtocolConfig.EpochCrsResult[] memory crsList
    ) internal pure returns (bytes32) {
        bytes32[] memory keyHashes = new bytes32[](keys.length);
        for (uint256 i = 0; i < keys.length; i++) {
            keyHashes[i] = keccak256(
                abi.encode(keys[i].prepKeygenId, keys[i].keyId, _hashKeyDigests(keys[i].keyDigests))
            );
        }
        bytes32[] memory crsHashes = new bytes32[](crsList.length);
        for (uint256 i = 0; i < crsList.length; i++) {
            crsHashes[i] = keccak256(abi.encode(crsList[i].crsId, crsList[i].maxBitLength, crsList[i].crsDigest));
        }
        return keccak256(abi.encode(keyHashes, crsHashes));
    }

    function _defineNewKmsContextAndEpoch(KmsNodeParams[] memory nodes, KmsThresholds memory thresholds) internal {
        _defineNewKmsContextAndEpoch(nodes, thresholds, "", new PcrValues[](0));
    }

    function _defineNewKmsContextAndEpoch(
        KmsNodeParams[] memory nodes,
        KmsThresholds memory thresholds,
        string memory softwareVersion,
        PcrValues[] memory pcrValues
    ) internal {
        string[] memory connectorUrls = _makeConnectorUrls(nodes.length);
        protocolConfig.defineNewKmsContextAndEpoch(nodes, connectorUrls, thresholds, softwareVersion, pcrValues);
        nodeConfigHashes[protocolConfig.getCurrentKmsContextIdCounter()] = _nodeConfigHash(
            nodes,
            connectorUrls,
            thresholds
        );
    }

    /// @dev Signs ContextCreationConfirmation with the latest active context as previousContextId.
    function _signContextCreation(
        uint256 contextId,
        uint256 pk,
        bytes memory extraData
    ) internal view returns (bytes memory) {
        return
            _computeSignature(
                pk,
                _hashContextCreation(
                    protocolConfig.getCurrentKmsContextId(),
                    contextId,
                    nodeConfigHashes[contextId],
                    extraData
                )
            );
    }

    function _confirmContextCreation(uint256 contextId, uint256 pk, bytes memory extraData) internal {
        protocolConfig.confirmKmsContextCreation(contextId, _signContextCreation(contextId, pk, extraData), extraData);
    }

    /// @dev One key result `keyId` and one CRS result `crsId`.
    function _buildEpochResults(
        uint256 keyId,
        uint256 crsId
    )
        internal
        pure
        returns (IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList)
    {
        keys = new IProtocolConfig.EpochKeyResult[](1);
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: _prepKeygenIdForKeyId(keyId),
            keyId: keyId,
            keyDigests: _mockKeyDigests()
        });

        crsList = new IProtocolConfig.EpochCrsResult[](1);
        crsList[0] = IProtocolConfig.EpochCrsResult({crsId: crsId, maxBitLength: 4096, crsDigest: hex"deadbeef"});
    }

    /// @dev Signs EpochActivationConfirmation with the latest active epoch as previousEpochId.
    function _signEpochActivation(
        uint256 contextId,
        uint256 epochId,
        uint256 pk,
        IProtocolConfig.EpochKeyResult[] memory keys,
        IProtocolConfig.EpochCrsResult[] memory crsList,
        bytes memory extraData
    ) internal view returns (bytes memory) {
        (, uint256 previousEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        return
            _computeSignature(
                pk,
                _hashEpochActivation(contextId, previousEpochId, epochId, _epochMaterialHash(keys, crsList), extraData)
            );
    }

    /// @dev Setup helper that drives a pending epoch to Active for suites that need a usable KMS context.
    ///      Callers need only the active epoch, not a specific payload, so this submits constant material.
    function _confirmEpochActivation(uint256 contextId, uint256 epochId, uint256 pk) internal {
        // An empty `keys` or `crsList` reverts with EmptyEpochActivationResults, so supply one deterministic
        // result of each kind with constant ids. Every signer produces the same epochMaterialHash.
        // The ids need not exist in KMSGeneration because confirmEpochActivation checks only signer recovery.
        _confirmEpochActivation(contextId, epochId, pk, KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
    }

    /// @dev Confirms `epochId` for `pk` with one key result `keyId` and one CRS result `crsId`.
    function _confirmEpochActivation(
        uint256 contextId,
        uint256 epochId,
        uint256 pk,
        uint256 keyId,
        uint256 crsId
    ) internal {
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(keyId, crsId);
        _submitEpochActivation(contextId, epochId, pk, keys, crsList);
    }

    /// @dev Signs `keys` and `crsList` for `pk` with empty extraData and submits them.
    function _submitEpochActivation(
        uint256 contextId,
        uint256 epochId,
        uint256 pk,
        IProtocolConfig.EpochKeyResult[] memory keys,
        IProtocolConfig.EpochCrsResult[] memory crsList
    ) internal {
        protocolConfig.confirmEpochActivation(
            epochId,
            keys,
            crsList,
            _signEpochActivation(contextId, epochId, pk, keys, crsList, ""),
            ""
        );
    }

    function _signContextDestruction(uint256 contextId, uint256 pk) internal view returns (bytes memory) {
        return _computeSignature(pk, _hashContextDestruction(contextId, ""));
    }

    function _signEpochDestruction(
        uint256 epochId,
        uint256 pk,
        bytes memory extraData
    ) internal view returns (bytes memory) {
        return _computeSignature(pk, _hashEpochDestruction(epochId, extraData));
    }
}

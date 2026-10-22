// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {ProtocolConfigTestBase} from "../protocolConfig/protocolConfig.t.sol";
import {DeployableERC1967Proxy} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsThresholds, KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

/// @dev The canonical collects the KMS confirmations through confirm*. The replica on another chain
///      applies the same signature bytes through its submit functions.
contract ProtocolConfigCanonicalToReplicaTest is ProtocolConfigTestBase {
    uint256 internal constant CTX1 = KMS_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant CTX2 = KMS_CONTEXT_COUNTER_BASE + 2;
    uint256 internal constant EPOCH1 = EPOCH_COUNTER_BASE + 1;
    uint256 internal constant EPOCH2 = EPOCH_COUNTER_BASE + 2;
    uint256 internal constant EPOCH3 = EPOCH_COUNTER_BASE + 3;
    uint64 internal constant REPLICA_CHAIN_ID = 8453;
    bytes internal constant EXTRA = hex"c0de";

    /// @dev Signatures and the epochMaterialHash of the `topic0` confirmations emitted since the last call.
    function _collect(bytes32 topic0) internal view returns (bytes[] memory signatures, bytes32 epochMaterialHash) {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        signatures = new bytes[](logs.length);
        uint256 count;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter != address(protocolConfig) || logs[i].topics[0] != topic0) {
                continue;
            }
            if (topic0 == IProtocolConfig.EpochActivationConfirmation.selector) {
                (epochMaterialHash, signatures[count], ) = abi.decode(logs[i].data, (bytes32, bytes, bytes));
            } else {
                (signatures[count], ) = abi.decode(logs[i].data, (bytes, bytes));
            }
            count++;
        }
        assembly {
            mstore(signatures, count)
        }
    }

    function _thresholdsForContext(uint256 contextId) internal view returns (KmsThresholds memory) {
        return
            KmsThresholds({
                publicDecryption: protocolConfig.getPublicDecryptionThresholdForContext(contextId),
                userDecryption: protocolConfig.getUserDecryptionThresholdForContext(contextId),
                kmsGen: protocolConfig.getKmsGenThresholdForContext(contextId),
                mpc: protocolConfig.getMpcThresholdForContext(contextId)
            });
    }

    function test_canonicalConfirmationsApplyOnReplica() public {
        // Canonical genesis committee A = {signer0, signer1}, on the default chain.
        _setupEpochLifecycle();
        uint64 canonicalChainId = uint64(block.chainid);

        // Replica bootstrapped from the canonical genesis state, on its own proxy.
        ProtocolConfigReplica replica = ProtocolConfigReplica(
            address(
                new DeployableERC1967Proxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()))
            )
        );
        address replicaImplementation = address(new ProtocolConfigReplica());
        vm.prank(owner);
        replica.upgradeToAndCall(
            replicaImplementation,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    CTX1,
                    EPOCH1,
                    _toKmsNodes(_makeKmsNodeParams(2), _makeConnectorUrls(2)),
                    _defaultThresholds(),
                    canonicalChainId,
                    address(protocolConfig)
                )
            )
        );

        // Context switch to B = {signer2, signer3}. The replica gets the one previous-committee
        // signature (n - t = 1) that canonical needed.
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        // Each _collect call consumes the logs recorded since the previous one.
        vm.recordLogs();
        _confirmContextCreation(CTX2, kmsPk0, EXTRA);
        (bytes[] memory contextSignatures, ) = _collect(IProtocolConfig.KmsContextCreationConfirmation.selector);
        _confirmContextCreation(CTX2, kmsPk2, EXTRA);
        _confirmContextCreation(CTX2, kmsPk3, EXTRA);
        _confirmEpochActivationWithDummyMaterial(CTX2, EPOCH2, kmsPk2);
        _confirmEpochActivationWithDummyMaterial(CTX2, EPOCH2, kmsPk3);
        (bytes[] memory switchSignatures, bytes32 switchMaterial) = _collect(
            IProtocolConfig.EpochActivationConfirmation.selector
        );

        // Key resharing under B.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpochActivationWithDummyMaterial(CTX2, EPOCH3, kmsPk2);
        _confirmEpochActivationWithDummyMaterial(CTX2, EPOCH3, kmsPk3);
        (bytes[] memory reshareSignatures, bytes32 reshareMaterial) = _collect(
            IProtocolConfig.EpochActivationConfirmation.selector
        );

        // Destruction of the superseded epoch and context, confirmed by B.
        vm.startPrank(owner);
        protocolConfig.destroyKmsEpoch(EPOCH2);
        protocolConfig.destroyKmsContext(CTX1);
        vm.stopPrank();
        protocolConfig.confirmKmsEpochDestruction(
            EPOCH2,
            _computeSignature(kmsPk2, _hashEpochDestruction(EPOCH2, EXTRA)),
            EXTRA
        );
        (bytes[] memory epochDestructionSignatures, ) = _collect(
            IProtocolConfig.KmsEpochDestructionConfirmation.selector
        );
        protocolConfig.confirmKmsContextDestruction(
            CTX1,
            _computeSignature(kmsPk2, _hashContextDestruction(CTX1, EXTRA)),
            EXTRA
        );
        (bytes[] memory contextDestructionSignatures, ) = _collect(
            IProtocolConfig.KmsContextDestructionConfirmation.selector
        );

        // Apply the same signature bytes on the replica chain, with nodes and thresholds read from canonical.
        vm.chainId(REPLICA_CHAIN_ID);
        replica.submitKmsContextSwitch(
            KmsContextSubmission({
                contextId: CTX2,
                nodes: protocolConfig.getKmsNodesForContext(CTX2),
                thresholds: _thresholdsForContext(CTX2),
                signatures: contextSignatures,
                extraData: EXTRA
            }),
            KmsEpochSubmission({
                epochId: EPOCH2,
                epochMaterialHash: switchMaterial,
                signatures: switchSignatures,
                extraData: ""
            })
        );
        replica.submitKmsKeyResharing(
            KmsEpochSubmission({
                epochId: EPOCH3,
                epochMaterialHash: reshareMaterial,
                signatures: reshareSignatures,
                extraData: ""
            })
        );
        replica.submitKmsEpochDestruction(EPOCH2, epochDestructionSignatures, EXTRA);
        replica.submitKmsContextDestruction(CTX1, contextDestructionSignatures, EXTRA);

        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        (uint256 replicaContextId, uint256 replicaEpochId) = replica.getCurrentKmsContextAndEpoch();
        assertEq(replicaContextId, contextId);
        assertEq(replicaEpochId, epochId);
        assertEq(
            keccak256(abi.encode(replica.getKmsNodesForContext(CTX2))),
            keccak256(abi.encode(protocolConfig.getKmsNodesForContext(CTX2)))
        );
        assertEq(replica.getKmsSignersForContext(CTX2), protocolConfig.getKmsSignersForContext(CTX2));
        assertEq(
            replica.getPublicDecryptionThresholdForContext(CTX2),
            protocolConfig.getPublicDecryptionThresholdForContext(CTX2)
        );
        assertEq(
            replica.getUserDecryptionThresholdForContext(CTX2),
            protocolConfig.getUserDecryptionThresholdForContext(CTX2)
        );
        assertEq(replica.getKmsGenThresholdForContext(CTX2), protocolConfig.getKmsGenThresholdForContext(CTX2));
        assertEq(replica.getMpcThresholdForContext(CTX2), protocolConfig.getMpcThresholdForContext(CTX2));
        assertTrue(replica.isValidEpochForContext(CTX2, EPOCH3));
        assertFalse(replica.isValidEpochForContext(CTX2, EPOCH2));
        assertFalse(replica.isLiveKmsContext(CTX1));
    }
}

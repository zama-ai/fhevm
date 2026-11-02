// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {DeployableERC1967Proxy, HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsThresholds, KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

/// @dev The canonical collects the KMS confirmations through confirm*. The replica on another chain
///      applies the same signature bytes through its submit functions.
contract ProtocolConfigCanonicalToReplicaTest is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);
    uint256 internal constant CTX1 = KMS_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant CTX2 = KMS_CONTEXT_COUNTER_BASE + 2;
    uint256 internal constant EPOCH1 = EPOCH_COUNTER_BASE + 1;
    uint256 internal constant EPOCH2 = EPOCH_COUNTER_BASE + 2;
    uint256 internal constant EPOCH3 = EPOCH_COUNTER_BASE + 3;
    uint64 internal constant REPLICA_CHAIN_ID = 8453;
    bytes internal constant EXTRA = hex"c0de";

    /// @dev Both committees have 4 nodes and MPC threshold 1, so the `n - t` quorum is 3.
    uint256 internal constant N = 4;
    uint256 internal constant QUORUM = 3;

    /// @dev Signatures, epochMaterialHash and extraData of the `topic0` confirmations emitted since the last call.
    function _collect(
        bytes32 topic0
    ) internal view returns (bytes[] memory signatures, bytes32 epochMaterialHash, bytes memory extraData) {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        signatures = new bytes[](logs.length);
        uint256 count;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter != address(protocolConfig) || logs[i].topics[0] != topic0) {
                continue;
            }
            if (topic0 == IProtocolConfig.EpochActivationConfirmation.selector) {
                (epochMaterialHash, signatures[count], extraData) = abi.decode(logs[i].data, (bytes32, bytes, bytes));
            } else {
                (signatures[count], extraData) = abi.decode(logs[i].data, (bytes, bytes));
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

    /// @dev Signer key `i` of the new committee B.
    function _newPk(uint256 i) internal pure returns (uint256) {
        return _defaultSignerPk(N + i);
    }

    /// @dev Every signer of B confirms `epochId` with a non-empty extraData.
    function _confirmEpochByNewCommittee(uint256 epochId) internal {
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        for (uint256 i = 0; i < N; i++) {
            _confirmEpochActivation(CTX2, epochId, _newPk(i), keys, crsList, EXTRA);
        }
    }

    function test_canonicalConfirmationsApplyOnReplica() public {
        // Canonical genesis committee A = signers 0 to 3, on the default chain.
        _deployACL(owner);
        (protocolConfig, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(N), _defaultThresholds());
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
                    _toKmsNodes(_makeKmsNodeParams(N)),
                    _defaultThresholds(),
                    canonicalChainId,
                    address(protocolConfig)
                )
            )
        );

        // Context switch to B = signers 4 to 7. The replica gets the `n - t` previous-committee
        // signatures that canonical needed.
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(N);
        for (uint256 i = 0; i < N; i++) {
            nodes[i].signerAddress = vm.addr(_newPk(i));
        }
        vm.prank(owner);
        bytes32 nodeConfigHash = _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        // Each _collect call consumes the logs recorded since the previous one.
        vm.recordLogs();
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _defaultSignerPk(i), EXTRA);
        }
        (bytes[] memory contextSignatures, , ) = _collect(IProtocolConfig.KmsContextCreationConfirmation.selector);
        assertEq(contextSignatures.length, QUORUM);
        for (uint256 i = 0; i < N; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _newPk(i), EXTRA);
        }
        _confirmEpochByNewCommittee(EPOCH2);
        KmsEpochSubmission memory switchEpoch;
        (switchEpoch.signatures, switchEpoch.epochMaterialHash, switchEpoch.extraData) = _collect(
            IProtocolConfig.EpochActivationConfirmation.selector
        );
        switchEpoch.epochId = EPOCH2;

        // Key resharing under B.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpochByNewCommittee(EPOCH3);
        KmsEpochSubmission memory reshareEpoch;
        (reshareEpoch.signatures, reshareEpoch.epochMaterialHash, reshareEpoch.extraData) = _collect(
            IProtocolConfig.EpochActivationConfirmation.selector
        );
        reshareEpoch.epochId = EPOCH3;

        // Destruction of the superseded epoch and context, confirmed by `n - t` signers of B.
        vm.startPrank(owner);
        protocolConfig.destroyKmsEpoch(EPOCH2);
        protocolConfig.destroyKmsContext(CTX1);
        vm.stopPrank();
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmEpochDestruction(EPOCH2, _newPk(i), EXTRA);
        }
        (bytes[] memory epochDestructionSignatures, , ) = _collect(
            IProtocolConfig.KmsEpochDestructionConfirmation.selector
        );
        assertEq(epochDestructionSignatures.length, QUORUM);
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextDestruction(CTX1, _newPk(i), EXTRA);
        }
        (bytes[] memory contextDestructionSignatures, , ) = _collect(
            IProtocolConfig.KmsContextDestructionConfirmation.selector
        );
        assertEq(contextDestructionSignatures.length, QUORUM);

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
            switchEpoch
        );
        replica.submitKmsKeyResharing(reshareEpoch);
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

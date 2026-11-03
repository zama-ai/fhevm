// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {DeployableERC1967Proxy} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfigReplicaTestBase} from "./ProtocolConfigReplicaTestBase.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsThresholds, KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

/// @dev The canonical collects the KMS confirmations through confirm*. The replica on another chain
///      applies the same signature bytes through its submit functions.
contract ProtocolConfigCanonicalToReplicaTest is ProtocolConfigReplicaTestBase {
    uint256 internal constant CTX2 = KMS_CONTEXT_COUNTER_BASE + 2;
    uint256 internal constant EPOCH2 = EPOCH_COUNTER_BASE + 2;
    uint256 internal constant EPOCH3 = EPOCH_COUNTER_BASE + 3;
    uint64 internal constant REPLICA_CHAIN_ID = 8453;

    /// @dev The canonical is the local ProtocolConfig, and the KMS signers sign under its domain.
    constructor() {
        signingChainId = block.chainid;
        signingVerifyingContract = protocolConfigAdd;
    }

    /// @dev Log data of the `topic0` confirmations emitted since the last call.
    function _collectData(bytes32 topic0) internal view returns (bytes[] memory data) {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        data = new bytes[](logs.length);
        uint256 count;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter == address(protocolConfig) && logs[i].topics[0] == topic0) {
                data[count++] = logs[i].data;
            }
        }
        assembly {
            mstore(data, count)
        }
    }

    /// @dev Signatures of the `topic0` confirmations emitted since the last call. `topic0` is a context
    ///      creation or a destruction confirmation.
    function _collectSignatures(bytes32 topic0) internal view returns (bytes[] memory signatures) {
        bytes[] memory data = _collectData(topic0);
        signatures = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            (signatures[i], ) = abi.decode(data[i], (bytes, bytes));
        }
    }

    /// @dev The activation of `epochId` with the EpochActivationConfirmation signatures emitted since the last call.
    function _collectEpoch(uint256 epochId) internal view returns (KmsEpochSubmission memory epoch) {
        bytes[] memory data = _collectData(IProtocolConfig.EpochActivationConfirmation.selector);
        epoch.epochId = epochId;
        epoch.signatures = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            (epoch.epochMaterialHash, epoch.signatures[i], epoch.extraData) = abi.decode(
                data[i],
                (bytes32, bytes, bytes)
            );
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

    /// @dev The switch to CTX2 with the canonical's stored nodes and thresholds.
    function _ctx2Submission(bytes[] memory signatures) internal view returns (KmsContextSubmission memory) {
        return
            KmsContextSubmission({
                contextId: CTX2,
                nodes: protocolConfig.getKmsNodesForContext(CTX2),
                thresholds: _thresholdsForContext(CTX2),
                signatures: signatures,
                extraData: EXTRA
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

    /// @dev Deploys the canonical with genesis committee A = signers 0 to 3 on the default chain, and a
    ///      replica bootstrapped from its genesis state on its own proxy. Then defines CTX2 with committee
    ///      B = signers 4 to 7, starts recording logs, and returns the nodeConfigHash that both committees sign.
    function _setupCanonicalAndReplica() internal returns (bytes32 nodeConfigHash) {
        _deployACL(owner);
        (protocolConfig, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(N), _defaultThresholds());

        protocolConfigReplica = ProtocolConfigReplica(
            address(
                new DeployableERC1967Proxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()))
            )
        );
        address replicaImplementation = address(new ProtocolConfigReplica());
        vm.prank(owner);
        protocolConfigReplica.upgradeToAndCall(
            replicaImplementation,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    CTX1,
                    EPOCH1,
                    _toKmsNodes(_makeKmsNodeParams(N)),
                    _defaultThresholds(),
                    uint64(block.chainid),
                    address(protocolConfig)
                )
            )
        );

        KmsNodeParams[] memory nodes = _makeKmsNodeParams(N);
        for (uint256 i = 0; i < N; i++) {
            nodes[i].signerAddress = vm.addr(_newPk(i));
        }
        vm.prank(owner);
        nodeConfigHash = _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        // Each collect call consumes the logs recorded since the previous one.
        vm.recordLogs();
    }

    function test_canonicalConfirmationsApplyOnReplica() public {
        bytes32 nodeConfigHash = _setupCanonicalAndReplica();

        // Context switch to B. The replica gets the `n - t` previous-committee signatures that canonical needed.
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _defaultSignerPk(i), EXTRA);
        }
        bytes[] memory contextSignatures = _collectSignatures(IProtocolConfig.KmsContextCreationConfirmation.selector);
        assertEq(contextSignatures.length, QUORUM);
        for (uint256 i = 0; i < N; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _newPk(i), EXTRA);
        }
        _confirmEpochByNewCommittee(EPOCH2);
        KmsEpochSubmission memory switchEpoch = _collectEpoch(EPOCH2);

        // Key resharing under B.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpochByNewCommittee(EPOCH3);
        KmsEpochSubmission memory reshareEpoch = _collectEpoch(EPOCH3);

        // Destruction of the superseded epoch and context, confirmed by `n - t` signers of B.
        vm.startPrank(owner);
        protocolConfig.destroyKmsEpoch(EPOCH2);
        protocolConfig.destroyKmsContext(CTX1);
        vm.stopPrank();
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmEpochDestruction(EPOCH2, _newPk(i), EXTRA);
        }
        bytes[] memory epochDestructionSignatures = _collectSignatures(
            IProtocolConfig.KmsEpochDestructionConfirmation.selector
        );
        assertEq(epochDestructionSignatures.length, QUORUM);
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextDestruction(CTX1, _newPk(i), EXTRA);
        }
        bytes[] memory contextDestructionSignatures = _collectSignatures(
            IProtocolConfig.KmsContextDestructionConfirmation.selector
        );
        assertEq(contextDestructionSignatures.length, QUORUM);

        // Apply the same signature bytes on the replica chain, with nodes and thresholds read from canonical.
        vm.chainId(REPLICA_CHAIN_ID);
        protocolConfigReplica.submitKmsContextSwitch(_ctx2Submission(contextSignatures), switchEpoch);
        protocolConfigReplica.submitKmsKeyResharing(reshareEpoch);
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH2, epochDestructionSignatures, EXTRA);
        protocolConfigReplica.submitKmsContextDestruction(CTX1, contextDestructionSignatures, EXTRA);

        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        _assertActive(contextId, epochId);
        assertEq(
            keccak256(abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX2))),
            keccak256(abi.encode(protocolConfig.getKmsNodesForContext(CTX2)))
        );
        assertEq(protocolConfigReplica.getKmsSignersForContext(CTX2), protocolConfig.getKmsSignersForContext(CTX2));
        assertEq(
            protocolConfigReplica.getPublicDecryptionThresholdForContext(CTX2),
            protocolConfig.getPublicDecryptionThresholdForContext(CTX2)
        );
        assertEq(
            protocolConfigReplica.getUserDecryptionThresholdForContext(CTX2),
            protocolConfig.getUserDecryptionThresholdForContext(CTX2)
        );
        assertEq(
            protocolConfigReplica.getKmsGenThresholdForContext(CTX2),
            protocolConfig.getKmsGenThresholdForContext(CTX2)
        );
        assertEq(protocolConfigReplica.getMpcThresholdForContext(CTX2), protocolConfig.getMpcThresholdForContext(CTX2));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH3));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH2));
        assertFalse(protocolConfigReplica.isLiveKmsContext(CTX1));
    }

    /// @dev The canonical emits KmsContextCreationConfirmation for both committees. A submitter that
    ///      forwards every event also forwards the new committee's signatures, which the replica rejects.
    function test_revertSwitchWithEveryCreationConfirmation() public {
        bytes32 nodeConfigHash = _setupCanonicalAndReplica();

        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _defaultSignerPk(i), EXTRA);
        }
        for (uint256 i = 0; i < N; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _newPk(i), EXTRA);
        }
        bytes[] memory contextSignatures = _collectSignatures(IProtocolConfig.KmsContextCreationConfirmation.selector);
        assertEq(contextSignatures.length, QUORUM + N);
        _confirmEpochByNewCommittee(EPOCH2);
        KmsEpochSubmission memory switchEpoch = _collectEpoch(EPOCH2);
        KmsContextSubmission memory context = _ctx2Submission(contextSignatures);

        vm.chainId(REPLICA_CHAIN_ID);
        // The first signature of B follows the `n - t` signatures of A.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, vm.addr(_newPk(0)), CTX1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, switchEpoch);
    }
}

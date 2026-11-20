// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {DeployableERC1967Proxy} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsThresholds, KmsNode, KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ProtocolConfigReplicaTestBase} from "./ProtocolConfigReplicaTestBase.sol";
import {ProtocolConfigV014StateWriter} from "../ProtocolConfigV014StateWriter.sol";
import {ProtocolConfigV014LayoutExample} from "@fhevm-host-contracts/examples/ProtocolConfigV014LayoutExample.sol";

contract ProtocolConfigReplicaTest is ProtocolConfigReplicaTestBase {
    uint64 internal constant REPLICA_CHAIN_ID = 8453;

    address internal replicaImplementation;

    // -----------------------------------------------------------------------
    // Canonical mirror initializer
    // -----------------------------------------------------------------------

    function _assertInitializeFromCanonicalReverts(uint256 contextId, uint256 epochId, bytes memory err) internal {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigReplica());
        KmsNode[] memory nodes = _nodes(SEED0);

        vm.expectRevert(err);
        _initializeReplica(
            protocolConfigAdd,
            impl,
            owner,
            contextId,
            epochId,
            nodes,
            _thresholds(),
            CANONICAL_ADDRESS,
            CANONICAL_CHAIN_ID
        );
    }

    function test_initializeFromCanonicalStoresNodes() public {
        _setupReplica();
        assertEq(abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX1)), abi.encode(_nodes(SEED0)));
    }

    function test_revertInitializeFromCanonicalInvalidEpochId() public {
        _assertInitializeFromCanonicalReverts(
            CTX1,
            EPOCH_COUNTER_BASE,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH_COUNTER_BASE)
        );
    }

    function test_revertInitializeFromCanonicalInvalidContextId() public {
        _assertInitializeFromCanonicalReverts(
            KMS_CONTEXT_COUNTER_BASE,
            EPOCH1,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, KMS_CONTEXT_COUNTER_BASE)
        );
    }

    // -----------------------------------------------------------------------
    // Upgrade from ProtocolConfig
    // -----------------------------------------------------------------------

    /// @dev Deploys a canonical ProtocolConfig with 2 nodes of committee SEED0, and the replica implementation.
    ///      `_upgradeToReplica` then makes no deployment, so a test can expect a revert on the upgrade.
    function _setupCanonical() internal returns (ProtocolConfig pc) {
        _deployACL(owner);
        (pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
        replicaImplementation = address(new ProtocolConfigReplica());
    }

    /// @dev Deploys a v0.14 ProtocolConfig proxy whose active pair is CTX1 and EPOCH1, with the 2 nodes of
    ///      committee SEED0, and the replica implementation.
    function _setupV014() internal returns (ProtocolConfigV014StateWriter writer) {
        _deployACL(owner);
        writer = _deployV014ProtocolConfig(owner);
        writer.storeActiveContext(CTX1, EPOCH1, _toV014Nodes(_nodes(SEED0, 2)), _defaultThresholds());
        replicaImplementation = address(new ProtocolConfigReplica());
    }

    function _upgradeToReplica() internal {
        vm.prank(owner);
        ProtocolConfig(protocolConfigAdd).upgradeToAndCall(
            replicaImplementation,
            abi.encodeCall(ProtocolConfigReplica.reinitializeV3, (CANONICAL_ADDRESS, CANONICAL_CHAIN_ID))
        );
        protocolConfigReplica = ProtocolConfigReplica(protocolConfigAdd);
    }

    // -----------------------------------------------------------------------
    // Re-initialization protection
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalOnUpgradeFromProtocolConfig() public {
        ProtocolConfig pc = _setupCanonical();

        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        pc.upgradeToAndCall(
            replicaImplementation,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (CTX1, EPOCH1, _nodes(SEED0), _thresholds(), CANONICAL_ADDRESS, CANONICAL_CHAIN_ID)
            )
        );
    }

    function test_reinitializeV3OnUpgradeFromProtocolConfigKeepsStateAndMirrors() public {
        _setupV014();
        uint256 contextId = CTX1;
        address[] memory signers = new address[](2);
        signers[0] = _signers(SEED0)[0];
        signers[1] = _signers(SEED0)[1];

        _upgradeToReplica();

        _assertCanonicalDomain();
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.3.0");
        _assertActive(contextId, EPOCH1);
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), signers);
        assertEq(
            keccak256(abi.encode(_thresholdsOf(protocolConfigReplica, contextId))),
            keccak256(abi.encode(_defaultThresholds()))
        );

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV3(CANONICAL_ADDRESS, CANONICAL_CHAIN_ID);

        // The replica verifies signatures against the committee that ProtocolConfig stored before the upgrade.
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, 2));
        _assertActive(contextId, EPOCH2);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, EPOCH3);
        _assertActive(contextId, EPOCH3);
    }

    /// @dev A pending context leaves the canonical counter above the active ID. The replica reports the active ID.
    function test_reinitializeV3ReportsActiveContextId() public {
        ProtocolConfigV014StateWriter writer = _setupV014();
        writer.storeContext(
            CTX2,
            _toV014Nodes(_nodes(SEED0, 2)),
            _defaultThresholds(),
            ProtocolConfigV014LayoutExample.ContextState.Pending
        );

        _upgradeToReplica();

        assertEq(protocolConfigReplica.getCurrentKmsContextId(), CTX1);

        // v0.14 stored the pending 2-node SEED0 committee under `CTX2`. Mirroring that ID replaces it.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH2, _nodes(SEED1), _thresholds());
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), CTX2);
        _assertActive(CTX2, EPOCH2);
        _assertMembers(CTX2, SEED1);
        _assertNotMembers(CTX2, SEED0);
    }

    /// @dev The v0.14 state holds a destroyed C2 and a pending E2 above the active pair. The upgrade
    ///      keeps the active pair, and the next canonical activations apply.
    function test_reinitializeV3WithDestroyedContextAndPendingEpoch() public {
        ProtocolConfigV014StateWriter writer = _setupV014();
        writer.storeContext(
            CTX2,
            _toV014Nodes(_nodes(SEED0, 2)),
            _defaultThresholds(),
            ProtocolConfigV014LayoutExample.ContextState.None
        );
        writer.storeEpoch(EPOCH2, CTX1, ProtocolConfigV014LayoutExample.EpochState.Pending);

        _upgradeToReplica();
        _assertActive(CTX1, EPOCH1);

        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, 2));
        _assertActive(CTX1, EPOCH2);
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX3, SEED1, SEED0, 2),
            _epochSubmission(CTX3, EPOCH2, EPOCH3, SEED1, N)
        );
        _assertActive(CTX3, EPOCH3);
    }

    /// @dev A v0.14 non-canonical host initialized from the canonical at CTX2, so CTX1 holds no committee,
    ///      then mirrored CTX3. It also holds the destroyed CTX4. The committees are stored in reverse address order, with long and short
    ///      strings. After the upgrade, the getters return the v0.14 fields in definition order with an empty
    ///      `connectorUrl`, and the CTX3 committee verifies an epoch submission. A mirror over CTX2 with one
    ///      node fewer then clears the slots of the last element, the data of its long `ipAddress` included.
    function test_reinitializeV3MigratesV014CommitteesThenMirrorsOverOne() public {
        _deployACL(owner);
        replicaImplementation = address(new ProtocolConfigReplica());
        KmsNode[] memory sortedNodes = _nodes(SEED0);
        KmsNode[] memory nodes2 = _v014Fixture(sortedNodes);
        KmsNode[] memory nodes3 = _v014Fixture(_nodes(SEED1));
        ProtocolConfigV014StateWriter writer = _deployV014ProtocolConfig(owner);
        writer.storeActiveContext(CTX2, EPOCH1, _toV014Nodes(nodes2), _thresholds());
        writer.storeActiveContext(CTX3, EPOCH2, _toV014Nodes(nodes3), _thresholds());
        KmsNode[] memory nodes4 = _v014Fixture(_nodes(SEED2));
        writer.storeContext(
            CTX4,
            _toV014Nodes(nodes4),
            _thresholds(),
            ProtocolConfigV014LayoutExample.ContextState.None
        );
        _upgradeToReplica();

        assertEq(uint256(vm.load(protocolConfigAdd, _kmsNodesSlot(CTX1))), 0);
        _assertActive(CTX3, EPOCH2);
        _assertStoredNodes(CTX2, nodes2);
        _assertStoredNodes(CTX3, nodes3);
        bytes32 destroyedSecondNode = bytes32(uint256(keccak256(abi.encode(_kmsNodesSlot(CTX4)))) + 5);
        assertEq(vm.load(protocolConfigAdd, destroyedSecondNode), bytes32(uint256(uint160(nodes4[1].txSenderAddress))));
        for (uint256 i = 0; i < N; i++) {
            assertEq(
                abi.encode(protocolConfigReplica.getKmsNodeForContext(CTX4, nodes4[i].txSenderAddress)),
                abi.encode(nodes4[i])
            );
        }
        address[] memory signers = protocolConfigReplica.getKmsSignersForContext(CTX3);
        for (uint256 i = 0; i < N; i++) {
            assertEq(signers[i], nodes3[i].signerAddress);
        }
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX3, EPOCH2, EPOCH3, SEED1, N));
        _assertActive(CTX3, EPOCH3);

        uint256 lastElement = uint256(keccak256(abi.encode(_kmsNodesSlot(CTX2)))) + 5 * (N - 1);
        bytes32 lastIpAddressData = keccak256(abi.encode(lastElement + 2));
        assertNotEq(vm.load(protocolConfigAdd, lastIpAddressData), bytes32(0));
        KmsNode[] memory newNodes = _nodes(SEED0, N - 1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH4, newNodes, _thresholds());

        _assertStoredNodes(CTX2, newNodes);
        for (uint256 i = 0; i < 5; i++) {
            assertEq(vm.load(protocolConfigAdd, bytes32(lastElement + i)), bytes32(0));
        }
        assertEq(vm.load(protocolConfigAdd, lastIpAddressData), bytes32(0));
        address droppedTxSender = sortedNodes[N - 1].txSenderAddress;
        KmsNode memory emptyNode;
        assertEq(abi.encode(protocolConfigReplica.getKmsNodeForContext(CTX2, droppedTxSender)), abi.encode(emptyNode));
        assertFalse(protocolConfigReplica.isKmsTxSenderForContext(CTX2, droppedTxSender));
        assertFalse(protocolConfigReplica.isKmsSignerForContext(CTX2, sortedNodes[N - 1].signerAddress));
    }

    // -----------------------------------------------------------------------
    // Canonical to replica: the canonical collects the KMS confirmations through confirm*.
    // The replica on another chain applies the same signature bytes through its submit functions.
    // -----------------------------------------------------------------------

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

    /// @dev The switch to `contextId` with the canonical's stored nodes and thresholds.
    function _canonicalContextSubmission(
        uint256 contextId,
        bytes[] memory signatures
    ) internal view returns (KmsContextSubmission memory) {
        return
            KmsContextSubmission({
                contextId: contextId,
                nodes: protocolConfig.getKmsNodesForContext(contextId),
                thresholds: _thresholdsOf(protocolConfig, contextId),
                signatures: signatures,
                extraData: EXTRA
            });
    }

    /// @dev Every signer in `signerKeys` confirms `epochId` under `contextId` with a non-empty extraData.
    function _confirmEpoch(uint256 contextId, uint256 epochId, uint256[] memory signerKeys) internal {
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _dummyEpochResults();
        for (uint256 i = 0; i < signerKeys.length; i++) {
            _confirmEpochActivation(
                contextId,
                epochId,
                signerKeys[i],
                keys,
                crsList,
                _materialExtraData(contextId, epochId),
                EXTRA
            );
        }
    }

    /// @dev Deploys the canonical with genesis committee A = signers 0 to 3 on the default chain, and a
    ///      replica bootstrapped from its genesis state on its own proxy. Then defines CTX2 with committee
    ///      B = SEED1, starts recording logs, and returns the nodeConfigHash that both committees sign.
    function _setupCanonicalAndReplica() internal returns (bytes32 nodeConfigHash) {
        _deployACL(owner);
        (protocolConfig, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(N), _defaultThresholds());

        protocolConfigReplica = ProtocolConfigReplica(
            address(
                new DeployableERC1967Proxy(address(new EmptyUUPSProxy()), abi.encodeCall(EmptyUUPSProxy.initialize, ()))
            )
        );
        replicaImplementation = address(new ProtocolConfigReplica());
        _initializeReplica(
            address(protocolConfigReplica),
            replicaImplementation,
            owner,
            CTX1,
            EPOCH1,
            _toKmsNodes(_makeKmsNodeParams(N)),
            _defaultThresholds(),
            address(protocolConfig),
            uint64(block.chainid)
        );

        vm.prank(owner);
        nodeConfigHash = _defineNewKmsContextAndEpoch(_nodeParams(SEED1, N), _defaultThresholds());
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
            _confirmContextCreation(CTX2, nodeConfigHash, _keys(SEED1, N)[i], EXTRA);
        }
        _confirmEpoch(CTX2, EPOCH2, _keys(SEED1, N));
        KmsEpochSubmission memory switchEpoch = _collectEpoch(EPOCH2);

        // Key resharing under B.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpoch(CTX2, EPOCH3, _keys(SEED1, N));
        KmsEpochSubmission memory reshareEpoch = _collectEpoch(EPOCH3);

        // Destruction of the superseded epoch and context, confirmed by `n - t` signers of B.
        vm.startPrank(owner);
        protocolConfig.destroyKmsEpoch(EPOCH2);
        protocolConfig.destroyKmsContext(CTX1);
        vm.stopPrank();
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmEpochDestruction(EPOCH2, _keys(SEED1, N)[i], EXTRA);
        }
        bytes[] memory epochDestructionSignatures = _collectSignatures(
            IProtocolConfig.KmsEpochDestructionConfirmation.selector
        );
        assertEq(epochDestructionSignatures.length, QUORUM);
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextDestruction(CTX1, _keys(SEED1, N)[i], EXTRA);
        }
        bytes[] memory contextDestructionSignatures = _collectSignatures(
            IProtocolConfig.KmsContextDestructionConfirmation.selector
        );
        assertEq(contextDestructionSignatures.length, QUORUM);

        // Apply the same signature bytes on the replica chain, with nodes and thresholds read from canonical.
        vm.chainId(REPLICA_CHAIN_ID);
        protocolConfigReplica.submitKmsContextAndEpoch(
            _canonicalContextSubmission(CTX2, contextSignatures),
            switchEpoch
        );
        protocolConfigReplica.submitKmsEpoch(reshareEpoch);
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
            keccak256(abi.encode(_thresholdsOf(protocolConfigReplica, CTX2))),
            keccak256(abi.encode(_thresholdsOf(protocolConfig, CTX2)))
        );
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH3));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH2));
        assertFalse(protocolConfigReplica.isLiveKmsContext(CTX1));
    }

    /// @dev Canonical: E2 stays pending under CTX1 and is destroyed, then E3 activates with previous
    ///      epoch E1. The CTX1 committee signs the E2 destruction. The replica never stores E2, so it
    ///      rejects the destruction before and after E3 activates.
    function test_revertDestroyedPendingEpochNeverStored() public {
        _setupCanonicalAndReplica();
        vm.startPrank(owner);
        // The pending CTX2 of the setup blocks a resharing, so drop it first.
        protocolConfig.destroyKmsContext(CTX2);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        protocolConfig.destroyKmsEpoch(EPOCH2);
        vm.stopPrank();
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmEpochDestruction(EPOCH2, _defaultSignerPk(i), EXTRA);
        }
        bytes[] memory destructionSignatures = _collectSignatures(
            IProtocolConfig.KmsEpochDestructionConfirmation.selector
        );
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpoch(CTX1, EPOCH3, _keys(SEED0, N));
        KmsEpochSubmission memory epoch3 = _collectEpoch(EPOCH3);

        vm.chainId(REPLICA_CHAIN_ID);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH2));
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH2, destructionSignatures, EXTRA);

        protocolConfigReplica.submitKmsEpoch(epoch3);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH2));
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH2, destructionSignatures, EXTRA);
        _assertActive(CTX1, EPOCH3);
    }

    /// @dev Canonical: CTX2 stays pending and is destroyed, then CTX3 activates with previous context
    ///      CTX1 and epoch E2. The CTX1 committee signs the CTX2 destruction. The replica never stores
    ///      CTX2, so it rejects the destruction before and after CTX3 activates.
    function test_revertDestroyedPendingContextNeverStored() public {
        _setupCanonicalAndReplica();
        vm.prank(owner);
        protocolConfig.destroyKmsContext(CTX2);
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextDestruction(CTX2, _defaultSignerPk(i), EXTRA);
        }
        bytes[] memory destructionSignatures = _collectSignatures(
            IProtocolConfig.KmsContextDestructionConfirmation.selector
        );

        KmsNodeParams[] memory params = _nodeParams(SEED2, N);
        uint256[] memory keys3 = _keys(SEED2, N);
        for (uint256 i = 0; i < QUORUM; i++) {
            keys3[i] = _defaultSignerPk(i);
            params[i].signerAddress = vm.addr(keys3[i]);
        }
        vm.prank(owner);
        bytes32 nodeConfigHash = _defineNewKmsContextAndEpoch(params, _defaultThresholds());
        // The shared signers confirm for both committees. The replica takes only the CTX1 signers.
        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextCreation(CTX3, nodeConfigHash, keys3[i], EXTRA);
        }
        bytes[] memory contextSignatures = _collectSignatures(IProtocolConfig.KmsContextCreationConfirmation.selector);
        _confirmContextCreation(CTX3, nodeConfigHash, keys3[N - 1], EXTRA);
        _confirmEpoch(CTX3, EPOCH2, keys3);
        KmsEpochSubmission memory epoch2 = _collectEpoch(EPOCH2);

        vm.chainId(REPLICA_CHAIN_ID);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, CTX2));
        protocolConfigReplica.submitKmsContextDestruction(CTX2, destructionSignatures, EXTRA);

        protocolConfigReplica.submitKmsContextAndEpoch(_canonicalContextSubmission(CTX3, contextSignatures), epoch2);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, CTX2));
        protocolConfigReplica.submitKmsContextDestruction(CTX2, destructionSignatures, EXTRA);
        _assertActive(CTX3, EPOCH2);
    }

    /// @dev The canonical emits KmsContextCreationConfirmation for both committees. A submitter that
    ///      forwards every event also forwards the new committee's signatures, which the replica rejects.
    function test_revertSwitchWithEveryCreationConfirmation() public {
        bytes32 nodeConfigHash = _setupCanonicalAndReplica();

        for (uint256 i = 0; i < QUORUM; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _defaultSignerPk(i), EXTRA);
        }
        for (uint256 i = 0; i < N; i++) {
            _confirmContextCreation(CTX2, nodeConfigHash, _keys(SEED1, N)[i], EXTRA);
        }
        bytes[] memory contextSignatures = _collectSignatures(IProtocolConfig.KmsContextCreationConfirmation.selector);
        assertEq(contextSignatures.length, QUORUM + N);
        _confirmEpoch(CTX2, EPOCH2, _keys(SEED1, N));
        KmsEpochSubmission memory switchEpoch = _collectEpoch(EPOCH2);
        KmsContextSubmission memory context = _canonicalContextSubmission(CTX2, contextSignatures);

        vm.chainId(REPLICA_CHAIN_ID);
        // The first signature of B follows the `n - t` signatures of A.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED1)[0], CTX1)
        );
        protocolConfigReplica.submitKmsContextAndEpoch(context, switchEpoch);
    }
}

/// @dev Starts each test from the replica that `_setupReplica` bootstraps.
contract ProtocolConfigReplicaBootstrappedTest is ProtocolConfigReplicaTestBase {
    function setUp() public {
        _setupReplica();
    }

    // -----------------------------------------------------------------------
    // Initialization and EIP-712 domain
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalAfterInit() public {
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfigReplica.initializeFromCanonical(
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            CANONICAL_ADDRESS,
            CANONICAL_CHAIN_ID
        );
    }

    function test_revertReinitializeV3OnFreshReplica() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV3(CANONICAL_ADDRESS, CANONICAL_CHAIN_ID);
    }

    function test_revertUpgradeNotOwner() public {
        address newImpl = address(new ProtocolConfigReplica());

        _expectNotOwner();
        protocolConfigReplica.upgradeToAndCall(newImpl, "");
    }

    function test_eip712DomainIsCanonical() public view {
        _assertCanonicalDomain();
    }

    /// @dev Expects a resharing signed under `domainSeparator` to be rejected.
    function _assertDomainRejected(bytes32 domainSeparator) internal {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N);
        epoch.signatures = _sign(
            _hashEpochActivationWithDomain(
                domainSeparator,
                CTX1,
                EPOCH1,
                epoch.epochId,
                epoch.epochMaterialHash,
                epoch.extraData
            ),
            SEED0,
            N
        );

        // The replica recovers the signer over the canonical digest.
        _assertResharingRejected(epoch);
    }

    function test_revertSubmitSignedForLocalDomain() public {
        _assertDomainRejected(_computeProtocolConfigDomainSeparator(block.chainid, protocolConfigAdd));
    }

    /// @dev A canonical at the same address on another chain, such as a testnet.
    function test_revertSubmitSignedForOtherCanonicalChain() public {
        _assertDomainRejected(_computeProtocolConfigDomainSeparator(CANONICAL_CHAIN_ID + 1, CANONICAL_ADDRESS));
    }

    /// @dev Another contract on the canonical chain. Only `verifyingContract` differs from the canonical domain.
    function test_revertSubmitSignedForOtherCanonicalContract() public {
        _assertDomainRejected(_computeProtocolConfigDomainSeparator(CANONICAL_CHAIN_ID, address(0xBEEF)));
    }

    function test_revertSubmitSignedForOtherDomainName() public {
        _assertDomainRejected(
            _computeEip712DomainSeparator("ProtocolConfigReplica", "1", CANONICAL_CHAIN_ID, CANONICAL_ADDRESS)
        );
    }

    function test_revertSubmitSignedForOtherDomainVersion() public {
        _assertDomainRejected(
            _computeEip712DomainSeparator("ProtocolConfig", "2", CANONICAL_CHAIN_ID, CANONICAL_ADDRESS)
        );
    }

    function test_revertSwitchSignedForOtherCanonicalContract() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.signatures = _sign(
            _hashContextCreationWithDomain(
                _computeProtocolConfigDomainSeparator(CANONICAL_CHAIN_ID, address(0xBEEF)),
                CTX1,
                context.contextId,
                keccak256(abi.encode(context.nodes, context.thresholds)),
                context.extraData
            ),
            SEED0,
            QUORUM
        );
        _assertSwitchContextRejected(context, epoch);
    }

    function test_revertDestructionSignedForOtherCanonicalContract() public {
        _mirrorAboveDestroyedIds();
        bytes[] memory signatures = _sign(
            _hashContextDestructionWithDomain(
                _computeProtocolConfigDomainSeparator(CANONICAL_CHAIN_ID, address(0xBEEF)),
                CTX1,
                EXTRA
            ),
            SEED0,
            QUORUM
        );

        // The replica recovers the signer over the canonical digest.
        _expectSignerNotInCommittee(_contextDestructionDigest(CTX1, EXTRA), signatures[0], CTX1 + 10);
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, EXTRA);
    }

    // -----------------------------------------------------------------------
    // submitKmsContextAndEpoch
    // -----------------------------------------------------------------------

    function test_submitKmsContextAndEpochActivates() public {
        uint256 contextId = CTX2;
        uint256 epochId = EPOCH2;

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.SubmitKmsContextAndEpoch(
            contextId,
            epochId,
            keccak256(abi.encode(_nodes(SEED1), _thresholds()))
        );
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, contextId, SEED1, SEED0, QUORUM),
            _epochSubmission(contextId, EPOCH1, epochId, SEED1, N)
        );

        _assertActive(contextId, epochId);
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), contextId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        _assertMembers(contextId, SEED1);
        _assertStoredNodes(contextId, _nodes(SEED1));
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), _thresholds().kmsGen);
        // The previous context stays valid, like on the canonical.
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_submitKmsContextAndEpochAcceptsAllPreviousSigners() public {
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, SEED1, SEED0, N),
            _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED1, N)
        );
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), CTX2);
    }

    /// @dev A switch from 4 nodes with t = 1 to 7 nodes with t = 2, then back to 4 nodes. Each switch needs
    ///      `n - t` signatures of the previous committee and all `n` signatures of the new committee.
    function test_switchBetweenCommitteeSizes() public {
        uint256 bigN = 7;
        KmsNode[] memory bigNodes = _nodes(SEED2, bigN);
        KmsThresholds memory bigThresholds = _thresholds();
        bigThresholds.mpc = 2;

        // 4 nodes with t = 1 to 7 nodes: the quorums are 3 previous and 7 new signatures.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 2, 3));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, bigNodes, bigThresholds, SEED0, 2),
            _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED2, bigN)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 6, 7));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, bigNodes, bigThresholds, SEED0, 3),
            _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED2, 6)
        );
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, bigNodes, bigThresholds, SEED0, 3),
            _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED2, bigN)
        );
        _assertActive(CTX2, EPOCH2);
        assertEq(
            keccak256(abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX2))),
            keccak256(abi.encode(bigNodes))
        );

        // 7 nodes with t = 2 to 4 nodes: the quorums are 5 previous and 4 new signatures.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 4, 5));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX2, CTX3, SEED1, SEED2, 4),
            _epochSubmission(CTX3, EPOCH2, EPOCH3, SEED1, N)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 3, 4));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX2, CTX3, SEED1, SEED2, 5),
            _epochSubmission(CTX3, EPOCH2, EPOCH3, SEED1, N - 1)
        );
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX2, CTX3, SEED1, SEED2, 5),
            _epochSubmission(CTX3, EPOCH2, EPOCH3, SEED1, N)
        );
        _assertActive(CTX3, EPOCH3);
        _assertMembers(CTX3, SEED1);
    }

    function test_revertSwitchEpochSignedByPreviousCommittee() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertSwitchEpochRejected(context, _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED0, N));
    }

    function test_revertSwitchDuplicateSigner() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.signatures[2] = context.signatures[0];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED0)[0]));
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);
    }

    function test_revertSwitchEpochDuplicateSigner() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.signatures[N - 1] = epoch.signatures[0];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED1)[0]));
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);
    }

    /// @dev Expects a switch to `nodes` and `thresholds`, signed by the active committee, to revert with `err`.
    function _assertSwitchCommitteeReverts(
        KmsNode[] memory nodes,
        KmsThresholds memory thresholds,
        bytes memory err
    ) internal {
        KmsContextSubmission memory context = _contextSubmission(CTX1, CTX2, nodes, thresholds, SEED0, QUORUM);
        KmsEpochSubmission memory epoch = _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED1, N);
        vm.expectRevert(err);
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);
    }

    function test_revertSwitchEmptyCommittee() public {
        _assertSwitchCommitteeReverts(
            new KmsNode[](0),
            _thresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.EmptyKmsNodes.selector)
        );
    }

    function test_revertSwitchCommitteeAboveProofFormatLimit() public {
        _assertSwitchCommitteeReverts(
            _nodes(SEED1, 256),
            _thresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsSignerSetExceedsProofFormatLimit.selector, 256, 255)
        );
    }

    function test_revertSwitchNullThreshold() public {
        KmsThresholds memory thresholds = _thresholds();
        thresholds.userDecryption = 0;
        _assertSwitchCommitteeReverts(
            _nodes(SEED1),
            thresholds,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "userDecryption")
        );
    }

    function test_revertSwitchHighThreshold() public {
        KmsThresholds memory thresholds = _thresholds();
        thresholds.kmsGen = N + 1;
        _assertSwitchCommitteeReverts(
            _nodes(SEED1),
            thresholds,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "kmsGen", N + 1, N)
        );
    }

    function test_revertSwitchNullSigner() public {
        KmsNode[] memory nodes = _nodes(SEED1);
        nodes[1].signerAddress = address(0);
        _assertSwitchCommitteeReverts(
            nodes,
            _thresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsNodeNullSigner.selector)
        );
    }

    function test_revertSwitchNullTxSender() public {
        KmsNode[] memory nodes = _nodes(SEED1);
        nodes[1].txSenderAddress = address(0);
        _assertSwitchCommitteeReverts(
            nodes,
            _thresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsNodeNullTxSender.selector)
        );
    }

    function test_revertSwitchDuplicateCommitteeSigner() public {
        KmsNode[] memory nodes = _nodes(SEED1);
        nodes[1].signerAddress = nodes[0].signerAddress;
        _assertSwitchCommitteeReverts(
            nodes,
            _thresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsSignerAlreadyRegistered.selector, nodes[0].signerAddress)
        );
    }

    function test_revertSwitchDuplicateCommitteeTxSender() public {
        KmsNode[] memory nodes = _nodes(SEED1);
        nodes[1].txSenderAddress = nodes[0].txSenderAddress;
        _assertSwitchCommitteeReverts(
            nodes,
            _thresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsTxSenderAlreadyRegistered.selector, nodes[0].txSenderAddress)
        );
    }

    function test_revertSwitchNonMemberSigner() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        // A signer of the new committee only is not in the previous committee.
        context.signatures[2] = _sign(_contextDigest(CTX1, context), SEED1, 1)[0];

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED1)[0], CTX1)
        );
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);
    }

    function test_revertSwitchTamperedNodes() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.nodes[0].ipAddress = "10.0.0.1";
        _assertSwitchContextRejected(context, epoch);
    }

    function test_revertSwitchTamperedConnectorUrl() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.nodes[1].connectorUrl = "https://attacker.example.com";
        _assertSwitchContextRejected(context, epoch);
    }

    /// @dev An invalid `mpc` also pins that the previous committee check runs before the store.
    function test_revertSwitchTamperedThresholds() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.thresholds.mpc = N + 1;
        _assertSwitchContextRejected(context, epoch);
    }

    function test_revertSwitchTamperedExtraData() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.extraData = hex"bad0";
        _assertSwitchContextRejected(context, epoch);
    }

    function test_revertSwitchTamperedContextId() public {
        KmsContextSubmission memory context = _contextSubmission(CTX1, CTX2, SEED1, SEED0, QUORUM);
        context.contextId = CTX3;
        _assertSwitchContextRejected(context, _epochSubmission(CTX3, EPOCH1, EPOCH2, SEED1, N));
    }

    function test_revertSwitchTamperedEpochMaterial() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.epochMaterialHash = keccak256("other material");
        _assertSwitchEpochRejected(context, epoch);
    }

    function test_revertSwitchTamperedEpochExtraData() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.extraData = hex"bad0";
        _assertSwitchEpochRejected(context, epoch);
    }

    function test_revertSwitchTamperedEpochId() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.epochId = EPOCH3;
        _assertSwitchEpochRejected(context, epoch);
    }

    function test_revertSwitchSignedForOtherPreviousContext() public {
        _assertSwitchContextRejected(
            _contextSubmission(CTX1 + 5, CTX1 + 6, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 6, EPOCH1, EPOCH2, SEED1, N)
        );
    }

    function test_revertSwitchSignedForOtherPreviousEpoch() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        // The epoch is signed over previous epoch EPOCH4, not the active EPOCH1.
        _assertSwitchEpochRejected(context, _epochSubmission(CTX2, EPOCH4, EPOCH1 + 4, SEED1, N));
    }

    function test_revertSwitchEpochContextMismatch() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertSwitchEpochRejected(context, _epochSubmission(CTX1 + 7, EPOCH1, EPOCH2, SEED1, N));
    }

    function test_revertSwitchReplayNotAboveActive() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextNotAboveActive.selector, CTX2));
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);
    }

    function test_revertSwitchOlderIdNotAboveActive() public {
        _switch(SEED0, CTX1 + 5, EPOCH1 + 5, SEED1);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextNotAboveActive.selector, CTX4));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX4, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX4, EPOCH1, EPOCH4, SEED2, N)
        );
    }

    /// @dev A signed switch must carry an epoch ID above the active one, like a resharing.
    function test_revertSwitchEqualEpochNotAboveActive() public {
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH3, SEED0, N));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochNotAboveActive.selector, EPOCH3));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX2, EPOCH3, EPOCH3, SEED1, N)
        );
    }

    function test_revertSwitchOlderEpochNotAboveActive() public {
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH3, SEED0, N));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochNotAboveActive.selector, EPOCH2));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX2, EPOCH3, EPOCH2, SEED1, N)
        );
    }

    function test_switchAfterResharingWithEpochGapActivatesPairAndKeepsSupersededEpochValid() public {
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH3, SEED0, N));

        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX2, EPOCH3, EPOCH4, SEED1, N)
        );

        _assertActive(CTX2, EPOCH4);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH3));
    }

    /// @dev After the owner moves the replica back, `CTX2` is above the active context and still
    ///      stores committee SEED1. A switch to a committee of another size onto it must not merge the
    ///      two committees.
    function test_revertSwitchOntoStoredCommittee() public {
        _switch(SEED0, CTX2, EPOCH2, SEED1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());

        uint256 bigN = 7;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyStored.selector, CTX2));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, _nodes(SEED2, bigN), _thresholds(), SEED0, QUORUM),
            _epochSubmission(CTX2, EPOCH1, EPOCH3, _keys(SEED2, bigN))
        );
        _assertActive(CTX1, EPOCH1);
        _assertMembers(CTX2, SEED1);
    }

    function test_revertSwitchOntoStoredCommitteeBeforeSignatures() public {
        _switch(SEED0, CTX2, EPOCH2, SEED1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());

        // SEED2 signs as both committees: invalid signatures, so the stored-committee check must run first.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyStored.selector, CTX2));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX1, CTX2, SEED2, SEED2, QUORUM),
            _epochSubmission(CTX2, EPOCH1, EPOCH3, SEED2, N)
        );
    }

    function test_switchesApplyInCanonicalOrder() public {
        (KmsContextSubmission memory contextB, KmsEpochSubmission memory epochB) = _defaultSwitch();
        KmsContextSubmission memory contextC = _contextSubmission(CTX2, CTX3, SEED2, SEED1, QUORUM);
        KmsEpochSubmission memory epochC = _epochSubmission(CTX3, EPOCH2, EPOCH3, SEED2, N);

        _assertSwitchContextRejected(contextC, epochC);

        protocolConfigReplica.submitKmsContextAndEpoch(contextB, epochB);
        protocolConfigReplica.submitKmsContextAndEpoch(contextC, epochC);

        _assertActive(CTX3, EPOCH3);
        _assertMembers(CTX3, SEED2);
    }

    // -----------------------------------------------------------------------
    // submitKmsEpoch
    // -----------------------------------------------------------------------

    function test_submitKmsEpochActivates() public {
        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.SubmitKmsEpoch(CTX1, EPOCH2);
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N));

        _assertActive(CTX1, EPOCH2);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH2));
    }

    function test_revertResharingNotUnanimous() public {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, N - 1, N));
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N - 1));
    }

    function test_revertResharingTamperedEpochMaterial() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N);
        epoch.epochMaterialHash = keccak256("other material");
        _assertResharingRejected(epoch);
    }

    function test_revertResharingTamperedExtraData() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N);
        epoch.extraData = hex"bad0";
        _assertResharingRejected(epoch);
    }

    function test_revertResharingTamperedEpochId() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N);
        epoch.epochId = EPOCH3;
        _assertResharingRejected(epoch);
    }

    function test_revertResharingSignedForOtherPreviousEpoch() public {
        _assertResharingRejected(_epochSubmission(CTX1, EPOCH2, EPOCH3, SEED0, N));
    }

    function test_revertResharingWrongContext() public {
        _assertResharingRejected(_epochSubmission(CTX2, EPOCH1, EPOCH2, SEED0, N));
    }

    function test_revertResharingReplayNotAboveActive() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N);
        protocolConfigReplica.submitKmsEpoch(epoch);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochNotAboveActive.selector, EPOCH2));
        protocolConfigReplica.submitKmsEpoch(epoch);
    }

    function test_revertResharingOlderIdNotAboveActive() public {
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH3, SEED0, N));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochNotAboveActive.selector, EPOCH2));
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N));
    }

    function test_revertSubmitMalformedSignature() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N);
        epoch.signatures[0] = new bytes(64);

        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureLength.selector, 64));
        protocolConfigReplica.submitKmsEpoch(epoch);
    }

    function test_revertResharingDuplicateSigner() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N);
        epoch.signatures[N - 1] = epoch.signatures[0];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED0)[0]));
        protocolConfigReplica.submitKmsEpoch(epoch);
    }

    // -----------------------------------------------------------------------
    // submitKmsContextDestruction / submitKmsEpochDestruction
    // -----------------------------------------------------------------------

    /// @dev The owner mirrors committee SEED0 to `CTX1 + 10` and `EPOCH1 + 10`, so the live `CTX1` and
    ///      `EPOCH1` are no longer the active IDs and can be destroyed.
    function _mirrorAboveDestroyedIds() internal {
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 10, EPOCH1 + 10, _nodes(SEED0), _thresholds());
    }

    function test_submitKmsContextDestructionOfSupersededContext() public {
        _switch(SEED0, CTX2, EPOCH2, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        _submitContextDestruction(CTX1, SEED1, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH2));
    }

    function test_revertRepeatedContextDestruction() public {
        _switch(SEED0, CTX2, EPOCH2, SEED1);
        _submitContextDestruction(CTX1, SEED1, QUORUM);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, CTX1));
        _submitContextDestruction(CTX1, SEED1, QUORUM);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
        assertFalse(protocolConfigReplica.isLiveKmsContext(CTX1));
    }

    function test_revertContextDestructionActive() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        _submitContextDestruction(CTX1, SEED0, QUORUM);
    }

    function test_revertContextDestructionQuorum() public {
        _mirrorAboveDestroyedIds();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _submitContextDestruction(CTX1, SEED0, QUORUM - 1);
    }

    function test_revertDestructionZeroQuorumCommittee() public {
        // With `mpc == n` the `n - t` quorum is 0, and the floor raises it to 1.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            CTX1 + 10,
            EPOCH1 + 10,
            _nodes(SEED0),
            KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 4})
        );

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 0, 1));
        protocolConfigReplica.submitKmsContextDestruction(CTX1, new bytes[](0), EXTRA);
    }

    function test_revertContextDestructionSignedForOtherIdOrExtraData() public {
        _mirrorAboveDestroyedIds();
        bytes[] memory otherId = _contextDestructionSignatures(CTX1 + 8, EXTRA, SEED0, QUORUM);
        _expectSignerNotInCommittee(_contextDestructionDigest(CTX1, EXTRA), otherId[0], CTX1 + 10);
        protocolConfigReplica.submitKmsContextDestruction(CTX1, otherId, EXTRA);

        bytes[] memory signatures = _contextDestructionSignatures(CTX1, EXTRA, SEED0, QUORUM);
        _expectSignerNotInCommittee(_contextDestructionDigest(CTX1, hex"bad0"), signatures[0], CTX1 + 10);
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, hex"bad0");
    }

    function test_revertEpochDestructionSignedForOtherIdOrExtraData() public {
        _mirrorAboveDestroyedIds();
        bytes[] memory otherId = _epochDestructionSignatures(EPOCH1 + 8, EXTRA, SEED0, QUORUM);
        _expectSignerNotInCommittee(_epochDestructionDigest(EPOCH1, EXTRA), otherId[0], CTX1 + 10);
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1, otherId, EXTRA);

        bytes[] memory signatures = _epochDestructionSignatures(EPOCH1, EXTRA, SEED0, QUORUM);
        _expectSignerNotInCommittee(_epochDestructionDigest(EPOCH1, hex"bad0"), signatures[0], CTX1 + 10);
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1, signatures, hex"bad0");
    }

    function test_revertContextDestructionDuplicateSigner() public {
        _mirrorAboveDestroyedIds();
        bytes[] memory signatures = _contextDestructionSignatures(CTX1, EXTRA, SEED0, QUORUM);
        signatures[QUORUM - 1] = signatures[1];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED0)[1]));
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, EXTRA);
    }

    /// @dev Canonical C2 (SEED1) destroys C1, but the replica switches to C3 (SEED2) before the
    ///      destruction arrives. The C2 signatures no longer match the active committee, so the owner
    ///      applies the destruction (accepted risk).
    function test_contextDestructionAfterRotationRecoversThroughMirror() public {
        _switch(SEED0, CTX2, EPOCH2, SEED1);
        bytes[] memory signatures = _contextDestructionSignatures(CTX1, EXTRA, SEED1, QUORUM);
        _switch(SEED1, CTX3, EPOCH3, SEED2);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED1)[0], CTX3)
        );
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, EXTRA);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
    }

    function test_revertRepeatedEpochDestruction() public {
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N));
        _submitEpochDestruction(EPOCH1, SEED0, QUORUM);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH1));
        _submitEpochDestruction(EPOCH1, SEED0, QUORUM);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_revertEpochDestructionActive() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        _submitEpochDestruction(EPOCH1, SEED0, QUORUM);
    }

    function test_revertEpochDestructionQuorum() public {
        _mirrorAboveDestroyedIds();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _submitEpochDestruction(EPOCH1, SEED0, QUORUM - 1);
    }

    function test_revertEpochDestructionNonMemberSigner() public {
        _mirrorAboveDestroyedIds();
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsSignerNotInCommittee.selector,
                _signers(SEED1)[0],
                CTX1 + 10
            )
        );
        _submitEpochDestruction(EPOCH1, SEED1, QUORUM);
    }

    function test_revertEpochDestructionDuplicateSigner() public {
        _mirrorAboveDestroyedIds();
        bytes[] memory signatures = _epochDestructionSignatures(EPOCH1, EXTRA, SEED0, QUORUM);
        signatures[QUORUM - 1] = signatures[0];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED0)[0]));
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1, signatures, EXTRA);
    }

    function test_supersededEpochDestructionAfterReplicaCatchesUp() public {
        // Canonical activates E2, then E3, then destroys the superseded E2. The replica catches up first.
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH2)
        );
        _submitEpochDestruction(EPOCH2, SEED0, QUORUM);
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH2, EPOCH3, SEED0, N));
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH2);
        _submitEpochDestruction(EPOCH2, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH2));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH3));
    }

    /// @dev Canonical activates E2, then E3, then destroys E2. The replica is still at E1 and receives
    ///      the E2 destruction first. It reverts until the replica activates E3.
    function test_supersededEpochDestructionWaitsForReplica() public {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH2));
        _submitEpochDestruction(EPOCH2, SEED0, QUORUM);

        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N));
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH2, EPOCH3, SEED0, N));
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH2);
        _submitEpochDestruction(EPOCH2, SEED0, QUORUM);

        _assertActive(CTX1, EPOCH3);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH2));
    }

    function test_supersededContextDestructionWaitsForReplica() public {
        // Canonical switches from C1 (SEED0) to C2 (SEED1), then to C3 (SEED2), then C3 destroys C2.
        // The replica is at C1, so C2 is not live on it.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, CTX2));
        _submitContextDestruction(CTX2, SEED2, QUORUM);

        _switch(SEED0, CTX2, EPOCH2, SEED1);
        _switch(SEED1, CTX3, EPOCH3, SEED2);
        _submitContextDestruction(CTX2, SEED2, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX2));
        assertTrue(protocolConfigReplica.isValidKmsContext(CTX3));
    }

    // -----------------------------------------------------------------------
    // mirrorKmsEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsEpochActivatesCanonicalEpoch() public {
        uint256 contextId = CTX1;
        uint256 epochId = EPOCH1 + 6;

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsEpoch(contextId, epochId);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, epochId);

        _assertActive(contextId, epochId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
    }

    function test_revertMirrorKmsEpochNotOwner() public {
        _expectNotOwner();
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH2);
    }

    function test_revertMirrorKmsEpochInvalidContext() public {
        uint256 invalidContextId = CTX2;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfigReplica.mirrorKmsEpoch(invalidContextId, EPOCH2);
    }

    function test_revertMirrorKmsEpochSupersededContext() public {
        uint256 supersededContextId = CTX1;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH2, _nodes(SEED1), _thresholds());

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, supersededContextId));
        protocolConfigReplica.mirrorKmsEpoch(supersededContextId, EPOCH3);
    }

    function test_mirrorKmsEpochMovesEpochBack() public {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 5);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH3);
        vm.stopPrank();

        _assertActive(CTX1, EPOCH3);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH3));
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextAndEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextAndEpochActivatesAndEmits() public {
        uint256 contextId = CTX1 + 4;
        uint256 epochId = EPOCH1 + 4;
        KmsNode[] memory nodes = _nodes(SEED1);
        KmsThresholds memory thresholds = _thresholds();

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);

        // The mirrored context and epoch are active at once, with no quorum replay.
        _assertActive(contextId, epochId);
        assertTrue(protocolConfigReplica.isValidKmsContext(contextId));
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        _assertMembers(contextId, SEED1);
        _assertStoredNodes(contextId, nodes);
    }

    function test_mirrorOverExistingIdKeepsTxSenderWithNewUrl() public {
        KmsNode[] memory nodes = _nodes(SEED0);
        nodes[0].connectorUrl = "https://kms-connector-0.new.example.com";
        nodes[1].connectorUrl = "";

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1 + 1, nodes, _thresholds());

        _assertStoredNodes(CTX1, nodes);
    }

    function test_mirrorKmsContextAndEpochMovesBack() public {
        _switch(SEED0, CTX3, EPOCH3, SEED1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH2, _nodes(SEED0), _thresholds());

        _assertActive(CTX2, EPOCH2);
        _assertMembers(CTX2, SEED0);
    }

    function test_mirrorKmsContextAndEpochShrinksCommittee() public {
        KmsNode[] memory nodes = _nodes(SEED1, N - 1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH2, nodes, _thresholds());

        assertEq(
            keccak256(abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX1))),
            keccak256(abi.encode(nodes))
        );
        assertEq(protocolConfigReplica.getKmsSignersForContext(CTX1).length, N - 1);
        _assertNotMembers(CTX1, SEED0);
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH2, EPOCH3, SEED1, N - 1));
        _assertActive(CTX1, EPOCH3);
    }

    function test_revertMirrorKmsContextAndEpochInvalidThresholds() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "mpc", 5, 4));
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            CTX2,
            EPOCH2,
            _nodes(SEED1),
            KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 5})
        );
    }

    function test_revertMirrorKmsContextAndEpochNotOwner() public {
        KmsNode[] memory nodes = _nodes(SEED1);
        KmsThresholds memory thresholds = _thresholds();

        _expectNotOwner();
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 4, EPOCH1 + 4, nodes, thresholds);
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextDestruction / mirrorKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_revertMirrorKmsContextDestructionNotOwner() public {
        _expectNotOwner();
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 9);
    }

    function test_revertMirrorKmsEpochDestructionNotOwner() public {
        _expectNotOwner();
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1 + 9);
    }

    function test_revertMirrorKmsContextDestructionActive() public {
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
    }

    function test_revertMirrorKmsEpochDestructionActive() public {
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
    }

    /// @dev The canonical can destroy IDs that the replica never stored, such as an aborted switch.
    function test_mirrorDestructionOfNeverStoredIdsKeepsThemActivatable() public {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX2);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH2);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH3);
        vm.stopPrank();

        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N));
        _switch(SEED0, CTX2, EPOCH3, SEED1);
        _assertActive(CTX2, EPOCH3);
        _assertMembers(CTX2, SEED1);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH2));
    }

    // -----------------------------------------------------------------------
    // Recovery
    // -----------------------------------------------------------------------

    // The owner recovers the replica with the mirror functions, and the submit path keeps working.

    /// @dev Canonical: C2 stays pending and is destroyed, then C1 switches to C3. A late signature set
    ///      activates C2 on the replica, so the replica rejects the C3 switch. The owner recovers
    ///      forward: it mirrors C3, then destroys C2 and E2 below it.
    function test_recoveryFromLateContextActivation() public {
        _switch(SEED0, CTX2, EPOCH2, SEED1);
        _assertActive(CTX2, EPOCH2);
        _assertSwitchContextRejected(
            _contextSubmission(CTX1, CTX3, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX3, EPOCH1, EPOCH3, SEED2, N)
        );

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX3, EPOCH3, _nodes(SEED2), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(CTX2);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH2);
        vm.stopPrank();

        _assertActive(CTX3, EPOCH3);
        _assertMembers(CTX3, SEED2);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX2));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH2));
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX3, EPOCH3, EPOCH4, SEED2, N));
        _assertActive(CTX3, EPOCH4);
    }

    /// @dev Canonical: E2 stays pending under C1 and is destroyed, then E3 activates after E1. A late
    ///      signature set activates E2 on the replica, so the replica rejects E3. The owner recovers
    ///      forward: it mirrors E3, then destroys E2 below it.
    function test_recoveryFromLateEpochActivation() public {
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH1, EPOCH2, SEED0, N));
        _assertResharingRejected(_epochSubmission(CTX1, EPOCH1, EPOCH3, SEED0, N));

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH3);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH2);
        vm.stopPrank();

        _assertActive(CTX1, EPOCH3);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH2));
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX1, EPOCH3, EPOCH4, SEED0, N));
        _assertActive(CTX1, EPOCH4);
    }

    function test_mirrorKmsContextAndEpochOverridesCompromisedCommittee() public {
        uint256 forgedId = CTX2;
        _switch(SEED0, forgedId, EPOCH2, SEED1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, EPOCH2, _nodes(SEED2), _thresholds());

        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(forgedId, EPOCH2, EPOCH3, SEED2, N));
        _assertActive(forgedId, EPOCH3);
    }

    /// @dev A forged switch activates C2 with committee SEED1. The C2 committee then submits C3 and E3, and
    ///      E4 under C3. The owner recovers by moving forward: it mirrors the honest committee SEED2 to a
    ///      new canonical C4, then destroys every forged context and epoch.
    function test_recoveryForwardDestroysForgedDescendants() public {
        address attacker = _signers(SEED1)[0];

        KmsContextSubmission memory context2 = _contextSubmission(CTX1, CTX2, SEED1, SEED0, QUORUM);
        KmsEpochSubmission memory epoch2 = _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED1, N);
        protocolConfigReplica.submitKmsContextAndEpoch(context2, epoch2);
        // The forged SEED1 committee keeps itself as the committee of C3.
        KmsContextSubmission memory context3 = _contextSubmission(CTX2, CTX3, SEED1, SEED1, QUORUM);
        KmsEpochSubmission memory epoch3 = _epochSubmission(CTX3, EPOCH2, EPOCH3, SEED1, N);
        protocolConfigReplica.submitKmsContextAndEpoch(context3, epoch3);
        KmsEpochSubmission memory epoch4 = _epochSubmission(CTX3, EPOCH3, EPOCH4, SEED1, N);
        protocolConfigReplica.submitKmsEpoch(epoch4);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX4, EPOCH4 + 1, _nodes(SEED2), _thresholds());
        assertTrue(protocolConfigReplica.isValidKmsContext(CTX3));

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX2);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX3);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH2);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH3);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH4);
        vm.stopPrank();

        _assertActive(CTX4, EPOCH4 + 1);
        _assertMembers(CTX4, SEED2);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX2));
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX3));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, CTX2));
        protocolConfigReplica.isKmsSignerForContext(CTX2, attacker);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, CTX3));
        protocolConfigReplica.isKmsSignerForContext(CTX3, attacker);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH2));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX3, EPOCH3));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX3, EPOCH4));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextNotAboveActive.selector, CTX2));
        protocolConfigReplica.submitKmsContextAndEpoch(context2, epoch2);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextNotAboveActive.selector, CTX3));
        protocolConfigReplica.submitKmsContextAndEpoch(context3, epoch3);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochNotAboveActive.selector, EPOCH4));
        protocolConfigReplica.submitKmsEpoch(epoch4);
    }

    /// @dev The compromised committee SEED0 signs a switch to a forged context far above canonical, with
    ///      committee SEED1, and SEED1 reshares under it. The owner mirrors back to the new canonical C2
    ///      with the honest committee SEED2, then destroys the forged context and epochs.
    function test_recoveryFromForgedHighContextId() public {
        uint256 forgedContextId = CTX1 + 1000;
        uint256 forgedEpochId = EPOCH1 + 1000;
        KmsContextSubmission memory forgedContext = _contextSubmission(CTX1, forgedContextId, SEED1, SEED0, QUORUM);
        KmsEpochSubmission memory forgedEpoch = _epochSubmission(forgedContextId, EPOCH1, forgedEpochId, SEED1, N);
        protocolConfigReplica.submitKmsContextAndEpoch(forgedContext, forgedEpoch);
        KmsEpochSubmission memory forgedResharing = _epochSubmission(
            forgedContextId,
            forgedEpochId,
            forgedEpochId + 1,
            SEED1,
            N
        );
        protocolConfigReplica.submitKmsEpoch(forgedResharing);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH2, _nodes(SEED2), _thresholds());
        // An activation does not demote older IDs, so the forged context stays valid until the owner destroys it.
        assertTrue(protocolConfigReplica.isValidKmsContext(forgedContextId));

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(forgedContextId);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId + 1);
        vm.stopPrank();

        _assertActive(CTX2, EPOCH2);
        _assertMembers(CTX2, SEED2);
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedContextId));
        assertFalse(protocolConfigReplica.isValidEpochForContext(forgedContextId, forgedEpochId));
        assertFalse(protocolConfigReplica.isValidEpochForContext(forgedContextId, forgedEpochId + 1));

        // The destroyed forged context keeps its committee, so the switch replay hits the stored committee.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyStored.selector, forgedContextId)
        );
        protocolConfigReplica.submitKmsContextAndEpoch(forgedContext, forgedEpoch);
        // The resharing replay was signed over a previous ID that is no longer active, so its digest does not match.
        _assertResharingRejected(forgedResharing);

        // The honest committee signs the next canonical resharing.
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX2, EPOCH2, EPOCH3, SEED2, N));
        _assertActive(CTX2, EPOCH3);
    }

    /// @dev The compromised committee SEED0 forges a switch to C3 with committee SEED1. The owner mirrors
    ///      the canonical C2 with committee SEED2 and destroys C3 and E3. C3 keeps its stored committee, so
    ///      the canonical switch from C2 to C3 with 7 nodes of SEED2 reverts. The owner mirrors C3 instead.
    function test_canonicalSwitchOntoDestroyedForgedIdNeedsMirror() public {
        _switch(SEED0, CTX3, EPOCH3, SEED1);
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH2, _nodes(SEED2), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(CTX3);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH3);
        vm.stopPrank();

        uint256 bigN = 7;
        KmsNode[] memory canonicalNodes = _nodes(SEED2, bigN);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyStored.selector, CTX3));
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(CTX2, CTX3, canonicalNodes, _thresholds(), SEED2, QUORUM),
            _epochSubmission(CTX3, EPOCH2, EPOCH3 + 1, _keys(SEED2, bigN))
        );
        _assertActive(CTX2, EPOCH2);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX3));

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX3, EPOCH3 + 1, canonicalNodes, _thresholds());
        _assertActive(CTX3, EPOCH3 + 1);
        assertEq(
            keccak256(abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX3))),
            keccak256(abi.encode(canonicalNodes))
        );
        _assertNotMembers(CTX3, SEED1);
    }

    /// @dev The compromised committee SEED0 signs a resharing of C1 to a forged epoch far above
    ///      canonical. The owner mirrors to the new canonical C2 with the honest committee SEED2, then
    ///      destroys the forged epoch.
    function test_recoveryFromForgedHighEpochId() public {
        uint256 forgedEpochId = EPOCH1 + 1000;
        KmsEpochSubmission memory forgedEpoch = _epochSubmission(CTX1, EPOCH1, forgedEpochId, SEED0, N);
        protocolConfigReplica.submitKmsEpoch(forgedEpoch);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH2, _nodes(SEED2), _thresholds());
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, forgedEpochId));
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);

        _assertActive(CTX2, EPOCH2);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, forgedEpochId));

        // The replay was signed under C1, so its digest does not match.
        _assertResharingRejected(forgedEpoch);

        // The honest committee signs the next canonical switch.
        _switch(SEED2, CTX3, EPOCH3, SEED1);
        _assertActive(CTX3, EPOCH3);
    }

    /// @dev The owner rolls the replica back from C3 to C2. C3 keeps its stored committee, so a replay of
    ///      the genuine C2 to C3 switch cannot store it again. The owner recovers with a mirror of C3.
    function test_switchReplayAfterRollbackNeedsMirror() public {
        _switch(SEED0, CTX2, EPOCH2, SEED1);
        KmsContextSubmission memory context3 = _contextSubmission(CTX2, CTX3, SEED2, SEED1, QUORUM);
        KmsEpochSubmission memory epoch3 = _epochSubmission(CTX3, EPOCH2, EPOCH3, SEED2, N);
        protocolConfigReplica.submitKmsContextAndEpoch(context3, epoch3);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX2, EPOCH2, _nodes(SEED1), _thresholds());

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyStored.selector, CTX3));
        protocolConfigReplica.submitKmsContextAndEpoch(context3, epoch3);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX3, EPOCH3, _nodes(SEED2), _thresholds());
        _assertActive(CTX3, EPOCH3);
        _assertMembers(CTX3, SEED2);
        protocolConfigReplica.submitKmsEpoch(_epochSubmission(CTX3, EPOCH3, EPOCH3 + 1, SEED2, N));
        _assertActive(CTX3, EPOCH3 + 1);
    }
}

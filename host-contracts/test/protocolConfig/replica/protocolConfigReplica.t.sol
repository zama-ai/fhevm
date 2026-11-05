// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {DeployableERC1967Proxy} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsThresholds, KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ProtocolConfigReplicaTestBase} from "./ProtocolConfigReplicaTestBase.sol";

contract ProtocolConfigReplicaTest is ProtocolConfigReplicaTestBase {
    // -----------------------------------------------------------------------
    // Canonical mirror initializer
    // -----------------------------------------------------------------------

    function _expectInitializeFromCanonicalReverts(
        uint256 contextId,
        uint256 epochId,
        uint64 chainId,
        address canonical,
        bytes memory err
    ) internal {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigReplica());

        vm.prank(owner);
        vm.expectRevert(err);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (contextId, epochId, _nodes(SEED0), _thresholds(), chainId, canonical)
            )
        );
    }

    function test_revertInitializeFromCanonicalInvalidEpochId() public {
        _expectInitializeFromCanonicalReverts(
            CTX1,
            EPOCH_COUNTER_BASE,
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH_COUNTER_BASE)
        );
    }

    function test_revertInitializeFromCanonicalInvalidContextId() public {
        _expectInitializeFromCanonicalReverts(
            KMS_CONTEXT_COUNTER_BASE,
            EPOCH1,
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, KMS_CONTEXT_COUNTER_BASE)
        );
    }

    function test_revertInitializeFromCanonicalZeroChainId() public {
        _expectInitializeFromCanonicalReverts(
            CTX1,
            EPOCH1,
            0,
            CANONICAL_ADDRESS,
            abi.encodeWithSelector(IProtocolConfigReplica.InvalidCanonicalDomain.selector, 0, CANONICAL_ADDRESS)
        );
    }

    function test_revertInitializeFromCanonicalZeroAddress() public {
        _expectInitializeFromCanonicalReverts(
            CTX1,
            EPOCH1,
            CANONICAL_CHAIN_ID,
            address(0),
            abi.encodeWithSelector(
                IProtocolConfigReplica.InvalidCanonicalDomain.selector,
                CANONICAL_CHAIN_ID,
                address(0)
            )
        );
    }

    // -----------------------------------------------------------------------
    // Upgrade from ProtocolConfig
    // -----------------------------------------------------------------------

    address internal replicaImplementation;

    /// @dev Deploys a canonical ProtocolConfig with 2 nodes of committee SEED0, and the replica implementation.
    ///      `_upgradeToReplica` then makes no deployment, so a test can expect a revert on the upgrade.
    function _setupCanonical() internal returns (ProtocolConfig pc) {
        _deployACL(owner);
        (pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
        replicaImplementation = address(new ProtocolConfigReplica());
    }

    function _upgradeToReplica(uint64 chainId, address canonical) internal {
        _pinToV014InitializedVersion();
        vm.prank(owner);
        ProtocolConfig(protocolConfigAdd).upgradeToAndCall(
            replicaImplementation,
            abi.encodeCall(ProtocolConfigReplica.reinitializeV3, (chainId, canonical))
        );
        protocolConfigReplica = ProtocolConfigReplica(protocolConfigAdd);
    }

    function _expectReinitializeV3Reverts(uint64 chainId, address canonical) internal {
        _setupCanonical();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.InvalidCanonicalDomain.selector, chainId, canonical)
        );
        _upgradeToReplica(chainId, canonical);
    }

    function test_revertReinitializeV3ZeroChainId() public {
        _expectReinitializeV3Reverts(0, CANONICAL_ADDRESS);
    }

    function test_revertReinitializeV3ZeroAddress() public {
        _expectReinitializeV3Reverts(CANONICAL_CHAIN_ID, address(0));
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
                (CTX1, EPOCH1, _nodes(SEED0), _thresholds(), CANONICAL_CHAIN_ID, CANONICAL_ADDRESS)
            )
        );
    }

    function test_reinitializeV3OnUpgradeFromProtocolConfigKeepsStateAndMirrors() public {
        ProtocolConfig pc = _setupCanonical();

        (uint256 contextId, uint256 epochId) = pc.getCurrentKmsContextAndEpoch();
        address[] memory signers = pc.getKmsSignersForContext(contextId);
        KmsThresholds memory thresholds = _thresholdsOf(pc, contextId);

        _upgradeToReplica(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 4);
        _assertCanonicalDomain();
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.3.0");
        _assertActive(contextId, epochId);
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), signers);
        assertEq(
            keccak256(abi.encode(_thresholdsOf(protocolConfigReplica, contextId))),
            keccak256(abi.encode(thresholds))
        );

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV3(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        // The replica verifies signatures against the committee that ProtocolConfig stored before the upgrade.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, 2));
        _assertActive(contextId, EPOCH1 + 1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, EPOCH1 + 2);
        _assertActive(contextId, EPOCH1 + 2);
    }

    /// @dev A pending context leaves the canonical counter above the active ID. The replica reports the active ID.
    function test_reinitializeV3CounterReturnsActiveContextId() public {
        ProtocolConfig pc = _setupCanonical();
        vm.prank(owner);
        pc.defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds(), "", new PcrValues[](0));
        assertEq(pc.getCurrentKmsContextIdCounter(), CTX1 + 1);

        _upgradeToReplica(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), CTX1);

        // The canonical stored the pending 2-node SEED0 committee under `CTX1 + 1`. Mirroring that ID replaces it.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 1, _nodes(SEED1), _thresholds());
        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), CTX1 + 1);
        _assertActive(CTX1 + 1, EPOCH1 + 1);
        _assertMembers(CTX1 + 1, SEED1);
        _assertNotMembers(CTX1 + 1, SEED0);
    }

    /// @dev The v0.14 state holds a destroyed C2 and a pending E2 above the active pair. The upgrade
    ///      keeps the active pair, and the next canonical activations apply.
    function test_reinitializeV3WithDestroyedContextAndPendingEpoch() public {
        ProtocolConfig pc = _setupCanonical();
        vm.startPrank(owner);
        pc.defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds(), "", new PcrValues[](0));
        pc.destroyKmsContext(CTX1 + 1);
        pc.defineNewEpochForCurrentKmsContext();
        vm.stopPrank();

        _upgradeToReplica(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
        _assertActive(CTX1, EPOCH1);

        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, 2));
        _assertActive(CTX1, EPOCH1 + 1);
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 2, SEED1, SEED0, 2),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED1, N)
        );
        _assertActive(CTX1 + 2, EPOCH1 + 2);
    }

    // -----------------------------------------------------------------------
    // Canonical to replica: the canonical collects the KMS confirmations through confirm*.
    // The replica on another chain applies the same signature bytes through its submit functions.
    // -----------------------------------------------------------------------

    uint256 internal constant CTX2 = KMS_CONTEXT_COUNTER_BASE + 2;
    uint256 internal constant CTX3 = KMS_CONTEXT_COUNTER_BASE + 3;
    uint256 internal constant EPOCH2 = EPOCH_COUNTER_BASE + 2;
    uint256 internal constant EPOCH3 = EPOCH_COUNTER_BASE + 3;
    uint64 internal constant REPLICA_CHAIN_ID = 8453;

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
    function _canonicalCtxSubmission(
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
        protocolConfigReplica.submitKmsContextSwitch(_canonicalCtxSubmission(CTX2, contextSignatures), switchEpoch);
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
            keccak256(abi.encode(_thresholdsOf(protocolConfigReplica, CTX2))),
            keccak256(abi.encode(_thresholdsOf(protocolConfig, CTX2)))
        );
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH3));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX2, EPOCH2));
        assertFalse(protocolConfigReplica.isLiveKmsContext(CTX1));
    }

    /// @dev Canonical: E2 stays pending under CTX1 and is destroyed, then E3 activates with previous
    ///      epoch E1. The CTX1 committee signs the E2 destruction. The replica at E1 rejects it until E3
    ///      activates.
    function test_destroyedPendingEpochWaitsForNextResharing() public {
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
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector, EPOCH2)
        );
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH2, destructionSignatures, EXTRA);

        protocolConfigReplica.submitKmsKeyResharing(epoch3);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH2);
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH2, destructionSignatures, EXTRA);
        _assertActive(CTX1, EPOCH3);
    }

    /// @dev Canonical: CTX2 stays pending and is destroyed, then CTX3 activates with previous context
    ///      CTX1 and epoch E2. The CTX1 committee signs the CTX2 destruction. The replica at CTX1
    ///      rejects it until CTX3 activates. Then it verifies the signatures against CTX3, so CTX3 keeps
    ///      the first three signers of CTX1.
    function test_destroyedPendingContextWaitsForNextSwitch() public {
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
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX2)
        );
        protocolConfigReplica.submitKmsContextDestruction(CTX2, destructionSignatures, EXTRA);

        protocolConfigReplica.submitKmsContextSwitch(_canonicalCtxSubmission(CTX3, contextSignatures), epoch2);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX2);
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
        KmsContextSubmission memory context = _canonicalCtxSubmission(CTX2, contextSignatures);

        vm.chainId(REPLICA_CHAIN_ID);
        // The first signature of B follows the `n - t` signatures of A.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED1)[0], CTX1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, switchEpoch);
    }
}

/// @dev Starts each test from the replica that `_setupReplica` bootstraps.
contract ProtocolConfigReplicaKmsTest is ProtocolConfigReplicaTestBase {
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
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS
        );
    }

    function test_revertReinitializeV3OnFreshReplica() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV3(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
    }

    function test_revertUpgradeNotOwner() public {
        address newImpl = address(new ProtocolConfigReplica());

        _expectNotOwner();
        protocolConfigReplica.upgradeToAndCall(newImpl, "");
    }

    function test_eip712DomainIsCanonical() public view {
        _assertCanonicalDomain();
    }

    /// @dev Expects a resharing signed under the domain `(chainId, verifyingContract)` to be rejected.
    function _expectRejectsDomain(uint256 chainId, address verifyingContract) internal {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.signatures = _sign(
            _hashEpochActivationWithDomain(
                _computeProtocolConfigDomainSeparator(chainId, verifyingContract),
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
        _expectRejectsDomain(block.chainid, protocolConfigAdd);
    }

    /// @dev A canonical at the same address on another chain, such as a testnet.
    function test_revertSubmitSignedForOtherCanonicalChain() public {
        _expectRejectsDomain(CANONICAL_CHAIN_ID + 1, CANONICAL_ADDRESS);
    }

    // -----------------------------------------------------------------------
    // submitKmsContextSwitch
    // -----------------------------------------------------------------------

    function test_submitKmsContextSwitchActivates() public {
        uint256 contextId = CTX1 + 1;
        uint256 epochId = EPOCH1 + 1;

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.KmsContextSwitchSubmitted(contextId, epochId);
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, contextId, SEED1, SEED0, QUORUM),
            _epochSubmission(contextId, EPOCH1, epochId, SEED1, N)
        );

        _assertActive(contextId, epochId);
        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), contextId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        _assertMembers(contextId, SEED1);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), _thresholds().kmsGen);
        // The previous context stays valid, like on the canonical.
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_submitKmsContextSwitchAcceptsAllPreviousSigners() public {
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, N),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), CTX1 + 1);
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
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, bigNodes, bigThresholds, SEED0, 2),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED2, bigN)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 6, 7));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, bigNodes, bigThresholds, SEED0, 3),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED2, 6)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, bigNodes, bigThresholds, SEED0, 3),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED2, bigN)
        );
        _assertActive(CTX1 + 1, EPOCH1 + 1);
        assertEq(
            keccak256(abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX1 + 1))),
            keccak256(abi.encode(bigNodes))
        );

        // 7 nodes with t = 2 to 4 nodes: the quorums are 5 previous and 4 new signatures.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 4, 5));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED1, SEED2, 4),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED1, N)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 3, 4));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED1, SEED2, 5),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED1, N - 1)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED1, SEED2, 5),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED1, N)
        );
        _assertActive(CTX1 + 2, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED1);
    }

    function test_revertSwitchEpochSignedByPreviousCommittee() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertSwitchRejected(context, _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED0, N), CTX1 + 1);
    }

    function test_revertSwitchDuplicateSigner() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.signatures[2] = context.signatures[0];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED0)[0]));
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertSwitchNonMemberSigner() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        // A signer of the new committee only is not in the previous committee.
        context.signatures[2] = _sign(_contextDigest(CTX1, context), SEED1, 1)[0];

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED1)[0], CTX1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertSwitchTamperedNodes() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.nodes[0].ipAddress = "10.0.0.1";
        _assertSwitchRejected(context, epoch, CTX1);
    }

    /// @dev An invalid `mpc` also pins that the previous committee check runs before the store.
    function test_revertSwitchTamperedThresholds() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.thresholds.mpc = N + 1;
        _assertSwitchRejected(context, epoch, CTX1);
    }

    function test_revertSwitchTamperedExtraData() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.extraData = hex"bad0";
        _assertSwitchRejected(context, epoch, CTX1);
    }

    function test_revertSwitchTamperedContextId() public {
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        context.contextId = CTX1 + 2;
        _assertSwitchRejected(context, _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 1, SEED1, N), CTX1);
    }

    function test_revertSwitchTamperedEpochMaterial() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.epochMaterialHash = keccak256("other material");
        _assertSwitchRejected(context, epoch, CTX1 + 1);
    }

    function test_revertSwitchTamperedEpochExtraData() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.extraData = hex"bad0";
        _assertSwitchRejected(context, epoch, CTX1 + 1);
    }

    function test_revertSwitchTamperedEpochId() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.epochId = EPOCH1 + 2;
        _assertSwitchRejected(context, epoch, CTX1 + 1);
    }

    function test_revertSwitchSignedForOtherPreviousContext() public {
        _assertSwitchRejected(
            _ctxSubmission(CTX1 + 5, CTX1 + 6, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 6, EPOCH1, EPOCH1 + 1, SEED1, N),
            CTX1
        );
    }

    function test_revertSwitchSignedForOtherPreviousEpoch() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertSwitchRejected(context, _epochSubmission(CTX1 + 1, EPOCH1 + 3, EPOCH1 + 4, SEED1, N), CTX1 + 1);
    }

    function test_revertSwitchEpochContextMismatch() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertSwitchRejected(context, _epochSubmission(CTX1 + 7, EPOCH1, EPOCH1 + 1, SEED1, N), CTX1 + 1);
    }

    function test_revertSwitchReplayAlreadyApplied() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextSwitchAlreadyApplied.selector, CTX1 + 1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertSwitchOlderIdAlreadyApplied() public {
        _switch(SEED0, CTX1 + 5, EPOCH1 + 5, SEED1);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextSwitchAlreadyApplied.selector, CTX1 + 3)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 3, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 3, EPOCH1, EPOCH1 + 3, SEED2, N)
        );
    }

    function test_switchesApplyInCanonicalOrder() public {
        (KmsContextSubmission memory contextB, KmsEpochSubmission memory epochB) = _defaultSwitch();
        KmsContextSubmission memory contextC = _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED2, SEED1, QUORUM);
        KmsEpochSubmission memory epochC = _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED2, N);

        _assertSwitchRejected(contextC, epochC, CTX1);

        protocolConfigReplica.submitKmsContextSwitch(contextB, epochB);
        protocolConfigReplica.submitKmsContextSwitch(contextC, epochC);

        _assertActive(CTX1 + 2, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED2);
    }

    function test_revertSwitchToDestroyedId() public {
        uint256 destroyedId = CTX1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(destroyedId);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsContextCannotBeActivated.selector, destroyedId)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, destroyedId, SEED1, SEED0, QUORUM),
            _epochSubmission(destroyedId, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function test_revertSwitchToDestroyedEpoch() public {
        uint256 destroyedId = EPOCH1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(destroyedId);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, destroyedId)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1, destroyedId, SEED1, N)
        );
    }

    /// @dev Canonical: C2 is destroyed while pending, then C3 activates with previous context C1.
    function test_switchSkipsDestroyedPendingContext() public {
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 1);

        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 2, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 2, SEED2, N)
        );
        _assertActive(CTX1 + 2, EPOCH1 + 2);
    }

    // -----------------------------------------------------------------------
    // submitKmsKeyResharing
    // -----------------------------------------------------------------------

    function test_submitKmsKeyResharingActivates() public {
        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.KmsKeyResharingSubmitted(CTX1, EPOCH1 + 1);
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));

        _assertActive(CTX1, EPOCH1 + 1);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
    }

    function test_revertResharingNotUnanimous() public {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, N - 1, N));
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N - 1));
    }

    function test_revertResharingTamperedEpochMaterial() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.epochMaterialHash = keccak256("other material");
        _assertResharingRejected(epoch);
    }

    function test_revertResharingTamperedExtraData() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.extraData = hex"bad0";
        _assertResharingRejected(epoch);
    }

    function test_revertResharingTamperedEpochId() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.epochId = EPOCH1 + 2;
        _assertResharingRejected(epoch);
    }

    function test_revertResharingSignedForOtherPreviousEpoch() public {
        _assertResharingRejected(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
    }

    function test_revertResharingWrongContext() public {
        _assertResharingRejected(_epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED0, N));
    }

    function test_revertResharingReplayAlreadyApplied() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        protocolConfigReplica.submitKmsKeyResharing(epoch);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsKeyResharingAlreadyApplied.selector, EPOCH1 + 1)
        );
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_revertResharingDestroyedEpoch() public {
        uint256 destroyedId = EPOCH1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(destroyedId);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, destroyedId)
        );
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, destroyedId, SEED0, N));
    }

    // -----------------------------------------------------------------------
    // submitKmsContextDestruction / submitKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_submitKmsContextDestructionOfSupersededContext() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        _destroyContext(CTX1, SEED1, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1 + 1, EPOCH1 + 1));
    }

    function test_revertContextDestructionReplay() public {
        // A destruction applies only to IDs that are not above the active ones.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 10, EPOCH1 + 10, _nodes(SEED0), _thresholds());
        bytes[] memory signatures = _sign(
            _hashContextDestructionWithDomain(_canonicalDomainSeparator(), CTX1 + 9, EXTRA),
            SEED0,
            QUORUM
        );
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1 + 9);
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, signatures, EXTRA);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1 + 9));
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, signatures, EXTRA);

        // After a rotation the old signatures no longer match the committee, but the replay still
        // reports the destruction as applied.
        _switch(SEED0, CTX1 + 11, EPOCH1 + 11, SEED1);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1 + 9));
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, signatures, EXTRA);
    }

    function test_revertContextDestructionActive() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        _destroyContext(CTX1, SEED0, QUORUM);
    }

    function test_revertContextDestructionQuorum() public {
        // A destruction applies only to IDs that are not above the active ones.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 10, EPOCH1 + 10, _nodes(SEED0), _thresholds());
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyContext(CTX1 + 9, SEED0, QUORUM - 1);
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
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, new bytes[](0), EXTRA);
    }

    /// @dev Canonical C2 (SEED1) destroys C1, but the replica switches to C3 (SEED2) before the
    ///      destruction arrives. The C2 signatures no longer match the active committee, so the owner
    ///      applies the destruction (accepted RFC 037 risk).
    function test_contextDestructionAfterRotationRecoversThroughMirror() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        bytes[] memory signatures = _sign(
            _hashContextDestructionWithDomain(_canonicalDomainSeparator(), CTX1, EXTRA),
            SEED1,
            QUORUM
        );
        _switch(SEED1, CTX1 + 2, EPOCH1 + 2, SEED2);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsSignerNotInCommittee.selector,
                _signers(SEED1)[0],
                CTX1 + 2
            )
        );
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, EXTRA);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1));
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, EXTRA);
    }

    function test_revertEpochDestructionReplay() public {
        // A destruction applies only to IDs that are not above the active ones.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 10, EPOCH1 + 10, _nodes(SEED0), _thresholds());
        bytes[] memory signatures = _sign(
            _hashEpochDestructionWithDomain(_canonicalDomainSeparator(), EPOCH1 + 9, EXTRA),
            SEED0,
            QUORUM
        );
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1 + 9, signatures, EXTRA);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1 + 9));
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1 + 9, signatures, EXTRA);

        // After a rotation the old signatures no longer match the committee, but the replay still
        // reports the destruction as applied.
        _switch(SEED0, CTX1 + 11, EPOCH1 + 11, SEED1);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1 + 9));
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1 + 9, signatures, EXTRA);
    }

    function test_revertEpochDestructionActive() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        _destroyEpoch(EPOCH1, SEED0, QUORUM);
    }

    function test_revertEpochDestructionQuorum() public {
        // A destruction applies only to IDs that are not above the active ones.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 10, EPOCH1 + 10, _nodes(SEED0), _thresholds());
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyEpoch(EPOCH1 + 9, SEED0, QUORUM - 1);
    }

    function test_supersededEpochDestructionAfterReplicaCatchesUp() public {
        // Canonical activates E2, then E3, then destroys the superseded E2. The replica catches up first.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1 + 1)
        );
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1 + 1);
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
    }

    /// @dev Canonical activates E2, then E3, then destroys E2. The replica is still at E1 and receives
    ///      the E2 destruction first. It reverts until the replica activates E3.
    function test_supersededEpochDestructionAheadOfReplicaWaitsForReplica() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector, EPOCH1 + 1)
        );
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1 + 1);
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        _assertActive(CTX1, EPOCH1 + 2);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
    }

    /// @dev Canonical activates E2, then E3, then destroys E2. The owner mirrors the E2 destruction on
    ///      the replica at E1. The replica then rejects both canonical resharings. The owner recovers it
    ///      by mirroring the canonical active epoch E3.
    function test_mirroredEpochDestructionAheadOfReplicaRecoversThroughMirror() public {
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1 + 1);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, EPOCH1 + 1)
        );
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));
        _assertResharingRejected(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 2);
        _assertActive(CTX1, EPOCH1 + 2);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));

        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1 + 2, EPOCH1 + 3, SEED0, N));
        _assertActive(CTX1, EPOCH1 + 3);
    }

    /// @dev Committee {A, B, C} of SEED0 plus the last node of committee `seed`, with its signer keys.
    function _overlapCommittee(uint256 seed) internal pure returns (KmsNode[] memory nodes, uint256[] memory keys) {
        nodes = _nodes(SEED0);
        keys = _keys(SEED0, N);
        nodes[N - 1] = _nodes(seed)[N - 1];
        keys[N - 1] = _keys(seed, N)[N - 1];
    }

    /// @dev An activation of `epochId` under `contextId`, signed with `keys`.
    function _epochSubmission(
        uint256 contextId,
        uint256 previousEpochId,
        uint256 epochId,
        uint256[] memory keys
    ) internal pure returns (KmsEpochSubmission memory epoch) {
        epoch = _epochSubmission(contextId, previousEpochId, epochId, SEED0, 0);
        bytes32 digest = _epochDigest(contextId, previousEpochId, epoch);
        epoch.signatures = new bytes[](keys.length);
        for (uint256 i = 0; i < keys.length; i++) {
            epoch.signatures[i] = _computeSignature(keys[i], digest);
        }
    }

    function test_supersededContextDestructionWaitsForReplica() public {
        // Canonical switches from C1 (SEED0) to C2 (SEED1), then to C3 (SEED2), then C3 destroys C2.
        // The replica is at C1, so C2 is above its active context.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1 + 1)
        );
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        _switch(SEED1, CTX1 + 2, EPOCH1 + 2, SEED2);
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
        assertTrue(protocolConfigReplica.isValidKmsContext(CTX1 + 2));
    }

    /// @dev n = 4, t = 1. C1 = {A, B, C, D}, C2 = {A, B, C, E}, C3 = {A, B, C, F}. Canonical switches
    ///      to C2, then to C3, then destroys C2. A, B and C of C3 sign the destruction. They are also in
    ///      C1, but the replica at C1 rejects the destruction until it activates C3.
    function test_supersededContextDestructionWaitsForReplicaWithOverlappingCommittees() public {
        (KmsNode[] memory nodes2, uint256[] memory keys2) = _overlapCommittee(SEED1);
        (KmsNode[] memory nodes3, uint256[] memory keys3) = _overlapCommittee(SEED2);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1 + 1)
        );
        _destroyContext(CTX1 + 1, SEED0, QUORUM);

        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, nodes2, _thresholds(), SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, keys2)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 1, CTX1 + 2, nodes3, _thresholds(), SEED0, QUORUM),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, keys3)
        );
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1 + 1);
        _destroyContext(CTX1 + 1, SEED0, QUORUM);

        _assertActive(CTX1 + 2, EPOCH1 + 2);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
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
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);
    }

    function test_revertMirrorKmsEpochInvalidContext() public {
        uint256 invalidContextId = CTX1 + 1;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfigReplica.mirrorKmsEpoch(invalidContextId, EPOCH1 + 1);
    }

    function test_revertMirrorKmsEpochSupersededContext() public {
        uint256 supersededContextId = CTX1;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 1, _nodes(SEED1), _thresholds());

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, supersededContextId));
        protocolConfigReplica.mirrorKmsEpoch(supersededContextId, EPOCH1 + 2);
    }

    function test_mirrorKmsEpochMovesEpochBack() public {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 5);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 2);
        vm.stopPrank();

        _assertActive(CTX1, EPOCH1 + 2);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
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
        // The skipped IDs in the gap remain invalid.
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 2));
    }

    function test_mirrorKmsContextAndEpochMovesBack() public {
        _switch(SEED0, CTX1 + 2, EPOCH1 + 2, SEED1);

        // Lower IDs move the replica back.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 1, _nodes(SEED0), _thresholds());

        _assertActive(CTX1 + 1, EPOCH1 + 1);
        _assertMembers(CTX1 + 1, SEED0);
    }

    function test_revertMirrorKmsContextAndEpochInvalidThresholds() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "mpc", 5, 4));
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            CTX1 + 1,
            EPOCH1 + 1,
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

    function test_revertMirrorKmsContextDestructionTwice() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.startPrank(owner);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1));
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
        vm.stopPrank();
    }

    function test_revertMirrorKmsEpochDestructionTwice() public {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1));
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
        vm.stopPrank();
    }

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

    // -----------------------------------------------------------------------
    // Recovery
    // -----------------------------------------------------------------------

    // The owner recovers the replica with the mirror functions, and the submit path keeps working.

    /// @dev The owner moves the replica back to the bootstrap pair and destroys `CTX1 + 1`.
    function _recoverToGenesis() internal {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 1);
        vm.stopPrank();
    }

    function test_recoveryFromLateActivation() public {
        // Canonical: C2 stays pending and is destroyed, then C1 switches to C3. A late signature set
        // activates C2 on the replica.
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        _recoverToGenesis();

        // The owner flag settles the destruction, so the signed path reports it as applied.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1 + 1));
        _destroyContext(CTX1 + 1, SEED0, QUORUM);

        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 2, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 2, SEED2, N)
        );

        _assertActive(CTX1 + 2, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED2);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
    }

    function test_mirrorKmsContextAndEpochOverridesCompromisedCommittee() public {
        uint256 forgedId = CTX1 + 1;
        // A compromised committee forges a switch to committee SEED1.
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, EPOCH1 + 1, _nodes(SEED2), _thresholds());

        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        // The mirrored committee signs the next resharing.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(forgedId, EPOCH1 + 1, EPOCH1 + 2, SEED2, N));
        _assertActive(forgedId, EPOCH1 + 2);
    }

    function test_recoveryBlocksContextSwitchReplay() public {
        uint256 forgedId = CTX1 + 1;
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
        _recoverToGenesis();

        _assertActive(CTX1, EPOCH1);
        _assertMembers(CTX1, SEED0);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));

        // The replayed calldata still matches the digest built from the restored pair.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsContextCannotBeActivated.selector, forgedId)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_recoveryBlocksResharingReplay() public {
        uint256 forgedEpochId = EPOCH1 + 1;
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, forgedEpochId, SEED0, N);
        protocolConfigReplica.submitKmsKeyResharing(epoch);

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);
        vm.stopPrank();

        _assertActive(CTX1, EPOCH1);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, forgedEpochId));

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, forgedEpochId)
        );
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_recoveryThenCanonicalUsesDestroyedId() public {
        uint256 forgedId = CTX1 + 1;
        uint256 forgedEpochId = EPOCH1 + 1;
        uint256 realEpochId = EPOCH1 + 2;
        _switch(SEED0, forgedId, forgedEpochId, SEED1);
        _recoverToGenesis();
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);

        // Canonical later allocates the same context ID to a real committee. The owner mirrors it, which
        // replaces the forged nodes and the Destroyed state.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, realEpochId, _nodes(SEED2), _thresholds());
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        assertTrue(protocolConfigReplica.isValidEpochForContext(forgedId, realEpochId));
        // The forged epoch stays destroyed, so it is not valid for the reused context ID.
        assertFalse(protocolConfigReplica.isValidEpochForContext(forgedId, forgedEpochId));

        // The real committee signs the next resharing, and a later canonical destruction applies.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(forgedId, realEpochId, realEpochId + 1, SEED2, N));
        _switch(SEED2, CTX1 + 2, EPOCH1 + 4, SEED1);
        _destroyContext(forgedId, SEED1, QUORUM);
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));
    }

    function test_mirroredDestroyedEpochCanBeDestroyedAgain() public {
        uint256 epochId = EPOCH1 + 1;
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(epochId);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, epochId);
        vm.stopPrank();
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, epochId, epochId + 1, SEED0, N));

        _destroyEpoch(epochId, SEED0, QUORUM);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, epochId));
    }

    function test_mirroredContextClearsDestroyedEpoch() public {
        uint256 epochId = EPOCH1 + 1;
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(epochId);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, epochId, _nodes(SEED1), _thresholds());
        vm.stopPrank();
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1 + 1, epochId, epochId + 1, SEED1, N));

        _destroyEpoch(epochId, SEED1, QUORUM);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1 + 1, epochId));
    }

    function test_submitSwitchReplacesForgedStoredId() public {
        uint256 forgedId = CTX1 + 1;
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        // The owner moves the replica back and does not destroy the forged ID.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());

        _switch(SEED0, forgedId, EPOCH1 + 1, SEED2);
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), forgedId);
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ProtocolConfigSigningTestUtils} from "@fhevm-foundry/ProtocolConfigSigningTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode, KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigReplicaTest is ProtocolConfigSigningTestUtils {
    ProtocolConfigReplica internal protocolConfigReplica;

    address internal constant owner = address(456);
    bytes32 internal constant INITIALIZABLE_STORAGE =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;

    /// @dev The canonical ProtocolConfig the replica verifies signatures for. The test chain is 31337.
    uint64 internal constant CANONICAL_CHAIN_ID = 1;
    address internal constant CANONICAL_ADDRESS = address(0xC0FFEE);

    uint256 internal constant CTX1 = KMS_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant EPOCH1 = EPOCH_COUNTER_BASE + 1;

    /// @dev Seed of the bootstrap committee: its keys are those of `_makeKmsNodeParams(4)`.
    uint256 internal constant SEED0 = 0;
    /// @dev Seeds of the other committees.
    uint256 internal constant SEED1 = 1;
    uint256 internal constant SEED2 = 2;

    bytes32 internal constant MATERIAL = keccak256("epoch material");
    bytes internal constant EXTRA = hex"c0de";

    /// @dev Every committee has 4 nodes and MPC threshold 1, so the `n - t` quorum is 3.
    uint256 internal constant N = 4;
    uint256 internal constant QUORUM = 3;

    /// @dev Signatures are made under the canonical domain, not the replica's own.
    function _hashProtocolConfigTypedData(bytes32 structHash) internal pure override returns (bytes32) {
        return
            MessageHashUtils.toTypedDataHash(
                _computeProtocolConfigDomainSeparator(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS),
                structHash
            );
    }

    /// @dev Bootstraps a replica on the same active context and epoch as a fresh canonical deploy.
    function _setupReplica() internal {
        _deployACL(owner);
        (protocolConfigReplica, ) = _deployProtocolConfigReplica(
            owner,
            CTX1,
            EPOCH1,
            _makeKmsNodeParams(N),
            _thresholds(),
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS
        );
    }

    function _setupCanonical() internal returns (ProtocolConfig pc) {
        _deployACL(owner);
        (pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
    }

    function _upgradeToReplica(ProtocolConfig pc) internal returns (ProtocolConfigReplica) {
        // Existing replica proxies run the v0.15 ProtocolConfig, which is at initialized version 4.
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(4)));
        address replicaImpl = address(new ProtocolConfigReplica());
        vm.prank(owner);
        pc.upgradeToAndCall(
            replicaImpl,
            abi.encodeCall(ProtocolConfigReplica.reinitializeV4, (CANONICAL_CHAIN_ID, CANONICAL_ADDRESS))
        );
        return ProtocolConfigReplica(protocolConfigAdd);
    }

    function _thresholds() internal pure returns (KmsThresholds memory) {
        return KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 1});
    }

    /// @dev Signer keys of committee `seed`. Seed 0 is the bootstrap committee.
    function _keys(uint256 seed) internal pure returns (uint256[] memory keys) {
        keys = new uint256[](N);
        for (uint256 i = 0; i < N; i++) {
            keys[i] = (i + 1) * 0x100 + seed;
        }
    }

    function _nodes(uint256 seed) internal pure returns (KmsNode[] memory nodes) {
        uint256[] memory keys = _keys(seed);
        nodes = new KmsNode[](N);
        for (uint256 i = 0; i < N; i++) {
            nodes[i] = KmsNode({
                txSenderAddress: address(uint160(0xA1 + i + seed * 0x10)),
                signerAddress: vm.addr(keys[i]),
                ipAddress: "127.0.0.1",
                storageUrl: "https://s.example.com"
            });
        }
    }

    function _signers(uint256 seed) internal pure returns (address[] memory signers) {
        uint256[] memory keys = _keys(seed);
        signers = new address[](N);
        for (uint256 i = 0; i < N; i++) {
            signers[i] = vm.addr(keys[i]);
        }
    }

    /// @dev Signs `digest` with the first `count` keys of committee `seed`.
    function _sign(bytes32 digest, uint256 seed, uint256 count) internal pure returns (bytes[] memory signatures) {
        uint256[] memory keys = _keys(seed);
        signatures = new bytes[](count);
        for (uint256 i = 0; i < count; i++) {
            signatures[i] = _computeSignature(keys[i], digest);
        }
    }

    /// @dev A switch from `previousContextId` to committee `seed` under `contextId`, signed by
    ///      `count` signers of committee `previousSeed`.
    function _ctxSubmission(
        uint256 previousContextId,
        uint256 contextId,
        uint256 seed,
        uint256 previousSeed,
        uint256 count
    ) internal view returns (KmsContextSubmission memory context) {
        context = KmsContextSubmission({
            previousContextId: previousContextId,
            contextId: contextId,
            nodes: _nodes(seed),
            thresholds: _thresholds(),
            signatures: new bytes[](0),
            extraData: EXTRA
        });
        context.signatures = _sign(_contextDigest(context), previousSeed, count);
    }

    /// @dev An activation of `epochId` under `contextId`, signed by `count` signers of committee `seed`.
    function _epochSubmission(
        uint256 contextId,
        uint256 previousEpochId,
        uint256 epochId,
        uint256 seed,
        uint256 count
    ) internal view returns (KmsEpochSubmission memory epoch) {
        epoch = KmsEpochSubmission({
            contextId: contextId,
            previousEpochId: previousEpochId,
            epochId: epochId,
            epochMaterialHash: MATERIAL,
            signatures: new bytes[](0),
            extraData: EXTRA
        });
        epoch.signatures = _sign(_epochDigest(epoch), seed, count);
    }

    /// @dev Applies a fully signed switch from the active pair to committee `seed`.
    function _switch(uint256 previousSeed, uint256 contextId, uint256 epochId, uint256 seed) internal {
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(activeContextId, contextId, seed, previousSeed, QUORUM),
            _epochSubmission(contextId, activeEpochId, epochId, seed, N)
        );
    }

    function _assertMembers(uint256 contextId, uint256 seed) internal view {
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), _signers(seed));
    }

    function _assertNotMembers(uint256 contextId, uint256 seed) internal view {
        KmsNode[] memory nodes = _nodes(seed);
        for (uint256 i = 0; i < N; i++) {
            assertFalse(protocolConfigReplica.isKmsSignerForContext(contextId, nodes[i].signerAddress));
            assertFalse(protocolConfigReplica.isKmsTxSenderForContext(contextId, nodes[i].txSenderAddress));
            assertEq(
                protocolConfigReplica.getKmsNodeForContext(contextId, nodes[i].txSenderAddress).signerAddress,
                address(0)
            );
        }
    }

    function _destroyContext(uint256 contextId, uint256 seed, uint256 count) internal {
        protocolConfigReplica.submitKmsContextDestruction(
            contextId,
            _sign(_hashContextDestruction(contextId, EXTRA), seed, count),
            EXTRA
        );
    }

    function _destroyEpoch(uint256 epochId, uint256 seed, uint256 count) internal {
        protocolConfigReplica.submitKmsEpochDestruction(
            epochId,
            _sign(_hashEpochDestruction(epochId, EXTRA), seed, count),
            EXTRA
        );
    }

    /// @dev Canonical digest of the fields that `context` carries.
    function _contextDigest(KmsContextSubmission memory context) internal view returns (bytes32) {
        return
            _hashContextCreation(
                context.previousContextId,
                context.contextId,
                keccak256(abi.encode(context.nodes, context.thresholds)),
                context.extraData
            );
    }

    /// @dev Canonical digest of the fields that `epoch` carries.
    function _epochDigest(KmsEpochSubmission memory epoch) internal view returns (bytes32) {
        return
            _hashEpochActivation(
                epoch.contextId,
                epoch.previousEpochId,
                epoch.epochId,
                epoch.epochMaterialHash,
                epoch.extraData
            );
    }

    /// @dev Expects `KmsSignerNotInCommittee` for the signer that `signature` recovers to over the
    ///      canonical digest `digest`.
    function _expectNotInCommittee(bytes32 digest, bytes memory signature, uint256 contextId) internal {
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsSignerNotInCommittee.selector,
                ECDSA.recover(digest, signature),
                contextId
            )
        );
    }

    // -----------------------------------------------------------------------
    // mirrorKmsEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsEpochActivatesCanonicalEpoch() public {
        _setupReplica();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 7;

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsEpoch(contextId, epochId);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, epochId);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
    }

    function test_revertMirrorKmsEpochNotOwner() public {
        _setupReplica();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsEpoch(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 2);
    }

    function test_revertMirrorKmsEpochInvalidContext() public {
        _setupReplica();
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfigReplica.mirrorKmsEpoch(invalidContextId, EPOCH_COUNTER_BASE + 2);
    }

    function test_revertMirrorKmsEpochSupersededContext() public {
        _setupReplica();
        uint256 supersededContextId = KMS_CONTEXT_COUNTER_BASE + 1;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            KMS_CONTEXT_COUNTER_BASE + 2,
            EPOCH_COUNTER_BASE + 2,
            _makeKmsNodes(2),
            _defaultThresholds()
        );

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, supersededContextId));
        protocolConfigReplica.mirrorKmsEpoch(supersededContextId, EPOCH_COUNTER_BASE + 3);
    }

    function test_mirrorKmsEpochLowerId() public {
        _setupReplica();

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 5);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 2);
        vm.stopPrank();

        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH1 + 2);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
    }

    // -----------------------------------------------------------------------
    // Canonical mirror initializer
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalInvalidEpochId() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();

        address impl = address(new ProtocolConfigReplica());
        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 invalidEpochId = EPOCH_COUNTER_BASE;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, invalidEpochId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (canonicalContextId, invalidEpochId, nodes, thresholds, CANONICAL_CHAIN_ID, CANONICAL_ADDRESS)
            )
        );
    }

    function test_revertInitializeFromCanonicalInvalidContextId() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();

        address impl = address(new ProtocolConfigReplica());
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 1;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (invalidContextId, canonicalEpochId, nodes, thresholds, CANONICAL_CHAIN_ID, CANONICAL_ADDRESS)
            )
        );
    }

    // -----------------------------------------------------------------------
    // Re-initialization protection
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalAfterInit() public {
        _setupReplica();

        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 5;
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfigReplica.initializeFromCanonical(
            canonicalContextId,
            canonicalEpochId,
            _makeKmsNodeParams(1),
            _defaultThresholds(),
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS
        );
    }

    function test_revertInitializeFromCanonicalOnUpgradeFromProtocolConfig() public {
        ProtocolConfig pc = _setupCanonical();

        address replicaImpl = address(new ProtocolConfigReplica());
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        pc.upgradeToAndCall(
            replicaImpl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    KMS_CONTEXT_COUNTER_BASE + 1,
                    EPOCH_COUNTER_BASE + 1,
                    _makeKmsNodeParams(2),
                    _defaultThresholds(),
                    CANONICAL_CHAIN_ID,
                    CANONICAL_ADDRESS
                )
            )
        );
    }

    function test_reinitializeV4SucceedsOnUpgradeFromProtocolConfig() public {
        ProtocolConfig pc = _setupCanonical();

        (uint256 contextId, uint256 epochId) = pc.getCurrentKmsContextAndEpoch();
        address[] memory signers = pc.getKmsSignersForContext(contextId);
        uint256 publicDecryption = pc.getPublicDecryptionThresholdForContext(contextId);
        uint256 userDecryption = pc.getUserDecryptionThresholdForContext(contextId);
        uint256 kmsGen = pc.getKmsGenThresholdForContext(contextId);
        uint256 mpc = pc.getMpcThresholdForContext(contextId);

        protocolConfigReplica = _upgradeToReplica(pc);

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 5);
        _assertCanonicalDomain();
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.4.0");
        (uint256 newContextId, uint256 newEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(newContextId, contextId);
        assertEq(newEpochId, epochId);
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), signers);
        assertEq(protocolConfigReplica.getPublicDecryptionThresholdForContext(contextId), publicDecryption);
        assertEq(protocolConfigReplica.getUserDecryptionThresholdForContext(contextId), userDecryption);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), kmsGen);
        assertEq(protocolConfigReplica.getMpcThresholdForContext(contextId), mpc);

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
    }

    function test_revertReinitializeV4OnFreshReplica() public {
        _setupReplica();

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
    }

    function test_revertUpgradeNotOwner() public {
        _setupReplica();
        address newImpl = address(new ProtocolConfigReplica());

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.upgradeToAndCall(newImpl, "");
    }

    // -----------------------------------------------------------------------
    // EIP-712 domain
    // -----------------------------------------------------------------------

    function _assertCanonicalDomain() internal view {
        (
            ,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            ,

        ) = protocolConfigReplica.eip712Domain();
        assertEq(name, "ProtocolConfig");
        assertEq(version, "1");
        assertEq(chainId, CANONICAL_CHAIN_ID);
        assertEq(verifyingContract, CANONICAL_ADDRESS);
    }

    function test_eip712DomainIsCanonical() public {
        _setupReplica();
        _assertCanonicalDomain();
    }

    function test_revertSubmitSignedForLocalDomain() public {
        _setupReplica();
        bytes32 structHash = keccak256(
            abi.encode(EIP712_EPOCH_ACTIVATION_TYPE_HASH, CTX1, EPOCH1, EPOCH1 + 1, MATERIAL, keccak256(EXTRA))
        );
        bytes32 localDigest = MessageHashUtils.toTypedDataHash(
            _computeProtocolConfigDomainSeparator(block.chainid, address(protocolConfigReplica)),
            structHash
        );
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.signatures = _sign(localDigest, SEED0, N);

        _expectNotInCommittee(_epochDigest(epoch), epoch.signatures[0], CTX1);
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    // -----------------------------------------------------------------------
    // submitKmsContextSwitch
    // -----------------------------------------------------------------------

    function test_submitKmsContextSwitchActivates() public {
        _setupReplica();
        uint256 contextId = CTX1 + 1;
        uint256 epochId = EPOCH1 + 1;

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.KmsContextSwitchSubmitted(contextId, epochId);
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, contextId, SEED1, SEED0, QUORUM),
            _epochSubmission(contextId, EPOCH1, epochId, SEED1, N)
        );

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), contextId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        _assertMembers(contextId, SEED1);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), _thresholds().kmsGen);
        // The previous context stays valid, like on the canonical.
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_submitKmsContextSwitchAcceptsAllPreviousSigners() public {
        _setupReplica();
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, N),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), CTX1 + 1);
    }

    function test_revertSwitchQuorumNotReached() public {
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM - 1),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function test_revertSwitchEpochNotUnanimous() public {
        _setupReplica();
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, N - 1, N));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N - 1)
        );
    }

    function test_revertSwitchDuplicateSigner() public {
        _setupReplica();
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        context.signatures[2] = context.signatures[0];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED0)[0]));
        protocolConfigReplica.submitKmsContextSwitch(context, _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchNonMemberSigner() public {
        _setupReplica();
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        // A signer of the new committee only is not in the previous committee.
        context.signatures[2] = _sign(_contextDigest(context), SEED1, 1)[0];

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED1)[0], CTX1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchTamperedNodes() public {
        _setupReplica();
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        context.nodes[0].ipAddress = "10.0.0.1";

        _expectNotInCommittee(_contextDigest(context), context.signatures[0], CTX1);
        protocolConfigReplica.submitKmsContextSwitch(context, _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchTamperedThresholds() public {
        _setupReplica();
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        context.thresholds.kmsGen = 2;

        _expectNotInCommittee(_contextDigest(context), context.signatures[0], CTX1);
        protocolConfigReplica.submitKmsContextSwitch(context, _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchTamperedExtraData() public {
        _setupReplica();
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        context.extraData = hex"bad0";

        _expectNotInCommittee(_contextDigest(context), context.signatures[0], CTX1);
        protocolConfigReplica.submitKmsContextSwitch(context, _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchTamperedContextId() public {
        _setupReplica();
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        context.contextId = CTX1 + 2;

        _expectNotInCommittee(_contextDigest(context), context.signatures[0], CTX1);
        protocolConfigReplica.submitKmsContextSwitch(context, _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchTamperedEpochMaterial() public {
        _setupReplica();
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N);
        epoch.epochMaterialHash = keccak256("other material");

        _expectNotInCommittee(_epochDigest(epoch), epoch.signatures[0], CTX1 + 1);
        protocolConfigReplica.submitKmsContextSwitch(_ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM), epoch);
    }

    function test_revertSwitchTamperedEpochExtraData() public {
        _setupReplica();
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N);
        epoch.extraData = hex"bad0";

        _expectNotInCommittee(_epochDigest(epoch), epoch.signatures[0], CTX1 + 1);
        protocolConfigReplica.submitKmsContextSwitch(_ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM), epoch);
    }

    function test_revertSwitchTamperedEpochId() public {
        _setupReplica();
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N);
        epoch.epochId = EPOCH1 + 2;

        _expectNotInCommittee(_epochDigest(epoch), epoch.signatures[0], CTX1 + 1);
        protocolConfigReplica.submitKmsContextSwitch(_ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM), epoch);
    }

    function test_revertSwitchLineageContext() public {
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextLineageMismatch.selector, CTX1 + 5, CTX1)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 5, CTX1 + 6, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 6, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function test_revertSwitchLineageEpoch() public {
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochLineageMismatch.selector, EPOCH1 + 3, EPOCH1)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1 + 3, EPOCH1 + 4, SEED1, N)
        );
    }

    function test_revertSwitchEpochContextMismatch() public {
        _setupReplica();
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, CTX1 + 7));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 7, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function test_revertSwitchReplayAlreadyApplied() public {
        _setupReplica();
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N);
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextSwitchAlreadyApplied.selector, CTX1 + 1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertSwitchOlderIdAlreadyApplied() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 5, EPOCH1 + 5, SEED1);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextSwitchAlreadyApplied.selector, CTX1 + 3)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 3, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 3, EPOCH1, EPOCH1 + 3, SEED2, N)
        );
    }

    function test_switchesInLineageOrder() public {
        _setupReplica();
        KmsContextSubmission memory contextB = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        KmsEpochSubmission memory epochB = _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N);
        KmsContextSubmission memory contextC = _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED2, SEED1, QUORUM);
        KmsEpochSubmission memory epochC = _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED2, N);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextLineageMismatch.selector, CTX1 + 1, CTX1)
        );
        protocolConfigReplica.submitKmsContextSwitch(contextC, epochC);

        protocolConfigReplica.submitKmsContextSwitch(contextB, epochB);
        protocolConfigReplica.submitKmsContextSwitch(contextC, epochC);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1 + 2);
        assertEq(activeEpochId, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED2);
    }

    function test_revertSwitchToDestroyedId() public {
        _setupReplica();
        uint256 destroyedId = CTX1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(destroyedId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextIdDestroyed.selector, destroyedId));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, destroyedId, SEED1, SEED0, QUORUM),
            _epochSubmission(destroyedId, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function test_revertSwitchToDestroyedEpoch() public {
        _setupReplica();
        uint256 destroyedId = EPOCH1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(destroyedId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochIdDestroyed.selector, destroyedId));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1, destroyedId, SEED1, N)
        );
    }

    // -----------------------------------------------------------------------
    // submitKmsKeyResharing
    // -----------------------------------------------------------------------

    function test_submitKmsKeyResharingActivates() public {
        _setupReplica();

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.KmsKeyResharingSubmitted(CTX1, EPOCH1 + 1);
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1);
        assertEq(activeEpochId, EPOCH1 + 1);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
    }

    function test_revertResharingNotUnanimous() public {
        _setupReplica();
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, N - 1, N));
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N - 1));
    }

    function test_revertResharingTampered() public {
        _setupReplica();
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.epochMaterialHash = keccak256("other material");

        _expectNotInCommittee(_epochDigest(epoch), epoch.signatures[0], CTX1);
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_revertResharingTamperedExtraData() public {
        _setupReplica();
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.extraData = hex"bad0";

        _expectNotInCommittee(_epochDigest(epoch), epoch.signatures[0], CTX1);
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_revertResharingTamperedEpochId() public {
        _setupReplica();
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.epochId = EPOCH1 + 2;

        _expectNotInCommittee(_epochDigest(epoch), epoch.signatures[0], CTX1);
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_revertResharingLineageEpoch() public {
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochLineageMismatch.selector, EPOCH1 + 1, EPOCH1)
        );
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
    }

    function test_revertResharingWrongContext() public {
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextLineageMismatch.selector, CTX1 + 1, CTX1)
        );
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED0, N));
    }

    function test_revertResharingReplayAlreadyApplied() public {
        _setupReplica();
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        protocolConfigReplica.submitKmsKeyResharing(epoch);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsKeyResharingAlreadyApplied.selector, EPOCH1 + 1)
        );
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_revertResharingDestroyedEpoch() public {
        _setupReplica();
        uint256 destroyedId = EPOCH1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(destroyedId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochIdDestroyed.selector, destroyedId));
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, destroyedId, SEED0, N));
    }

    // -----------------------------------------------------------------------
    // submitKmsContextDestruction / submitKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_submitKmsContextDestructionOfSupersededContext() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        _destroyContext(CTX1, SEED1, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1 + 1, EPOCH1 + 1));
    }

    /// @dev Moves the replica to `CTX1 + 10` and `EPOCH1 + 10` under committee SEED0, so the IDs
    ///      below are never stored and not ahead of the replica.
    function _mirrorAhead() internal {
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 10, EPOCH1 + 10, _nodes(SEED0), _thresholds());
    }

    function test_revertContextDestructionReplay() public {
        _setupReplica();
        _mirrorAhead();
        bytes[] memory signatures = _sign(_hashContextDestruction(CTX1 + 9, EXTRA), SEED0, QUORUM);
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
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1, CTX1)
        );
        _destroyContext(CTX1, SEED0, QUORUM);
    }

    function test_revertContextDestructionQuorum() public {
        _setupReplica();
        _mirrorAhead();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyContext(CTX1 + 9, SEED0, QUORUM - 1);
    }

    function test_revertDestructionZeroQuorumCommittee() public {
        _setupReplica();
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

    function test_revertContextDestructionSignedByOldCommittee() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsSignerNotInCommittee.selector,
                _signers(SEED0)[0],
                CTX1 + 1
            )
        );
        _destroyContext(CTX1, SEED0, QUORUM);
    }

    function test_submitKmsEpochDestruction() public {
        _setupReplica();
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        _destroyEpoch(EPOCH1, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
    }

    function test_revertEpochDestructionReplay() public {
        _setupReplica();
        _mirrorAhead();
        bytes[] memory signatures = _sign(_hashEpochDestruction(EPOCH1 + 9, EXTRA), SEED0, QUORUM);
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
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector, EPOCH1, EPOCH1)
        );
        _destroyEpoch(EPOCH1, SEED0, QUORUM);
    }

    function test_revertEpochDestructionQuorum() public {
        _setupReplica();
        _mirrorAhead();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyEpoch(EPOCH1 + 9, SEED0, QUORUM - 1);
    }

    function test_supersededEpochDestructionWaitsForReplica() public {
        _setupReplica();
        // Canonical activates E2, then E3, then destroys the superseded E2. The replica is still at E1.
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector,
                EPOCH1 + 1,
                EPOCH1
            )
        );
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
    }

    function test_epochDestructionAheadSignedByUnknownCommittee() public {
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector,
                EPOCH1 + 1,
                EPOCH1
            )
        );
        _destroyEpoch(EPOCH1 + 1, SEED2, QUORUM);
    }

    function test_supersededContextDestructionWaitsForReplica() public {
        _setupReplica();
        // Canonical switches from C1 (SEED0) to C2 (SEED1), then to C3 (SEED2), then C3 destroys C2.
        // The replica is at C1.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1 + 1, CTX1)
        );
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        _switch(SEED1, CTX1 + 2, EPOCH1 + 2, SEED2);
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
        assertTrue(protocolConfigReplica.isValidKmsContext(CTX1 + 2));
    }

    function test_destroyedPendingEpochAppliesAfterNextResharing() public {
        _setupReplica();
        // Canonical: E2 stays pending and is destroyed, then E3 activates with previous epoch E1.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 2, SEED0, N));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH1 + 2);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1 + 1));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);
    }

    function test_recoveryFromLateActivation() public {
        _setupReplica();
        // Canonical: C2 stays pending and is destroyed, then C1 switches to C3. A late signature set
        // activates C2 on the replica.
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 1);
        vm.stopPrank();

        // The ahead check runs before the destroy checks, so the owner flag does not settle it.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1 + 1, CTX1)
        );
        _destroyContext(CTX1 + 1, SEED0, QUORUM);

        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 2, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 2, SEED2, N)
        );

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1 + 2);
        assertEq(activeEpochId, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED2);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextAndEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextAndEpochActivatesAndEmits() public {
        _setupReplica();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 epochId = EPOCH_COUNTER_BASE + 5;
        KmsNode[] memory nodes = _makeKmsNodes(2);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);

        // Mirrored context and epoch are immediately active (no quorum replay).
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfigReplica.isValidKmsContext(contextId));
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId).length, 2);
    }

    function test_mirrorKmsContextAndEpochAllowsGap() public {
        _setupReplica();
        // Active context is BASE + 1; mirror a non-contiguous (gapped) context ID.
        uint256 gappedContextId = KMS_CONTEXT_COUNTER_BASE + 10;
        uint256 gappedEpochId = EPOCH_COUNTER_BASE + 10;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            gappedContextId,
            gappedEpochId,
            _makeKmsNodes(2),
            _defaultThresholds()
        );

        assertEq(protocolConfigReplica.getCurrentKmsContextId(), gappedContextId);
        assertTrue(protocolConfigReplica.isValidKmsContext(gappedContextId));
        // The skipped IDs in the gap remain invalid.
        assertFalse(protocolConfigReplica.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 5));
    }

    function test_mirrorKmsContextAndEpochLowerId() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 2, EPOCH1 + 2, SEED1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 1, _nodes(SEED2), _thresholds());

        assertEq(protocolConfigReplica.getCurrentKmsContextId(), CTX1 + 1);
        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), CTX1 + 1);
        _assertMembers(CTX1 + 1, SEED2);
    }

    function test_mirrorKmsContextAndEpochOverridesCompromisedCommittee() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        // A compromised committee forges a switch to committee SEED1.
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, EPOCH1 + 1, _nodes(SEED2), _thresholds());

        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        // The mirrored committee signs the next resharing.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(forgedId, EPOCH1 + 1, EPOCH1 + 2, SEED2, N));
        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH1 + 2);
    }

    function test_recoveryInvalidatesForgedId() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        uint256 forgedEpochId = EPOCH1 + 7;
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, forgedId, SEED1, SEED0, QUORUM),
            _epochSubmission(forgedId, EPOCH1, forgedEpochId, SEED1, N)
        );

        // Recovery: mirror back to the canonical active context, then destroy the forged IDs.
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(forgedId);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);
        vm.stopPrank();
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector,
                forgedEpochId,
                EPOCH1
            )
        );
        _destroyEpoch(forgedEpochId, SEED0, QUORUM);
        assertFalse(protocolConfigReplica.isValidEpochForContext(forgedId, forgedEpochId));

        // The real canonical context under the forged ID cannot land through the submit path.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextIdDestroyed.selector, forgedId));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, forgedId, SEED2, SEED0, QUORUM),
            _epochSubmission(forgedId, EPOCH1, EPOCH1 + 1, SEED2, N)
        );

        // The owner mirrors it, which replaces the forged nodes. The forged epoch stays invalid.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, EPOCH1 + 1, _nodes(SEED2), _thresholds());
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        assertFalse(protocolConfigReplica.isValidEpochForContext(forgedId, forgedEpochId));

        // The mirror cleared the destroyed flag, so a later canonical destruction applies.
        _switch(SEED2, CTX1 + 2, EPOCH1 + 2, SEED1);
        _destroyContext(forgedId, SEED1, QUORUM);
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));
    }

    function test_mirroredDestroyedEpochCanBeDestroyedAgain() public {
        _setupReplica();
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
        _setupReplica();
        uint256 epochId = EPOCH1 + 1;
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(epochId);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, epochId, _nodes(SEED1), _thresholds());
        vm.stopPrank();
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1 + 1, epochId, epochId + 1, SEED1, N));

        _destroyEpoch(epochId, SEED1, QUORUM);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1 + 1, epochId));
    }

    function test_revertMirrorKmsContextAndEpochInvalidThresholds() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "mpc", 5, 4));
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            CTX1 + 1,
            EPOCH1 + 1,
            _nodes(SEED1),
            KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 5})
        );
    }

    function test_submitSwitchReplacesForgedStoredId() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());

        _switch(SEED0, forgedId, EPOCH1 + 1, SEED2);
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), forgedId);
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
    }

    function test_revertMirrorKmsContextAndEpochNotOwner() public {
        _setupReplica();
        KmsNode[] memory nodes = _makeKmsNodes(2);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            KMS_CONTEXT_COUNTER_BASE + 5,
            EPOCH_COUNTER_BASE + 5,
            nodes,
            thresholds
        );
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextDestruction / mirrorKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextDestruction() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
    }

    function test_mirrorKmsEpochDestruction() public {
        _setupReplica();
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
        vm.stopPrank();

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_revertMirrorKmsContextDestructionNotOwner() public {
        _setupReplica();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 9);
    }

    function test_revertMirrorKmsEpochDestructionNotOwner() public {
        _setupReplica();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1 + 9);
    }

    function test_revertMirrorKmsContextDestructionActive() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
    }

    function test_revertMirrorKmsEpochDestructionActive() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
    }
}

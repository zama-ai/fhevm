// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

abstract contract ProtocolConfigReplicaTestBase is HostContractsDeployerTestUtils {
    ProtocolConfigReplica internal protocolConfigReplica;

    address internal constant owner = address(456);
    bytes32 internal constant INITIALIZABLE_STORAGE =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;

    /// @dev The canonical ProtocolConfig the replica verifies signatures for. The test chain is 31337.
    uint64 internal constant CANONICAL_CHAIN_ID = 1;
    address internal constant CANONICAL_ADDRESS = address(0xC0FFEE);

    uint256 internal constant CTX1 = KMS_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant EPOCH1 = EPOCH_COUNTER_BASE + 1;

    /// @dev Seed of the bootstrap committee: its keys are those of `_makeKmsNodeParams(N)`.
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
            _toKmsNodes(_makeKmsNodeParams(N), _makeConnectorUrls(N)),
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
        string[] memory connectorUrls = _makeConnectorUrls(N);
        nodes = new KmsNode[](N);
        for (uint256 i = 0; i < N; i++) {
            nodes[i] = KmsNode({
                txSenderAddress: address(uint160(0xA1 + i + seed * 0x10)),
                signerAddress: vm.addr(keys[i]),
                ipAddress: "127.0.0.1",
                storageUrl: "https://s.example.com",
                connectorUrl: connectorUrls[i]
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
            contextId: contextId,
            nodes: _nodes(seed),
            thresholds: _thresholds(),
            signatures: new bytes[](0),
            extraData: EXTRA
        });
        context.signatures = _sign(_contextDigest(previousContextId, context), previousSeed, count);
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
            epochId: epochId,
            epochMaterialHash: MATERIAL,
            signatures: new bytes[](0),
            extraData: EXTRA
        });
        epoch.signatures = _sign(_epochDigest(contextId, previousEpochId, epoch), seed, count);
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

    /// @dev Canonical digest of a switch from `previousContextId` with the fields that `context` carries.
    function _contextDigest(
        uint256 previousContextId,
        KmsContextSubmission memory context
    ) internal view returns (bytes32) {
        return
            _hashContextCreation(
                previousContextId,
                context.contextId,
                keccak256(abi.encode(context.nodes, context.thresholds)),
                context.extraData
            );
    }

    /// @dev Canonical digest of an activation under `contextId` after `previousEpochId` with the
    ///      fields that `epoch` carries.
    function _epochDigest(
        uint256 contextId,
        uint256 previousEpochId,
        KmsEpochSubmission memory epoch
    ) internal view returns (bytes32) {
        return
            _hashEpochActivation(contextId, previousEpochId, epoch.epochId, epoch.epochMaterialHash, epoch.extraData);
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

    function _ids(uint256 id) internal pure returns (uint256[] memory ids) {
        ids = new uint256[](1);
        ids[0] = id;
    }
}

contract ProtocolConfigReplicaTest is ProtocolConfigReplicaTestBase {
    // -----------------------------------------------------------------------
    // Canonical mirror initializer
    // -----------------------------------------------------------------------

    function test_initializeFromCanonicalStoresNodes() public {
        _setupReplica();
        assertEq(
            abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX1)),
            abi.encode(_toKmsNodes(_makeKmsNodeParams(N), _makeConnectorUrls(N)))
        );
    }

    function test_revertInitializeFromCanonicalInvalidEpochId() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();

        address impl = address(new ProtocolConfigReplica());
        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 invalidEpochId = EPOCH_COUNTER_BASE;
        KmsNode[] memory nodes = _toKmsNodes(_makeKmsNodeParams(2), _makeConnectorUrls(2));
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
        KmsNode[] memory nodes = _toKmsNodes(_makeKmsNodeParams(2), _makeConnectorUrls(2));
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

    function _expectInitializeFromCanonicalReverts(uint64 chainId, address canonical) internal {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();

        address impl = address(new ProtocolConfigReplica());
        KmsNode[] memory nodes = _toKmsNodes(_makeKmsNodeParams(2), _makeConnectorUrls(2));
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.InvalidCanonicalDomain.selector, chainId, canonical)
        );
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (CTX1, EPOCH1, nodes, thresholds, chainId, canonical)
            )
        );
    }

    function test_revertInitializeFromCanonicalZeroChainId() public {
        _expectInitializeFromCanonicalReverts(0, CANONICAL_ADDRESS);
    }

    function test_revertInitializeFromCanonicalZeroAddress() public {
        _expectInitializeFromCanonicalReverts(CANONICAL_CHAIN_ID, address(0));
    }

    function _expectReinitializeV4Reverts(uint64 chainId, address canonical) internal {
        ProtocolConfig pc = _setupCanonical();
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(4)));
        address replicaImpl = address(new ProtocolConfigReplica());

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.InvalidCanonicalDomain.selector, chainId, canonical)
        );
        pc.upgradeToAndCall(replicaImpl, abi.encodeCall(ProtocolConfigReplica.reinitializeV4, (chainId, canonical)));
    }

    function test_revertReinitializeV4ZeroChainId() public {
        _expectReinitializeV4Reverts(0, CANONICAL_ADDRESS);
    }

    function test_revertReinitializeV4ZeroAddress() public {
        _expectReinitializeV4Reverts(CANONICAL_CHAIN_ID, address(0));
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
            _toKmsNodes(_makeKmsNodeParams(1), _makeConnectorUrls(1)),
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
                    _toKmsNodes(_makeKmsNodeParams(2), _makeConnectorUrls(2)),
                    _defaultThresholds(),
                    CANONICAL_CHAIN_ID,
                    CANONICAL_ADDRESS
                )
            )
        );
    }

    function test_reinitializeV4OnUpgradeFromProtocolConfigKeepsStateAndMirrors() public {
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

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, epochId + 1);
        (newContextId, newEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(newContextId, contextId);
        assertEq(newEpochId, epochId + 1);
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

        _expectNotInCommittee(_epochDigest(CTX1, EPOCH1, epoch), epoch.signatures[0], CTX1);
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsThresholds, KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";

abstract contract ProtocolConfigReplicaTestBase is HostContractsDeployerTestUtils {
    ProtocolConfigReplica internal protocolConfigReplica;

    address internal constant owner = address(456);

    /// @dev The canonical ProtocolConfig the replica verifies signatures for. The test chain is 31337.
    uint64 internal constant CANONICAL_CHAIN_ID = 1;
    address internal constant CANONICAL_ADDRESS = address(0xC0FFEE);

    uint256 internal constant CTX1 = KMS_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant EPOCH1 = EPOCH_COUNTER_BASE + 1;

    /// @dev Seed of the bootstrap committee: its nodes are those of `_makeKmsNodeParams(N)`.
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
    constructor() {
        signingChainId = CANONICAL_CHAIN_ID;
        signingVerifyingContract = CANONICAL_ADDRESS;
    }

    /// @dev Bootstraps a replica on the same active context and epoch as a fresh canonical deploy.
    function _setupReplica() internal {
        _deployACL(owner);
        (protocolConfigReplica, ) = _deployProtocolConfigReplica(
            owner,
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS
        );
    }

    function _thresholds() internal pure returns (KmsThresholds memory) {
        return KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 1});
    }

    /// @dev The first `n` signer keys of committee `seed`. Seed 0 is the bootstrap committee.
    function _keys(uint256 seed, uint256 n) internal pure returns (uint256[] memory keys) {
        keys = new uint256[](n);
        for (uint256 i = 0; i < n; i++) {
            keys[i] = _defaultSignerPk(i) + seed;
        }
    }

    function _nodes(uint256 seed) internal pure returns (KmsNode[] memory) {
        return _nodes(seed, N);
    }

    /// @dev Committee `seed` with `n` nodes.
    function _nodes(uint256 seed, uint256 n) internal pure returns (KmsNode[] memory nodes) {
        uint256[] memory keys = _keys(seed, n);
        nodes = _toKmsNodes(_makeKmsNodeParams(n));
        for (uint256 i = 0; i < n; i++) {
            nodes[i].txSenderAddress = address(uint160(0xA1 + i + seed * 0x10));
            nodes[i].signerAddress = vm.addr(keys[i]);
        }
    }

    function _signers(uint256 seed) internal pure returns (address[] memory signers) {
        uint256[] memory keys = _keys(seed, N);
        signers = new address[](N);
        for (uint256 i = 0; i < N; i++) {
            signers[i] = vm.addr(keys[i]);
        }
    }

    /// @dev Signs `digest` with the first `count` keys of committee `seed`.
    function _sign(bytes32 digest, uint256 seed, uint256 count) internal pure returns (bytes[] memory signatures) {
        uint256[] memory keys = _keys(seed, count);
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
    ) internal view returns (KmsContextSubmission memory) {
        return _ctxSubmission(previousContextId, contextId, _nodes(seed), _thresholds(), previousSeed, count);
    }

    /// @dev A switch from `previousContextId` to `nodes` and `thresholds` under `contextId`, signed by
    ///      `count` signers of committee `previousSeed`.
    function _ctxSubmission(
        uint256 previousContextId,
        uint256 contextId,
        KmsNode[] memory nodes,
        KmsThresholds memory thresholds,
        uint256 previousSeed,
        uint256 count
    ) internal view returns (KmsContextSubmission memory context) {
        context = KmsContextSubmission({
            contextId: contextId,
            nodes: nodes,
            thresholds: thresholds,
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

    /// @dev The switch from the bootstrap pair to committee SEED1 under `CTX1 + 1` and `EPOCH1 + 1`.
    function _defaultSwitch() internal view returns (KmsContextSubmission memory, KmsEpochSubmission memory) {
        return (
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function _assertActive(uint256 contextId, uint256 epochId) internal view {
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
    }

    /// @dev Checks that `contextId` stores exactly the nodes of committee `seed`.
    function _assertMembers(uint256 contextId, uint256 seed) internal view {
        assertEq(
            keccak256(abi.encode(protocolConfigReplica.getKmsNodesForContext(contextId))),
            keccak256(abi.encode(_nodes(seed)))
        );
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

    /// @dev Expects the switch to revert on its first context signature, checked against the active committee.
    function _assertContextRejected(KmsContextSubmission memory context, KmsEpochSubmission memory epoch) internal {
        uint256 activeContextId = protocolConfigReplica.getCurrentKmsContextId();
        _expectNotInCommittee(_contextDigest(activeContextId, context), context.signatures[0], activeContextId);
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    /// @dev Expects the switch to revert on its first epoch signature, checked against the new committee.
    function _assertEpochRejected(KmsContextSubmission memory context, KmsEpochSubmission memory epoch) internal {
        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        _expectNotInCommittee(
            _epochDigest(context.contextId, activeEpochId, epoch),
            epoch.signatures[0],
            context.contextId
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    /// @dev Expects the resharing to revert on its first signature, checked against the active committee.
    function _assertResharingRejected(KmsEpochSubmission memory epoch) internal {
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        _expectNotInCommittee(
            _epochDigest(activeContextId, activeEpochId, epoch),
            epoch.signatures[0],
            activeContextId
        );
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    /// @dev Pranks a non-owner and expects the next call to revert with `NotHostOwner`.
    function _expectNotOwner() internal {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
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
}

/// @dev Starts each test from the replica that `_setupReplica` bootstraps.
abstract contract ProtocolConfigReplicaFixture is ProtocolConfigReplicaTestBase {
    function setUp() public virtual {
        _setupReplica();
    }
}

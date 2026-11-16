// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {ProtocolConfigTestBase} from "../ProtocolConfigTestBase.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsThresholds, KmsNode, KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";

abstract contract ProtocolConfigReplicaTestBase is ProtocolConfigTestBase {
    ProtocolConfigReplica internal protocolConfigReplica;

    address internal constant owner = address(456);

    /// @dev The canonical ProtocolConfig the replica verifies signatures for. The test chain is 31337.
    uint64 internal constant CANONICAL_CHAIN_ID = 1;
    address internal constant CANONICAL_ADDRESS = address(0xC0FFEE);

    uint256 internal constant CTX1 = KMS_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant CTX2 = KMS_CONTEXT_COUNTER_BASE + 2;
    uint256 internal constant CTX3 = KMS_CONTEXT_COUNTER_BASE + 3;
    uint256 internal constant CTX4 = KMS_CONTEXT_COUNTER_BASE + 4;
    uint256 internal constant EPOCH1 = EPOCH_COUNTER_BASE + 1;
    uint256 internal constant EPOCH2 = EPOCH_COUNTER_BASE + 2;
    uint256 internal constant EPOCH3 = EPOCH_COUNTER_BASE + 3;
    uint256 internal constant EPOCH4 = EPOCH_COUNTER_BASE + 4;

    bytes32 internal constant MATERIAL = keccak256("epoch material");
    bytes internal constant EXTRA = hex"c0de";

    /// @dev Every committee has 4 nodes and MPC threshold 1, so the `n - t` quorum is 3.
    uint256 internal constant N = 4;
    uint256 internal constant QUORUM = 3;

    /// @dev Domain that the replica verifies signatures under, not the replica's own.
    function _canonicalDomainSeparator() internal pure returns (bytes32) {
        return _computeProtocolConfigDomainSeparator(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
    }

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

    /// @dev Bootstraps a replica on the same active context and epoch as a fresh canonical deploy.
    function _setupReplica() internal {
        _deployACL(owner);
        (protocolConfigReplica, ) = _deployProtocolConfigReplica(
            owner,
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            CANONICAL_ADDRESS,
            CANONICAL_CHAIN_ID
        );
    }

    function _thresholds() internal pure returns (KmsThresholds memory) {
        return KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 1});
    }

    function _thresholdsOf(IProtocolConfigBase pc, uint256 contextId) internal view returns (KmsThresholds memory) {
        return
            KmsThresholds({
                publicDecryption: pc.getPublicDecryptionThresholdForContext(contextId),
                userDecryption: pc.getUserDecryptionThresholdForContext(contextId),
                kmsGen: pc.getKmsGenThresholdForContext(contextId),
                mpc: pc.getMpcThresholdForContext(contextId)
            });
    }

    /// @dev Committee `seed` with `n` nodes, as the params that `defineNewKmsContextAndEpoch` takes.
    function _nodeParams(uint256 seed, uint256 n) internal pure returns (KmsNodeParams[] memory params) {
        uint256[] memory keys = _keys(seed, n);
        params = _makeKmsNodeParams(n);
        for (uint256 i = 0; i < n; i++) {
            params[i].txSenderAddress = address(uint160(0xA1 + i + seed * 0x10));
            params[i].signerAddress = vm.addr(keys[i]);
        }
    }

    function _nodes(uint256 seed, uint256 n) internal pure returns (KmsNode[] memory) {
        return _toKmsNodes(_nodeParams(seed, n));
    }

    function _nodes(uint256 seed) internal pure returns (KmsNode[] memory) {
        return _nodes(seed, N);
    }

    function _signers(uint256 seed) internal pure returns (address[] memory signers) {
        uint256[] memory keys = _keys(seed, N);
        signers = new address[](N);
        for (uint256 i = 0; i < N; i++) {
            signers[i] = vm.addr(keys[i]);
        }
    }

    function _sign(bytes32 digest, uint256[] memory keys) internal pure returns (bytes[] memory signatures) {
        signatures = new bytes[](keys.length);
        for (uint256 i = 0; i < keys.length; i++) {
            signatures[i] = _computeSignature(keys[i], digest);
        }
    }

    /// @dev Signs `digest` with the first `count` keys of committee `seed`.
    function _sign(bytes32 digest, uint256 seed, uint256 count) internal pure returns (bytes[] memory) {
        return _sign(digest, _keys(seed, count));
    }

    /// @dev A switch from `previousContextId` to committee `seed` under `contextId`, signed by
    ///      `count` signers of committee `previousSeed`.
    function _contextSubmission(
        uint256 previousContextId,
        uint256 contextId,
        uint256 seed,
        uint256 previousSeed,
        uint256 count
    ) internal pure returns (KmsContextSubmission memory) {
        return _contextSubmission(previousContextId, contextId, _nodes(seed), _thresholds(), previousSeed, count);
    }

    /// @dev A switch from `previousContextId` to `nodes` and `thresholds` under `contextId`, signed by
    ///      `count` signers of committee `previousSeed`.
    function _contextSubmission(
        uint256 previousContextId,
        uint256 contextId,
        KmsNode[] memory nodes,
        KmsThresholds memory thresholds,
        uint256 previousSeed,
        uint256 count
    ) internal pure returns (KmsContextSubmission memory context) {
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
    ) internal pure returns (KmsEpochSubmission memory) {
        return _epochSubmission(contextId, previousEpochId, epochId, _keys(seed, count));
    }

    /// @dev An activation of `epochId` under `contextId`, signed with `keys`.
    function _epochSubmission(
        uint256 contextId,
        uint256 previousEpochId,
        uint256 epochId,
        uint256[] memory keys
    ) internal pure returns (KmsEpochSubmission memory epoch) {
        epoch = KmsEpochSubmission({
            epochId: epochId,
            epochMaterialHash: MATERIAL,
            signatures: new bytes[](0),
            extraData: EXTRA
        });
        epoch.signatures = _sign(_epochDigest(contextId, previousEpochId, epoch), keys);
    }

    /// @dev Applies a fully signed switch from the active pair to committee `seed`.
    function _switch(uint256 previousSeed, uint256 contextId, uint256 epochId, uint256 seed) internal {
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        protocolConfigReplica.submitKmsContextAndEpoch(
            _contextSubmission(activeContextId, contextId, seed, previousSeed, QUORUM),
            _epochSubmission(contextId, activeEpochId, epochId, seed, N)
        );
    }

    /// @dev The switch from the bootstrap pair to committee SEED1 under `CTX2` and `EPOCH2`.
    function _defaultSwitch() internal pure returns (KmsContextSubmission memory, KmsEpochSubmission memory) {
        return (_contextSubmission(CTX1, CTX2, SEED1, SEED0, QUORUM), _epochSubmission(CTX2, EPOCH1, EPOCH2, SEED1, N));
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

    /// @dev Canonical digest of a destruction of context `contextId`.
    function _contextDestructionDigest(uint256 contextId, bytes memory extraData) internal pure returns (bytes32) {
        return _hashContextDestructionWithDomain(_canonicalDomainSeparator(), contextId, extraData);
    }

    /// @dev Canonical digest of a destruction of epoch `epochId`.
    function _epochDestructionDigest(uint256 epochId, bytes memory extraData) internal pure returns (bytes32) {
        return _hashEpochDestructionWithDomain(_canonicalDomainSeparator(), epochId, extraData);
    }

    /// @dev A destruction of context `contextId`, signed by the first `count` signers of committee `seed`.
    function _contextDestructionSignatures(
        uint256 contextId,
        bytes memory extraData,
        uint256 seed,
        uint256 count
    ) internal pure returns (bytes[] memory) {
        return _sign(_contextDestructionDigest(contextId, extraData), seed, count);
    }

    /// @dev A destruction of epoch `epochId`, signed by the first `count` signers of committee `seed`.
    function _epochDestructionSignatures(
        uint256 epochId,
        bytes memory extraData,
        uint256 seed,
        uint256 count
    ) internal pure returns (bytes[] memory) {
        return _sign(_epochDestructionDigest(epochId, extraData), seed, count);
    }

    function _submitContextDestruction(uint256 contextId, uint256 seed, uint256 count) internal {
        protocolConfigReplica.submitKmsContextDestruction(
            contextId,
            _contextDestructionSignatures(contextId, EXTRA, seed, count),
            EXTRA
        );
    }

    function _submitEpochDestruction(uint256 epochId, uint256 seed, uint256 count) internal {
        protocolConfigReplica.submitKmsEpochDestruction(
            epochId,
            _epochDestructionSignatures(epochId, EXTRA, seed, count),
            EXTRA
        );
    }

    /// @dev Canonical digest of a switch from `previousContextId` with the fields that `context` carries.
    function _contextDigest(
        uint256 previousContextId,
        KmsContextSubmission memory context
    ) internal pure returns (bytes32) {
        return
            _hashContextCreationWithDomain(
                _canonicalDomainSeparator(),
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
    ) internal pure returns (bytes32) {
        return
            _hashEpochActivationWithDomain(
                _canonicalDomainSeparator(),
                contextId,
                previousEpochId,
                epoch.epochId,
                epoch.epochMaterialHash,
                epoch.extraData
            );
    }

    /// @dev Expects the next call to revert with `KmsSignerNotInCommittee` for the signer that
    ///      `signature` recovers to over `digest`, checked against the committee of `contextId`.
    function _expectSignerNotInCommittee(bytes32 digest, bytes memory signature, uint256 contextId) internal {
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsSignerNotInCommittee.selector,
                ECDSA.recover(digest, signature),
                contextId
            )
        );
    }

    /// @dev Expects the switch to revert on its first context signature, checked against the active committee.
    function _assertSwitchContextRejected(
        KmsContextSubmission memory context,
        KmsEpochSubmission memory epoch
    ) internal {
        (uint256 activeContextId, ) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        _expectSignerNotInCommittee(_contextDigest(activeContextId, context), context.signatures[0], activeContextId);
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);
    }

    /// @dev Expects the switch to revert on its first epoch signature, checked against the committee of
    ///      `context.contextId`.
    function _assertSwitchEpochRejected(KmsContextSubmission memory context, KmsEpochSubmission memory epoch) internal {
        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        _expectSignerNotInCommittee(
            _epochDigest(context.contextId, activeEpochId, epoch),
            epoch.signatures[0],
            context.contextId
        );
        protocolConfigReplica.submitKmsContextAndEpoch(context, epoch);
    }

    /// @dev Expects the resharing to revert on its first signature, checked against the active committee.
    function _assertResharingRejected(KmsEpochSubmission memory epoch) internal {
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        _expectSignerNotInCommittee(
            _epochDigest(activeContextId, activeEpochId, epoch),
            epoch.signatures[0],
            activeContextId
        );
        protocolConfigReplica.submitKmsEpoch(epoch);
    }

    function _expectNotOwner() internal {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
    }
}

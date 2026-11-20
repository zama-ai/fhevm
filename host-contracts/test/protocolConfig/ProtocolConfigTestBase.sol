// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {KmsNode, KmsThresholds} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {ProtocolConfigV014LayoutExample} from "@fhevm-host-contracts/examples/ProtocolConfigV014LayoutExample.sol";
import {ProtocolConfigV014StateWriter} from "./ProtocolConfigV014StateWriter.sol";

/// @dev Helpers shared by the ProtocolConfig and ProtocolConfigReplica suites only.
abstract contract ProtocolConfigTestBase is HostContractsDeployerTestUtils {
    bytes32 internal constant EIP712_CONTEXT_DESTRUCTION_TYPE_HASH =
        keccak256("ContextDestructionConfirmation(uint256 destroyedContextId,bytes extraData)");
    bytes32 internal constant EIP712_EPOCH_DESTRUCTION_TYPE_HASH =
        keccak256("EpochDestructionConfirmation(uint256 destroyedEpochId,bytes extraData)");
    bytes32 internal constant PROTOCOL_CONFIG_STORAGE_LOCATION =
        0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;
    /// @dev OZ Initializable storage slot, where the initialized version lives.
    bytes32 internal constant INITIALIZABLE_STORAGE =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;

    /// @dev Seed of the bootstrap committee: its nodes are those of `_makeKmsNodeParams`.
    uint256 internal constant SEED0 = 0;
    uint256 internal constant SEED1 = 1;
    uint256 internal constant SEED2 = 2;

    function _pinToV014InitializedVersion() internal {
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(3)));
    }

    /// @dev Deploys a proxy at the v0.14 initialized version whose implementation writes the v0.14 storage layout.
    function _deployV014ProtocolConfig(address owner) internal returns (ProtocolConfigV014StateWriter) {
        _deployEmptyProtocolConfigProxy();
        address writer = address(new ProtocolConfigV014StateWriter());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            writer,
            abi.encodeCall(ProtocolConfigV014LayoutExample.initializeFromEmptyProxy, ())
        );
        return ProtocolConfigV014StateWriter(protocolConfigAdd);
    }

    /// @dev Deploys a proxy that holds the active context `contextId` in the v0.14 storage layout, at the
    ///      v0.14 initialized version.
    function _deployV014ProtocolConfig(
        address owner,
        uint256 contextId,
        uint256 epochId,
        KmsNode[] memory nodes,
        KmsThresholds memory thresholds
    ) internal {
        _deployV014ProtocolConfig(owner).storeActiveContext(contextId, epochId, _toV014Nodes(nodes), thresholds);
    }

    /// @dev `nodes` without `connectorUrl`, which v0.14 does not have.
    function _toV014Nodes(
        KmsNode[] memory nodes
    ) internal pure returns (ProtocolConfigV014LayoutExample.KmsNode[] memory v014Nodes) {
        v014Nodes = new ProtocolConfigV014LayoutExample.KmsNode[](nodes.length);
        for (uint256 i = 0; i < nodes.length; i++) {
            v014Nodes[i] = ProtocolConfigV014LayoutExample.KmsNode({
                txSenderAddress: nodes[i].txSenderAddress,
                signerAddress: nodes[i].signerAddress,
                ipAddress: nodes[i].ipAddress,
                storageUrl: nodes[i].storageUrl
            });
        }
    }

    /// @dev `nodes` in reverse order, with an empty `connectorUrl`. Even indexes get a short `ipAddress` and a
    ///      `storageUrl` of 32 bytes or more. Odd indexes get the opposite. A string of 32 bytes or more keeps
    ///      its data outside its header slot.
    function _v014Fixture(KmsNode[] memory nodes) internal pure returns (KmsNode[] memory fixture) {
        fixture = new KmsNode[](nodes.length);
        for (uint256 i = 0; i < nodes.length; i++) {
            fixture[i] = nodes[nodes.length - 1 - i];
            string memory index = vm.toString(i);
            if (i % 2 == 0) {
                fixture[i].ipAddress = string.concat("10.0.0.", index);
                fixture[i].storageUrl = string.concat(
                    "https://kms-storage-",
                    index,
                    ".s3.eu-west-1.amazonaws.com/public-prefix"
                );
            } else {
                fixture[i].ipAddress = string.concat("kms-node-", index, ".mpc.internal.example.org");
                fixture[i].storageUrl = string.concat("s3://b", index);
            }
            fixture[i].connectorUrl = "";
        }
    }

    /// @dev Slot of the `kmsNodesForContext[contextId]` array, which holds its length.
    function _kmsNodesSlot(uint256 contextId) internal pure returns (bytes32) {
        return keccak256(abi.encode(contextId, uint256(PROTOCOL_CONFIG_STORAGE_LOCATION) + 1));
    }

    /// @dev The first `n` signer keys of committee `seed`. Seed 0 is the bootstrap committee.
    function _keys(uint256 seed, uint256 n) internal pure returns (uint256[] memory keys) {
        keys = new uint256[](n);
        for (uint256 i = 0; i < n; i++) {
            keys[i] = _defaultSignerPk(i) + seed;
        }
    }

    function _hashContextDestructionWithDomain(
        bytes32 domainSeparator,
        uint256 destroyedContextId,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return
            MessageHashUtils.toTypedDataHash(
                domainSeparator,
                keccak256(abi.encode(EIP712_CONTEXT_DESTRUCTION_TYPE_HASH, destroyedContextId, keccak256(extraData)))
            );
    }

    function _hashEpochDestructionWithDomain(
        bytes32 domainSeparator,
        uint256 destroyedEpochId,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return
            MessageHashUtils.toTypedDataHash(
                domainSeparator,
                keccak256(abi.encode(EIP712_EPOCH_DESTRUCTION_TYPE_HASH, destroyedEpochId, keccak256(extraData)))
            );
    }

    function _hashContextDestruction(
        uint256 destroyedContextId,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashContextDestructionWithDomain(_localDomainSeparator(), destroyedContextId, extraData);
    }

    function _hashEpochDestruction(uint256 destroyedEpochId, bytes memory extraData) internal view returns (bytes32) {
        return _hashEpochDestructionWithDomain(_localDomainSeparator(), destroyedEpochId, extraData);
    }

    function _signContextDestruction(
        uint256 destroyedContextId,
        uint256 pk,
        bytes memory extraData
    ) internal view returns (bytes memory) {
        bytes32 digest = _hashContextDestruction(destroyedContextId, extraData);
        return _computeSignature(pk, digest);
    }

    function _signEpochDestruction(
        uint256 destroyedEpochId,
        uint256 pk,
        bytes memory extraData
    ) internal view returns (bytes memory) {
        bytes32 digest = _hashEpochDestruction(destroyedEpochId, extraData);
        return _computeSignature(pk, digest);
    }

    function _confirmContextDestruction(uint256 destroyedContextId, uint256 pk, bytes memory extraData) internal {
        bytes memory signature = _signContextDestruction(destroyedContextId, pk, extraData);
        protocolConfig.confirmKmsContextDestruction(destroyedContextId, signature, extraData);
    }

    function _confirmEpochDestruction(uint256 destroyedEpochId, uint256 pk, bytes memory extraData) internal {
        bytes memory signature = _signEpochDestruction(destroyedEpochId, pk, extraData);
        protocolConfig.confirmKmsEpochDestruction(destroyedEpochId, signature, extraData);
    }
}

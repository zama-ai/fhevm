// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

/// @dev Helpers shared by the ProtocolConfig and ProtocolConfigReplica suites only.
abstract contract ProtocolConfigTestBase is HostContractsDeployerTestUtils {
    bytes32 internal constant EIP712_CONTEXT_DESTRUCTION_TYPE_HASH =
        keccak256("ContextDestructionConfirmation(uint256 destroyedContextId,bytes extraData)");
    bytes32 internal constant EIP712_EPOCH_DESTRUCTION_TYPE_HASH =
        keccak256("EpochDestructionConfirmation(uint256 destroyedEpochId,bytes extraData)");
    /// @dev OZ Initializable storage slot, where the initialized version lives.
    bytes32 internal constant INITIALIZABLE_STORAGE =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;

    /// @dev Seed of the bootstrap committee: its nodes are those of `_makeKmsNodeParams`.
    uint256 internal constant SEED0 = 0;
    /// @dev Seeds of the other committees.
    uint256 internal constant SEED1 = 1;
    uint256 internal constant SEED2 = 2;

    /// @dev Pins the proxy at the initialized version of the v0.14 ProtocolConfig, which is 3.
    function _pinToV014InitializedVersion() internal {
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(3)));
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

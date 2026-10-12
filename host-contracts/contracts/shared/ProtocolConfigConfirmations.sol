// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNode, KmsThresholds} from "./Structs.sol";

/**
 * @title   ProtocolConfigConfirmations library
 * @notice  EIP-712 type hashes and struct hashes of the four KMS lifecycle confirmations.
 * @dev     The caller wraps each struct hash with its own EIP-712 domain separator.
 */
library ProtocolConfigConfirmations {
    bytes32 internal constant CONTEXT_CREATION_TYPE_HASH =
        keccak256(
            "ContextCreationConfirmation(uint256 previousContextId,uint256 newContextId,bytes32 nodeConfigHash,bytes extraData)"
        );

    bytes32 internal constant EPOCH_ACTIVATION_TYPE_HASH =
        keccak256(
            "EpochActivationConfirmation(uint256 contextId,uint256 previousEpochId,uint256 epochId,bytes32 epochMaterialHash,bytes extraData)"
        );

    bytes32 internal constant CONTEXT_DESTRUCTION_TYPE_HASH =
        keccak256(
            "ContextDestructionConfirmation(uint256 destroyedContextId,uint256[] destroyedEpochIds,bytes extraData)"
        );

    bytes32 internal constant EPOCH_DESTRUCTION_TYPE_HASH =
        keccak256("EpochDestructionConfirmation(uint256 destroyedEpochId,bytes extraData)");

    /// @notice Returns the nodeConfigHash signed in ContextCreationConfirmation.
    function nodeConfigHash(KmsNode[] memory nodes, KmsThresholds memory thresholds) internal pure returns (bytes32) {
        return keccak256(abi.encode(nodes, thresholds));
    }

    function contextCreationStructHash(
        uint256 previousContextId,
        uint256 newContextId,
        bytes32 contextNodeConfigHash,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    CONTEXT_CREATION_TYPE_HASH,
                    previousContextId,
                    newContextId,
                    contextNodeConfigHash,
                    keccak256(extraData)
                )
            );
    }

    function epochActivationStructHash(
        uint256 contextId,
        uint256 previousEpochId,
        uint256 epochId,
        bytes32 epochMaterialHash,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EPOCH_ACTIVATION_TYPE_HASH,
                    contextId,
                    previousEpochId,
                    epochId,
                    epochMaterialHash,
                    keccak256(extraData)
                )
            );
    }

    function contextDestructionStructHash(
        uint256 destroyedContextId,
        uint256[] memory destroyedEpochIds,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    CONTEXT_DESTRUCTION_TYPE_HASH,
                    destroyedContextId,
                    keccak256(abi.encodePacked(destroyedEpochIds)),
                    keccak256(extraData)
                )
            );
    }

    function epochDestructionStructHash(
        uint256 destroyedEpochId,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return keccak256(abi.encode(EPOCH_DESTRUCTION_TYPE_HASH, destroyedEpochId, keccak256(extraData)));
    }
}

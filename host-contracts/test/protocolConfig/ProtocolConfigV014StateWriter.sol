// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsThresholds} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {ProtocolConfigV014LayoutExample} from "@fhevm-host-contracts/examples/ProtocolConfigV014LayoutExample.sol";

/// @dev Writes KMS contexts and epochs into the v0.14 storage layout, as v0.14 ProtocolConfig does.
contract ProtocolConfigV014StateWriter is ProtocolConfigV014LayoutExample {
    /// @dev Stores a committee as v0.14 `_storeKmsContext` does, then sets the context state.
    ///      A v0.14 destruction kept the committee and set the state to None.
    function storeContext(
        uint256 contextId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds,
        ContextState state
    ) public {
        ProtocolConfigStorage storage $ = _storage();
        $.currentKmsContextId = contextId;
        for (uint256 i = 0; i < nodes.length; i++) {
            $.kmsNodesForContext[contextId].push(nodes[i]);
            $.isKmsTxSenderForContext[contextId][nodes[i].txSenderAddress] = true;
            $.isKmsSignerForContext[contextId][nodes[i].signerAddress] = true;
            $.kmsNodeByTxSenderForContext[contextId][nodes[i].txSenderAddress] = nodes[i];
            $.kmsSignerAddressesForContext[contextId].push(nodes[i].signerAddress);
        }
        $.publicDecryptionThresholdForContext[contextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[contextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[contextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[contextId] = thresholds.mpc;
        $.contextState[contextId] = state;
    }

    /// @dev Stores an epoch of `contextId`. An Active epoch also makes the pair the latest active one.
    function storeEpoch(uint256 epochId, uint256 contextId, EpochState state) public {
        ProtocolConfigStorage storage $ = _storage();
        $.epochCounter = epochId;
        $.epochState[epochId] = state;
        $.contextForEpoch[epochId] = contextId;
        if (state == EpochState.Active) {
            $.latestActiveKmsContextId = contextId;
            $.latestActiveEpochId = epochId;
        }
    }

    /// @dev Stores an active context and epoch, as v0.14 `_storeAndActivateKmsContextAndEpoch` does.
    function storeActiveContext(
        uint256 contextId,
        uint256 epochId,
        KmsNode[] calldata nodes,
        KmsThresholds calldata thresholds
    ) external {
        storeContext(contextId, nodes, thresholds, ContextState.Active);
        storeEpoch(epochId, contextId, EpochState.Active);
    }

    function _storage() private pure returns (ProtocolConfigStorage storage $) {
        assembly {
            $.slot := 0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200
        }
    }
}

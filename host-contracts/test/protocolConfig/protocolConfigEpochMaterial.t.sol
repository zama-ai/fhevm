// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigTestBase} from "./protocolConfig.t.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE, KEY_COUNTER_BASE, CRS_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

/// @dev Epoch activation with two key results and two CRS results.
contract ProtocolConfigEpochMaterialTest is ProtocolConfigTestBase {
    uint256 internal constant contextId = KMS_CONTEXT_COUNTER_BASE + 1;
    uint256 internal constant epochId = EPOCH_COUNTER_BASE + 2;

    function setUp() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    /// @dev Entry i has key id KEY_COUNTER_BASE + 1 + i and CRS id CRS_COUNTER_BASE + 1 + i.
    function _buildTwoEpochResults()
        internal
        pure
        returns (IProtocolConfig.EpochKeyResult[] memory keys, IProtocolConfig.EpochCrsResult[] memory crsList)
    {
        keys = new IProtocolConfig.EpochKeyResult[](2);
        crsList = new IProtocolConfig.EpochCrsResult[](2);
        IProtocolConfig.EpochKeyResult[] memory oneKey;
        IProtocolConfig.EpochCrsResult[] memory oneCrs;
        (oneKey, oneCrs) = _buildEpochResults(KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        keys[0] = oneKey[0];
        crsList[0] = oneCrs[0];
        (oneKey, oneCrs) = _buildEpochResults(KEY_COUNTER_BASE + 2, CRS_COUNTER_BASE + 2);
        keys[1] = oneKey[0];
        crsList[1] = oneCrs[0];
    }

    function _submit(
        uint256 pk,
        IProtocolConfig.EpochKeyResult[] memory keys,
        IProtocolConfig.EpochCrsResult[] memory crsList
    ) internal {
        protocolConfig.confirmEpochActivation(
            epochId,
            keys,
            crsList,
            _signEpochActivation(contextId, epochId, pk, keys, crsList, ""),
            ""
        );
    }

    function _activeEpochId() internal view returns (uint256 activeEpochId) {
        (, activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
    }

    /// @dev Both signers submit the same order. Each confirmation event carries the epochMaterialHash
    ///      computed by the test helper, and the epoch activates.
    function test_confirmEpochActivationTwoEntriesSameOrderActivates() public {
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildTwoEpochResults();
        bytes memory signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, "");
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.EpochActivationConfirmation(
            epochId,
            vm.addr(kmsPk0),
            _epochMaterialHash(keys, crsList),
            signature,
            ""
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");

        (keys, crsList) = _buildTwoEpochResults();
        _submit(kmsPk1, keys, crsList);
        assertEq(_activeEpochId(), epochId);
    }

    /// @dev The key order is part of epochMaterialHash, so swapped keys split the vote.
    function test_confirmEpochActivationSwappedKeysDoNotActivate() public {
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildTwoEpochResults();
        _submit(kmsPk0, keys, crsList);

        (keys, crsList) = _buildTwoEpochResults();
        (keys[0], keys[1]) = (keys[1], keys[0]);
        _submit(kmsPk1, keys, crsList);
        assertEq(_activeEpochId(), EPOCH_COUNTER_BASE + 1);
    }

    /// @dev The CRS order is part of epochMaterialHash, so swapped CRS results split the vote.
    function test_confirmEpochActivationSwappedCrsDoNotActivate() public {
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildTwoEpochResults();
        _submit(kmsPk0, keys, crsList);

        (keys, crsList) = _buildTwoEpochResults();
        (crsList[0], crsList[1]) = (crsList[1], crsList[0]);
        _submit(kmsPk1, keys, crsList);
        assertEq(_activeEpochId(), EPOCH_COUNTER_BASE + 1);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {BaseScenarioInvariant} from "../../base/BaseScenarioInvariant.t.sol";

abstract contract SignerGovernanceInvariantBase is BaseScenarioInvariant {
    function _getSigners() internal view virtual returns (address[] memory);
    function _getThreshold() internal view virtual returns (uint256);
    function _isSigner(address signer) internal view virtual returns (bool);
    function _candidateDomainSize() internal view virtual returns (uint256);
    function _candidateAt(uint256 seed) internal view virtual returns (address);

    function _assertThresholdWithinSignerBounds() internal view {
        address[] memory signers = _getSigners();
        uint256 threshold = _getThreshold();
        assertGe(signers.length, 1);
        assertGe(threshold, 1);
        assertLe(threshold, signers.length);
    }

    function _assertSignerSetUniqueAndConsistent() internal view {
        address[] memory signers = _getSigners();
        for (uint256 i = 0; i < signers.length; i++) {
            assertTrue(_isSigner(signers[i]));
            for (uint256 j = i + 1; j < signers.length; j++) {
                assertTrue(signers[i] != signers[j]);
            }
        }

        uint256 domain = _candidateDomainSize();
        for (uint256 i = 0; i < domain; i++) {
            address candidate = _candidateAt(i);
            assertEq(_isSigner(candidate), _contains(signers, candidate));
        }
    }
}

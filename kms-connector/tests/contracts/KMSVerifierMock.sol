// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev Minimal mock returning a fixed KMS context ID (1).
contract KMSVerifierMock {
    function getCurrentKmsContextId() external pure returns (uint256) {
        return 1;
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev This contract is a mock of the CiphertextManager contract from the Gateway.
/// source: github.com/zama-ai/gateway-l2/blob/main/contracts/CiphertextManager.sol
contract CiphertextManager {
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) public {}
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev This contract is a mock of the CiphertextCommits contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/CiphertextCommits.sol
contract CiphertextCommits {
    error CoprocessorAlreadyAdded(bytes32 ctHandle, address coprocessorTxSenderAddress);

    event Dummy();

    bool alreadyAddedRevert;

    constructor(bool _alreadyAddedRevert) {
        alreadyAddedRevert = _alreadyAddedRevert;
    }

    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 /* keyId */,
        bytes32 /* ciphertextDigest */,
        bytes32 /* snsCiphertextDigest */
    ) public {
        // Silence compiler warnings about chaning function mutability to view.
        emit Dummy();

        if (alreadyAddedRevert) {
            revert CoprocessorAlreadyAdded(ctHandle, msg.sender);
        }
    }
}

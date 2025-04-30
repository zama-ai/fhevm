// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev This contract is a mock of the CiphertextCommits contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/CiphertextCommits.sol
contract CiphertextCommits {
    error CoprocessorTxSenderAlreadyAdded(address coprocessorTxSenderAddress);

    event AddCiphertextMaterial(
        bytes32 indexed ctHandle,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest,
        address[] coprocessorTxSenderAddresses
    );

    bool alreadyAddedRevert;

    constructor(bool _alreadyAddedRevert) {
        alreadyAddedRevert = _alreadyAddedRevert;
    }

    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 /* keyId */,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) public {
        if (alreadyAddedRevert) {
            revert CoprocessorTxSenderAlreadyAdded(msg.sender);
        }

        emit AddCiphertextMaterial(
            ctHandle,
            ciphertextDigest,
            snsCiphertextDigest,
            new address[](0)
        );
    }
}

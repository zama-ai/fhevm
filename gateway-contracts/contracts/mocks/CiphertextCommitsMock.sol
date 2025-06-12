// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract CiphertextCommitsMock {
    event AddCiphertextMaterial(
        bytes32 indexed ctHandle,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest,
        address[] coprocessorTxSenders
    );

    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external {
        address[] memory coprocessorTxSenders = new address[](1);

        emit AddCiphertextMaterial(ctHandle, ciphertextDigest, snsCiphertextDigest, coprocessorTxSenders);
    }
}

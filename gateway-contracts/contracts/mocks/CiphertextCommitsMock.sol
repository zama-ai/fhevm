// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract CiphertextCommitsMock {
    event AddCiphertextMaterial(
        bytes32 indexed ctHandle,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest,
        uint256 coprocessorContextId
    );

    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external {
        uint256 coprocessorContextId;

        emit AddCiphertextMaterial(ctHandle, ciphertextDigest, snsCiphertextDigest, coprocessorContextId);
    }
}

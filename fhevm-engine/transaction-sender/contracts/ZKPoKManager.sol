// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

contract ZKPoKManager {
    event VerifyProofResponseCalled(uint256, bytes32[], bytes);

    error CoprocessorHasAlreadySigned(uint256 zkProofId, address signer);

    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata handles,
        bytes calldata signature
    ) public {
        emit VerifyProofResponseCalled(zkProofId, handles, signature);
    }
}

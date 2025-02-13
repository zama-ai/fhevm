// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

contract ZKPoKManager {
    event VerifyProofResponseCalled(uint256, bytes32[], bytes);

    error CoprocessorHasAlreadySigned(uint256 zkProofId, address signer);

    bool alreadySignedRevert;
    bool generalRevert;

    constructor(bool _alreadySignedRevert, bool _generalRevert) {
        alreadySignedRevert = _alreadySignedRevert;
        generalRevert = _generalRevert;
    }

    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata handles,
        bytes calldata signature
    ) public {
        if (generalRevert) {
            revert("General revert");
        }

        if (alreadySignedRevert) {
            revert CoprocessorHasAlreadySigned(zkProofId, msg.sender);
        }

        emit VerifyProofResponseCalled(zkProofId, handles, signature);
    }
}

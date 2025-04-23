// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev This contract is a mock of the InputVerification contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/InputVerification.sol
contract InputVerification {
    event VerifyProofResponseCalled(uint256, bytes32[], bytes);
    event RejectProofResponseCalled(uint256);

    error CoprocessorSignerAlreadyResponded(uint256 zkProofId, address signer);

    bool alreadyRespondedRevert;
    bool generalRevert;

    constructor(bool _alreadyRespondedRevert, bool _generalRevert) {
        alreadyRespondedRevert = _alreadyRespondedRevert;
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

        if (alreadyRespondedRevert) {
            revert CoprocessorSignerAlreadyResponded(zkProofId, msg.sender);
        }

        emit VerifyProofResponseCalled(zkProofId, handles, signature);
    }

    function rejectProofResponse(uint256 zkProofId) public {
        if (generalRevert) {
            revert("General revert");
        }

        if (alreadyRespondedRevert) {
            revert CoprocessorSignerAlreadyResponded(zkProofId, msg.sender);
        }

        emit RejectProofResponseCalled(zkProofId);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev This contract is a mock of the InputVerification contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/InputVerification.sol
contract InputVerification {
    event VerifyProofResponseCalled(uint256, bytes32[], bytes);
    event RejectProofResponseCalled(uint256);

    error CoprocessorSignerAlreadyVerified(uint256 zkProofId, address signer);
    error CoprocessorSignerAlreadyRejected(uint256 zkProofId, address signer);

    bool alreadyVerifiedRevert;
    bool alreadyRejectedRevert;
    bool otherRevert;

    constructor(
        bool _alreadyVerifiedRevert,
        bool _alreadyRejectedRevert,
        bool _otherRevert
    ) {
        alreadyVerifiedRevert = _alreadyVerifiedRevert;
        alreadyRejectedRevert = _alreadyRejectedRevert;
        otherRevert = _otherRevert;
    }

    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata handles,
        bytes calldata signature
    ) public {
        if (otherRevert) {
            revert("Other revert");
        }

        if (alreadyVerifiedRevert) {
            revert CoprocessorSignerAlreadyVerified(zkProofId, msg.sender);
        }

        emit VerifyProofResponseCalled(zkProofId, handles, signature);
    }

    function rejectProofResponse(uint256 zkProofId) public {
        if (otherRevert) {
            revert("Other revert");
        }

        if (alreadyRejectedRevert) {
            revert CoprocessorSignerAlreadyRejected(zkProofId, msg.sender);
        }

        emit RejectProofResponseCalled(zkProofId);
    }
}

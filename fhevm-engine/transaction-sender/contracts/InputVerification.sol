// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev This contract is a mock of the InputVerification contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/InputVerification.sol
contract InputVerification {
    event VerifyProofResponse(
        uint256 indexed zkProofId,
        bytes32[] ctHandles,
        bytes[] signatures
    );
    event RejectProofResponse(uint256 indexed zkProofId);

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

        bytes[] memory signatures = new bytes[](1);
        signatures[0] = signature;
        emit VerifyProofResponse(zkProofId, handles, signatures);
    }

    function rejectProofResponse(uint256 zkProofId) public {
        if (otherRevert) {
            revert("Other revert");
        }

        if (alreadyRejectedRevert) {
            revert CoprocessorSignerAlreadyRejected(zkProofId, msg.sender);
        }

        emit RejectProofResponse(zkProofId);
    }
}

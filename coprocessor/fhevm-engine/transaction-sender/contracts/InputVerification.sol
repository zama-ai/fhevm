// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev This contract is a mock of the InputVerification contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/InputVerification.sol
contract InputVerification {
    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);
    event RejectProofResponse(uint256 indexed zkProofId);
    /**
     * @notice Error indicating that the coprocessor has already verified the ZKPoK.
     * @param zkProofId The ID of the ZKPoK.
     * @param txSender The transaction sender address of the coprocessor that has already verified.
     * @param signer The signer address of the coprocessor that has already verified.
     */
    error CoprocessorAlreadyVerified(uint256 zkProofId, address txSender, address signer);

    /**
     * @notice Error indicating that the coprocessor has already rejected the ZKPoK.
     * @param zkProofId The ID of the ZKPoK.
     * @param txSender The transaction sender address of the coprocessor that has already rejected.
     * @param signer The signer address of the coprocessor that has already rejected.
     */
    error CoprocessorAlreadyRejected(uint256 zkProofId, address txSender, address signer);

    bool alreadyVerifiedRevert;
    bool alreadyRejectedRevert;
    bool otherRevert;

    constructor(bool _alreadyVerifiedRevert, bool _alreadyRejectedRevert, bool _otherRevert) {
        alreadyVerifiedRevert = _alreadyVerifiedRevert;
        alreadyRejectedRevert = _alreadyRejectedRevert;
        otherRevert = _otherRevert;
    }

    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata handles,
        bytes calldata signature,
        bytes calldata /* extraData */
    ) public {
        if (otherRevert) {
            revert("Other revert");
        }

        if (alreadyVerifiedRevert) {
            revert CoprocessorAlreadyVerified(zkProofId, msg.sender, msg.sender);
        }

        bytes[] memory signatures = new bytes[](1);
        signatures[0] = signature;
        emit VerifyProofResponse(zkProofId, handles, signatures);
    }

    function rejectProofResponse(uint256 zkProofId, bytes calldata /* extraData */) public {
        if (otherRevert) {
            revert("Other revert");
        }

        if (alreadyRejectedRevert) {
            revert CoprocessorAlreadyRejected(zkProofId, msg.sender, msg.sender);
        }

        emit RejectProofResponse(zkProofId);
    }
}

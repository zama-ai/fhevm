// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev This contract is a mock of the InputVerification contract from the Gateway.
/// source: github.com/zama-ai/fhevm/blob/main/gateway-contracts/contracts/InputVerification.sol
contract InputVerification {
    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof,
        bytes extraData
    );
    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);
    event RejectProofResponse(uint256 indexed zkProofId);

    uint256 zkProofIdCounter = 0;

    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof,
        bytes calldata extraData
    ) public {
        uint256 zkProofId = zkProofIdCounter;
        zkProofIdCounter += 1;
        emit VerifyProofRequest(
            zkProofId,
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithZKProof,
            extraData
        );
    }

    function emitVerifyProofResponse(uint256 zkProofId, bytes32[] calldata ctHandles) public {
        emit VerifyProofResponse(zkProofId, ctHandles, new bytes[](0));
    }

    function emitRejectProofResponse(uint256 zkProofId) public {
        emit RejectProofResponse(zkProofId);
    }
}

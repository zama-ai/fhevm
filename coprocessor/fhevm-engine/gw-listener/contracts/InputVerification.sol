// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev This contract is a mock of the InputVerification contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/InputVerification.sol
contract InputVerification {
    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof
    );

    uint256 zkProofIdCounter = 0;

    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) public {
        uint256 zkProofId = zkProofIdCounter;
        zkProofIdCounter += 1;
        emit VerifyProofRequest(
            zkProofId,
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithZKProof
        );
    }
}

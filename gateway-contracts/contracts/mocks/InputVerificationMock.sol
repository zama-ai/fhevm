// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract InputVerificationMock {
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

    uint256 zkProofIdCounter;

    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof,
        bytes calldata extraData
    ) external {
        zkProofIdCounter++;
        uint256 zkProofId = zkProofIdCounter;

        emit VerifyProofRequest(
            zkProofId,
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithZKProof,
            extraData
        );
    }

    function verifyProofResponse(
        uint256 zkProofId,
        bytes32[] calldata ctHandles,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        bytes[] memory signatures = new bytes[](1);

        emit VerifyProofResponse(zkProofId, ctHandles, signatures);
    }

    function rejectProofResponse(uint256 zkProofId, bytes calldata /* unusedVariable */) external {
        emit RejectProofResponse(zkProofId);
    }
}

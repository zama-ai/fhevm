// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract InputVerificationMock {
    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof
    );

    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);

    event RejectProofResponse(uint256 indexed zkProofId);

    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) public {
        uint256 zkProofId;
        uint256 contractChainId;
        address contractAddress;
        address userAddress;
        bytes memory ciphertextWithZKProof;
        emit VerifyProofRequest(zkProofId, contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    }

    function verifyProofResponse(uint256 zkProofId, bytes32[] calldata ctHandles, bytes calldata signature) public {
        uint256 zkProofId;
        bytes32[] memory ctHandles = new bytes32[](1);
        bytes[] memory signatures = new bytes[](1);
        emit VerifyProofResponse(zkProofId, ctHandles, signatures);
    }

    function rejectProofResponse(uint256 zkProofId) public {
        uint256 zkProofId;
        emit RejectProofResponse(zkProofId);
    }
}

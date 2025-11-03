// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract InputVerificationMock {
    struct CiphertextVerification {
        bytes32[] ctHandles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
        bytes extraData;
    }

    struct ZKProofInput {
        uint256 contractChainId;
        address contractAddress;
        address userAddress;
    }

    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof,
        bytes extraData
    );

    event VerifyProofResponseCall(
        uint256 indexed zkProofId,
        bytes32[] ctHandles,
        bytes signature,
        address coprocessorTxSender,
        bytes extraData
    );

    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);

    event RejectProofResponseCall(uint256 indexed zkProofId, bytes extraData);

    event RejectProofResponse(uint256 indexed zkProofId);

    uint256 zkProofIdCounter = 0;

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
        address coprocessorTxSender;
        bytes[] memory signatures = new bytes[](1);

        emit VerifyProofResponseCall(zkProofId, ctHandles, signature, coprocessorTxSender, extraData);

        emit VerifyProofResponse(zkProofId, ctHandles, signatures);
    }

    function rejectProofResponse(uint256 zkProofId, bytes calldata extraData) external {
        emit RejectProofResponseCall(zkProofId, extraData);

        emit RejectProofResponse(zkProofId);
    }
}

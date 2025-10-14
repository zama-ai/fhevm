// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Enums.sol";

contract InputVerificationMock {
    struct ZKProofInput {
        uint256 contractChainId;
        address contractAddress;
        address userAddress;
    }

    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed coprocessorContextId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof,
        bytes extraData
    );

    event VerifyProofResponse(
        uint256 indexed zkProofId,
        uint256 indexed coprocessorContextId,
        bytes32[] ctHandles,
        bytes[] signatures,
        bytes extraData
    );

    event RejectProofResponse(uint256 indexed zkProofId, bytes extraData);

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
        uint256 coprocessorContextId;

        emit VerifyProofRequest(
            zkProofId,
            coprocessorContextId,
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
        uint256 coprocessorContextId;
        bytes[] memory signatures = new bytes[](1);

        emit VerifyProofResponse(zkProofId, coprocessorContextId, ctHandles, signatures, extraData);
    }

    function rejectProofResponse(uint256 zkProofId, bytes calldata extraData) external {
        emit RejectProofResponse(zkProofId, extraData);
    }
}

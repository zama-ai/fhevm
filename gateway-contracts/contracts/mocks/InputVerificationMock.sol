// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Enums.sol";

contract InputVerificationMock {
    event VerifyProofRequest(
        uint256 indexed zkProofId,
        uint256 indexed contextId,
        uint256 indexed contractChainId,
        address contractAddress,
        address userAddress,
        bytes ciphertextWithZKProof
    );

    event VerifyProofResponse(uint256 indexed zkProofId, bytes32[] ctHandles, bytes[] signatures);

    event RejectProofResponse(uint256 indexed zkProofId);

    uint256 zkProofIdCounter;

    function verifyProofRequest(
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata ciphertextWithZKProof
    ) external {
        zkProofIdCounter++;
        uint256 zkProofId = zkProofIdCounter;
        uint256 contextId;

        emit VerifyProofRequest(
            zkProofId,
            contextId,
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithZKProof
        );
    }

    function verifyProofResponse(uint256 zkProofId, bytes32[] calldata ctHandles, bytes calldata signature) external {
        bytes[] memory signatures = new bytes[](1);

        emit VerifyProofResponse(zkProofId, ctHandles, signatures);
    }

    function rejectProofResponse(uint256 zkProofId) external {
        emit RejectProofResponse(zkProofId);
    }
}

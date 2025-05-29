// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract DecryptionMock {
    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }

    event PublicDecryptionRequest(
        uint256 indexed decryptionId,
        uint256 indexed contextId,
        SnsCiphertextMaterial[] snsCtMaterials
    );

    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures);

    event UserDecryptionRequest(
        uint256 indexed decryptionId,
        uint256 indexed contextId,
        SnsCiphertextMaterial[] snsCtMaterials,
        address userAddress,
        bytes publicKey
    );

    event UserDecryptionResponse(uint256 indexed decryptionId, bytes[] userDecryptedShares, bytes[] signatures);

    uint256 _decryptionRequestCounter;

    function publicDecryptionRequest(bytes32[] calldata ctHandles) external {
        _decryptionRequestCounter++;
        uint256 decryptionId = _decryptionRequestCounter;
        uint256 contextId;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        emit PublicDecryptionRequest(decryptionId, contextId, snsCtMaterials);
    }

    function publicDecryptionResponse(
        uint256 decryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) external {
        bytes[] memory signatures = new bytes[](1);

        emit PublicDecryptionResponse(decryptionId, decryptedResult, signatures);
    }

    function userDecryptionResponse(
        uint256 decryptionId,
        bytes calldata userDecryptedShare,
        bytes calldata signature
    ) external {
        bytes[] memory userDecryptedShares = new bytes[](1);
        bytes[] memory signatures = new bytes[](1);

        emit UserDecryptionResponse(decryptionId, userDecryptedShares, signatures);
    }
}

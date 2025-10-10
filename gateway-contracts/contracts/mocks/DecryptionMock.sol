// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract DecryptionMock {
    struct ContractsInfo {
        uint256 chainId;
        address[] addresses;
    }

    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }

    struct UserDecryptionPayload {
        bytes publicKey;
        bytes32[] ctHandles;
    }

    event PublicDecryptionRequest(uint256 indexed decryptionId, bytes32[] ctHandles, bytes extraData);

    event PublicDecryptionResponse(
        uint256 indexed decryptionId,
        bytes decryptedResult,
        bytes[] signatures,
        bytes extraData
    );

    event UserDecryptionRequest(
        uint256 indexed decryptionId,
        bytes32[] ctHandles,
        address userAddress,
        bytes publicKey,
        bytes extraData
    );

    event UserDecryptionResponse(
        uint256 indexed decryptionId,
        uint256 indexShare,
        bytes userDecryptedShare,
        bytes signature,
        bytes extraData
    );

    event UserDecryptionResponseThresholdReached(uint256 indexed decryptionId);

    uint256 publicDecryptionCounter = 1 << 248;
    uint256 userDecryptionCounter = 2 << 248;

    function publicDecryptionRequest(bytes32[] calldata ctHandles, bytes calldata extraData) external {
        publicDecryptionCounter++;
        uint256 decryptionId = publicDecryptionCounter;

        emit PublicDecryptionRequest(decryptionId, ctHandles, extraData);
    }

    function publicDecryptionResponse(
        uint256 decryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        bytes[] memory signatures = new bytes[](1);

        emit PublicDecryptionResponse(decryptionId, decryptedResult, signatures, extraData);
    }

    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        ContractsInfo calldata contractsInfo,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        userDecryptionCounter++;
        uint256 decryptionId = userDecryptionCounter;
        bytes32[] memory ctHandles = new bytes32[](1);

        emit UserDecryptionRequest(decryptionId, ctHandles, userAddress, publicKey, extraData);
    }

    function userDecryptionResponse(
        uint256 decryptionId,
        bytes calldata userDecryptedShare,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        uint256 indexShare;

        emit UserDecryptionResponse(decryptionId, indexShare, userDecryptedShare, signature, extraData);

        emit UserDecryptionResponseThresholdReached(decryptionId);
    }
}

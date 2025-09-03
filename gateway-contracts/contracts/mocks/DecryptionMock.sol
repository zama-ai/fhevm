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

    struct PublicDecryptVerification {
        bytes32[] ctHandles;
        bytes decryptedResult;
        bytes extraData;
    }

    struct UserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        uint256 contractsChainId;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }

    struct UserDecryptResponseVerification {
        bytes publicKey;
        bytes32[] ctHandles;
        bytes userDecryptedShare;
        bytes extraData;
    }

    struct DelegatedUserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        address delegatorAddress;
        uint256 contractsChainId;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }

    struct UserDecryptionPayload {
        bytes publicKey;
        bytes32[] ctHandles;
    }

    event PublicDecryptionRequest(
        uint256 indexed publicDecryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        bytes extraData
    );

    event PublicDecryptionResponse(
        uint256 indexed decryptionId,
        bytes decryptedResult,
        bytes[] signatures,
        bytes extraData
    );

    event UserDecryptionRequest(
        uint256 indexed userDecryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        address userAddress,
        bytes publicKey,
        bytes extraData
    );

    event UserDecryptionResponse(
        uint256 indexed decryptionId,
        bytes[] userDecryptedShares,
        bytes[] signatures,
        bytes extraData
    );

    uint256 publicDecryptionCounter = 1 << 248;
    uint256 userDecryptionCounter = 2 << 248;

    function publicDecryptionRequest(bytes32[] calldata ctHandles, bytes calldata extraData) external {
        publicDecryptionCounter++;
        uint256 publicDecryptionId = publicDecryptionCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        emit PublicDecryptionRequest(publicDecryptionId, snsCtMaterials, extraData);
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
        uint256 userDecryptionId = userDecryptionCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        emit UserDecryptionRequest(userDecryptionId, snsCtMaterials, userAddress, publicKey, extraData);
    }

    function delegatedUserDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        DelegationAccounts calldata delegationAccounts,
        ContractsInfo calldata contractsInfo,
        bytes calldata publicKey,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        userDecryptionCounter++;
        uint256 userDecryptionId = userDecryptionCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        address userAddress;

        emit UserDecryptionRequest(userDecryptionId, snsCtMaterials, userAddress, publicKey, extraData);
    }

    function userDecryptionResponse(
        uint256 decryptionId,
        bytes calldata userDecryptedShare,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        bytes[] memory userDecryptedShares = new bytes[](1);
        bytes[] memory signatures = new bytes[](1);

        emit UserDecryptionResponse(decryptionId, userDecryptedShares, signatures, extraData);
    }
}

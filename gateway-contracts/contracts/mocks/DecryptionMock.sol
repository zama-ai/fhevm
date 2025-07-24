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

    event PublicDecryptionRequest(
        uint256 indexed decryptionId,
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
        uint256 indexed decryptionId,
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

    uint256 _decryptionRequestCounter;

    function publicDecryptionRequest(bytes32[] calldata ctHandles, bytes calldata extraData) external {
        _decryptionRequestCounter++;
        uint256 decryptionId = _decryptionRequestCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        emit PublicDecryptionRequest(decryptionId, snsCtMaterials, extraData);
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
        _decryptionRequestCounter++;
        uint256 decryptionId = _decryptionRequestCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        emit UserDecryptionRequest(decryptionId, snsCtMaterials, userAddress, publicKey, extraData);
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
        _decryptionRequestCounter++;
        uint256 decryptionId = _decryptionRequestCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        address userAddress;

        emit UserDecryptionRequest(decryptionId, snsCtMaterials, userAddress, publicKey, extraData);
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

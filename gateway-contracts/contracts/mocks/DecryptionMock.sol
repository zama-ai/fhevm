// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract DecryptionMock {
    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }

    event PublicDecryptionRequest(
        uint256 indexed decryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        bytes extraData
    );

    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures);

    event UserDecryptionRequest(
        uint256 indexed decryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        address userAddress,
        bytes publicKey,
        bytes extraData
    );

    event UserDecryptionResponse(uint256 indexed decryptionId, bytes[] userDecryptedShares, bytes[] signatures);

    uint256 _decryptionRequestCounter;

    function publicDecryptionRequest(bytes32[] calldata ctHandles) external {
        _decryptionRequestCounter++;
        uint256 decryptionId = _decryptionRequestCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        bytes memory extraData;

        emit PublicDecryptionRequest(decryptionId, snsCtMaterials, extraData);
    }

    function publicDecryptionResponse(
        uint256 decryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        bytes[] memory signatures = new bytes[](1);

        emit PublicDecryptionResponse(decryptionId, decryptedResult, signatures);
    }

    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        uint256 contractsChainId,
        address[] calldata contractAddresses,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature
    ) external {
        _decryptionRequestCounter++;
        uint256 decryptionId = _decryptionRequestCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        bytes memory extraData;

        emit UserDecryptionRequest(decryptionId, snsCtMaterials, userAddress, publicKey, extraData);
    }

    function delegatedUserDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        DelegationAccounts calldata delegationAccounts,
        uint256 contractsChainId,
        address[] calldata contractAddresses,
        bytes calldata publicKey,
        bytes calldata signature
    ) external {
        _decryptionRequestCounter++;
        uint256 decryptionId = _decryptionRequestCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        address userAddress;
        bytes memory extraData;

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

        emit UserDecryptionResponse(decryptionId, userDecryptedShares, signatures);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";

contract DecryptionMock {
    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }

    event PublicDecryptionRequest(uint256 indexed publicDecryptionId, SnsCiphertextMaterial[] snsCtMaterials);

    event PublicDecryptionResponse(uint256 indexed publicDecryptionId, bytes decryptedResult, bytes[] signatures);

    event UserDecryptionRequest(
        uint256 indexed userDecryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        address userAddress,
        bytes publicKey
    );

    event UserDecryptionResponse(uint256 indexed userDecryptionId, bytes[] reencryptedShares, bytes[] signatures);

    function publicDecryptionRequest(bytes32[] calldata ctHandles) external {
        uint256 publicDecryptionId;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        emit PublicDecryptionRequest(publicDecryptionId, snsCtMaterials);
    }

    function publicDecryptionResponse(
        uint256 publicDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) external {
        uint256 publicDecryptionId;
        bytes memory decryptedResult;
        bytes[] memory signatures = new bytes[](1);
        emit PublicDecryptionResponse(publicDecryptionId, decryptedResult, signatures);
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
        uint256 userDecryptionId;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        address userAddress;
        bytes memory publicKey;
        emit UserDecryptionRequest(userDecryptionId, snsCtMaterials, userAddress, publicKey);
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
        uint256 userDecryptionId;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);
        address userAddress;
        bytes memory publicKey;
        emit UserDecryptionRequest(userDecryptionId, snsCtMaterials, userAddress, publicKey);
    }

    function userDecryptionResponse(
        uint256 userDecryptionId,
        bytes calldata reencryptedShare,
        bytes calldata signature
    ) external {
        uint256 userDecryptionId;
        bytes[] memory reencryptedShares = new bytes[](1);
        bytes[] memory signatures = new bytes[](1);
        emit UserDecryptionResponse(userDecryptionId, reencryptedShares, signatures);
    }
}

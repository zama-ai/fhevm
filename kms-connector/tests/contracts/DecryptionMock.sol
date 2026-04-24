// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// KMS-connector-only fork of `gateway-contracts/contracts/mocks/DecryptionMock.sol`. The upstream
// mock is auto-generated and kept in sync with the production `Decryption.sol` ABI; duplicating it
// here lets us layer test-only extensions (e.g. the RFC016 `userDecryptionRequest_0` overload) that
// the production ABI will not carry until the relayer-sdk deprecation window closes.
//
// The structs below are inlined from `gateway-contracts/contracts/shared/Structs.sol` so this file
// is self-contained and the `forge create --root` invocation does not need to reach into the
// gateway-contracts source tree.
//
// Function selectors depend only on Solidity signatures, so the selectors emitted here match the
// ones in `fhevm_gateway_bindings::decryption::Decryption` — the `DecryptionInstance` binding
// calls dispatch to these mock bodies without any binding regeneration.

struct SnsCiphertextMaterial {
    bytes32 ctHandle;
    uint256 keyId;
    bytes32 snsCiphertextDigest;
    address[] coprocessorTxSenderAddresses;
}

struct CtHandleContractPair {
    bytes32 ctHandle;
    address contractAddress;
}

struct HandleEntry {
    bytes32 handle;
    address contractAddress;
    address ownerAddress;
}

contract DecryptionMock {
    struct ContractsInfo {
        uint256 chainId;
        address[] addresses;
    }

    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }

    struct RequestValiditySeconds {
        uint256 startTimestamp;
        uint256 durationSeconds;
    }

    struct UserDecryptionRequestPayload {
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        RequestValiditySeconds requestValidity;
        bytes extraData;
        bytes signature;
    }

    struct DelegationAccounts {
        address delegatorAddress;
        address delegateAddress;
    }

    struct PublicDecryptVerification {
        bytes32[] ctHandles;
        bytes decryptedResult;
        bytes extraData;
    }

    struct UserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        uint256 startTimestamp;
        uint256 durationDays;
        bytes extraData;
    }

    struct DelegatedUserDecryptRequestVerification {
        bytes publicKey;
        address[] contractAddresses;
        address delegatorAddress;
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

    struct UserDecryptionPayload {
        bytes publicKey;
        bytes32[] ctHandles;
    }

    event PublicDecryptionRequest(
        uint256 indexed decryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        bytes extraData
    );

    event PublicDecryptionResponseCall(
        uint256 indexed decryptionId,
        bytes decryptedResult,
        bytes signature,
        address kmsTxSender,
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

    event UserDecryptionRequest(
        uint256 indexed decryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        HandleEntry[] handles,
        UserDecryptionRequestPayload payload
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
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        emit PublicDecryptionRequest(decryptionId, snsCtMaterials, extraData);
    }

    function publicDecryptionResponse(
        uint256 decryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        address kmsTxSender;
        bytes[] memory signatures = new bytes[](1);

        emit PublicDecryptionResponseCall(decryptionId, decryptedResult, signature, kmsTxSender, extraData);

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
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        emit UserDecryptionRequest(decryptionId, snsCtMaterials, userAddress, publicKey, extraData);
    }

    // RFC016 unified EIP-712 overload. Selector matches the `userDecryptionRequest_0` binding
    // generated from the real `Decryption.sol`, so `DecryptionInstance::userDecryptionRequest_0`
    // dispatches here when this mock is deployed in tests.
    function userDecryptionRequest(
        HandleEntry[] calldata handles,
        address userAddress,
        bytes calldata publicKey,
        address[] calldata allowedContracts,
        RequestValiditySeconds calldata requestValidity,
        bytes calldata signature,
        bytes calldata extraData
    ) external {
        userDecryptionCounter++;
        uint256 decryptionId = userDecryptionCounter;
        SnsCiphertextMaterial[] memory snsCtMaterials = new SnsCiphertextMaterial[](1);

        UserDecryptionRequestPayload memory payload = UserDecryptionRequestPayload(
            userAddress,
            publicKey,
            allowedContracts,
            requestValidity,
            extraData,
            signature
        );

        emit UserDecryptionRequest(decryptionId, snsCtMaterials, handles, payload);
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
        uint256 decryptionId = userDecryptionCounter;
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
        uint256 indexShare;

        emit UserDecryptionResponse(decryptionId, indexShare, userDecryptedShare, signature, extraData);

        emit UserDecryptionResponseThresholdReached(decryptionId);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

interface IDecryptionManager {
    event OracleDecryptionId(uint256 indexed oracleDecryptionId);
    event OracleDecryptionRequest(
        uint256 indexed keychainId,
        uint256 indexed oracleDecryptionId,
        uint256 chainId,
        address kmsVerifier,
        address acl,
        uint256[] ciphertextHandles
    );

    event OracleDecryptionResponse(uint256 indexed oracleDecryptionId, bytes decryptedResult, bytes[] signatures);

    function oracleDecryptionRequest(
        uint256 keychainId,
        uint256 chainId,
        address kmsVerifier,
        address acl,
        uint256[] calldata ciphertextHandles
    ) external;

    function oracleDecryptionResponse(
        uint256 oracleDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) external;
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @notice Struct that contains metadata about the protocol.
 */
struct ProtocolMetadata {
    /// @notice Name of the protocol
    string name;
    /// @notice Website of the protocol
    string website;
}

/**
 * @notice Struct that represents a KMS (Key Management Service) node.
 */
struct KmsNode {
    /// @notice Address of the KMS node's transaction sender (also called KMS connector)
    address txSenderAddress;
    /// @notice Address of the KMS node's signer (used for signing inputs with EIP712 signatures)
    address signerAddress;
    /// @notice IP address of the KMS node
    string ipAddress;
    /// @notice URL address of the KMS node' storage where ciphertexts are stored
    string storageUrl;
}

/**
 * @notice Struct that represents a coprocessor.
 */
struct Coprocessor {
    /// @notice Address of the coprocessor's transaction sender
    address txSenderAddress;
    /// @notice Address of the coprocessor's signer (used for signing inputs with EIP712 signatures)
    address signerAddress;
    /// @notice URL address of the coprocessor's S3 bucket where ciphertexts are stored
    string s3BucketUrl;
}

/**
 * @notice Struct that represents a custodian.
 */
struct Custodian {
    /// @notice Address of the custodian's transaction sender.
    address txSenderAddress;
    /// @notice Address of the custodian's signer (used for signing inputs with EIP712 signatures)
    address signerAddress;
    /// @notice Post-quantum secure public key used for encrypting symmetric key shares during backup.
    bytes encryptionKey;
}

/**
 * @notice Struct that represents a host chain.
 */
struct HostChain {
    /// @notice Chain ID of the host chain (unique identifier)
    uint256 chainId;
    /// @notice Address where the fhevm library contract is deployed
    address fhevmExecutorAddress;
    /// @notice Address where the ACL contract is deployed
    address aclAddress;
    /// @notice Name of the host chain
    string name;
    /// @notice Website of the host chain
    string website;
}

/**
 * @notice Data structure used to transfer a Switch and Squash (SNS) ciphertext and some of its metadata between
 * the Gateway contracts.
 */
struct SnsCiphertextMaterial {
    bytes32 ctHandle;
    uint256 keyId;
    bytes32 snsCiphertextDigest;
    address[] coprocessorTxSenderAddresses;
}

/**
 * @notice Data structure used to transfer a regular ciphertext and some of its metadata between
 * the Gateway contracts.
 */
struct CiphertextMaterial {
    bytes32 ctHandle;
    uint256 keyId;
    bytes32 ciphertextDigest;
    address[] coprocessorTxSenderAddresses;
}

/**
 * @notice A struct that contains a ciphertext handle and a contract address that is
 * expected to be allowed to decrypt this ciphertext
 */
struct CtHandleContractPair {
    /// @notice The handle of the ciphertext
    bytes32 ctHandle;
    /// @notice The address of the contract
    address contractAddress;
}

/**
 * @notice A struct that contains user decryption delegation data.
 */
struct UserDecryptionDelegation {
    /// @notice The expiration date for the intended delegation.
    uint64 expiryDate;
    /// @notice A counter specific to the (delegator, delegate, contract) triple tied to the delegation.
    uint64 delegationCounter;
}

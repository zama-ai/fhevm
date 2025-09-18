// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice Struct that contains metadata about the protocol
struct ProtocolMetadata {
    /// @notice Name of the protocol
    string name;
    /// @notice Website of the protocol
    string website;
}

/// @notice Struct that represents a KMS (Key Management Service) node
struct KmsNode {
    /// @notice Address of the KMS node's transaction sender (also called KMS connector)
    address txSenderAddress;
    /// @notice Address of the KMS node's signer (used for signing inputs with EIP712 signatures)
    address signerAddress;
    /// @notice IP address of the KMS node
    string ipAddress;
}

/// @notice Struct that represents a coprocessor
struct CoprocessorV2 {
    /// @notice Name of the coprocessor, as a human-readable identifier
    string name;
    /// @notice Address of the coprocessor's transaction sender
    address txSenderAddress;
    /// @notice Address of the coprocessor's signer (used for signing inputs with EIP712 signatures)
    address signerAddress;
    /// @notice URL address of the coprocessor's storage where ciphertexts are stored
    string storageUrl;
}

/// @notice Struct that represents a coprocessor context
struct CoprocessorContext {
    /// @notice The ID of the coprocessor context
    uint256 contextId;
    /// @notice The ID of the previous (active)coprocessor context
    uint256 previousContextId;
    /// @notice The feature set of the coprocessor context
    uint256 featureSet;
    /// @notice The coprocessors in the coprocessor context
    CoprocessorV2[] coprocessors;
}

/// @notice Struct that represents the time periods for a coprocessor context
struct CoprocessorContextTimePeriods {
    /// @notice The time period for the pre-activation period (before activating the coprocessor context)
    uint256 preActivationTimePeriod;
    /// @notice The time period for the suspended period (before deactivating the previous coprocessor context)
    uint256 suspendedTimePeriod;
}

/// @notice Struct that represents a custodian
struct Custodian {
    /// @notice Address of the custodian's transaction sender.
    address txSenderAddress;
    /// @notice Address of the custodian's signer (used for signing inputs with EIP712 signatures)
    address signerAddress;
    /// @notice Post-quantum secure public key used for encrypting symmetric key shares during backup.
    bytes encryptionKey;
}

/// @notice Struct that represents a host chain
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
    /// @notice The handle of the ciphertext
    bytes32 ctHandle;
    /// @notice The key ID that was used to generate the ciphertext
    uint256 keyId;
    /// @notice The digest of the SNS ciphertext
    bytes32 snsCiphertextDigest;
}

/**
 * @notice Data structure used to transfer a regular ciphertext and some of its metadata between
 * the Gateway contracts.
 */
struct CiphertextMaterial {
    /// @notice The handle of the ciphertext
    bytes32 ctHandle;
    /// @notice The key ID that was used to generate the ciphertext
    uint256 keyId;
    /// @notice The digest of the regular ciphertext
    bytes32 ciphertextDigest;
}

/**
 * @notice A struct that contains a ciphertext handle and a contract address that is
 * @notice expected to be allowed to decrypt this ciphertext
 */
struct CtHandleContractPair {
    /// @notice The handle of the ciphertext
    bytes32 ctHandle;
    /// @notice The address of the contract
    address contractAddress;
}

/**
 * @notice A struct that contains the delegator and the delegated addresses
 */
struct DelegationAccounts {
    /// @notice The address of the account that delegates access to its handles.
    address delegatorAddress;
    /// @notice The address of the account that receives the delegation.
    address delegatedAddress;
}

// ----------------------------------------------------------------------------------------------
// DEPRECATED
// ----------------------------------------------------------------------------------------------

struct CoprocessorV1 {
    address txSenderAddress;
    address signerAddress;
    string s3BucketUrl;
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the KmsManagement contract
 * @notice The KmsManagement contract is responsible for managing the public materials such as FHE keys,
 * CRS (Common Reference String) and KSKs (Key Switching Keys) used by the Fhevm Gateway.
 * @dev The KmsManagement contract contains:
 * - the cryptographic parameters to consider when generating public materials (FHE keys, CRS, KSKs)
 * - the generated public materials (FHE keys, CRS, KSKs) IDs
 *
 * The KmsManagement contract has an owner. It can generate public material, activate FHE keys or update
 * cryptographic parameters.
 * Some functions are restricted to KMS connectors (contracts representing each KMS node) or coprocessors.
 * Some view functions are accessible to everyone (ex: getting the current activated FHE key ID).
 */
interface IKmsManagement {
    /// @notice Emitted to trigger a key generation preprocessing
    /// @param preKeyRequestId The preprocessed key request ID
    /// @param fheParamsDigest The digest of the FHE parameters to use
    event PreprocessKeygenRequest(uint256 preKeyRequestId, bytes32 fheParamsDigest);

    /// @notice Emitted when the key generation preprocessing is completed
    /// @param preKeyRequestId The preprocessed key request ID
    /// @param preKeyId The preprocessed key ID
    event PreprocessKeygenResponse(uint256 preKeyRequestId, uint256 preKeyId);

    /// @notice Emitted to trigger a KSK generation preprocessing
    /// @param preKskRequestId The preprocessed KSK request ID
    /// @param fheParamsDigest The digest of the FHE parameters to use
    event PreprocessKskgenRequest(uint256 preKskRequestId, bytes32 fheParamsDigest);

    /// @notice Emitted when the KSK generation preprocessing is completed
    /// @param preKskRequestId The preprocessed KSK request ID
    /// @param preKskId The preprocessed KSK ID
    event PreprocessKskgenResponse(uint256 preKskRequestId, uint256 preKskId);

    /// @notice Emitted to trigger a key generation
    /// @param preKeyId The preprocessed key ID
    /// @param fheParamsDigest The digest of the FHE parameters to use
    event KeygenRequest(uint256 preKeyId, bytes32 fheParamsDigest);

    /// @notice Emitted when the key generation is completed
    /// @param preKeyId The preprocessed key ID
    /// @param keygenId The generated key ID
    /// @param fheParamsDigest The digest of the FHE parameters used for the key generation
    event KeygenResponse(uint256 preKeyId, uint256 keygenId, bytes32 fheParamsDigest);

    /// @notice Emitted to trigger a CRS (Common Reference String) generation
    /// @param crsgenRequestId The CRS generation request ID
    /// @param fheParamsDigest The digest of the FHE parameters to use
    event CrsgenRequest(uint256 crsgenRequestId, bytes32 fheParamsDigest);

    /// @notice Emitted when the CRS generation is completed
    /// @param crsgenRequestId The CRS generation request ID
    /// @param crsId The generated CRS ID
    /// @param fheParamsDigest The digest of the FHE parameters used for the CRS generation
    event CrsgenResponse(uint256 crsgenRequestId, uint256 crsId, bytes32 fheParamsDigest);

    /// @notice Emitted to trigger a KSK generation
    /// @param preKskId The preprocessed KSK ID
    /// @param sourceKeyId The key ID to key switch from
    /// @param destKeyId The key ID to key switch to
    /// @param fheParamsDigest The digest of the FHE parameters to use
    event KskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId, bytes32 fheParamsDigest);

    /// @notice Emitted when the KSK generation is completed
    /// @param preKskId The preprocessed KSK ID
    /// @param kskId The generated KSK ID
    /// @param fheParamsDigest The digest of the FHE parameters used for the KSK generation
    event KskgenResponse(uint256 preKskId, uint256 kskId, bytes32 fheParamsDigest);

    /// @notice Emitted to activate the key in coprocessors
    /// @param keyId The key ID
    event ActivateKeyRequest(uint256 keyId);

    /// @notice Emitted when the key has been activated in all coprocessors
    /// @param keyId The key ID
    event ActivateKeyResponse(uint256 keyId);

    /// @notice Emitted when the FHE parameters have been set (happens only once)
    /// @param fheParamsName The semantic name of the FHE params
    /// @param fheParamsDigest The digest of the FHE params
    event AddFheParams(string fheParamsName, bytes32 fheParamsDigest);

    /// @notice Emitted when the FHE parameters have been updated
    /// @param fheParamsName The semantic name of the FHE params updated
    /// @param fheParamsDigest The new digest of the FHE params
    event UpdateFheParams(string fheParamsName, bytes32 fheParamsDigest);

    /// @notice Error thrown when the FHE params are not initialized
    error FheParamsNotInitialized();

    /// @notice Error thrown when a KMS node has already responded to a key generation preprocessing step
    error PreprocessKeygenKmsNodeAlreadyResponded(uint256 preKeyId);

    /// @notice Error thrown when a KMS node has already responded to a KSK generation preprocessing step
    error PreprocessKskgenKmsNodeAlreadyResponded(uint256 preKskId);

    /// @notice Error thrown when a key generation request has already been sent
    error KeygenRequestAlreadySent(uint256 preKeyId);

    /// @notice Error thrown when a key generation step requires preprocessing
    error KeygenPreprocessingRequired(uint256 preKeyId);

    /// @notice Error thrown when a KMS node has already responded to a key generation step
    error KeygenKmsNodeAlreadyResponded(uint256 keyId);

    /// @notice Error thrown when a KMS node has already responded to a CRS generation step
    error CrsgenKmsNodeAlreadyResponded(uint256 crsId);

    /// @notice Error thrown when a KSK generation request has already been sent
    error KskgenRequestAlreadySent(uint256 preKskId);

    /// @notice Error thrown when a KSK generation step requires preprocessing
    error KskgenPreprocessingRequired(uint256 preKskId);

    /// @notice Error thrown when a KSK generation source key ID is the same as the destination key ID
    error KskgenSameSrcAndDestKeyIds(uint256 keyId);

    /// @notice Error thrown when a KSK generation source key ID is not generated
    error KskgenSourceKeyNotGenerated(uint256 sourceKeyId);

    /// @notice Error thrown when a KSK generation destination key ID is not generated
    error KskgenDestKeyNotGenerated(uint256 destKeyId);

    /// @notice Error thrown when a KMS node has already responded to a KSK generation step
    error KskgenKmsNodeAlreadyResponded(uint256 kskId);

    /// @notice Error thrown when a key activation request has already been sent
    error ActivateKeyRequestAlreadySent(uint256 keyId);

    /// @notice Error thrown when a key activation step requires a key generation step
    error ActivateKeyRequiresKeygen(uint256 keyId);

    /// @notice Error thrown when a key activation step requires a KSK generation step from the
    /// @notice current key to the given key
    error ActivateKeyRequiresKskgen(uint256 currentKeyId, uint256 keyId);

    /// @notice Error thrown when a KMS node has already responded to a key activation step
    error ActivateKeyKmsNodeAlreadyResponded(uint256 keyId);

    /// @notice Error thrown when the FHE params are already initialized
    /// @param fheParamsName The semantic name of the already initialized the FHE params
    error FheParamsAlreadyInitialized(string fheParamsName);

    /// @notice Trigger a key generation preprocessing
    function preprocessKeygenRequest(string calldata fheParamsName) external;

    /// @notice Handle the response of a key generation preprocessing
    /// @dev This function can only be called by a KMS connector
    /// @param preKeyRequestId The preprocessed key request ID
    /// @param preKeyId The preprocessed key ID
    function preprocessKeygenResponse(uint256 preKeyRequestId, uint256 preKeyId) external;

    /// @notice Trigger a KSK generation preprocessing
    function preprocessKskgenRequest(string calldata fheParamsName) external;

    /// @notice Handle the response of a KSK generation preprocessing
    /// @dev This function can only be called by a KMS connector
    /// @param preKskRequestId The preprocessed KSK request ID
    /// @param preKskId The preprocessed KSK ID
    function preprocessKskgenResponse(uint256 preKskRequestId, uint256 preKskId) external;

    /// @notice Trigger a key generation
    /// @param preKeyId The preprocessed key ID
    function keygenRequest(uint256 preKeyId) external;

    /// @notice Handle the response of a key generation
    /// @dev This function can only be called by a KMS connector
    /// @param preKeyId The preprocessed key ID
    /// @param keyId The generated key ID
    function keygenResponse(uint256 preKeyId, uint256 keyId) external;

    /// @notice Trigger a CRS generation
    function crsgenRequest(string calldata fheParamsName) external;

    /// @notice Handle the response of a CRS generation
    /// @dev This function can only be called by a KMS connector
    /// @param crsgenRequestId The CRS generation request ID
    /// @param crsId The generated CRS ID
    function crsgenResponse(uint256 crsgenRequestId, uint256 crsId) external;

    /// @notice Trigger a KSK generation
    /// @param preKskId The preprocessed KSK ID
    /// @param sourceKeyId The key ID to key switch from
    /// @param destKeyId The key ID to key switch to
    function kskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId) external;

    /// @notice Handle the response of a KSK generation
    /// @dev This function can only be called by a KMS connector
    /// @param preKskId The preprocessed KSK ID
    /// @param kskId The generated KSK ID
    function kskgenResponse(uint256 preKskId, uint256 kskId) external;

    /// @notice Activate the key in coprocessors
    /// @dev A key can only be activated if a key switch key from the current key to this key has
    /// @dev already been generated
    /// @param keyId The key ID
    function activateKeyRequest(uint256 keyId) external;

    // TODO: Check if this is needed
    /// @notice Handle the response of a key activation
    /// @dev This function can only be called by a coprocessor
    /// @param keyId The key ID
    function activateKeyResponse(uint256 keyId) external;

    /// @notice Add a new FHE params name and its digest
    /// @dev This function can only be called by the owner
    /// @dev This function can only be called once per fheParamsName, during the overall initialization of the protocol
    /// @param fheParamsName The semantic name of the FHE params
    /// @param fheParamsDigest The digest of the FHE params
    function addFheParams(string calldata fheParamsName, bytes32 fheParamsDigest) external;

    /// @notice Update the digest of the given FHE params name
    /// @dev This function can only be called by the owner
    /// @param fheParamsName The semantic name of the FHE params to update
    /// @param fheParamsDigest The new digest of the FHE params
    function updateFheParams(string calldata fheParamsName, bytes32 fheParamsDigest) external;

    /// @notice Get the digest of the given FHE params name
    /// @return The digest of the given FHE params name
    function fheParamsDigests(string calldata fheParamsName) external view returns (bytes32);

    /// @notice Get the current (activated) keyId
    /// @return The current (activated) keyId
    function getCurrentKeyId() external view returns (uint256);

    /// @notice Get the activated keyId with the given index
    /// @return The activated keyId with the given index
    function activatedKeyIds(uint256 index) external view returns (uint256);

    /// @notice Get the generator FHE params digest associated with the key ID
    /// @return The generator FHE params digest associated with the key ID
    function keyFheParamsDigests(uint256 keyId) external view returns (bytes32);

    /// @notice Get the generator FHE params digest associated with the KSK ID
    /// @return The generator FHE params digest associated with the KSK ID
    function kskFheParamsDigests(uint256 kskId) external view returns (bytes32);

    /// @notice Get the generator FHE params digest associated with the CRS ID
    /// @return The generator FHE params digest associated with the CRS ID
    function crsFheParamsDigests(uint256 crsId) external view returns (bytes32);

    /// @notice Returns the versions of the KmsManagement contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
    function getVersion() external pure returns (string memory);
}

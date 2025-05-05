// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the KmsManagement contract.
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
    /**
     * @notice Emitted to trigger a key generation preprocessing.
     * @param preKeyRequestId The ID of the preprocessed key request.
     * @param fheParamsDigest The digest of the FHE parameters to use.
     */
    event PreprocessKeygenRequest(uint256 preKeyRequestId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted when the key generation preprocessing is completed.
     * @param preKeyRequestId The ID of the preprocessed key request.
     * @param preKeyId The ID of the preprocessed key.
     */
    event PreprocessKeygenResponse(uint256 preKeyRequestId, uint256 preKeyId);

    /**
     * @notice Emitted to trigger a KSK generation preprocessing.
     * @param preKskRequestId The ID of the preprocessed KSK request.
     * @param fheParamsDigest The digest of the FHE parameters to use.
     */
    event PreprocessKskgenRequest(uint256 preKskRequestId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted when the KSK generation preprocessing is completed.
     * @param preKskRequestId The ID of the preprocessed KSK request.
     * @param preKskId The ID of the preprocessed KSK.
     */
    event PreprocessKskgenResponse(uint256 preKskRequestId, uint256 preKskId);

    /**
     * @notice Emitted to trigger a key generation.
     * @param preKeyId The ID of the preprocessed key.
     * @param fheParamsDigest The digest of the FHE parameters to use.
     */
    event KeygenRequest(uint256 preKeyId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted when the key generation is completed.
     * @param preKeyId The ID of the preprocessed key.
     * @param keygenId The ID of the generated key.
     * @param fheParamsDigest The digest of the FHE parameters used for the key generation.
     */
    event KeygenResponse(uint256 preKeyId, uint256 keygenId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted to trigger a CRS (Common Reference String) generation.
     * @param crsgenRequestId The ID of the CRS generation request.
     * @param fheParamsDigest The digest of the FHE parameters to use.
     */
    event CrsgenRequest(uint256 crsgenRequestId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted when the CRS generation is completed.
     * @param crsgenRequestId The ID of the CRS generation request.
     * @param crsId The ID of the generated CRS.
     * @param fheParamsDigest The digest of the FHE parameters used for the CRS generation.
     */
    event CrsgenResponse(uint256 crsgenRequestId, uint256 crsId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted to trigger a KSK generation.
     * @param preKskId The ID of the preprocessed KSK.
     * @param sourceKeyId The ID of the key to switch from.
     * @param destKeyId The ID of the key to switch to.
     * @param fheParamsDigest The digest of the FHE parameters to use.
     */
    event KskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted when the KSK generation is completed.
     * @param preKskId The ID of the preprocessed KSK.
     * @param kskId The ID of the generated KSK.
     * @param fheParamsDigest The digest of the FHE parameters used for the KSK generation.
     */
    event KskgenResponse(uint256 preKskId, uint256 kskId, bytes32 fheParamsDigest);

    /**
     * @notice Emitted to activate the key in coprocessors.
     * @param keyId The ID of the key requested for activation.
     */
    event ActivateKeyRequest(uint256 keyId);

    /**
     * @notice Emitted when the key has been activated in all coprocessors.
     * @param keyId The ID of the activated key.
     */
    event ActivateKeyResponse(uint256 keyId);

    /**
     * @notice Emitted when the FHE parameters have been set (happens only once).
     * @param fheParamsName The semantic name of the FHE params.
     * @param fheParamsDigest The digest of the FHE params.
     */
    event AddFheParams(string fheParamsName, bytes32 fheParamsDigest);

    /**
     * @notice Emitted when the FHE parameters have been updated.
     * @param fheParamsName The semantic name of the FHE params updated.
     * @param fheParamsDigest The new digest of the FHE params.
     */
    event UpdateFheParams(string fheParamsName, bytes32 fheParamsDigest);

    /// @notice Error thrown when the FHE params are not initialized.
    error FheParamsNotInitialized();

    /**
     * @notice Error thrown when a KMS node has already responded to a key generation preprocessing step.
     * @param preKeyId The ID of the preprocessed key that has already been responded.
     */
    error PreprocessKeygenKmsNodeAlreadyResponded(uint256 preKeyId);

    /**
     * @notice Error thrown when a KMS node has already responded to a KSK generation preprocessing step.
     * @param preKskId The ID of the preprocessed KSK that has already been responded.
     */
    error PreprocessKskgenKmsNodeAlreadyResponded(uint256 preKskId);

    /**
     * @notice Error thrown when a key generation request has already been sent.
     * @param preKeyId The ID of the preprocessed key that has already been sent for key generation.
     */
    error KeygenRequestAlreadySent(uint256 preKeyId);

    /**
     * @notice Error thrown when a key generation step requires preprocessing.
     * @param preKeyId The ID of the preprocessed key that is required for the key generation step.
     */
    error KeygenPreprocessingRequired(uint256 preKeyId);

    /**
     * @notice Error thrown when a KMS node has already responded to a key generation step.
     * @param keyId The ID of the key that has already been responded.
     */
    error KeygenKmsNodeAlreadyResponded(uint256 keyId);

    /**
     * @notice Error thrown when a KMS node has already responded to a CRS generation step.
     * @param crsId The ID of the CRS that has already been responded.
     */
    error CrsgenKmsNodeAlreadyResponded(uint256 crsId);

    /**
     * @notice Error thrown when a KSK generation request has already been sent.
     * @param preKskId The ID of the preprocessed KSK that has already been sent for KSK generation.
     */
    error KskgenRequestAlreadySent(uint256 preKskId);

    /**
     * @notice Error thrown when a KSK generation step requires preprocessing.
     * @param preKskId The preprocessed KSK ID that is required for the KSK generation step.
     */
    error KskgenPreprocessingRequired(uint256 preKskId);

    /**
     * @notice Error thrown when a key ID is the same for the source and destination
     * during a KSK generation.
     * @param keyId The ID of the key that is the same for the source and destination keys.
     */
    error KskgenSameSrcAndDestKeyIds(uint256 keyId);

    /**
     * @notice Error thrown when a KSK generation source key ID is not generated.
     * @param sourceKeyId The ID of the source key that is not generated.
     */
    error KskgenSourceKeyNotGenerated(uint256 sourceKeyId);

    /**
     * @notice Error thrown when a KSK generation destination key ID is not generated.
     * @param destKeyId The ID of the destination key that is not generated.
     */
    error KskgenDestKeyNotGenerated(uint256 destKeyId);

    /**
     * @notice Error thrown when a KMS node has already responded to a KSK generation step.
     * @param kskId The ID of the KSK that has already been responded.
     */
    error KskgenKmsNodeAlreadyResponded(uint256 kskId);

    /**
     * @notice Error thrown when a key activation request has already been sent.
     * @param keyId The ID of the key that has already been sent for activation.
     */
    error ActivateKeyRequestAlreadySent(uint256 keyId);

    /**
     * @notice Error thrown when a key activation step requires a key generation step.
     * @param keyId The ID of the key that is required for the key generation step.
     */
    error ActivateKeyRequiresKeygen(uint256 keyId);

    /**
     * @notice Error thrown when a key activation step requires a KSK generation step from the
     * current key to the given key.
     * @param currentKeyId The current key ID that is required for the KSK generation step.
     * @param keyId The ID of the key that is required for the KSK generation step.
     */
    error ActivateKeyRequiresKskgen(uint256 currentKeyId, uint256 keyId);

    /**
     * @notice Error thrown when a KMS node has already responded to a key activation step.
     * @param keyId The ID of the key that has already been responded.
     */
    error ActivateKeyKmsNodeAlreadyResponded(uint256 keyId);

    /**
     * @notice Error thrown when the FHE params are already initialized.
     * @param fheParamsName The semantic name of the already initialized the FHE params.
     */
    error FheParamsAlreadyInitialized(string fheParamsName);

    /**
     * @notice Trigger a key generation preprocessing.
     * @param fheParamsName The semantic name of the FHE params to use.
     */
    function preprocessKeygenRequest(string calldata fheParamsName) external;

    /**
     * @notice Handle the response of a key generation preprocessing.
     * @dev This function can only be called by a KMS connector.
     * @param preKeyRequestId The ID of the preprocessed key request.
     * @param preKeyId The ID of the preprocessed key.
     */
    function preprocessKeygenResponse(uint256 preKeyRequestId, uint256 preKeyId) external;

    /**
     * @notice Trigger a KSK generation preprocessing.
     * @param fheParamsName The semantic name of the FHE params to use.
     */
    function preprocessKskgenRequest(string calldata fheParamsName) external;

    /**
     * @notice Handle the response of a KSK generation preprocessing.
     * @dev This function can only be called by a KMS connector.
     * @param preKskRequestId The ID of the preprocessed KSK request.
     * @param preKskId The ID of the preprocessed KSK.
     */
    function preprocessKskgenResponse(uint256 preKskRequestId, uint256 preKskId) external;

    /**
     * @notice Trigger a key generation.
     * @param preKeyId The ID of the preprocessed key.
     */
    function keygenRequest(uint256 preKeyId) external;

    /**
     * @notice Handle the response of a key generation.
     * @dev This function can only be called by a KMS connector.
     * @param preKeyId The ID of the preprocessed key.
     * @param keyId The ID of the generated key.
     */
    function keygenResponse(uint256 preKeyId, uint256 keyId) external;

    /**
     * @notice Trigger a CRS generation.
     * @param fheParamsName The semantic name of the FHE params to use.
     */
    function crsgenRequest(string calldata fheParamsName) external;

    /**
     * @notice Handle the response of a CRS generation.
     * @dev This function can only be called by a KMS connector.
     * @param crsgenRequestId The ID of the CRS generation request.
     * @param crsId The ID of the generated CRS.
     */
    function crsgenResponse(uint256 crsgenRequestId, uint256 crsId) external;

    /**
     * @notice Trigger a KSK generation.
     * @param preKskId The ID of the preprocessed KSK.
     * @param sourceKeyId The ID of the key switch from.
     * @param destKeyId The ID of the key to switch to.
     */
    function kskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId) external;

    /**
     * @notice Handle the response of a KSK generation.
     * @dev This function can only be called by a KMS connector.
     * @param preKskId The ID of the preprocessed KSK.
     * @param kskId The ID of the generated KSK.
     */
    function kskgenResponse(uint256 preKskId, uint256 kskId) external;

    /**
     * @notice Activate the key in coprocessors.
     * @dev A key can only be activated if a KSK from the current key to this key has
     * already been generated.
     * @param keyId The ID of the key to activate.
     */
    function activateKeyRequest(uint256 keyId) external;

    // TODO: Check if this is needed
    /**
     * @notice Handle the response of a key activation.
     * @dev This function can only be called by a coprocessor.
     * @param keyId The ID of the activated key.
     */
    function activateKeyResponse(uint256 keyId) external;

    /**
     * @notice Add a new FHE params name and its digest.
     * @dev This function can only be called by the owner.
     * @dev This function can only be called once per fheParamsName, during the overall initialization of the protocol.
     * @param fheParamsName The semantic name of the FHE params.
     * @param fheParamsDigest The digest of the FHE params.
     */
    function addFheParams(string calldata fheParamsName, bytes32 fheParamsDigest) external;

    /**
     * @notice Update the digest of the given FHE params name.
     * @dev This function can only be called by the owner.
     * @param fheParamsName The semantic name of the FHE params to update.
     * @param fheParamsDigest The new digest of the FHE params.
     */
    function updateFheParams(string calldata fheParamsName, bytes32 fheParamsDigest) external;

    /**
     * @notice Get the digest of the given FHE params name.
     * @return The digest of the given FHE params name.
     */
    function fheParamsDigests(string calldata fheParamsName) external view returns (bytes32);

    /**
     * @notice Get the ID of the current activated key.
     * @return The current ID of the activated key.
     */
    function getCurrentKeyId() external view returns (uint256);

    /**
     * @notice Get the ID of the activated key with the given index.
     * @return The ID of the activated key with the given index.
     */
    function activatedKeyIds(uint256 index) external view returns (uint256);

    /**
     * @notice Get the generator FHE params digest associated with the key ID.
     * @return The generator FHE params digest associated with the key ID.
     */
    function keyFheParamsDigests(uint256 keyId) external view returns (bytes32);

    /**
     * @notice Get the generator FHE params digest associated with the KSK ID.
     * @return The generator FHE params digest associated with the KSK ID.
     */
    function kskFheParamsDigests(uint256 kskId) external view returns (bytes32);

    /**
     * @notice Get the generator FHE params digest associated with the CRS ID.
     * @return The generator FHE params digest associated with the CRS ID.
     */
    function crsFheParamsDigests(uint256 crsId) external view returns (bytes32);

    /**
     * @notice Returns the versions of the KmsManagement contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

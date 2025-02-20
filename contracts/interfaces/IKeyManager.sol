// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/**
 * @title Interface for the KeyManager contract
 * @notice The KeyManager contract is responsible for managing the public materials such as FHE keys,
 * CRS (Common Reference String) and KSKs (Key Switching Keys) used by the Gateway L2.
 * @dev The KeyManager contract contains:
 * - the cryptographic parameters to consider when generating public materials (FHE keys, CRS, KSKs)
 * - the generated public materials (FHE keys, CRS, KSKs) IDs
 *
 * The KeyManager contract is owned by a DAO governance contract that can be used for updating the
 * cryptographic parameters.
 * The KeyManager contract is also managed by administrators that can generate any public material,
 * and activate FHE keys.
 * Some functions are restricted to KMS connectors (contracts representing each KMS node) or coprocessors.
 * Some view functions are accessible to everyone (ex: getting the current activated FHE key ID).
 */
interface IKeyManager {
    // TODO: To be defined: https://github.com/zama-ai/gateway-l2/issues/50
    /// @notice Struct that represents FHE parameters
    /// @dev FHE parameters are used for FHE key, CRS and KSK generation (including preprocessing steps)
    struct FheParams {
        /// @notice Placeholder for FHE parameters
        string dummy;
    }

    /// @notice Emitted to trigger a key generation preprocessing
    /// @param preKeyRequestId The preprocessed key request ID
    /// @param fheParams The FHE parameters to use
    event PreprocessKeygenRequest(uint256 preKeyRequestId, FheParams fheParams);

    /// @notice Emitted when the key generation preprocessing is completed
    /// @param preKeyRequestId The preprocessed key request ID
    /// @param preKeyId The preprocessed key ID
    event PreprocessKeygenResponse(uint256 preKeyRequestId, uint256 preKeyId);

    /// @notice Emitted to trigger a KSK generation preprocessing
    /// @param preKskRequestId The preprocessed KSK request ID
    /// @param fheParams The FHE parameters to use
    event PreprocessKskgenRequest(uint256 preKskRequestId, FheParams fheParams);

    /// @notice Emitted when the KSK generation preprocessing is completed
    /// @param preKskRequestId The preprocessed KSK request ID
    /// @param preKskId The preprocessed KSK ID
    event PreprocessKskgenResponse(uint256 preKskRequestId, uint256 preKskId);

    /// @notice Emitted to trigger a key generation
    /// @param preKeyId The preprocessed key ID
    /// @param fheParams The FHE parameters to use
    event KeygenRequest(uint256 preKeyId, FheParams fheParams);

    /// @notice Emitted when the key generation is completed
    /// @param preKeyId The preprocessed key ID
    /// @param keygenId The generated key ID
    /// @param fheParams The FHE parameters used for the key generation
    event KeygenResponse(uint256 preKeyId, uint256 keygenId, FheParams fheParams);

    /// @notice Emitted to trigger a CRS (Common Reference String) generation
    /// @param preCrsId The preprocessed CRS ID
    /// @param fheParams The FHE parameters to use
    event CrsgenRequest(uint256 preCrsId, FheParams fheParams);

    /// @notice Emitted when the CRS generation is completed
    /// @param preCrsId The preprocessed CRS ID
    /// @param crsId The generated CRS ID
    /// @param fheParams The FHE parameters used for the CRS generation
    event CrsgenResponse(uint256 preCrsId, uint256 crsId, FheParams fheParams);

    /// @notice Emitted to trigger a KSK generation
    /// @param preKskId The preprocessed KSK ID
    /// @param sourceKeyId The key ID to key switch from
    /// @param destKeyId The key ID to key switch to
    /// @param fheParams The FHE parameters to use
    event KskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId, FheParams fheParams);

    /// @notice Emitted when the KSK generation is completed
    /// @param preKskId The preprocessed KSK ID
    /// @param kskId The generated KSK ID
    /// @param fheParams The FHE parameters used for the KSK generation
    event KskgenResponse(uint256 preKskId, uint256 kskId, FheParams fheParams);

    /// @notice Emitted to activate the key in coprocessors
    /// @param keyId The key ID
    event ActivateKeyRequest(uint256 keyId);

    /// @notice Emitted when the key has been activated in all coprocessors
    /// @param keyId The key ID
    event ActivateKeyResponse(uint256 keyId);

    /// @notice Emitted when the FHE parameters have been set (happens only once)
    /// @param newFheParams The new FHE parameters
    event SetFheParams(FheParams newFheParams);

    /// @notice Emitted when the FHE parameters have been updated
    /// @param newFheParams The new FHE parameters
    event UpdateFheParams(FheParams newFheParams);

    /// @notice Error thrown when the FHE params are not initialized
    error FheParamsNotInitialized();

    /// @notice Error thrown when the sender is not an admin
    error InvalidAdminSender(address sender);

    /// @notice Error thrown when the sender is not a KMS node
    error InvalidKmsNodeSender(address sender);

    /// @notice Error thrown when the sender is not a Coprocessor
    error InvalidCoprocessorSender(address sender);

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
    error FheParamsAlreadyInitialized();

    /// @notice Trigger a key generation preprocessing
    /// @dev This function can only be called by an administrator
    function preprocessKeygenRequest() external;

    /// @notice Handle the response of a key generation preprocessing
    /// @dev This function can only be called by a KMS connector
    /// @param preKeyRequestId The preprocessed key request ID
    /// @param preKeyId The preprocessed key ID
    function preprocessKeygenResponse(uint256 preKeyRequestId, uint256 preKeyId) external;

    /// @notice Trigger a KSK generation preprocessing
    /// @dev This function can only be called by an administrator
    function preprocessKskgenRequest() external;

    /// @notice Handle the response of a KSK generation preprocessing
    /// @dev This function can only be called by a KMS connector
    /// @param preKskRequestId The preprocessed KSK request ID
    /// @param preKskId The preprocessed KSK ID
    function preprocessKskgenResponse(uint256 preKskRequestId, uint256 preKskId) external;

    /// @notice Trigger a key generation
    /// @dev This function can only be called by an administrator
    /// @param preKeyId The preprocessed key ID
    function keygenRequest(uint256 preKeyId) external;

    /// @notice Handle the response of a key generation
    /// @dev This function can only be called by a KMS connector
    /// @param preKeyId The preprocessed key ID
    /// @param keyId The generated key ID
    function keygenResponse(uint256 preKeyId, uint256 keyId) external;

    /// @notice Trigger a CRS generation
    /// @dev This function can only be called by an administrator
    function crsgenRequest() external;

    /// @notice Handle the response of a CRS generation
    /// @dev This function can only be called by a KMS connector
    /// @param preCrsId The preprocessed CRS ID
    /// @param crsId The generated CRS ID
    function crsgenResponse(uint256 preCrsId, uint256 crsId) external;

    /// @notice Trigger a KSK generation
    /// @dev This function can only be called by an administrator
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
    /// @dev This function can only be called by an administrator
    /// @dev A key can only be activated if a key switch key from the current key to this key has
    /// @dev already been generated
    /// @param keyId The key ID
    function activateKeyRequest(uint256 keyId) external;

    // TODO: Check if this is needed
    /// @notice Handle the response of a key activation
    /// @dev This function can only be called by a coprocessor
    /// @param keyId The key ID
    function activateKeyResponse(uint256 keyId) external;

    /// @notice Set the FHE params
    /// @dev This function can only be called by the owner
    /// @dev This function can only be called once, during the overall initialization of the protocol
    /// @param newFheParams The new FHE params
    function setFheParams(FheParams memory newFheParams) external;

    /// @notice Update the FHE params
    /// @dev This function can only be called by the owner
    /// @param newFheParams The new FHE params
    function updateFheParams(FheParams memory newFheParams) external;

    // TODO: May not be needed if contracts are made pausable
    // https://github.com/zama-ai/gateway-l2/issues/51
    /// @notice Check if a given key ID is the current one
    /// @dev The current key is the latest generated key that has been activated
    /// @param keyId The key ID to check
    /// @return True if the key ID is the current one, false otherwise
    function isCurrentKeyId(uint256 keyId) external view returns (bool);
}

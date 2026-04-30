// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the host-side KMSGeneration contract.
 * @notice Manages FHE key and CRS generation on the Ethereum host chain, reading config from
 * ProtocolConfig.
 */
interface IKMSGeneration {
    /**
     * @notice The type of the parameters to use for the generation requests.
     */
    enum ParamsType {
        Default, // 0
        Test // 1
    }

    /**
     * @notice The type of the generated key.
     */
    enum KeyType {
        Server, // 0
        Public // 1
    }

    /**
     * @notice The struct representing a generated key.
     */
    struct KeyDigest {
        /// @notice The type of the generated key.
        KeyType keyType;
        /// @notice The digest of the generated key.
        bytes digest;
    }

    /**
     * @notice Emitted to trigger an FHE key generation preprocessing.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param paramsType The type of the parameters to use.
     * @param extraData Additional context data (0x01 || contextId).
     */
    event PrepKeygenRequest(uint256 prepKeygenId, ParamsType paramsType, bytes extraData);

    /**
     * @notice Emitted when a KMS node has responded to a preprocessing keygen request.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param signature The signature of the KMS node that has responded.
     * @param kmsTxSender The transaction sender of the KMS node that has called the function.
     */
    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);

    /**
     * @notice Emitted to trigger an FHE key generation.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param keyId The ID of the key to generate.
     * @param extraData Additional context data.
     */
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);

    /**
     * @notice Emitted when a KMS node has responded to a keygen request.
     * @param keyId The ID of the key.
     * @param keyDigests The digests of the generated keys.
     * @param signature The signature of the KMS node that has responded.
     * @param kmsTxSender The transaction sender of the KMS node that has called the function.
     */
    event KeygenResponse(uint256 keyId, KeyDigest[] keyDigests, bytes signature, address kmsTxSender);

    /**
     * @notice Emitted when the key is activated.
     * @param keyId The ID of the activated key.
     * @param kmsNodeStorageUrls The KMS nodes' storage URL that participated in the consensus.
     * @param keyDigests The digests of the generated keys.
     */
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    /**
     * @notice Emitted to trigger a CRS (Common Reference String) generation.
     * @param crsId The ID of the CRS to generate.
     * @param maxBitLength The max bit length for generating the CRS.
     * @param paramsType The type of CRS parameters to use.
     * @param extraData Additional context data.
     */
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType, bytes extraData);

    /**
     * @notice Emitted when a KMS node has responded to a CRS generation request.
     * @param crsId The ID of the CRS.
     * @param crsDigest The digest of the generated CRS.
     * @param signature The signature of the KMS node that has responded.
     * @param kmsTxSender The transaction sender of the KMS node that has called the function.
     */
    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);

    /**
     * @notice Emitted when the CRS is activated.
     * @param crsId The ID of the generated CRS.
     * @param kmsNodeStorageUrls The KMS nodes' storage URL that participated in the consensus.
     * @param crsDigest The digest of the generated CRS.
     */
    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);

    /**
     * @notice Emitted when a keygen is aborted.
     * @param prepKeygenId The ID of the aborted preprocessing keygen.
     */
    event AbortKeygen(uint256 prepKeygenId);

    /**
     * @notice Emitted when a CRS generation is aborted.
     * @param crsId The ID of the aborted CRS generation.
     */
    event AbortCrsgen(uint256 crsId);

    /**
     * @notice Error indicating that the preprocessing keygen request is not requested yet.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     */
    error PrepKeygenNotRequested(uint256 prepKeygenId);

    /**
     * @notice Error thrown when a keygen request is ongoing.
     * @param keyId The ID of the ongoing keygen request.
     */
    error KeygenOngoing(uint256 keyId);

    /**
     * @notice Error thrown when a KMS node has already signed for a preprocessing keygen response.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param kmsSigner The signer address of the KMS node.
     */
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);

    /**
     * @notice Error indicating that the keygen request is not requested yet.
     * @param keyId The ID of the key.
     */
    error KeygenNotRequested(uint256 keyId);

    /**
     * @notice Error thrown when the keygen response contains no key digests.
     * @param keyId The ID of the key.
     */
    error EmptyKeyDigests(uint256 keyId);

    /**
     * @notice Error thrown when a KMS node has already signed for a keygen response.
     * @param keyId The ID of the key.
     * @param kmsSigner The signer address of the KMS node.
     */
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);

    /**
     * @notice Error indicating that the CRS generation request is not requested yet.
     * @param crsId The ID of the CRS.
     */
    error CrsgenNotRequested(uint256 crsId);

    /**
     * @notice Error thrown when a crsgen request is ongoing.
     * @param crsId The ID of the ongoing crsgen request.
     */
    error CrsgenOngoing(uint256 crsId);

    /**
     * @notice Error thrown when a KMS node has already signed for a CRS generation response.
     * @param crsId The ID of the CRS.
     * @param kmsSigner The signer address of the KMS node.
     */
    error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);

    /**
     * @notice Error thrown when an FHE key has not been generated.
     * @param keyId The ID of the key.
     */
    error KeyNotGenerated(uint256 keyId);

    /**
     * @notice Error thrown when an FHE key generation was aborted.
     * @param keyId The ID of the key.
     */
    error KeyAborted(uint256 keyId);

    /**
     * @notice Error thrown when a CRS has not been generated.
     * @param crsId The ID of the CRS.
     */
    error CrsNotGenerated(uint256 crsId);

    /**
     * @notice Error thrown when a CRS generation was aborted.
     * @param crsId The ID of the CRS.
     */
    error CrsAborted(uint256 crsId);

    /**
     * @notice Error thrown when the deserializing of the extra data fails.
     */
    error DeserializingExtraDataFail();

    /**
     * @notice Error thrown when the caller is not a KMS tx sender.
     * @param txSenderAddress The caller address.
     */
    error NotKmsTxSender(address txSenderAddress);

    /**
     * @notice Error thrown when the recovered signer does not match the tx sender's KMS node.
     * @param signerAddress The recovered signer.
     * @param txSenderAddress The tx sender.
     */
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);

    /**
     * @notice Error thrown when the extra data version is unsupported.
     * @param version The unsupported version byte.
     */
    error UnsupportedExtraDataVersion(uint8 version);

    /**
     * @notice Error thrown when a key management request is already pending.
     */
    error KeyManagementRequestPending();

    /**
     * @notice Error thrown when the abort keygen ID is invalid.
     * @param prepKeygenId The invalid preprocessing keygen ID.
     */
    error AbortKeygenInvalidId(uint256 prepKeygenId);

    /**
     * @notice Error thrown when the keygen was already completed and cannot be aborted.
     * @param prepKeygenId The preprocessing keygen ID.
     */
    error AbortKeygenAlreadyDone(uint256 prepKeygenId);

    /**
     * @notice Error thrown when the abort CRS gen ID is invalid.
     * @param crsId The invalid CRS ID.
     */
    error AbortCrsgenInvalidId(uint256 crsId);

    /**
     * @notice Error thrown when the CRS gen was already completed and cannot be aborted.
     * @param crsId The CRS ID.
     */
    error AbortCrsgenAlreadyDone(uint256 crsId);

    /**
     * @notice Trigger an FHE key generation.
     * @param paramsType The type of FHE parameters to use.
     */
    function keygen(ParamsType paramsType) external;

    /**
     * @notice Handle the response of a preprocessing keygen request.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param signature The signature of the KMS node that has responded.
     */
    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external;

    /**
     * @notice Handle the response of a keygen request.
     * @param keyId The ID of the key.
     * @param keyDigests The digests of the generated keys.
     * @param signature The signature of the KMS node that has responded.
     */
    function keygenResponse(uint256 keyId, KeyDigest[] calldata keyDigests, bytes calldata signature) external;

    /**
     * @notice Trigger a CRS generation.
     * @param maxBitLength The max bit length for generating the CRS.
     * @param paramsType The type of parameters to use.
     */
    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external;

    /**
     * @notice Handle the response of a CRS generation.
     * @param crsId The ID of the generated CRS.
     * @param crsDigest The digest of the generated CRS.
     * @param signature The signature of the KMS node that has responded.
     */
    function crsgenResponse(uint256 crsId, bytes calldata crsDigest, bytes calldata signature) external;

    /**
     * @notice Abort an ongoing keygen request.
     * @param prepKeygenId The ID of the preprocessing keygen to abort.
     */
    function abortKeygen(uint256 prepKeygenId) external;

    /**
     * @notice Abort an ongoing CRS generation request.
     * @param crsId The ID of the CRS generation to abort.
     */
    function abortCrsgen(uint256 crsId) external;

    /**
     * @notice Get the parameters type used for the key generation.
     * @param keyId The ID of the key.
     * @return The parameters type used for the key generation.
     */
    function getKeyParamsType(uint256 keyId) external view returns (ParamsType);

    /**
     * @notice Get the parameters type used for the CRS generation.
     * @param crsId The ID of the CRS.
     * @return The parameters type used for the CRS generation.
     */
    function getCrsParamsType(uint256 crsId) external view returns (ParamsType);

    /**
     * @notice Get the ID of the current active key.
     * @return The current ID of the active key.
     */
    function getActiveKeyId() external view returns (uint256);

    /**
     * @notice Get the ID of the current active CRS.
     * @return The current ID of the active CRS.
     */
    function getActiveCrsId() external view returns (uint256);

    /**
     * @notice Get the current key request counter.
     * @dev Request IDs are type-tagged in the high byte.
     * @return The current key request counter.
     */
    function getKeyCounter() external view returns (uint256);

    /**
     * @notice Get the current CRS request counter.
     * @dev Request IDs are type-tagged in the high byte.
     * @return The current CRS request counter.
     */
    function getCrsCounter() external view returns (uint256);

    /**
     * @notice Check whether a request has reached a terminal done state.
     * @dev `requestId` may be a prep-keygen, keygen, or CRS request ID; all share the same
     *      type-tagged request-ID space.
     * @param requestId The ID of the request.
     * @return Whether the request is done.
     */
    function isRequestDone(uint256 requestId) external view returns (bool);

    /**
     * @notice Get the KMS transaction sender addresses that propagated valid signatures for a request.
     * @dev Returns an empty list for requests that are pending or aborted, as no consensus digest is stored.
     * @param requestId The ID of the request.
     * @return The KMS transaction sender addresses.
     */
    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);

    /**
     * @notice Get the key materials for a given key ID.
     * @param keyId The ID of the key.
     * @return The key materials (storage URLs, key digests).
     */
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, KeyDigest[] memory);

    /**
     * @notice Get the CRS materials for a given CRS ID.
     * @param crsId The ID of the CRS.
     * @return The CRS materials (storage URLs, CRS digest).
     */
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);

    /**
     * @notice Returns the versions of the KMSGeneration contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

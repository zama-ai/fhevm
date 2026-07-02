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
     * @notice The full record of a completed key, used to seed resharing of previous materials.
     * @param prepKeygenId The preprocessing keygen ID the key derives from.
     * @param keyId The key ID.
     * @param paramsType The key parameters type.
     * @param keyDigests The per-type digests of the key.
     */
    struct KeyInfo {
        uint256 prepKeygenId;
        uint256 keyId;
        ParamsType paramsType;
        KeyDigest[] keyDigests;
    }

    /**
     * @notice Emitted to trigger an FHE key generation preprocessing.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param paramsType The type of the parameters to use.
     * @param extraData Additional context data (0x01 || contextId, or 0x02 || contextId || epochId).
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
     * @param kmsNodeStorageUrls The KMS nodes' storage URLs that participated in the consensus.
     * @param keyDigests The digests of the generated keys.
     */
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    /**
     * @notice A per-host-chain cutover block for the one-time compressed-key migration (RFC-029).
     * @param chainId The host chain ID.
     * @param cutoverBlock The block number at which the chain switches to compressed material
     * (inclusive: blocks at or after it use the compressed material).
     */
    struct HostChainCutover {
        uint256 chainId;
        uint64 cutoverBlock;
    }

    /**
     * @notice Emitted to trigger the preprocessing round of the one-time compressed-key
     * migration keygen (RFC-029): re-materializing compressed key material for an existing,
     * already-active key. Not a key rotation: the key and its ID never change, and this flow
     * never activates anything.
     * @param prepKeygenId The ID of the preprocessing request.
     * @param keyId The ID of the existing key whose material is being re-materialized.
     * @param paramsType The parameters type of the existing key.
     * @param extraData Additional context data.
     */
    event CompressedKeyMigrationPrepKeygenRequest(
        uint256 prepKeygenId,
        uint256 keyId,
        ParamsType paramsType,
        bytes extraData
    );

    /**
     * @notice Emitted when the migration preprocessing reaches consensus, triggering the
     * compressed-key migration keygen itself.
     * @param prepKeygenId The ID of the preprocessing request.
     * @param migrationRequestId The migration keygen request ID. This is a request correlation
     * handle, never a key identity: it must not appear anywhere outside this request lifecycle.
     * @param keyId The ID of the existing key whose material is being re-materialized.
     * @param extraData Additional context data.
     */
    event CompressedKeyMigrationKeygenRequest(
        uint256 prepKeygenId,
        uint256 migrationRequestId,
        uint256 keyId,
        bytes extraData
    );

    /**
     * @notice Emitted when a KMS node has responded to a compressed-key migration keygen request.
     * @param migrationRequestId The migration keygen request ID.
     * @param keyDigests The digests of the re-materialized compressed key material.
     * @param signature The signature of the KMS node that has responded.
     * @param kmsTxSender The transaction sender of the KMS node that has called the function.
     */
    event CompressedKeyMaterialResponse(
        uint256 migrationRequestId,
        KeyDigest[] keyDigests,
        bytes signature,
        address kmsTxSender
    );

    /**
     * @notice Emitted when KMS consensus is reached on the re-materialized compressed key
     * material for an existing key. Does not activate anything: the active key is unchanged
     * and workers only start using the material at the scheduled cutover.
     * @param keyId The ID of the existing key.
     * @param kmsNodeStorageUrls The KMS nodes' storage URLs that participated in the consensus.
     * @param keyDigests The digests of the compressed key material.
     */
    event CompressedKeyMaterialAdded(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    /**
     * @notice Emitted when governance schedules the one-time compressed-key cutover.
     * @param keyId The ID of the existing key.
     * @param hostChainCutovers The per-host-chain cutover blocks.
     * @param gatewayCutoverBlock The Gateway block at which input verification switches
     * (inclusive on the compressed side).
     */
    event CompressedKeyCutoverScheduled(
        uint256 keyId,
        HostChainCutover[] hostChainCutovers,
        uint64 gatewayCutoverBlock
    );

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
     * @param kmsNodeStorageUrls The KMS nodes' storage URLs that participated in the consensus.
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
     * @notice Error thrown when the recovered signer is not a registered KMS signer for the context.
     * @param signerAddress The recovered signer.
     */
    error NotKmsSigner(address signerAddress);

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
     * @notice Error thrown when a response is sent to the wrong endpoint: a normal keygen
     * response for a migration request, or a migration response for a normal keygen request.
     * @param requestId The mismatched request ID.
     */
    error WrongKeygenResponseEndpoint(uint256 requestId);

    /**
     * @notice Error thrown when compressed key materials already exist for the key.
     * @param keyId The ID of the key.
     */
    error CompressedKeyMaterialsAlreadyAdded(uint256 keyId);

    /**
     * @notice Error thrown when compressed key materials do not exist (yet) for the key.
     * @param keyId The ID of the key.
     */
    error CompressedKeyMaterialsNotAdded(uint256 keyId);

    /**
     * @notice Error thrown when a compressed-key cutover is already scheduled for the key.
     * @param keyId The ID of the key.
     */
    error CompressedKeyCutoverAlreadyScheduled(uint256 keyId);

    /**
     * @notice Error thrown when the host chain cutover list is empty.
     */
    error EmptyHostChainCutovers();

    /**
     * @notice Error thrown when the host chain cutover list contains a duplicate chain ID.
     * @param chainId The duplicated chain ID.
     */
    error DuplicateCutoverChainId(uint256 chainId);

    /**
     * @notice Error thrown when a cutover block is zero.
     */
    error InvalidCutoverBlock();

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
     * @notice Trigger the one-time compressed-key migration keygen for an existing key
     * (RFC-029). The key and its ID are unchanged; only new material bytes are produced.
     * This flow never emits {ActivateKey} and never changes the active key.
     * @param keyId The ID of the existing, generated key.
     */
    function compressedKeyMigrationKeygen(uint256 keyId) external;

    /**
     * @notice Handle a KMS node's response to a compressed-key migration keygen request and,
     * on consensus, publish the compressed key material digests for the existing key.
     * Reverts for normal keygen request IDs; see {WrongKeygenResponseEndpoint}.
     * @param migrationRequestId The migration keygen request ID.
     * @param keyDigests The digests of the compressed key material.
     * @param signature The signature of the KMS node that has responded.
     */
    function addCompressedKeyMaterials(
        uint256 migrationRequestId,
        KeyDigest[] calldata keyDigests,
        bytes calldata signature
    ) external;

    /**
     * @notice Schedule the one-time compressed-key cutover for an existing key whose
     * compressed materials have been published. Single-assignment: a second call reverts
     * even with identical values.
     * @param keyId The ID of the existing key.
     * @param hostChainCutovers The per-host-chain cutover blocks (non-empty, unique chain IDs).
     * @param gatewayCutoverBlock The Gateway block boundary for input verification.
     */
    function scheduleCompressedKeyCutover(
        uint256 keyId,
        HostChainCutover[] calldata hostChainCutovers,
        uint64 gatewayCutoverBlock
    ) external;

    /**
     * @notice Get the compressed key materials published for a given key ID.
     * @param keyId The ID of the key.
     * @return The compressed key materials (storage URLs, key digests).
     */
    function getCompressedKeyMaterials(uint256 keyId) external view returns (string[] memory, KeyDigest[] memory);

    /**
     * @notice Get the stored compressed-key cutover schedule for a given key ID.
     * @param keyId The ID of the key.
     * @return exists Whether a schedule is stored.
     * @return hostChainCutovers The per-host-chain cutover blocks.
     * @return gatewayCutoverBlock The Gateway block boundary.
     */
    function getCompressedKeyCutoverSchedule(
        uint256 keyId
    ) external view returns (bool exists, HostChainCutover[] memory hostChainCutovers, uint64 gatewayCutoverBlock);

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
     * @dev The returned list remains empty until the consensus is reached, including for aborted requests
     *      (no consensus digest is stored in either case).
     * @param requestId The ID of the request.
     * @return The KMS transaction sender addresses.
     */
    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);

    /**
     * @notice Get the key IDs that reached consensus.
     * @return The completed key IDs.
     */
    function getCompletedKeyIds() external view returns (uint256[] memory);

    /**
     * @notice Get the CRS IDs that reached consensus.
     * @return The completed CRS IDs.
     */
    function getCompletedCrsIds() external view returns (uint256[] memory);

    /**
     * @notice Get the key materials for a given key ID.
     * @param keyId The ID of the key.
     * @return The key materials (storage URLs, key digests).
     */
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, KeyDigest[] memory);

    /**
     * @notice Get the full completed-key record for a given key ID, in a single call.
     * @param keyId The ID of the key.
     * @return The key record (prepKeygenId, keyId, paramsType, key digests).
     */
    function getKeyInfo(uint256 keyId) external view returns (KeyInfo memory);

    /**
     * @notice Get the CRS materials for a given CRS ID.
     * @param crsId The ID of the CRS.
     * @return The CRS materials (storage URLs, CRS digest).
     */
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);

    /**
     * @notice Returns the version of the KMSGeneration contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

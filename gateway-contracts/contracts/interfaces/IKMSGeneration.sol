// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the KMSGeneration contract.
 * @notice The KMSGeneration contract is responsible for managing the KMS public materials used
 * within the fhevm protocol. These materials include FHE keys (logical, physical), KSKs (Key Switching Keys)
 * and CRS (Common Reference String).
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
     * @param epochId The ID of the resharing epoch.
     * @param paramsType The type of the parameters to use.
     */
    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, ParamsType paramsType);

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
     */
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId);

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
     */
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType);

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
     * @notice Emitted to trigger the initialization of the PRSS (Pseudo-Random Secret Sharing).
     * @dev This is a temporary event to initialize PRSS until implementation of a proper key resharing.
     */
    event PRSSInit();

    /**
     * @notice Emitted to trigger the reshare of the specified key ID.
     * @dev This is a temporary event to reshare the specified key ID until implementation of a proper key resharing.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param keyId The ID of the key to reshare.
     * @param keyReshareId The ID of the key reshare request.
     * @param paramsType The type of FHE parameters to use.
     */
    event KeyReshareSameSet(uint256 prepKeygenId, uint256 keyId, uint256 keyReshareId, ParamsType paramsType);

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
     * @notice Error thrown when a CRS has not been generated.
     * @param crsId The ID of the CRS.
     */
    error CrsNotGenerated(uint256 crsId);

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
     * @notice Trigger the initialization of the PRSS (Pseudo-Random Secret Sharing).
     * @dev This is a temporary method to initialize PRSS until implementation of a proper key resharing.
     */
    function prssInit() external;

    /**
     * @notice Trigger the reshare of the given key ID.
     * @dev This is a temporary method to reshare the specified key ID until implementation of a proper key resharing.
     * @param keyId The ID of the key to reshare.
     */
    function keyReshareSameSet(uint256 keyId) external;

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
     * @notice Get the KMS transaction sender addresses that propagated valid signatures for a request.
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

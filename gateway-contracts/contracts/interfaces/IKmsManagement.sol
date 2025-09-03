// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the KmsManagement contract.
 * @notice The KmsManagement contract is responsible for managing the KMS public materials used
 * within the fhevm protocol. These materials include FHE keys (logical, physical), KSKs (Key Switching Keys)
 * and CRS (Common Reference String).
 */
interface IKmsManagement {
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
     * @notice Emitted to trigger an FHE key generation.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param keyId The ID of the key to generate.
     */
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId);

    /**
     * @notice Emitted when the key is activated.
     * @param keyId The ID of the activated key.
     * @param kmsNodeS3BucketUrls The KMS nodes' s3 bucket URL that participated in the consensus.
     * @param keyDigests The digests of the generated keys.
     */
    event ActivateKey(uint256 keyId, string[] kmsNodeS3BucketUrls, KeyDigest[] keyDigests);

    /**
     * @notice Emitted to trigger a CRS (Common Reference String) generation.
     * @param crsId The ID of the CRS to generate.
     * @param maxBitLength The max bit length for generating the CRS.
     * @param paramsType The type of CRS parameters to use.
     */
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType);

    /**
     * @notice Emitted when the CRS is activated.
     * @param crsId The ID of the generated CRS.
     * @param kmsNodeS3BucketUrls The KMS nodes' s3 bucket URL that participated in the consensus.
     * @param crsDigest The digest of the generated CRS.
     */
    event ActivateCrs(uint256 crsId, string[] kmsNodeS3BucketUrls, bytes crsDigest);

    /**
     * @notice Error thrown when a KMS node has already signed for a preprocessing keygen response.
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param kmsSigner The signer address of the KMS node.
     */
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);

    /**
     * @notice Error thrown when a KMS node has already signed for a keygen response.
     * @param keyId The ID of the key.
     * @param kmsSigner The signer address of the KMS node.
     */
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);

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
    function keygenRequest(ParamsType paramsType) external;

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
     * @return The key materials (s3 bucket URLs, key digests).
     */
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, KeyDigest[] memory);

    /**
     * @notice Get the CRS materials for a given CRS ID.
     * @param crsId The ID of the CRS.
     * @return The CRS materials (s3 bucket URLs, CRS digest).
     */
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);

    /**
     * @notice Returns the versions of the KmsManagement contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}

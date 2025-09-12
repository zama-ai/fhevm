// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev This contract is a mock of the KmsManagement contract from the Gateway.
/// source: github.com/zama-ai/fhevm/blob/main/gateway-contracts/contracts/KmsManagement.sol
contract KmsManagement {
    /**
     * @notice Emitted to activate the key in coprocessors.
     * @param keyId The ID of the key requested for activation.
     */
     enum KeyType {
        Server, // 0
        Public // 1
    }

    struct KeyDigest {
        /// @notice The type of the generated key.
        KeyType keyType;
        /// @notice The digest of the generated key.
        bytes digest;
    }
    /**
     * @notice Emitted when the key is activated.
     * @param keyId The ID of the activated key.
     * @param kmsNodeS3BucketUrls The KMS nodes' s3 bucket URL that participated in the consensus.
     * @param keyDigests The digests of the generated keys.
     */
    event ActivateKey(uint256 keyId, string[] kmsNodeS3BucketUrls, KeyDigest[] keyDigests);

    function activateKey(uint256 keyId) external virtual {
        string[] memory urls = new string[](1);
        urls[0] = "https://s3.amazonaws.com/bucket-name-1/PUB-p1/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088";
        KeyDigest[] memory digests = new KeyDigest[](1);
        digests[0] = KeyDigest({ keyType: KeyType.Server, digest: "digest" });
        emit ActivateKey(keyId, urls, digests);
    }
}

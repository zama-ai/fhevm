// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev Test mock of the host-chain KMSGeneration contract.
/// Event signatures match host-contracts/contracts/interfaces/IKMSGeneration.sol so that
/// the production event decoders in host-listener can decode events emitted by this mock.
contract KMSGeneration {
    enum ParamsType {
        Default,
        Test
    }

    enum KeyType {
        Server,
        Public
    }

    struct KeyDigest {
        KeyType keyType;
        bytes digest;
    }

    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);
    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);

    function keygen_public_key() external {
        uint256 keyId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://test-bucket1.s3.region.amazonaws.com";
        urls[1] = "https://s3.region.amazonaws.com/test-bucket2";
        urls[2] = "https://s3.region.amazonaws.com/test-bucket3";
        urls[3] = "https://s3.region.amazonaws.com/test-bucket4";
        KeyDigest[] memory digests = new KeyDigest[](1);
        // python: bytes([..]) hash for "key_bytes"
        digests[0] = KeyDigest({ keyType: KeyType.Public, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        emit ActivateKey(keyId, urls, digests);
    }

    function keygen_server_key() external {
        uint256 keyId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://s3.region.amazonaws.com/test-bucket1";
        urls[1] = "https://s3.region.amazonaws.com/test-bucket2";
        urls[2] = "https://s3.region.amazonaws.com/test-bucket3";
        urls[3] = "https://s3.region.amazonaws.com/test-bucket4";
        KeyDigest[] memory digests = new KeyDigest[](1);
        // python: bytes([..]) hash for "key_bytes"
        digests[0] = KeyDigest({ keyType: KeyType.Server, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        emit ActivateKey(keyId, urls, digests);
    }

    function keygen(ParamsType /*paramsType*/) external {
        uint256 keyId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://s3.region.amazonaws.com/test-bucket1";
        urls[1] = "https://s3.region.amazonaws.com/test-bucket2";
        urls[2] = "https://s3.region.amazonaws.com/test-bucket3";
        urls[3] = "https://s3.region.amazonaws.com/test-bucket4";
        KeyDigest[] memory digests = new KeyDigest[](2);
        // python: bytes([..]) hash for "key_bytes"
        digests[0] = KeyDigest({ keyType: KeyType.Public, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        digests[1] = KeyDigest({ keyType: KeyType.Server, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        emit ActivateKey(keyId, urls, digests);
    }

    function crsgen() external {
        uint256 crsId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://s3.region.amazonaws.com/test-bucket1";
        urls[1] = "https://s3.region.amazonaws.com/test-bucket2";
        urls[2] = "https://s3.region.amazonaws.com/test-bucket3";
        urls[3] = "https://s3.region.amazonaws.com/test-bucket4";
        // python: bytes([..]) hash for "key_bytes"
        emit ActivateCrs(crsId, urls, '9\xf1\xe6"\xf9L\xe2\xd9(\xf7DlBNZzg\xe1\xc8\x94\x0f\xa6\x95\xacJ\x8b\xc0\xdc\x86\xd0\x93$');
    }
}

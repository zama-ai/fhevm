// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;
import "contracts/interfaces/IKMSGeneration.sol";

/// @dev This contract is a mock of the KmsManagement contract from the Gateway.
/// source: github.com/zama-ai/fhevm/blob/main/gateway-contracts/contracts/KmsManagement.sol
contract KMSGeneration is IKMSGeneration {
    function keygen_public_key() external {
        uint256 keyId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://s3.amazonaws.com/test-bucket1/PUB-P1";
        urls[1] = "https://s3.amazonaws.com/test-bucket2/PUB-P2";
        urls[2] = "https://s3.amazonaws.com/test-bucket3/PUB-P3";
        urls[3] = "https://s3.amazonaws.com/test-bucket4/PUB-P4";
        KeyDigest[] memory digests = new KeyDigest[](1);
        // python: bytes([..]) hash for "key_bytes"
        digests[0] = KeyDigest({ keyType: KeyType.Public, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        emit ActivateKey(keyId, urls, digests);
    }

    function keygen_server_key() external {
        uint256 keyId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://s3.amazonaws.com/test-bucket1/PUB-P1";
        urls[1] = "https://s3.amazonaws.com/test-bucket2/PUB-P2";
        urls[2] = "https://s3.amazonaws.com/test-bucket3/PUB-P3";
        urls[3] = "https://s3.amazonaws.com/test-bucket4/PUB-P4";
        KeyDigest[] memory digests = new KeyDigest[](1);
        // python: bytes([..]) hash for "key_bytes"
        digests[0] = KeyDigest({ keyType: KeyType.Server, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        emit ActivateKey(keyId, urls, digests);
    }

    function keygen(ParamsType paramsType) external {
        uint256 keyId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://s3.amazonaws.com/test-bucket1/PUB-P1";
        urls[1] = "https://s3.amazonaws.com/test-bucket2/PUB-P2";
        urls[2] = "https://s3.amazonaws.com/test-bucket3/PUB-P3";
        urls[3] = "https://s3.amazonaws.com/test-bucket4/PUB-P4";
        KeyDigest[] memory digests = new KeyDigest[](2);
        // python: bytes([..]) hash for "key_bytes"
        digests[0] = KeyDigest({ keyType: KeyType.Public, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        digests[1] = KeyDigest({ keyType: KeyType.Server, digest: "]\xe8\xc3\xa0e\xd7H\xb7\xb7\xaf)\x1f\xc3\x0cR\x85\x00m\xaf\xbe\xad\x9e\xd5\x1e\xb7\xd4\xdd\xebN\xb2JV"});
        emit ActivateKey(keyId, urls, digests);
    }


    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        uint256 keyId = 16;
        string[] memory urls = new string[](4);
        urls[0] = "https://s3.amazonaws.com/test-bucket1/PUB-P1";
        urls[1] = "https://s3.amazonaws.com/test-bucket2/PUB-P2";
        urls[2] = "https://s3.amazonaws.com/test-bucket3/PUB-P3";
        urls[3] = "https://s3.amazonaws.com/test-bucket4/PUB-P4";
        // python: bytes([..]) hash for "key_bytes"
        emit ActivateCrs(keyId, urls, '9\xf1\xe6"\xf9L\xe2\xd9(\xf7DlBNZzg\xe1\xc8\x94\x0f\xa6\x95\xacJ\x8b\xc0\xdc\x86\xd0\x93$');
    }

    function crsgen() external {
        uint256 keyId = 1;
        this.crsgenRequest(1, ParamsType.Default);
    }

    function crsgenResponse(uint256 crsId, bytes calldata crsDigest, bytes calldata signature) external {}
    function getActiveCrsId() external view returns (uint256) {}
    function getActiveKeyId() external view returns (uint256) {}
    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory) {}
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory) {}
    function getCrsParamsType(uint256 crsId) external view returns (ParamsType) {}
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, KeyDigest[] memory) {}
    function getKeyParamsType(uint256 keyId) external view returns (ParamsType) {}
    function getVersion() external pure returns (string memory) {}
    function keygenResponse(uint256 keyId, KeyDigest[] calldata keyDigests, bytes calldata signature) external {}
    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external {}
}

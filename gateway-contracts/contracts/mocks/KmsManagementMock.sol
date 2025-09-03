// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract KmsManagementMock {
    struct KeyDigest {
        KeyType keyType;
        bytes digest;
    }

    enum ParamsType {
        Default,
        Test
    }

    enum KeyType {
        Server,
        Public
    }

    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, ParamsType paramsType);

    event KeygenRequest(uint256 prepKeygenId, uint256 keyId);

    event ActivateKey(uint256 keyId, string[] kmsNodeS3BucketUrls, KeyDigest[] keyDigests);

    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType);

    event ActivateCrs(uint256 crsId, string[] kmsNodeS3BucketUrls, bytes crsDigest);

    uint256 prepKeygenCounter = 3 << 248;
    uint256 keyCounter = 4 << 248;
    uint256 crsCounter = 5 << 248;

    function keygenRequest(ParamsType paramsType) external {
        prepKeygenCounter++;
        uint256 prepKeygenId = prepKeygenCounter;
        uint256 epochId;

        emit PrepKeygenRequest(prepKeygenId, epochId, paramsType);
    }

    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external {
        keyCounter++;
        uint256 keyId = keyCounter;

        emit KeygenRequest(prepKeygenId, keyId);
    }

    function keygenResponse(uint256 keyId, KeyDigest[] calldata keyDigests, bytes calldata signature) external {
        string[] memory kmsNodeS3BucketUrls = new string[](1);

        emit ActivateKey(keyId, kmsNodeS3BucketUrls, keyDigests);
    }

    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        crsCounter++;
        uint256 crsId = crsCounter;

        emit CrsgenRequest(crsId, maxBitLength, paramsType);
    }

    function crsgenResponse(uint256 crsId, bytes calldata crsDigest, bytes calldata signature) external {
        string[] memory kmsNodeS3BucketUrls = new string[](1);

        emit ActivateCrs(crsId, kmsNodeS3BucketUrls, crsDigest);
    }
}

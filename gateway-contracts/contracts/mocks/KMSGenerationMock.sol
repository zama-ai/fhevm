// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract KMSGenerationMock {
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

    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);

    event KeygenRequest(uint256 prepKeygenId, uint256 keyId);

    event KeygenResponse(uint256 keyId, KeyDigest[] keyDigests, bytes extraData, bytes signature, address kmsTxSender);

    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType);

    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes extraData, bytes signature, address kmsTxSender);

    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);

    uint256 prepKeygenCounter = 3 << 248;
    uint256 keyCounter = 4 << 248;
    uint256 crsCounter = 5 << 248;

    function keygen(ParamsType paramsType) external {
        prepKeygenCounter++;
        uint256 prepKeygenId = prepKeygenCounter;
        uint256 epochId;

        emit PrepKeygenRequest(prepKeygenId, epochId, paramsType);
    }

    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external {
        address kmsTxSender;
        keyCounter++;
        uint256 keyId = keyCounter;

        emit PrepKeygenResponse(prepKeygenId, signature, kmsTxSender);

        emit KeygenRequest(prepKeygenId, keyId);
    }

    function keygenResponse(
        uint256 keyId,
        KeyDigest[] calldata keyDigests,
        bytes calldata extraData,
        bytes calldata signature
    ) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit KeygenResponse(keyId, keyDigests, extraData, signature, kmsTxSender);

        emit ActivateKey(keyId, kmsNodeStorageUrls, keyDigests);
    }

    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        crsCounter++;
        uint256 crsId = crsCounter;

        emit CrsgenRequest(crsId, maxBitLength, paramsType);
    }

    function crsgenResponse(
        uint256 crsId,
        bytes calldata crsDigest,
        bytes calldata extraData,
        bytes calldata signature
    ) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit CrsgenResponse(crsId, crsDigest, extraData, signature, kmsTxSender);

        emit ActivateCrs(crsId, kmsNodeStorageUrls, crsDigest);
    }
}

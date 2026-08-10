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
        Public,
        Reserved,
        CompressedKeySet
    }

    event PrepKeygenRequest(uint256 prepKeygenId, ParamsType paramsType, uint256 existingKeyId, bytes extraData);

    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);

    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, uint256 existingKeyId, bytes extraData);

    event KeygenResponse(uint256 keyId, KeyDigest[] keyDigests, bytes signature, address kmsTxSender);

    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType, bytes extraData);

    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);

    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);

    event AbortKeygen(uint256 prepKeygenId);

    event AbortCrsgen(uint256 crsId);

    uint256 prepKeygenCounter = 3 << 248;
    uint256 keyCounter = 4 << 248;
    uint256 crsCounter = 5 << 248;
    mapping(uint256 prepKeygenId => uint256 keyId) keyIdByPrepKeygenId;
    mapping(uint256 keyId => uint256 existingKeyId) existingKeyIdByKeyId;

    function keygen(ParamsType paramsType, uint256 existingKeyId) external {
        prepKeygenCounter++;
        uint256 prepKeygenId = prepKeygenCounter;
        keyCounter++;
        uint256 keyId = keyCounter;
        keyIdByPrepKeygenId[prepKeygenId] = keyId;
        existingKeyIdByKeyId[keyId] = existingKeyId;

        emit PrepKeygenRequest(prepKeygenId, paramsType, existingKeyId, "");
    }

    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external {
        address kmsTxSender;
        uint256 keyId = keyIdByPrepKeygenId[prepKeygenId];
        if (keyId == 0) {
            keyCounter++;
            keyId = keyCounter;
        }

        emit PrepKeygenResponse(prepKeygenId, signature, kmsTxSender);

        emit KeygenRequest(prepKeygenId, keyId, existingKeyIdByKeyId[keyId], "");
    }

    function keygenResponse(uint256 keyId, KeyDigest[] calldata keyDigests, bytes calldata signature) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit KeygenResponse(keyId, keyDigests, signature, kmsTxSender);
        if (existingKeyIdByKeyId[keyId] == 0) {
            emit ActivateKey(keyId, kmsNodeStorageUrls, keyDigests);
        }
    }

    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        crsCounter++;
        uint256 crsId = crsCounter;

        emit CrsgenRequest(crsId, maxBitLength, paramsType, "");
    }

    function crsgenResponse(uint256 crsId, bytes calldata crsDigest, bytes calldata signature) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit CrsgenResponse(crsId, crsDigest, signature, kmsTxSender);

        emit ActivateCrs(crsId, kmsNodeStorageUrls, crsDigest);
    }

    function abortKeygen(uint256 prepKeygenId) external {
        emit AbortKeygen(prepKeygenId);
    }

    function abortCrsgen(uint256 crsId) external {
        emit AbortCrsgen(crsId);
    }
}

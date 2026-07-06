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

    enum KeygenMode {
        Fresh,
        FromExisting
    }

    event PrepKeygenRequest(
        uint256 prepKeygenId,
        ParamsType paramsType,
        KeygenMode mode,
        uint256 existingKeyId,
        bytes extraData
    );

    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);

    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, KeygenMode mode, uint256 existingKeyId, bytes extraData);

    event KeygenResponse(uint256 keyId, KeyDigest[] keyDigests, bytes signature, address kmsTxSender);

    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType, bytes extraData);

    event AbortKeygen(uint256 prepKeygenId);

    event AbortCrsgen(uint256 crsId);

    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);

    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);

    uint256 prepKeygenCounter = 3 << 248;
    uint256 keyCounter = 4 << 248;
    uint256 crsCounter = 5 << 248;

    function keygen(ParamsType paramsType, KeygenMode mode, uint256 existingKeyId) external {
        prepKeygenCounter++;
        uint256 prepKeygenId = prepKeygenCounter;
        keyCounter++;
        uint256 keyId = keyCounter;

        emit PrepKeygenRequest(prepKeygenId, paramsType, mode, existingKeyId, "");

        // Mock convenience: FromExisting emits the keygen request in the same
        // tx so listener tests exercise the mode-carrying event directly.
        if (mode == KeygenMode.FromExisting) {
            emit KeygenRequest(prepKeygenId, keyId, mode, existingKeyId, "");
        }
    }

    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external {
        address kmsTxSender;
        keyCounter++;
        uint256 keyId = keyCounter;

        emit PrepKeygenResponse(prepKeygenId, signature, kmsTxSender);

        emit KeygenRequest(prepKeygenId, keyId, KeygenMode.Fresh, 0, "");
    }

    function keygenResponse(uint256 keyId, KeyDigest[] calldata keyDigests, bytes calldata signature) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit KeygenResponse(keyId, keyDigests, signature, kmsTxSender);

        emit ActivateKey(keyId, kmsNodeStorageUrls, keyDigests);
    }


    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        crsCounter++;
        uint256 crsId = crsCounter;

        emit CrsgenRequest(crsId, maxBitLength, paramsType, "");
    }

    function abortKeygen(uint256 prepKeygenId) external {
        emit AbortKeygen(prepKeygenId);
    }

    function abortCrsgen(uint256 crsId) external {
        emit AbortCrsgen(crsId);
    }

    function getKeyCounter() external view returns (uint256) {
        return keyCounter;
    }

    function getCrsCounter() external view returns (uint256) {
        return crsCounter;
    }

    function crsgenResponse(uint256 crsId, bytes calldata crsDigest, bytes calldata signature) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit CrsgenResponse(crsId, crsDigest, signature, kmsTxSender);

        emit ActivateCrs(crsId, kmsNodeStorageUrls, crsDigest);
    }
}

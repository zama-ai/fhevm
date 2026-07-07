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
        CompressedPublic,
        CompressedKeyset
    }

    enum KeygenRequestKind {
        Fresh,
        Migration
    }

    event PrepKeygenRequest(
        uint256 prepKeygenId,
        ParamsType paramsType,
        KeygenRequestKind requestKind,
        uint256 keyId,
        bytes extraData
    );

    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);

    event KeygenRequest(
        uint256 prepKeygenId,
        uint256 requestId,
        KeygenRequestKind requestKind,
        uint256 keyId,
        bytes extraData
    );

    event KeygenResponse(uint256 keyId, KeyDigest[] keyDigests, bytes signature, address kmsTxSender);

    event MigrationResponse(uint256 migrationRequestId, KeyDigest[] keyDigests, bytes signature, address kmsTxSender);

    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    event CompressedKeyMaterialAdded(uint256 indexed keyId, string[] kmsNodeStorageUrls, KeyDigest[] keyDigests);

    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType, bytes extraData);

    event AbortKeygen(uint256 prepKeygenId);

    event AbortCrsgen(uint256 crsId);

    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);

    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);

    uint256 prepKeygenCounter = 3 << 248;
    uint256 keyCounter = 4 << 248;
    uint256 crsCounter = 5 << 248;
    mapping(uint256 migrationRequestId => uint256 keyId) keyIdByMigrationRequestId;

    function keygen(ParamsType paramsType) external {
        prepKeygenCounter++;
        uint256 prepKeygenId = prepKeygenCounter;
        keyCounter++;
        uint256 keyId = keyCounter;

        emit PrepKeygenRequest(prepKeygenId, paramsType, KeygenRequestKind.Fresh, keyId, '');
    }

    function migrateKey(uint256 keyId) external {
        prepKeygenCounter++;
        uint256 prepKeygenId = prepKeygenCounter;
        keyCounter++;
        uint256 migrationRequestId = keyCounter;
        keyIdByMigrationRequestId[migrationRequestId] = keyId;

        emit PrepKeygenRequest(prepKeygenId, ParamsType.Default, KeygenRequestKind.Migration, keyId, '');
        emit KeygenRequest(prepKeygenId, migrationRequestId, KeygenRequestKind.Migration, keyId, '');
    }

    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external {
        address kmsTxSender;
        keyCounter++;
        uint256 keyId = keyCounter;

        emit PrepKeygenResponse(prepKeygenId, signature, kmsTxSender);

        emit KeygenRequest(prepKeygenId, keyId, KeygenRequestKind.Fresh, keyId, '');
    }

    function keygenResponse(uint256 keyId, KeyDigest[] calldata keyDigests, bytes calldata signature) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit KeygenResponse(keyId, keyDigests, signature, kmsTxSender);

        emit ActivateKey(keyId, kmsNodeStorageUrls, keyDigests);
    }

    function migrationResponse(
        uint256 migrationRequestId,
        KeyDigest[] calldata keyDigests,
        bytes calldata signature
    ) external {
        address kmsTxSender;
        string[] memory kmsNodeStorageUrls = new string[](1);

        emit MigrationResponse(migrationRequestId, keyDigests, signature, kmsTxSender);
        emit CompressedKeyMaterialAdded(keyIdByMigrationRequestId[migrationRequestId], kmsNodeStorageUrls, keyDigests);
    }

    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        crsCounter++;
        uint256 crsId = crsCounter;

        emit CrsgenRequest(crsId, maxBitLength, paramsType, '');
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

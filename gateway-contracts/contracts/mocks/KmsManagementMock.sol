// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract KmsManagementMock {
    struct PrepKeygenVerification {
        uint256 prepKeygenId;
    }

    struct KeygenVerification {
        uint256 prepKeygenId;
        uint256 keyId;
        bytes serverKeyDigest;
        bytes publicKeyDigest;
    }

    struct CrsgenVerification {
        uint256 crsId;
        uint256 maxBitLength;
        bytes crsDigest;
    }

    enum ParamsType {
        Default,
        Test
    }

    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, ParamsType paramsType);

    event KeygenRequest(uint256 prepKeygenId, uint256 keyId);

    event ActivateKey(uint256 keyId, string[] consensusS3BucketUrls);

    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType);

    event ActivateCrs(uint256 crsId, string[] consensusS3BucketUrls);

    uint256 prepKeygenCounter;
    uint256 keyCounter;
    uint256 crsCounter;

    function keygen(ParamsType paramsType) external {
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

    function keygenResponse(
        uint256 keyId,
        bytes calldata serverKeyDigest,
        bytes calldata publicKeyDigest,
        bytes calldata signature
    ) external {
        string[] memory consensusS3BucketUrls = new string[](1);

        emit ActivateKey(keyId, consensusS3BucketUrls);
    }

    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        crsCounter++;
        uint256 crsId = crsCounter;

        emit CrsgenRequest(crsId, maxBitLength, paramsType);
    }

    function crsgenResponse(uint256 crsId, bytes calldata crsDigest, bytes calldata signature) external {
        string[] memory consensusS3BucketUrls = new string[](1);

        emit ActivateCrs(crsId, consensusS3BucketUrls);
    }
}

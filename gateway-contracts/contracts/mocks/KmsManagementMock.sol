// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract KmsManagementMock {
    event PreprocessKeygenRequest(uint256 preKeygenRequestId, bytes32 fheParamsDigest);

    event PreprocessKeygenResponse(uint256 preKeygenRequestId, uint256 preKeyId);

    event PreprocessKskgenRequest(uint256 preKskgenRequestId, bytes32 fheParamsDigest);

    event PreprocessKskgenResponse(uint256 preKskgenRequestId, uint256 preKskId);

    event KeygenRequest(uint256 preKeyId, bytes32 fheParamsDigest);

    event KeygenResponse(uint256 preKeyId, uint256 keygenId, bytes32 fheParamsDigest);

    event CrsgenRequest(uint256 crsgenRequestId, bytes32 fheParamsDigest);

    event CrsgenResponse(uint256 crsgenRequestId, uint256 crsId, bytes32 fheParamsDigest);

    event KskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId, bytes32 fheParamsDigest);

    event KskgenResponse(uint256 preKskId, uint256 kskId, bytes32 fheParamsDigest);

    event ActivateKeyRequest(uint256 keyId);

    event ActivateKeyResponse(uint256 keyId);

    event AddFheParams(string fheParamsName, bytes32 fheParamsDigest);

    event UpdateFheParams(string fheParamsName, bytes32 fheParamsDigest);

    uint256 _preKeygenRequestCounter;
    uint256 _preKskgenRequestCounter;
    uint256 _crsgenRequestCounter;

    function preprocessKeygenRequest(string calldata fheParamsName) external {
        _preKeygenRequestCounter++;
        uint256 preKeygenRequestId = _preKeygenRequestCounter;
        bytes32 fheParamsDigest;

        emit PreprocessKeygenRequest(preKeygenRequestId, fheParamsDigest);
    }

    function preprocessKeygenResponse(uint256 preKeygenRequestId, uint256 preKeyId) external {
        emit PreprocessKeygenResponse(preKeygenRequestId, preKeyId);
    }

    function preprocessKskgenRequest(string calldata fheParamsName) external {
        _preKskgenRequestCounter++;
        uint256 preKskgenRequestId = _preKskgenRequestCounter;
        bytes32 fheParamsDigest;

        emit PreprocessKskgenRequest(preKskgenRequestId, fheParamsDigest);
    }

    function preprocessKskgenResponse(uint256 preKskgenRequestId, uint256 preKskId) external {
        emit PreprocessKskgenResponse(preKskgenRequestId, preKskId);
    }

    function keygenRequest(uint256 preKeyId) external {
        bytes32 fheParamsDigest;

        emit KeygenRequest(preKeyId, fheParamsDigest);
    }

    function keygenResponse(uint256 preKeyId, uint256 keyId) external {
        uint256 keygenId;
        bytes32 fheParamsDigest;

        emit KeygenResponse(preKeyId, keygenId, fheParamsDigest);
    }

    function crsgenRequest(string calldata fheParamsName) external {
        _crsgenRequestCounter++;
        uint256 crsgenRequestId = _crsgenRequestCounter;
        bytes32 fheParamsDigest;

        emit CrsgenRequest(crsgenRequestId, fheParamsDigest);
    }

    function crsgenResponse(uint256 crsgenRequestId, uint256 crsId) external {
        bytes32 fheParamsDigest;

        emit CrsgenResponse(crsgenRequestId, crsId, fheParamsDigest);
    }

    function kskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId) external {
        bytes32 fheParamsDigest;

        emit KskgenRequest(preKskId, sourceKeyId, destKeyId, fheParamsDigest);
    }

    function kskgenResponse(uint256 preKskId, uint256 kskId) external {
        bytes32 fheParamsDigest;

        emit KskgenResponse(preKskId, kskId, fheParamsDigest);
    }

    function activateKeyRequest(uint256 keyId) external {
        emit ActivateKeyRequest(keyId);
    }

    function activateKeyResponse(uint256 keyId) external {
        emit ActivateKeyResponse(keyId);
    }

    function addFheParams(string calldata fheParamsName, bytes32 fheParamsDigest) external {
        emit AddFheParams(fheParamsName, fheParamsDigest);
    }

    function updateFheParams(string calldata fheParamsName, bytes32 fheParamsDigest) external {
        emit UpdateFheParams(fheParamsName, fheParamsDigest);
    }
}

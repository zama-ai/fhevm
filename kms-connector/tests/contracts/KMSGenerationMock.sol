// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev Mock for the host-chain KMSGeneration contract.
/// Events include `extraData` to match host binding signatures.
contract KMSGenerationMock {
    enum ParamsType {
        Default,
        Test
    }

    event PrepKeygenRequest(uint256 prepKeygenId, uint256 epochId, ParamsType paramsType, bytes extraData);

    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);

    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, ParamsType paramsType, bytes extraData);

    uint256 prepKeygenCounter = 3 << 248;
    uint256 keyCounter = 4 << 248;
    uint256 crsCounter = 5 << 248;

    function keygen(ParamsType paramsType) external {
        prepKeygenCounter++;
        uint256 prepKeygenId = prepKeygenCounter;
        uint256 epochId;

        emit PrepKeygenRequest(prepKeygenId, epochId, paramsType, "");
    }

    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external {
        keyCounter++;
        uint256 keyId = keyCounter;

        emit KeygenRequest(prepKeygenId, keyId, "");
    }

    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external {
        crsCounter++;
        uint256 crsId = crsCounter;

        emit CrsgenRequest(crsId, maxBitLength, paramsType, "");
    }
}

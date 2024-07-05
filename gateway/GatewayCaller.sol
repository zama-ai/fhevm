// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "./lib/Gateway.sol";

abstract contract GatewayCaller {
    modifier onlyGateway() {
        require(msg.sender == Gateway.GatewayContractAddress());
        _;
    }
    mapping(uint256 => ebool[]) private paramsEBool;
    mapping(uint256 => euint4[]) private paramsEUint4;
    mapping(uint256 => euint8[]) private paramsEUint8;
    mapping(uint256 => euint16[]) private paramsEUint16;
    mapping(uint256 => euint32[]) private paramsEUint32;
    mapping(uint256 => euint64[]) private paramsEUint64;
    mapping(uint256 => eaddress[]) private paramsEAddress;
    mapping(uint256 => address[]) private paramsAddress;
    mapping(uint256 => uint256[]) private paramsUint256;
    mapping(uint256 => uint256[]) private requestedHandles;

    constructor() {}

    function addParamsEBool(uint256 requestID, ebool _ebool) internal {
        paramsEBool[requestID].push(_ebool);
    }

    function addParamsEUint4(uint256 requestID, euint4 _euint4) internal {
        paramsEUint4[requestID].push(_euint4);
    }

    function addParamsEUint8(uint256 requestID, euint8 _euint8) internal {
        paramsEUint8[requestID].push(_euint8);
    }

    function addParamsEUint16(uint256 requestID, euint16 _euint16) internal {
        paramsEUint16[requestID].push(_euint16);
    }

    function addParamsEUint32(uint256 requestID, euint32 _euint32) internal {
        paramsEUint32[requestID].push(_euint32);
    }

    function addParamsEUint64(uint256 requestID, euint64 _euint64) internal {
        paramsEUint64[requestID].push(_euint64);
    }

    function addParamsEAddress(uint256 requestID, eaddress _eaddress) internal {
        paramsEAddress[requestID].push(_eaddress);
    }

    function addParamsAddress(uint256 requestID, address _address) internal {
        paramsAddress[requestID].push(_address);
    }

    function addParamsUint256(uint256 requestID, uint256 _uint) internal {
        paramsUint256[requestID].push(_uint);
    }

    function saveRequestedHandles(uint256 requestID, uint256[] memory handlesList) internal {
        require(requestedHandles[requestID].length == 0, "requested handles already saved");
        requestedHandles[requestID] = handlesList;
    }

    function loadRequestedHandles(uint256 requestID) internal view returns (uint256[] memory) {
        require(requestedHandles[requestID].length != 0, "requested handles were not saved for this requestID");
        return requestedHandles[requestID];
    }

    function getParamsEBool(uint256 requestID) internal view returns (ebool[] memory) {
        return paramsEBool[requestID];
    }

    function getParamsEUint4(uint256 requestID) internal view returns (euint4[] memory) {
        return paramsEUint4[requestID];
    }

    function getParamsEUint8(uint256 requestID) internal view returns (euint8[] memory) {
        return paramsEUint8[requestID];
    }

    function getParamsEUint16(uint256 requestID) internal view returns (euint16[] memory) {
        return paramsEUint16[requestID];
    }

    function getParamsEUint32(uint256 requestID) internal view returns (euint32[] memory) {
        return paramsEUint32[requestID];
    }

    function getParamsEUint64(uint256 requestID) internal view returns (euint64[] memory) {
        return paramsEUint64[requestID];
    }

    function getParamsEAddress(uint256 requestID) internal view returns (eaddress[] memory) {
        return paramsEAddress[requestID];
    }

    function getParamsAddress(uint256 requestID) internal view returns (address[] memory) {
        return paramsAddress[requestID];
    }

    function getParamsUint256(uint256 requestID) internal view returns (uint256[] memory) {
        return paramsUint256[requestID];
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../lib/TFHE.sol";
import "./lib/Oracle.sol";

contract OracleCaller {
    modifier onlyOracle() {
        require(msg.sender == Oracle.OraclePredeployAddress());
        _;
    }
    mapping(uint256 => ebool[]) private paramsEBool;
    mapping(uint256 => euint8[]) private paramsEUint8;
    mapping(uint256 => euint16[]) private paramsEUint16;
    mapping(uint256 => euint32[]) private paramsEUint32;
    mapping(uint256 => euint64[]) private paramsEUint64;
    mapping(uint256 => address[]) private paramsAddress;
    mapping(uint256 => uint[]) private paramsUint;

    constructor() {}

    function addParamsEBool(uint256 requestID, ebool _ebool) internal {
        paramsEBool[requestID].push(_ebool);
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

    function addParamsAddress(uint256 requestID, address _address) internal {
        paramsAddress[requestID].push(_address);
    }

    function addParamsUint(uint256 requestID, uint256 _uint) internal {
        paramsUint[requestID].push(_uint);
    }

    function getParamsEBool(uint256 requestID) internal view returns (ebool[] memory) {
        return paramsEBool[requestID];
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

    function getParamsAddress(uint256 requestID) internal view returns (address[] memory) {
        return paramsAddress[requestID];
    }

    function getParamsUint(uint256 requestID) internal view returns (uint256[] memory) {
        return paramsUint[requestID];
    }
}

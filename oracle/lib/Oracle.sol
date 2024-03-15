// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../OraclePredeploy.sol";
import {ORACLE_PREDEPLOY_ADDRESS} from "./PredeployAddress.sol";

OraclePredeploy constant oraclePredeploy = OraclePredeploy(ORACLE_PREDEPLOY_ADDRESS);

library Oracle {
    function OraclePredeployAddress() internal pure returns (address) {
        return ORACLE_PREDEPLOY_ADDRESS;
    }

    function requestDecryption(
        ebool[] memory ct,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 initialCounter) {
        initialCounter = oraclePredeploy.requestDecryptionEBool(ct, callbackSelector, msgValue, maxTimestamp);
    }

    function requestDecryption(
        euint4[] memory ct,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 initialCounter) {
        initialCounter = oraclePredeploy.requestDecryptionEUint4(ct, callbackSelector, msgValue, maxTimestamp);
    }

    function requestDecryption(
        euint8[] memory ct,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 initialCounter) {
        initialCounter = oraclePredeploy.requestDecryptionEUint8(ct, callbackSelector, msgValue, maxTimestamp);
    }

    function requestDecryption(
        euint16[] memory ct,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 initialCounter) {
        initialCounter = oraclePredeploy.requestDecryptionEUint16(ct, callbackSelector, msgValue, maxTimestamp);
    }

    function requestDecryption(
        euint32[] memory ct,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 initialCounter) {
        initialCounter = oraclePredeploy.requestDecryptionEUint32(ct, callbackSelector, msgValue, maxTimestamp);
    }

    function requestDecryption(
        euint64[] memory ct,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 initialCounter) {
        initialCounter = oraclePredeploy.requestDecryptionEUint64(ct, callbackSelector, msgValue, maxTimestamp);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../oracle/OraclePredeploy.sol";

contract OraclePredeployTest is OraclePredeploy {
    constructor(address predeployOwner) OraclePredeploy(predeployOwner) {}

    // ONLY FOR TESTING PURPOSE : decryptTest function MUST BE REMOVED IN PRODUCTION
    // This is used in the test to simulate the threshold decryption by the KMS after the request
    function decryptTestBool(uint256 requestID) external view onlyRelayer returns (bool) {
        return TFHE.decrypt(decryptionRequestsEBool[requestID].cts[0]);
    }

    // ONLY FOR TESTING PURPOSE : decryptTest function MUST BE REMOVED IN PRODUCTION
    // This is used in the test to simulate the threshold decryption by the KMS after the request
    function decryptTestUint8(uint256 requestID) external view onlyRelayer returns (uint8) {
        return TFHE.decrypt(decryptionRequestsEUint8[requestID].cts[0]);
    }

    // ONLY FOR TESTING PURPOSE : decryptTest function MUST BE REMOVED IN PRODUCTION
    // This is used in the test to simulate the threshold decryption by the KMS after the request
    function decryptTestUint16(uint256 requestID) external view onlyRelayer returns (uint16) {
        return TFHE.decrypt(decryptionRequestsEUint16[requestID].cts[0]);
    }

    // ONLY FOR TESTING PURPOSE : decryptTest function MUST BE REMOVED IN PRODUCTION
    // This is used in the test to simulate the threshold decryption by the KMS after the request
    function decryptTestUint32(uint256 requestID) external view onlyRelayer returns (uint32) {
        return TFHE.decrypt(decryptionRequestsEUint32[requestID].cts[0]);
    }

    // ONLY FOR TESTING PURPOSE : decryptTest function MUST BE REMOVED IN PRODUCTION
    // This is used in the test to simulate the threshold decryption by the KMS after the request
    function decryptTestUint64(uint256 requestID) external view onlyRelayer returns (uint64) {
        return TFHE.decrypt(decryptionRequestsEUint64[requestID].cts[0]);
    }
}

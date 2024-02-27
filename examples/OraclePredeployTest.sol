// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../oracle/OraclePredeploy.sol";

contract OraclePredeployTest is OraclePredeploy {
    constructor(address predeployOwner) OraclePredeploy(predeployOwner) {}

    // ONLY FOR TESTING PURPOSE : decryptTest function MUST BE REMOVED IN PRODUCTION
    // This is used in the test to simulate the threshold decryption by the KMS after the request
    function decryptTest(uint256 requestID) external view onlyRelayer returns (uint32) {
        return TFHE.decrypt(decryptionRequestsEUint32[requestID].cts[0]);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "../OraclePredeploy.sol";
import {ORACLE_CONTRACT_PREDEPLOY_ADDRESS} from "./PredeployAddress.sol";

OraclePredeploy constant oraclePredeploy = OraclePredeploy(ORACLE_CONTRACT_PREDEPLOY_ADDRESS);

library Oracle {
    function OraclePredeployAddress() internal pure returns (address) {
        return ORACLE_CONTRACT_PREDEPLOY_ADDRESS;
    }

    function toCiphertext(ebool newCT) internal returns (Ciphertext memory ct) {
        oraclePredeploy.approveEBool(newCT); // to make sure that ciphertext is honestly obtained
        ct = Ciphertext({ctHandle: ebool.unwrap(newCT), ctType: CiphertextType.EBOOL});
    }

    function toCiphertext(euint4 newCT) internal returns (Ciphertext memory ct) {
        oraclePredeploy.approveEUint4(newCT); // to make sure that ciphertext is honestly obtained
        ct = Ciphertext({ctHandle: euint4.unwrap(newCT), ctType: CiphertextType.EUINT4});
    }

    function toCiphertext(euint8 newCT) internal returns (Ciphertext memory ct) {
        oraclePredeploy.approveEUint8(newCT); // to make sure that ciphertext is honestly obtained
        ct = Ciphertext({ctHandle: euint8.unwrap(newCT), ctType: CiphertextType.EUINT8});
    }

    function toCiphertext(euint16 newCT) internal returns (Ciphertext memory ct) {
        oraclePredeploy.approveEUint16(newCT); // to make sure that ciphertext is honestly obtained
        ct = Ciphertext({ctHandle: euint16.unwrap(newCT), ctType: CiphertextType.EUINT16});
    }

    function toCiphertext(euint32 newCT) internal returns (Ciphertext memory ct) {
        oraclePredeploy.approveEUint32(newCT); // to make sure that ciphertext is honestly obtained
        ct = Ciphertext({ctHandle: euint32.unwrap(newCT), ctType: CiphertextType.EUINT32});
    }

    function toCiphertext(euint64 newCT) internal returns (Ciphertext memory ct) {
        oraclePredeploy.approveEUint64(newCT); // to make sure that ciphertext is honestly obtained
        ct = Ciphertext({ctHandle: euint64.unwrap(newCT), ctType: CiphertextType.EUINT64});
    }

    function toCiphertext(eaddress newCT) internal returns (Ciphertext memory ct) {
        oraclePredeploy.approveEAddress(newCT); // to make sure that ciphertext is honestly obtained
        ct = Ciphertext({ctHandle: eaddress.unwrap(newCT), ctType: CiphertextType.EADDRESS});
    }

    function requestDecryption(
        Ciphertext[] memory cts,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 requestID) {
        requestID = oraclePredeploy.requestDecryption(cts, callbackSelector, msgValue, maxTimestamp);
    }
}

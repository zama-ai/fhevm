// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.25;

import "../GatewayContract.sol";
import "../../lib/ACL.sol";

GatewayContract constant gatewayContract = GatewayContract(0xc8c9303Cd7F337fab769686B593B87DC3403E0ce); // Replace by GatewayContract address
ACL constant acl = ACL(0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92); // Replace by ACL address

library Gateway {
    function GatewayGatewayAddress() internal pure returns (address) {
        return address(gatewayContract);
    }

    function toCiphertext(ebool newCT) internal pure returns (Ciphertext memory ct) {
        ct = Ciphertext({ctHandle: ebool.unwrap(newCT), ctType: CiphertextType.EBOOL});
    }

    function toCiphertext(euint4 newCT) internal pure returns (Ciphertext memory ct) {
        ct = Ciphertext({ctHandle: euint4.unwrap(newCT), ctType: CiphertextType.EUINT4});
    }

    function toCiphertext(euint8 newCT) internal pure returns (Ciphertext memory ct) {
        ct = Ciphertext({ctHandle: euint8.unwrap(newCT), ctType: CiphertextType.EUINT8});
    }

    function toCiphertext(euint16 newCT) internal pure returns (Ciphertext memory ct) {
        ct = Ciphertext({ctHandle: euint16.unwrap(newCT), ctType: CiphertextType.EUINT16});
    }

    function toCiphertext(euint32 newCT) internal pure returns (Ciphertext memory ct) {
        ct = Ciphertext({ctHandle: euint32.unwrap(newCT), ctType: CiphertextType.EUINT32});
    }

    function toCiphertext(euint64 newCT) internal pure returns (Ciphertext memory ct) {
        ct = Ciphertext({ctHandle: euint64.unwrap(newCT), ctType: CiphertextType.EUINT64});
    }

    function toCiphertext(eaddress newCT) internal pure returns (Ciphertext memory ct) {
        ct = Ciphertext({ctHandle: eaddress.unwrap(newCT), ctType: CiphertextType.EADDRESS});
    }

    function requestDecryption(
        Ciphertext[] memory cts,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp
    ) internal returns (uint256 requestID) {
        uint256 len = cts.length;
        uint256[] memory ctsHandles = new uint256[](len);
        for (uint256 k = 0; k < len; k++) {
            ctsHandles[k] = cts[k].ctHandle;
        }
        acl.allowForDecryption(ctsHandles);
        requestID = gatewayContract.requestDecryption(cts, callbackSelector, msgValue, maxTimestamp);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./Common.sol";
import "./Impl.sol";

library Ciphertext {
    function asEuint8(bytes memory ciphertext) internal view returns (euint8) {
        return euint8.wrap(Impl.verify(ciphertext, Common.euint8_t));
    }

    function reencrypt(
        euint8 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint8.unwrap(ciphertext), Common.euint8_t);
    }

    function delegate(euint8 ciphertext) internal view {
        Impl.delegate(euint8.unwrap(ciphertext));
    }

    function requireCt(euint8 ciphertext) internal view {
        Impl.requireCt(euint8.unwrap(ciphertext));
    }

    function optimisticRequireCt(euint8 ciphertext) internal view {
        Impl.optimisticRequireCt(euint8.unwrap(ciphertext));
    }

    function asEuint16(
        bytes memory ciphertext
    ) internal view returns (euint16) {
        return euint16.wrap(Impl.verify(ciphertext, Common.euint16_t));
    }

    function reencrypt(
        euint16 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint16.unwrap(ciphertext), Common.euint16_t);
    }

    function delegate(euint16 ciphertext) internal view {
        Impl.delegate(euint16.unwrap(ciphertext));
    }

    function requireCt(euint16 ciphertext) internal view {
        Impl.requireCt(euint16.unwrap(ciphertext));
    }

    function optimisticRequireCt(euint16 ciphertext) internal view {
        Impl.optimisticRequireCt(euint16.unwrap(ciphertext));
    }

    function asEuint32(
        bytes memory ciphertext
    ) internal view returns (euint32) {
        return euint32.wrap(Impl.verify(ciphertext, Common.euint32_t));
    }

    function reencrypt(
        euint32 ciphertext
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint32.unwrap(ciphertext), Common.euint32_t);
    }

    function delegate(euint32 ciphertext) internal view {
        Impl.delegate(euint32.unwrap(ciphertext));
    }

    function requireCt(euint32 ciphertext) internal view {
        Impl.requireCt(euint32.unwrap(ciphertext));
    }

    function optimisticRequireCt(euint32 ciphertext) internal view {
        Impl.optimisticRequireCt(euint32.unwrap(ciphertext));
    }
}

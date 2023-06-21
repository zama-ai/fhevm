// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./Common.sol";
import "./Impl.sol";

library TFHE {
    function add(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function sub(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function mul(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.mul(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function lte(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lte(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function lt(euint8 a, euint8 b) internal view returns (euint8) {
        return euint8.wrap(Impl.lt(euint8.unwrap(a), euint8.unwrap(b)));
    }

    function add(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function sub(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function mul(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function lte(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.lte(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function lt(euint16 a, euint16 b) internal view returns (euint16) {
        return euint16.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(b)));
    }

    function add(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function sub(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function mul(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function lte(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.lte(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function lt(euint32 a, euint32 b) internal view returns (euint32) {
        return euint32.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(b)));
    }

    function cmux(
        euint8 control,
        euint8 a,
        euint8 b
    ) internal view returns (euint8) {
        return
            euint8.wrap(
                Impl.cmux(
                    euint8.unwrap(control),
                    euint8.unwrap(a),
                    euint8.unwrap(b)
                )
            );
    }

    function cmux(
        euint8 control,
        euint16 a,
        euint16 b
    ) internal view returns (euint16) {
        return
            euint16.wrap(
                Impl.cmux(
                    euint8.unwrap(control),
                    euint16.unwrap(a),
                    euint16.unwrap(b)
                )
            );
    }

    function cmux(
        euint8 control,
        euint32 a,
        euint32 b
    ) internal view returns (euint32) {
        return
            euint32.wrap(
                Impl.cmux(
                    euint8.unwrap(control),
                    euint32.unwrap(a),
                    euint32.unwrap(b)
                )
            );
    }

    function asEuint8(bytes memory ciphertext) internal view returns (euint8) {
        return euint8.wrap(Impl.verify(ciphertext, Common.euint8_t));
    }

    function asEuint8(uint256 value) internal view returns (euint8) {
        return euint8.wrap(Impl.trivialEncrypt(value, Common.euint8_t));
    }

    function reencrypt(
        euint8 ciphertext,
        bytes32 publicKey
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint8.unwrap(ciphertext), publicKey);
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

    function asEuint16(uint256 value) internal view returns (euint16) {
        return euint16.wrap(Impl.trivialEncrypt(value, Common.euint16_t));
    }

    function reencrypt(
        euint16 ciphertext,
        bytes32 publicKey
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint16.unwrap(ciphertext), publicKey);
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

    function asEuint32(uint256 value) internal view returns (euint32) {
        return euint32.wrap(Impl.trivialEncrypt(value, Common.euint32_t));
    }

    function reencrypt(
        euint32 ciphertext,
        bytes32 publicKey
    ) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint32.unwrap(ciphertext), publicKey);
    }

    function requireCt(euint32 ciphertext) internal view {
        Impl.requireCt(euint32.unwrap(ciphertext));
    }

    function optimisticRequireCt(euint32 ciphertext) internal view {
        Impl.optimisticRequireCt(euint32.unwrap(ciphertext));
    }

    // Returns the network public FHE key.
    function fhePubKey() internal view returns (bytes memory) {
        return Impl.fhePubKey();
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FhevmTest} from "../../src/testing/FhevmTest.sol";
import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";

import {
    ebool,
    euint8,
    euint16,
    euint32,
    euint64,
    euint128,
    euint256,
    eaddress,
    externalEbool,
    externalEuint8,
    externalEuint16,
    externalEuint32,
    externalEuint64,
    externalEuint128,
    externalEuint256,
    externalEaddress
} from "encrypted-types/EncryptedTypes.sol";

contract FhevmTestDecryptTest is FhevmTest {
    // ── Typed decrypt overloads ──────────────────────────────────────────

    function test_decrypt_bool_true() public {
        (externalEbool handle, bytes memory proof) = encryptBool(true, address(this));
        ebool verified =
            ebool.wrap(_executor.verifyInput(externalEbool.unwrap(handle), address(this), proof, FheType.Bool));
        assertTrue(decrypt(verified));
    }

    function test_decrypt_bool_false() public {
        (externalEbool handle, bytes memory proof) = encryptBool(false, address(this));
        ebool verified =
            ebool.wrap(_executor.verifyInput(externalEbool.unwrap(handle), address(this), proof, FheType.Bool));
        assertFalse(decrypt(verified));
    }

    function test_decrypt_uint8() public {
        (externalEuint8 handle, bytes memory proof) = encryptUint8(42, address(this));
        euint8 verified =
            euint8.wrap(_executor.verifyInput(externalEuint8.unwrap(handle), address(this), proof, FheType.Uint8));
        assertEq(decrypt(verified), 42);
    }

    function test_decrypt_uint16() public {
        (externalEuint16 handle, bytes memory proof) = encryptUint16(1_337, address(this));
        euint16 verified =
            euint16.wrap(_executor.verifyInput(externalEuint16.unwrap(handle), address(this), proof, FheType.Uint16));
        assertEq(decrypt(verified), 1_337);
    }

    function test_decrypt_uint32() public {
        (externalEuint32 handle, bytes memory proof) = encryptUint32(91_337, address(this));
        euint32 verified =
            euint32.wrap(_executor.verifyInput(externalEuint32.unwrap(handle), address(this), proof, FheType.Uint32));
        assertEq(decrypt(verified), 91_337);
    }

    function test_decrypt_uint64() public {
        (externalEuint64 handle, bytes memory proof) = encryptUint64(123_456_789, address(this));
        euint64 verified =
            euint64.wrap(_executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64));
        assertEq(decrypt(verified), 123_456_789);
    }

    function test_decrypt_uint128() public {
        uint128 value = type(uint128).max - 7;
        (externalEuint128 handle, bytes memory proof) = encryptUint128(value, address(this));
        euint128 verified = euint128.wrap(
            _executor.verifyInput(externalEuint128.unwrap(handle), address(this), proof, FheType.Uint128)
        );
        assertEq(decrypt(verified), value);
    }

    function test_decrypt_uint256() public {
        uint256 value = type(uint256).max - 5;
        (externalEuint256 handle, bytes memory proof) = encryptUint256(value, address(this));
        euint256 verified = euint256.wrap(
            _executor.verifyInput(externalEuint256.unwrap(handle), address(this), proof, FheType.Uint256)
        );
        assertEq(decrypt(verified), value);
    }

    function test_decrypt_address() public {
        address value = address(0xA11CE);
        (externalEaddress handle, bytes memory proof) = encryptAddress(value, address(this));
        eaddress verified = eaddress.wrap(
            _executor.verifyInput(externalEaddress.unwrap(handle), address(this), proof, FheType.Uint160)
        );
        assertEq(decrypt(verified), value);
    }

    // ── buildDecryptionProof ─────────────────────────────────────────────

    function test_buildDecryptionProof_singleHandle_verifiableByKMSVerifier() public {
        (externalEuint64 handle, bytes memory inputProof) = encryptUint64(9090, address(this));
        bytes32 verifiedHandle =
            _executor.verifyInput(externalEuint64.unwrap(handle), address(this), inputProof, FheType.Uint64);

        uint64 cleartext = decrypt(euint64.wrap(verifiedHandle));
        bytes memory decryptionProof = buildDecryptionProof(verifiedHandle, abi.encode(cleartext));

        bytes32[] memory handles = new bytes32[](1);
        handles[0] = verifiedHandle;
        bool verified =
            _kmsVerifier.verifyDecryptionEIP712KMSSignatures(handles, abi.encode(cleartext), decryptionProof);
        assertTrue(verified);
    }

    function test_buildDecryptionProof_multipleHandles_verifiableByKMSVerifier() public {
        (externalEuint64 h0, bytes memory p0) = encryptUint64(111, address(this));
        (externalEuint64 h1, bytes memory p1) = encryptUint64(222, address(this));

        bytes32 v0 = _executor.verifyInput(externalEuint64.unwrap(h0), address(this), p0, FheType.Uint64);
        bytes32 v1 = _executor.verifyInput(externalEuint64.unwrap(h1), address(this), p1, FheType.Uint64);

        bytes32[] memory handles = new bytes32[](2);
        handles[0] = v0;
        handles[1] = v1;

        uint64 clear0 = decrypt(euint64.wrap(v0));
        uint64 clear1 = decrypt(euint64.wrap(v1));
        bytes memory abiEncodedCleartexts = abi.encode(clear0, clear1);

        bytes memory decryptionProof = buildDecryptionProof(handles, abiEncodedCleartexts);
        bool verified = _kmsVerifier.verifyDecryptionEIP712KMSSignatures(handles, abiEncodedCleartexts, decryptionProof);
        assertTrue(verified);
    }

    function test_buildDecryptionProof_singleHandleOverload_matchesMultiHandle() public {
        (externalEuint64 handle, bytes memory inputProof) = encryptUint64(42, address(this));
        bytes32 verifiedHandle =
            _executor.verifyInput(externalEuint64.unwrap(handle), address(this), inputProof, FheType.Uint64);

        uint64 cleartext = decrypt(euint64.wrap(verifiedHandle));

        bytes memory proofSingle = buildDecryptionProof(verifiedHandle, abi.encode(cleartext));

        bytes32[] memory handles = new bytes32[](1);
        handles[0] = verifiedHandle;
        bytes memory proofMulti = buildDecryptionProof(handles, abi.encode(cleartext));

        assertEq(proofSingle, proofMulti);
    }
}

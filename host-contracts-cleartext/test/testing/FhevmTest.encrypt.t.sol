// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FhevmTest} from "../../src/testing/FhevmTest.sol";
import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";

import {
    externalEbool,
    externalEuint8,
    externalEuint16,
    externalEuint32,
    externalEuint64,
    externalEuint128,
    externalEuint256,
    externalEaddress
} from "encrypted-types/EncryptedTypes.sol";

/// @dev On-chain-model port of forge-fhevm's encrypt suite. The original asserted on the harness's
///      off-chain `_plaintexts` DB; here the cleartext lands in `CleartextACL` when the input is
///      consumed via `_executor.verifyInput`, so normalization is checked through `decrypt(handle)`.
contract FhevmTestEncryptTest is FhevmTest {
    function test_internalEncrypt_bool_nonzero_normalizesStoredPlaintext() public {
        (bytes32 handle, bytes memory proof) = _encrypt(2, FheType.Bool, address(this), address(this));
        _executor.verifyInput(handle, address(this), proof, FheType.Bool);
        assertEq(decrypt(handle), 1);
    }

    function test_internalEncrypt_bool_high_byte_only_normalizesToFalse() public {
        (bytes32 handle, bytes memory proof) = _encrypt(0x0100, FheType.Bool, address(this), address(this));
        _executor.verifyInput(handle, address(this), proof, FheType.Bool);
        assertEq(decrypt(handle), 0);
    }

    function test_encryptUint64_returnsValidHandle() public {
        (externalEuint64 handle, bytes memory proof) = encryptUint64(42, address(this));

        assertNotEq(externalEuint64.unwrap(handle), bytes32(0));
        assertGt(proof.length, 0);
    }

    function test_encryptUint64_handleHasCorrectType() public {
        (externalEuint64 handle,) = encryptUint64(42, address(this));

        assertEq(uint8(externalEuint64.unwrap(handle)[30]), uint8(FheType.Uint64));
    }

    function test_encryptUint64_storesPlaintextInACL() public {
        (externalEuint64 handle, bytes memory proof) = encryptUint64(42, address(this));

        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        assertEq(decrypt(externalEuint64.unwrap(handle)), 42);
    }

    function test_encryptUint64_proofVerifiableByInputVerifier() public {
        (externalEuint64 handle, bytes memory proof) = encryptUint64(42, address(this));

        bytes32 verified = _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        assertEq(verified, externalEuint64.unwrap(handle));
    }

    function test_encryptBool_works() public {
        (externalEbool handle, bytes memory proof) = encryptBool(true, address(this));
        assertEq(uint8(externalEbool.unwrap(handle)[30]), uint8(FheType.Bool));
        assertEq(
            _executor.verifyInput(externalEbool.unwrap(handle), address(this), proof, FheType.Bool),
            externalEbool.unwrap(handle)
        );
        assertEq(decrypt(externalEbool.unwrap(handle)), 1);
    }

    function test_encryptUint8_works() public {
        (externalEuint8 handle, bytes memory proof) = encryptUint8(13, address(this));
        assertEq(uint8(externalEuint8.unwrap(handle)[30]), uint8(FheType.Uint8));
        assertEq(
            _executor.verifyInput(externalEuint8.unwrap(handle), address(this), proof, FheType.Uint8),
            externalEuint8.unwrap(handle)
        );
        assertEq(decrypt(externalEuint8.unwrap(handle)), 13);
    }

    function test_encryptUint16_works() public {
        (externalEuint16 handle, bytes memory proof) = encryptUint16(513, address(this));
        assertEq(uint8(externalEuint16.unwrap(handle)[30]), uint8(FheType.Uint16));
        assertEq(
            _executor.verifyInput(externalEuint16.unwrap(handle), address(this), proof, FheType.Uint16),
            externalEuint16.unwrap(handle)
        );
        assertEq(decrypt(externalEuint16.unwrap(handle)), 513);
    }

    function test_encryptUint32_works() public {
        (externalEuint32 handle, bytes memory proof) = encryptUint32(91_337, address(this));
        assertEq(uint8(externalEuint32.unwrap(handle)[30]), uint8(FheType.Uint32));
        assertEq(
            _executor.verifyInput(externalEuint32.unwrap(handle), address(this), proof, FheType.Uint32),
            externalEuint32.unwrap(handle)
        );
        assertEq(decrypt(externalEuint32.unwrap(handle)), 91_337);
    }

    function test_encryptUint128_works() public {
        uint128 value = type(uint128).max - 7;
        (externalEuint128 handle, bytes memory proof) = encryptUint128(value, address(this));
        assertEq(uint8(externalEuint128.unwrap(handle)[30]), uint8(FheType.Uint128));
        assertEq(
            _executor.verifyInput(externalEuint128.unwrap(handle), address(this), proof, FheType.Uint128),
            externalEuint128.unwrap(handle)
        );
        assertEq(decrypt(externalEuint128.unwrap(handle)), value);
    }

    function test_encryptUint256_works() public {
        uint256 value = type(uint256).max - 5;
        (externalEuint256 handle, bytes memory proof) = encryptUint256(value, address(this));
        assertEq(uint8(externalEuint256.unwrap(handle)[30]), uint8(FheType.Uint256));
        assertEq(
            _executor.verifyInput(externalEuint256.unwrap(handle), address(this), proof, FheType.Uint256),
            externalEuint256.unwrap(handle)
        );
        assertEq(decrypt(externalEuint256.unwrap(handle)), value);
    }

    function test_encryptAddress_works() public {
        address value = address(0xA11CE);
        (externalEaddress handle, bytes memory proof) = encryptAddress(value, address(this));

        assertEq(uint8(externalEaddress.unwrap(handle)[30]), uint8(FheType.Uint160));
        assertEq(
            _executor.verifyInput(externalEaddress.unwrap(handle), address(this), proof, FheType.Uint160),
            externalEaddress.unwrap(handle)
        );
        assertEq(decrypt(externalEaddress.unwrap(handle)), uint256(uint160(value)));
    }

    function test_encryptList_works() public {
        uint256[] memory values = new uint256[](7);
        values[0] = 1;
        values[1] = 13;
        values[2] = 513;
        values[3] = 91_337;
        values[4] = type(uint64).max - 7;
        values[5] = type(uint256).max - 5;
        values[6] = uint256(uint160(address(0xA11CE)));

        FheType[] memory fheTypes = new FheType[](7);
        fheTypes[0] = FheType.Bool;
        fheTypes[1] = FheType.Uint8;
        fheTypes[2] = FheType.Uint16;
        fheTypes[3] = FheType.Uint32;
        fheTypes[4] = FheType.Uint64;
        fheTypes[5] = FheType.Uint256;
        fheTypes[6] = FheType.Uint160;

        (bytes32[] memory handles, bytes memory proof) = encrypt(values, fheTypes, address(this));

        for (uint256 i; i < values.length; ++i) {
            assertEq(uint8(handles[i][30]), uint8(fheTypes[i]));
            assertEq(_executor.verifyInput(handles[i], address(this), proof, fheTypes[i]), handles[i]);
            assertEq(decrypt(handles[i]), values[i]);
        }
    }

    /// forge-config: default.allow_internal_expect_revert = true
    function test_encryptList_withMismatchedValuesAndTypesReverts() public {
        uint256[] memory values = new uint256[](2);
        FheType[] memory fheTypes = new FheType[](3);

        vm.expectRevert(abi.encodeWithSelector(EncryptInputLengthMismatch.selector, 2, 3));
        encrypt(values, fheTypes, address(this));
    }

    /// forge-config: default.allow_internal_expect_revert = true
    function test_encryptList_withTooManyValuesReverts() public {
        uint256[] memory values = new uint256[](300);
        FheType[] memory fheTypes = new FheType[](300);

        vm.expectRevert(EncryptInputTooLong.selector);
        encrypt(values, fheTypes, address(this));
    }

    function test_encrypt_withExplicitUserAndContract() public {
        address user = address(0xA11CE);
        address target = address(0xBEEF);

        (externalEuint64 handle, bytes memory proof) = encryptUint64(777, user, target);

        vm.prank(target);
        bytes32 verified = _executor.verifyInput(externalEuint64.unwrap(handle), user, proof, FheType.Uint64);

        assertEq(verified, externalEuint64.unwrap(handle));
    }

    function test_encrypt_differentNoncesProduceDifferentHandles() public {
        (externalEuint64 first,) = encryptUint64(123, address(this));
        (externalEuint64 second,) = encryptUint64(123, address(this));

        assertNotEq(externalEuint64.unwrap(first), externalEuint64.unwrap(second));
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FhevmTest} from "../../src/testing/FhevmTest.sol";
import {UserDecryptHelper} from "../../src/testing/UserDecryptHelper.sol";
import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";
import {kmsVerifierAdd} from "@fhevm/host-contracts/addresses/FHEVMHostAddresses.sol";

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

contract FhevmTestUserDecryptTest is FhevmTest {
    uint256 internal constant USER_PK = 0xA11CE;

    function test_userDecrypt_revertsWhenUserEqContract() public {
        (externalEuint64 handle, bytes memory proof) = encryptUint64(10, address(this));
        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        _acl.allow(externalEuint64.unwrap(handle), address(this));

        bytes memory signature = signUserDecrypt(USER_PK, address(this));

        vm.expectRevert(UserAddressEqualsContractAddress.selector);
        this.callUserDecrypt(externalEuint64.unwrap(handle), address(this), address(this), signature);
    }

    function test_userDecrypt_revertsWhenUserLacksPersistAllowed() public {
        address user = vm.addr(USER_PK);
        address contractAddress = address(0xCAFE);

        (externalEuint64 handle, bytes memory proof) = encryptUint64(10, address(this));
        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        _acl.allow(externalEuint64.unwrap(handle), contractAddress);

        bytes memory signature = signUserDecrypt(USER_PK, contractAddress);

        vm.expectRevert(
            abi.encodeWithSelector(UserNotAuthorizedForDecrypt.selector, externalEuint64.unwrap(handle), user)
        );
        this.callUserDecrypt(externalEuint64.unwrap(handle), user, contractAddress, signature);
    }

    function test_userDecrypt_revertsWhenUserHasOnlyTransient() public {
        address user = vm.addr(USER_PK);
        address contractAddress = address(0xCAFE);

        (externalEuint64 handle, bytes memory proof) = encryptUint64(10, address(this));
        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        _acl.allow(externalEuint64.unwrap(handle), contractAddress);
        vm.prank(address(_executor));
        _acl.allowTransient(externalEuint64.unwrap(handle), user);

        bytes memory signature = signUserDecrypt(USER_PK, contractAddress);

        vm.expectRevert(
            abi.encodeWithSelector(UserNotAuthorizedForDecrypt.selector, externalEuint64.unwrap(handle), user)
        );
        this.callUserDecrypt(externalEuint64.unwrap(handle), user, contractAddress, signature);
    }

    function test_userDecrypt_revertsWhenContractLacksPersistAllowed() public {
        address user = vm.addr(USER_PK);
        address contractAddress = address(0xCAFE);

        (externalEuint64 handle, bytes memory proof) = encryptUint64(10, address(this));
        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        _acl.allow(externalEuint64.unwrap(handle), user);

        bytes memory signature = signUserDecrypt(USER_PK, contractAddress);

        vm.expectRevert(
            abi.encodeWithSelector(
                ContractNotAuthorizedForDecrypt.selector, externalEuint64.unwrap(handle), contractAddress
            )
        );
        this.callUserDecrypt(externalEuint64.unwrap(handle), user, contractAddress, signature);
    }

    function test_userDecrypt_revertsWhenSignatureInvalid() public {
        address user = vm.addr(USER_PK);
        address contractAddress = address(0xCAFE);

        (externalEuint64 handle, bytes memory proof) = encryptUint64(10, address(this));
        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        _acl.allow(externalEuint64.unwrap(handle), user);
        _acl.allow(externalEuint64.unwrap(handle), contractAddress);

        bytes memory badSignature = signUserDecrypt(0xB0B, contractAddress);

        vm.expectRevert(InvalidUserDecryptSignature.selector);
        this.callUserDecrypt(externalEuint64.unwrap(handle), user, contractAddress, badSignature);
    }

    function test_userDecrypt_returnsCorrectPlaintext() public {
        address user = vm.addr(USER_PK);
        address contractAddress = address(0xCAFE);

        (externalEuint64 handle, bytes memory proof) = encryptUint64(777, address(this));
        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        _acl.allow(externalEuint64.unwrap(handle), user);
        _acl.allow(externalEuint64.unwrap(handle), contractAddress);

        bytes memory signature = signUserDecrypt(USER_PK, contractAddress);
        uint256 cleartext = userDecrypt(externalEuint64.unwrap(handle), user, contractAddress, signature);

        assertEq(cleartext, 777);
    }

    function test_userDecrypt_worksWithAllTypes() public {
        address user = vm.addr(USER_PK);
        address contractAddress = address(0xCAFE);
        bytes memory signature = signUserDecrypt(USER_PK, contractAddress);

        {
            (externalEbool h0, bytes memory p0) = encryptBool(true, address(this));
            _verifyPersistAndDecrypt(externalEbool.unwrap(h0), p0, FheType.Bool, 1, user, contractAddress, signature);
        }
        {
            (externalEuint8 h1, bytes memory p1) = encryptUint8(1, address(this));
            _verifyPersistAndDecrypt(externalEuint8.unwrap(h1), p1, FheType.Uint8, 1, user, contractAddress, signature);
        }
        {
            (externalEuint16 h2, bytes memory p2) = encryptUint16(2, address(this));
            _verifyPersistAndDecrypt(
                externalEuint16.unwrap(h2), p2, FheType.Uint16, 2, user, contractAddress, signature
            );
        }
        {
            (externalEuint32 h3, bytes memory p3) = encryptUint32(3, address(this));
            _verifyPersistAndDecrypt(
                externalEuint32.unwrap(h3), p3, FheType.Uint32, 3, user, contractAddress, signature
            );
        }
        {
            (externalEuint64 h4, bytes memory p4) = encryptUint64(4, address(this));
            _verifyPersistAndDecrypt(
                externalEuint64.unwrap(h4), p4, FheType.Uint64, 4, user, contractAddress, signature
            );
        }
        {
            (externalEuint128 h5, bytes memory p5) = encryptUint128(5, address(this));
            _verifyPersistAndDecrypt(
                externalEuint128.unwrap(h5), p5, FheType.Uint128, 5, user, contractAddress, signature
            );
        }
        {
            (externalEuint256 h6, bytes memory p6) = encryptUint256(6, address(this));
            _verifyPersistAndDecrypt(
                externalEuint256.unwrap(h6), p6, FheType.Uint256, 6, user, contractAddress, signature
            );
        }
        {
            (externalEaddress h7, bytes memory p7) = encryptAddress(address(7), address(this));
            _verifyPersistAndDecrypt(
                externalEaddress.unwrap(h7), p7, FheType.Uint160, 7, user, contractAddress, signature
            );
        }
    }

    function test_signUserDecrypt_producesValidSignature() public view {
        address user = vm.addr(USER_PK);
        address contractAddress = address(0xCAFE);
        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = contractAddress;

        bytes memory signature = signUserDecrypt(USER_PK, contractAddresses, block.timestamp, 1);

        bytes32 domain = UserDecryptHelper.computeUserDecryptDomainSeparator(block.chainid, kmsVerifierAdd);
        bytes32 digest = UserDecryptHelper.computeUserDecryptDigest(
            abi.encodePacked(user), contractAddresses, block.timestamp, 1, EMPTY_EXTRA_DATA, domain
        );

        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := mload(add(signature, 0x20))
            s := mload(add(signature, 0x40))
            v := byte(0, mload(add(signature, 0x60)))
        }

        assertEq(ecrecover(digest, v, r, s), user);
    }

    function _verifyPersistAndDecrypt(
        bytes32 handle,
        bytes memory inputProof,
        FheType fheType,
        uint256 expectedCleartext,
        address user,
        address contractAddress,
        bytes memory signature
    ) internal {
        _executor.verifyInput(handle, address(this), inputProof, fheType);
        _acl.allow(handle, user);
        _acl.allow(handle, contractAddress);
        assertEq(userDecrypt(handle, user, contractAddress, signature), expectedCleartext);
    }

    function callUserDecrypt(bytes32 handle, address userAddress, address contractAddress, bytes memory signature)
        external
        returns (uint256)
    {
        return userDecrypt(handle, userAddress, contractAddress, signature);
    }
}

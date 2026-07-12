// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FHE} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {FhevmTest} from "../../src/testing/FhevmTest.sol";
import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";

import {externalEuint16, externalEuint64, externalEuint256, externalEaddress} from "encrypted-types/EncryptedTypes.sol";

contract FhevmPublicDecryptVerifier is ZamaEthereumConfig {
    function verify(bytes32[] memory handles, bytes memory abiEncodedCleartexts, bytes memory decryptionProof)
        external
    {
        FHE.checkSignatures(handles, abiEncodedCleartexts, decryptionProof);
    }
}

contract FhevmTestPublicDecryptTest is FhevmTest {
    function test_publicDecrypt_revertsWhenHandleNotAllowedForDecryption() public {
        (externalEuint64 handle,) = encryptUint64(55, address(this));

        bytes32[] memory handles = new bytes32[](1);
        handles[0] = externalEuint64.unwrap(handle);

        vm.expectRevert(abi.encodeWithSelector(HandleNotAllowedForPublicDecryption.selector, handles[0]));
        this.callPublicDecrypt(handles);
    }

    function test_publicDecrypt_returnsCorrectCleartexts() public {
        (externalEuint64 handle, bytes memory proof) = encryptUint64(55, address(this));

        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = externalEuint64.unwrap(handle);
        _acl.allowForDecryption(handles);

        (uint256[] memory cleartexts,) = publicDecrypt(handles);
        assertEq(cleartexts.length, 1);
        assertEq(cleartexts[0], 55);
    }

    function test_publicDecrypt_proofVerifiableByRealKMSVerifier() public {
        (externalEuint64 handle, bytes memory proof) = encryptUint64(1234, address(this));

        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = externalEuint64.unwrap(handle);
        _acl.allowForDecryption(handles);

        (uint256[] memory cleartexts, bytes memory decryptionProof) = publicDecrypt(handles);
        bytes memory abiEncodedCleartexts = abi.encodePacked(cleartexts);
        bool verified = _kmsVerifier.verifyDecryptionEIP712KMSSignatures(handles, abiEncodedCleartexts, decryptionProof);
        assertTrue(verified);
    }

    function test_publicDecrypt_addressZeroProofVerifiesAgainstProductionTupleEncoding() public {
        (externalEaddress handle, bytes memory proof) = encryptAddress(address(0), address(this));

        _executor.verifyInput(externalEaddress.unwrap(handle), address(this), proof, FheType.Uint160);
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = externalEaddress.unwrap(handle);
        _acl.allowForDecryption(handles);

        (, bytes memory decryptionProof) = publicDecrypt(handles);

        bool verified =
            _kmsVerifier.verifyDecryptionEIP712KMSSignatures(handles, abi.encode(address(0)), decryptionProof);
        assertTrue(verified);
    }

    function test_publicDecrypt_multipleHandles() public {
        (externalEuint16 h0, bytes memory p0) = encryptUint16(11, address(this));
        (externalEuint64 h1, bytes memory p1) = encryptUint64(22, address(this));
        (externalEuint256 h2, bytes memory p2) = encryptUint256(33, address(this));

        _executor.verifyInput(externalEuint16.unwrap(h0), address(this), p0, FheType.Uint16);
        _executor.verifyInput(externalEuint64.unwrap(h1), address(this), p1, FheType.Uint64);
        _executor.verifyInput(externalEuint256.unwrap(h2), address(this), p2, FheType.Uint256);

        bytes32[] memory handles = new bytes32[](3);
        handles[0] = externalEuint16.unwrap(h0);
        handles[1] = externalEuint64.unwrap(h1);
        handles[2] = externalEuint256.unwrap(h2);

        _acl.allowForDecryption(handles);

        (uint256[] memory cleartexts, bytes memory decryptionProof) = publicDecrypt(handles);
        assertEq(cleartexts.length, 3);
        assertEq(cleartexts[0], 11);
        assertEq(cleartexts[1], 22);
        assertEq(cleartexts[2], 33);
        bytes memory abiEncodedCleartexts = abi.encodePacked(cleartexts);
        assertTrue(_kmsVerifier.verifyDecryptionEIP712KMSSignatures(handles, abiEncodedCleartexts, decryptionProof));
    }

    function test_publicDecrypt_endToEndWithCheckSignatures() public {
        FhevmPublicDecryptVerifier verifier = new FhevmPublicDecryptVerifier();
        (externalEuint64 handle, bytes memory proof) = encryptUint64(9090, address(this));

        _executor.verifyInput(externalEuint64.unwrap(handle), address(this), proof, FheType.Uint64);

        bytes32[] memory handles = new bytes32[](1);
        handles[0] = externalEuint64.unwrap(handle);
        _acl.allowForDecryption(handles);

        (uint256[] memory cleartexts, bytes memory decryptionProof) = publicDecrypt(handles);
        verifier.verify(handles, abi.encodePacked(cleartexts), decryptionProof);
    }

    function callPublicDecrypt(bytes32[] memory handles)
        external
        returns (uint256[] memory cleartexts, bytes memory decryptionProof)
    {
        return publicDecrypt(handles);
    }
}

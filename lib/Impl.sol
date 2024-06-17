// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "./TFHE.sol";
import "./TFHEExecutor.sol";
import "./TFHEExecutorAddress.sol";
import "./ACL.sol";
import "./ACLAddress.sol";

library Impl {
    // 32 bytes for the 'byte' type header + 48 bytes for the NaCl anonymous
    // box overhead + 4 bytes for the plaintext value.
    uint256 constant reencryptedSize = 32 + 48 + 4;

    // 32 bytes for the 'byte' header + 16553 bytes of key data.
    uint256 constant fhePubKeySize = 32 + 16553;

    ACL private constant acl = ACL(address(ACL_CONTRACT_ADDRESS));
    TFHEExecutor private constant exec = TFHEExecutor(address(TFHE_EXECUTOR_CONTRACT_ADDRESS));

    function add(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheAdd(lhs, rhs, scalar);
    }

    function sub(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheSub(lhs, rhs, scalar);
    }

    function mul(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheMul(lhs, rhs, scalar);
    }

    function div(uint256 lhs, uint256 rhs) internal returns (uint256 result) {
        result = exec.fheDiv(lhs, rhs);
    }

    function rem(uint256 lhs, uint256 rhs) internal returns (uint256 result) {
        result = exec.fheRem(lhs, rhs);
    }

    function and(uint256 lhs, uint256 rhs) internal returns (uint256 result) {
        result = exec.fheBitAnd(lhs, rhs);
    }

    function or(uint256 lhs, uint256 rhs) internal returns (uint256 result) {
        result = exec.fheBitOr(lhs, rhs);
    }

    function xor(uint256 lhs, uint256 rhs) internal returns (uint256 result) {
        result = exec.fheBitXor(lhs, rhs);
    }

    function shl(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheShl(lhs, rhs, scalar);
    }

    function shr(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheShr(lhs, rhs, scalar);
    }

    function rotl(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheRotl(lhs, rhs, scalar);
    }

    function rotr(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheRotr(lhs, rhs, scalar);
    }

    function eq(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheEq(lhs, rhs, scalar);
    }

    function ne(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheNe(lhs, rhs, scalar);
    }

    function ge(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheGe(lhs, rhs, scalar);
    }

    function gt(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheGt(lhs, rhs, scalar);
    }

    function le(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheLe(lhs, rhs, scalar);
    }

    function lt(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheLt(lhs, rhs, scalar);
    }

    function min(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheMin(lhs, rhs, scalar);
    }

    function max(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        result = exec.fheMax(lhs, rhs, scalar);
    }

    function neg(uint256 ct) internal returns (uint256 result) {
        result = exec.fheNeg(ct);
    }

    function not(uint256 ct) internal returns (uint256 result) {
        result = exec.fheNot(ct);
    }

    // If 'control's value is 'true', the result has the same value as 'ifTrue'.
    // If 'control's value is 'false', the result has the same value as 'ifFalse'.
    function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal returns (uint256 result) {
        result = exec.fheIfThenElse(control, ifTrue, ifFalse);
    }

    function eq(uint256[] memory lhs, uint256[] memory rhs) internal pure returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheArrayEq(lhs, rhs);
    }

    function reencrypt(uint256 ciphertext, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return exec.reencrypt(ciphertext, uint256(publicKey));
    }

    function fhePubKey() internal view returns (bytes memory key) {
        // Set a byte value of 1 to signal the call comes from the library.
        key = exec.fhePubKey(bytes1(0x01));
    }

    function verify(bytes32 inputHandle, bytes memory inputProof, uint8 toType) internal returns (uint256 result) {
        result = exec.verifyCiphertext(inputHandle, msg.sender, inputProof, bytes1(toType));
        acl.allowTransient(result, msg.sender);
    }

    function cast(uint256 ciphertext, uint8 toType) internal returns (uint256 result) {
        result = exec.cast(ciphertext, bytes1(toType));
    }

    function trivialEncrypt(uint256 value, uint8 toType) internal returns (uint256 result) {
        result = exec.trivialEncrypt(value, bytes1(toType));
    }

    function decrypt(uint256 ciphertext) internal view returns (uint256 result) {
        result = exec.decrypt(ciphertext);
    }

    function rand(uint8 randType) internal returns (uint256 result) {
        result = exec.fheRand(bytes1(randType));
    }

    function randBounded(uint256 upperBound, uint8 randType) internal returns (uint256 result) {
        result = exec.fheRandBounded(upperBound, bytes1(randType));
    }

    function allowTransient(uint256 handle, address account) internal {
        acl.allowTransient(handle, account);
    }

    function allowedTransient(uint256 handle, address account) internal view returns (bool) {
        return acl.allowedTransient(handle, account);
    }

    function cleanAllTransientAllowed() internal {
        acl.cleanAllTransientAllowed();
    }

    function allow(uint256 handle, address account) internal {
        acl.allow(handle, account);
    }

    function persistAllowed(uint256 handle, address account) internal view returns (bool) {
        return acl.persistAllowed(handle, account);
    }

    function isAllowed(uint256 handle, address account) internal view returns (bool) {
        return acl.isAllowed(handle, account);
    }

    function delegateAccount(address delegatee) internal {
        acl.delegateAccount(delegatee);
    }

    function removeDelegation(address delegatee) internal {
        acl.removeDelegation(delegatee);
    }

    function allowedOnBehalf(address delegatee, uint256 handle, address account) internal view returns (bool) {
        return acl.allowedOnBehalf(delegatee, handle, account);
    }

    function allowForDecryption(uint256[] memory ctsHandles) internal {
        acl.allowForDecryption(ctsHandles);
    }
}

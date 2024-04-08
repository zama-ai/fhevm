// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "./TFHE.sol";

library Impl {
    function add(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        unchecked {
            result = lhs + rhs;
        }
    }

    function sub(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        unchecked {
            result = lhs - rhs;
        }
    }

    function mul(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        unchecked {
            result = lhs * rhs;
        }
    }

    function div(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
        result = lhs / rhs; // unchecked does not change behaviour even when dividing by 0
    }

    function rem(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
        result = lhs % rhs;
    }

    function and(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
        result = lhs & rhs;
    }

    function or(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
        result = lhs | rhs;
    }

    function xor(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
        result = lhs ^ rhs;
    }

    function shl(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = lhs << rhs;
    }

    function shr(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = lhs >> rhs;
    }

    function rotl(uint256 lhs, uint256 rhs, uint256 bits, bool /*scalar*/) internal pure returns (uint256 result) {
        uint count = rhs;
        result = (lhs << count) | (lhs >> (bits - count));
    }

    function rotr(uint256 lhs, uint256 rhs, uint256 bits, bool /*scalar*/) internal pure returns (uint256 result) {
        uint count = rhs;
        result = (lhs >> count) | (lhs << (bits - count));
    }

    function eq(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs == rhs) ? 1 : 0;
    }

    function ne(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs != rhs) ? 1 : 0;
    }

    function ge(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs >= rhs) ? 1 : 0;
    }

    function gt(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs > rhs) ? 1 : 0;
    }

    function le(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs <= rhs) ? 1 : 0;
    }

    function lt(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs < rhs) ? 1 : 0;
    }

    function min(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs < rhs) ? lhs : rhs;
    }

    function max(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
        result = (lhs > rhs) ? lhs : rhs;
    }

    function neg(uint256 ct) internal pure returns (uint256 result) {
        uint256 y;
        assembly {
            y := not(ct)
        }
        unchecked {
            return y + 1;
        }
    }

    function not(uint256 ct) internal pure returns (uint256 result) {
        uint256 y;
        assembly {
            y := not(ct)
        }
        return y;
    }

    function cmux(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
        result = (control == 1) ? ifTrue : ifFalse;
    }

    function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
        result = (control == 1) ? ifTrue : ifFalse;
    }

    function optReq(uint256 ciphertext) internal view {
        this; // silence state mutability warning
        require(ciphertext == 1, "transaction execution reverted");
    }

    function reencrypt(uint256 ciphertext, bytes32 /*publicKey*/) internal view returns (bytes memory reencrypted) {
        this; // silence state mutability warning
        reencrypted = new bytes(32);
        assembly {
            mstore(add(reencrypted, 32), ciphertext)
        }
        return reencrypted;
    }

    function fhePubKey() internal view returns (bytes memory key) {
        this; // silence state mutability warning
        key = hex"0123456789ABCDEF";
    }

    function verify(bytes memory _ciphertextBytes, uint8 /*_toType*/) internal pure returns (uint256 result) {
        uint256 x;
        assembly {
            switch gt(mload(_ciphertextBytes), 31)
            case 1 {
                x := mload(add(_ciphertextBytes, add(32, sub(mload(_ciphertextBytes), 32))))
            }
            default {
                x := mload(add(_ciphertextBytes, 32))
            }
        }
        if (_ciphertextBytes.length < 32) {
            x = x >> ((32 - _ciphertextBytes.length) * 8);
        }
        return x;
    }

    function cast(uint256 ciphertext, uint8 toType) internal pure returns (uint256 result) {
        if (toType == 0) {
            result = uint256(uint8(ciphertext));
        }
        if (toType == 1) {
            result = uint256(uint8(ciphertext));
        }
        if (toType == 2) {
            result = uint256(uint8(ciphertext));
        }
        if (toType == 3) {
            result = uint256(uint16(ciphertext));
        }
        if (toType == 4) {
            result = uint256(uint32(ciphertext));
        }
        if (toType == 5) {
            result = uint256(uint64(ciphertext));
        }
    }

    function trivialEncrypt(uint256 value, uint8 /*toType*/) internal pure returns (uint256 result) {
        result = value;
    }

    function decrypt(uint256 ciphertext) internal view returns (uint256 result) {
        this; // silence state mutability warning
        result = ciphertext;
    }

    function rand(uint8 randType) internal view returns (uint256 result) {
        uint256 randomness = uint256(keccak256(abi.encodePacked(block.number, gasleft(), msg.sender))); // assuming no duplicated tx by same sender in a single block
        if (randType == Common.euint8_t) {
            result = uint8(randomness);
        } else if (randType == Common.euint16_t) {
            result = uint16(randomness);
        } else if (randType == Common.euint32_t) {
            result = uint32(randomness);
        } else if (randType == Common.euint64_t) {
            result = uint64(randomness);
        } else {
            revert("rand() mock invalid type");
        }
    }

    function randBounded(uint256 upperBound, uint8 randType) internal view returns (uint256 result) {
        // Here, we assume upperBound is a power of 2. Therefore, using modulo is secure.
        // If not a power of 2, we might have to do something else (though might not matter
        // much as this is a mock).
        result = rand(randType) % upperBound;
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "./Common.sol";
import "./Precompiles.sol";

library Impl {
    // 32 bytes for the 'byte' type header + 48 bytes for the NaCl anonymous
    // box overhead + 4 bytes for the plaintext value.
    uint256 constant reencryptedSize = 32 + 48 + 4;

    // 32 bytes for the 'byte' header + 16553 bytes of key data.
    uint256 constant fhePubKeySize = 32 + 16553;

    function add(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the add precompile.
        uint256 precompile = Precompiles.Add;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function sub(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the sub precompile.
        uint256 precompile = Precompiles.Subtract;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function mul(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the mul precompile.
        uint256 precompile = Precompiles.Multiply;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function div(uint256 lhs, uint256 rhs) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), bytes1(0x01));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the div precompile.
        uint256 precompile = Precompiles.Divide;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function and(uint256 lhs, uint256 rhs) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the and precompile.
        uint256 precompile = Precompiles.BitwiseAnd;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function or(uint256 lhs, uint256 rhs) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the or precompile.
        uint256 precompile = Precompiles.BitwiseOr;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function xor(uint256 lhs, uint256 rhs) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the xor precompile.
        uint256 precompile = Precompiles.BitwiseXor;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function shl(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the shl precompile.
        uint256 precompile = Precompiles.ShiftLeft;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function shr(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the shr precompile.
        uint256 precompile = Precompiles.ShiftRight;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function eq(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the eq precompile.
        uint256 precompile = Precompiles.Equal;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function ne(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the ne precompile.
        uint256 precompile = Precompiles.NotEqual;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function ge(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the ge precompile.
        uint256 precompile = Precompiles.GreaterThanOrEqual;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function gt(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the gt precompile.
        uint256 precompile = Precompiles.GreaterThan;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function le(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the le precompile.
        uint256 precompile = Precompiles.LessThanOrEqual;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function lt(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the lt precompile.
        uint256 precompile = Precompiles.LessThan;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function min(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the min precompile.
        uint256 precompile = Precompiles.Min;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function max(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the max precompile.
        uint256 precompile = Precompiles.Max;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function neg(uint256 ct) internal view returns (uint256 result) {
        bytes32[1] memory input;
        input[0] = bytes32(ct);
        uint256 inputLen = 32;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the neg precompile.
        uint256 precompile = Precompiles.Negate;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function not(uint256 ct) internal view returns (uint256 result) {
        bytes32[1] memory input;
        input[0] = bytes32(ct);
        uint256 inputLen = 32;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the not precompile.
        uint256 precompile = Precompiles.Not;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    // If 'control's value is 'true', the result has the same value as 'ifTrue'.
    // If 'control's value is 'false', the result has the same value as 'ifFalse'.
    function cmux(uint256 control, uint256 ifTrue, uint256 ifFalse) internal view returns (uint256 result) {
        // result = (ifTrue - ifFalse) * control + ifFalse
        bytes memory input = bytes.concat(bytes32(ifTrue), bytes32(ifFalse), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory subOutput;
        uint256 outputLen = 32;

        // Call the sub precompile.
        uint256 precompile = Precompiles.Subtract;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, subOutput, outputLen)) {
                revert(0, 0)
            }
        }

        // Call the mul precompile.
        input = bytes.concat(bytes32(control), bytes32(subOutput[0]), bytes1(0x00));
        inputLen = input.length;
        precompile = Precompiles.Multiply;
        bytes32[1] memory mulOutput;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, mulOutput, outputLen)) {
                revert(0, 0)
            }
        }

        // Call the add precompile.
        input = bytes.concat(bytes32(mulOutput[0]), bytes32(ifFalse), bytes1(0x00));
        inputLen = input.length;
        precompile = Precompiles.Add;
        bytes32[1] memory addOutput;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, addOutput, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(addOutput[0]);
    }

    function optReq(uint256 ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        // Call the optimistic require precompile.
        uint256 precompile = Precompiles.OptimisticRequire;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }

    function reencrypt(uint256 ciphertext, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        bytes32[2] memory input;
        input[0] = bytes32(ciphertext);
        input[1] = publicKey;
        uint256 inputLen = 64;

        reencrypted = new bytes(reencryptedSize);

        // Call the reencrypt precompile.
        uint256 precompile = Precompiles.Reencrypt;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, reencrypted, reencryptedSize)) {
                revert(0, 0)
            }
        }
    }

    function fhePubKey() internal view returns (bytes memory key) {
        // Set a byte value of 1 to signal the call comes from the library.
        bytes1[1] memory input;
        input[0] = 0x01;
        uint256 inputLen = 1;

        key = new bytes(fhePubKeySize);

        // Call the fhePubKey precompile.
        uint256 precompile = Precompiles.FhePubKey;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, key, fhePubKeySize)) {
                revert(0, 0)
            }
        }
    }

    function verify(bytes memory _ciphertextBytes, uint8 _toType) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(_ciphertextBytes, bytes1(_toType));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the verify precompile.
        uint256 precompile = Precompiles.Verify;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }
        result = uint256(output[0]);
    }

    function cast(uint256 ciphertext, uint8 toType) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(ciphertext), bytes1(toType));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the cast precompile.
        uint256 precompile = Precompiles.Cast;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }
        result = uint256(output[0]);
    }

    function trivialEncrypt(uint256 value, uint8 toType) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(value), bytes1(toType));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the trivialEncrypt precompile.
        uint256 precompile = Precompiles.TrivialEncrypt;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }
        result = uint256(output[0]);
    }

    function decrypt(uint256 ciphertext) internal view returns (uint256 result) {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the decrypt precompile.
        uint256 precompile = Precompiles.Decrypt;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }
        // The output is a 32-byte buffer of a 256-bit big-endian unsigned integer.
        result = uint256(output[0]);
    }
}

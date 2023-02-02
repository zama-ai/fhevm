// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./Precompiles.sol";
import "./Common.sol";

// A library of functions for homomorphic operations on ciphertexts.
// NOTE: Currently, all ciphertexts are of the same type, i.e. an 4-bit integer. This
// is likely to change in the future.
library FHEOps {

    // Add ciphertext `a` to ciphertext `b` and, if successful, return the resulting ciphertext.
    // If not successful, fail.
    // If successful, the resulting ciphertext is automatically verified.
    function add(FHEUInt a, FHEUInt b) internal view returns (FHEUInt result) {
        if (FHEUInt.unwrap(a) == 0) {
            return b;
        } else if (FHEUInt.unwrap(b) == 0) {
            return a;
        }
        bytes32[2] memory input;
        input[0] = bytes32(FHEUInt.unwrap(a));
        input[1] = bytes32(FHEUInt.unwrap(b));
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the add precompile.
        uint256 precompile = Precompiles.Add;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = FHEUInt.wrap(uint256(output[0]));
    }

    // Subtract ciphertext `b` from ciphertext `a` and, if successful, return the resulting ciphertext.
    // If not successful, fail.
    // If successful, the resulting ciphertext is automatically verified.
    function sub(FHEUInt a, FHEUInt b) internal view returns (FHEUInt result) {
        bytes32[2] memory input;
        input[0] = bytes32(FHEUInt.unwrap(a));
        input[1] = bytes32(FHEUInt.unwrap(b));
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the subtract precompile.
        uint256 precompile = Precompiles.Subtract;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = FHEUInt.wrap(uint256(output[0]));
    }

    // Multiply ciphertext `a` by ciphertext `b` and, if successful, return the resulting ciphertext.
    // If not successful, fail.
    // If successful, the resulting ciphertext is automatically verified.
    function mul(FHEUInt a, FHEUInt b) internal view returns (FHEUInt result) {
        bytes32[2] memory input;
        input[0] = bytes32(FHEUInt.unwrap(a));
        input[1] = bytes32(FHEUInt.unwrap(b));
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the multiply precompile.
        uint256 precompile = Precompiles.Multiply;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = FHEUInt.wrap(uint256(output[0]));
    }

    // Evaluate `lhs <= rhs` on the given ciphertexts and, if successful, return the resulting ciphertext.
    // If successful, the resulting ciphertext is automatically verified.
    function lte(FHEUInt lhs, FHEUInt rhs) internal view returns (FHEUInt result) {
        bytes32[2] memory input;
        input[0] = bytes32(FHEUInt.unwrap(lhs));
        input[1] = bytes32(FHEUInt.unwrap(rhs));
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the lte precompile.
        uint256 precompile = Precompiles.LessThanOrEqual;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = FHEUInt.wrap(uint256(output[0]));
    }

    // Evaluate `lhs < rhs` on the given ciphertexts and, if successful, return the resulting ciphertext.
    // If successful, the resulting ciphertext is automatically verified.
    function lt(FHEUInt lhs, FHEUInt rhs) internal view returns (FHEUInt result) {
        bytes32[2] memory input;
        input[0] = bytes32(FHEUInt.unwrap(lhs));
        input[1] = bytes32(FHEUInt.unwrap(rhs));
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the lt precompile.
        uint256 precompile = Precompiles.LessThan;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = FHEUInt.wrap(uint256(output[0]));
    }

    // If `control`'s value is 1, the resulting value is the same value as `ifTrue`.
    // If `control`'s value is 0, the resulting value is the same value as `ifFalse`.
    // If successful, the resulting ciphertext is automatically verified.
    function cmux(FHEUInt control, FHEUInt ifTrue, FHEUInt ifFalse) internal view returns (FHEUInt result) {
        // result = (ifTrue - ifFalse) * control + ifFalse

        bytes32[2] memory input;
        uint256 inputLen = 64;
        uint256 outputLen = 32;

        // Call the sub precompile.
        input[0] = bytes32(FHEUInt.unwrap(ifTrue));
        input[1] = bytes32(FHEUInt.unwrap(ifFalse));
        uint256 precompile = Precompiles.Subtract;
        bytes32[1] memory subOutput;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, subOutput, outputLen)) {
                revert(0, 0)
            }
        }

        // Call the mul precompile.
        input[0] = bytes32(FHEUInt.unwrap(control));
        input[1] = bytes32(subOutput[0]);
        precompile = Precompiles.Multiply;
        bytes32[1] memory mulOutput;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, mulOutput, outputLen)) {
                revert(0, 0)
            }
        }

        // Call the add precompile.
        input[0] = bytes32(mulOutput[0]);
        input[1] = bytes32(FHEUInt.unwrap(ifFalse));
        precompile = Precompiles.Add;
        bytes32[1] memory addOutput;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, addOutput, outputLen)) {
                revert(0, 0)
            }
        }

        result = FHEUInt.wrap(uint256(addOutput[0]));
    }
}

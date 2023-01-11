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
}

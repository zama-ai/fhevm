// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./Precompiles.sol";

// Represents a ciphertext.
type FHEUInt is uint256;

library Common {
    // Requires that the `ciphertext` is true.
    // If true, the runction returns. Otherwise, it reverts.
    function requireCt(FHEUInt ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(FHEUInt.unwrap(ciphertext));
        uint256 inputLen = 32;

        // Call the require precompile.
        uint256 precompile = Precompiles.Require;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }

    // Optimistically requires that the `ciphertext` is true.
    //
    // This function does not evaluate the given `ciphertext` at the time of the call.
    // Instead, it accumulates all optimistic requires and evaluates a single combined
    // require at the end through the decryption oracle. A side effect of this mechanism
    // is that a method call with a failed optimistic require will always incur the full
    // gas cost, as if all optimistic requires were true. Yet, the transaction will be
    // reverted at the end if any of the optimisic requires were false.
    //
    // The benefit of optimistic requires is that they are faster than non-optimistic ones,
    // because there is a single call to the decryption oracle per transaction, irrespective
    // of how many optimistic requires were used.
    function optimisticRequireCt(FHEUInt ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(FHEUInt.unwrap(ciphertext));
        uint256 inputLen = 32;

        // Call the optimistic require precompile.
        uint256 precompile = Precompiles.OptimisticRequire;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }
}

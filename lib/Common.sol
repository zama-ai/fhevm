// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./Precompiles.sol";

// Represents a ciphertext.
type FheUInt is uint256;

library Common {
    // Requires that the `ciphertext` is true.
    // If true, the runction returns. Otherwise, it reverts.
    function requireCt(FheUInt ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(FheUInt.unwrap(ciphertext));
        uint256 inputLen = 32;

        // Call the require precompile.
        uint256 precompile = Precompiles.Require;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.7.0 <0.9.0;

import "./Common.sol";

// A library of functions for homomorphic operations on ciphertexts.
// NOTE: Currently, all handles refer to ciphertexts of the same type, i.e. an 8-bit integer. This
// is likely to change in the future.
library FHEOperations {

    // Add the ciphertext behind `handleA` to the ciphertext behind `handleB` and,
    // if successful, return a handle to the resulting ciphertext. If not successful, fail.
    // If successful, the resulting handle is automatically verified.
    function add(uint256 handleA, uint256 handleB) internal view returns (uint256 resultHandle) {
        bytes32[2] memory input;
        input[0] = bytes32(handleA);
        input[1] = bytes32(handleB);

        bytes32[1] memory output;

        // Call the add precompile.
        uint256 precompile = Precompiles.Add;
        assembly {
            if iszero(staticcall(gas(), precompile, input, 64, output, 32)) {
                revert(0, 0)
            }
        }

        // Copy the handle to the output.
        resultHandle = uint256(output[0]);
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.7.0 <0.9.0;

import "./Precompiles.sol";

// A library of functions for managing ciphertexts.
library Ciphertext {

    // The maximum ciphertext output length in bytes.
    // Rationale: `bytes` ciphertext layout is 32 bytes of length metadata, followed by 65544 bytes of ciphertext.
    uint256 constant MaxCiphertextBytesLen = 32 + 65544;

    // Reencrypt the given ciphertext `handle` to the original caller's public key.
    // If successful, return the reencrypted ciphertext. Else, fail.
    // Currently, can only be used in `eth_call`. If called in a transaction, it will fail.
    function reencrypt(uint256 handle) internal view returns (bytes memory ciphertext) {
        bytes32[1] memory input;
        input[0] = bytes32(handle);
        uint256 inputLen = 32;
        ciphertext = new bytes(MaxCiphertextBytesLen);

        // Call the reencrypt precompile.
        uint256 precompile = Precompiles.Reencrypt;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, ciphertext, MaxCiphertextBytesLen)) {
                revert(0, 0)
            }
        }
    }

    // Verify the given ciphertext.
    // If successful, return the handle to it. Else, fail.
    // It is expected that the ciphertext is serialized such that it contains a zero-knowledge proof of knowledge of the plaintext.
    function verify(bytes memory ciphertextWithProof) internal view returns (uint256 handle) {
        bytes32[1] memory output;
        uint256 inputLen = ciphertextWithProof.length;

        // Call the verify precompile. Skip 32 bytes of lenght metadata for the input `bytes`.
        uint256 precompile = Precompiles.Verify;
        assembly {
            if iszero(staticcall(gas(), precompile, add(ciphertextWithProof, 32), inputLen, output, 32)) {
                revert(0, 0)
            }
        }

        // Copy the handle to the output.
        handle = uint256(output[0]);
    }

    // Delegate the given ciphertext `handle` for use in the outer scope.
    // If successful, return. Else, fail.
    function delegate(uint256 handle) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(handle);
        uint256 inputLen = 32;

        // Call the delegate precompile.
        uint256 precompile = Precompiles.Delegate;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }
}

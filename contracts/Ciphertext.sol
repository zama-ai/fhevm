// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.7.0 <0.9.0;

import "./Common.sol";

// A library of functions for managing ciphertexts.
library Ciphertext {

    // The maximum ciphertext output length in bytes.
    // Rationale: `bytes` ciphertext layout is 32 bytes of length metadata, followed by 175328 bytes of ciphertext.
    uint256 constant MaxOutputCiphertextBytesLen = 32 + 175328;

    // Reencrypt the given ciphertext `handle` to the given `publicKey`.
    // If successful, return the reencrypted ciphertext. Else, fail.
    // Currently, can only be used in read-only requests. If called on a write request, it will fail.
    function reencrypt(uint256 handle, bytes memory publicKey) internal view returns (bytes memory ciphertext) {
        // 32 bytes for the handle + the actual length.
        uint256 inputLen = 32 + publicKey.length;

        bytes memory input = new bytes(inputLen);

        // Store the handle first.
        assembly { mstore(add(input, 32), handle) }

        // And then the actual public key.
        for (uint256 i = 0; i < publicKey.length; i++) {
            input[i + 32] = publicKey[i];
        }

        // Call the reencrypt precompile. Skip 32 bytes of lenght metadata for the input `bytes`.
        uint256 precompile = Precompiles.Reencrypt;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, ciphertext, MaxOutputCiphertextBytesLen)) {
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

        // Call the delegate precompile.
        uint256 precompile = Precompiles.Delegate;
        assembly {
            if iszero(staticcall(gas(), precompile, input, 32, 0, 0)) {
                revert(0, 0)
            }
        }
    }
}

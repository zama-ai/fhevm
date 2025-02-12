// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

contract CiphertextStorage {
    function addCiphertext(
        uint256 ctHandle,
        uint256 keyId,
        bytes calldata ciphertext64,
        bytes calldata ciphertext128
    ) external {}
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.25;

import "../abstracts/EIP712WithModifier.sol";

contract AuthorizationToken is EIP712WithModifier {
    constructor() EIP712WithModifier("Authorization token", "1") {}

    function verify(
        bytes32 publicKey,
        bytes calldata signature
    ) public view onlySignedPublicKey(publicKey, signature) returns (bool) {
        return true;
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

interface IKMSVerifier {
    function verifySignatures(
        uint256[] memory handlesList,
        bytes memory decryptedResult,
        bytes[] memory signatures
    ) external returns (bool);
}

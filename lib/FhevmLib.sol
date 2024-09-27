// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

interface FhevmLib {
    function verifyCiphertext(
        bytes32 inputHandle,
        address callerAddress,
        address contractAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) external pure returns (uint256 result);
}

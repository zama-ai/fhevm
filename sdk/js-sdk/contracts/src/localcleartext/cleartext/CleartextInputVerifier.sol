// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {InputVerifier} from "../host-contracts/contracts/InputVerifier.sol";

/**
 * @title CleartextInputVerifier
 * @dev The RFC-030 pad decryption mock is not here: it lives on `CleartextPaddedInputter`.
 */
contract CleartextInputVerifier is InputVerifier {
    /// @notice Marks a cleartext (mock) implementation. Real host contracts have no such selector, so a
    ///         consumer can probe it to tell a cleartext stack from a production deployment.
    bool public constant IS_CLEARTEXT = true;

    function inputProof(
        bytes32[] calldata ctHandles,
        address userAddress,
        address contractAddress,
        bytes calldata extraData
    ) public view returns (bytes32 digest, address[] memory signers, uint256 threshold) {
        CiphertextVerification memory ctVerif;
        ctVerif.ctHandles = ctHandles;
        ctVerif.userAddress = userAddress;
        ctVerif.contractAddress = contractAddress;
        ctVerif.contractChainId = block.chainid;
        ctVerif.extraData = extraData;

        digest = _hashEIP712InputVerification(ctVerif);

        InputVerifierStorage storage $ = _getInputVerifierStorage();
        signers = $.signers;
        threshold = $.threshold;
    }
}

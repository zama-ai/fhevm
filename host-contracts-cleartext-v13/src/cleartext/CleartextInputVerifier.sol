// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {InputVerifier} from "../host-contracts/contracts/InputVerifier.sol";
import {aclAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";
import {CleartextACL} from "./CleartextACL.sol";

/**
 * @title CleartextInputVerifier
 */
contract CleartextInputVerifier is InputVerifier {
    function inputProof(
        bytes32[] calldata ctHandles,
        address userAddress,
        address contractAddress,
        bytes calldata extraData
    ) public view returns (bytes32 digest, address[] memory signers, uint256 threshold) {
        CleartextACL(aclAdd).requireNotPaused();

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

// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";

contract TestInput {
    euint64 public resUint64;

    constructor(address aclAddress, address coprocessorAddress, address kmsVerifierAddress) {
        FHE.setCoprocessor(
            CoprocessorConfig({
                ACLAddress: aclAddress,
                CoprocessorAddress: coprocessorAddress,
                KMSVerifierAddress: kmsVerifierAddress
            })
        );
    }

    function requestUint64NonTrivial(externalEuint64 inputHandle, bytes calldata inputProof) public {
        resUint64 = FHE.fromExternal(inputHandle, inputProof);
        FHE.allowThis(resUint64);
    }

    // Adds a trivially-encrypted 42 to the user-provided encrypted uint64 input.
    function add42ToInput64(externalEuint64 inputHandle, bytes calldata inputProof) public {
        euint64 input = FHE.fromExternal(inputHandle, inputProof);
        euint64 trivial42 = FHE.asEuint64(42);
        resUint64 = FHE.add(input, trivial42);
        FHE.allowThis(resUint64);
        FHE.allow(resUint64, msg.sender);
        FHE.makePubliclyDecryptable(resUint64);
    }
}

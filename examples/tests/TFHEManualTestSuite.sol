// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.25;

import "../../lib/TFHE.sol";

contract TFHEManualTestSuite {
    ebool public res;

    function eqEbytes256(
        einput inp1,
        bytes calldata inputProof1,
        einput inp2,
        bytes calldata inputProof2
    ) external returns (ebool) {
        ebytes256 input1 = TFHE.asEbytes256(inp1, inputProof1);
        ebytes256 input2 = TFHE.asEbytes256(inp2, inputProof2);
        ebool result = TFHE.eq(input1, input2);
        res = result;
        return result;
    }

    function neEbytes256(
        einput inp1,
        bytes calldata inputProof1,
        einput inp2,
        bytes calldata inputProof2
    ) external returns (ebool) {
        ebytes256 input1 = TFHE.asEbytes256(inp1, inputProof1);
        ebytes256 input2 = TFHE.asEbytes256(inp2, inputProof2);
        ebool result = TFHE.ne(input1, input2);
        res = result;
        return result;
    }
}

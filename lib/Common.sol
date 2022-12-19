// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.7.0 <0.9.0;

import "./Precompiles.sol";

library Common {
    // Requires that the ciphertext under the given handle is true.
    // If true, the runction returns. Otherwise, it reverts.
    function requireCt(uint256 handle) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(handle);
        uint256 inputOutputLen = 32;
        bytes32[1] memory output;

        // Call the require precompile.
        uint256 precompile = Precompiles.Require;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputOutputLen, output, inputOutputLen)) {
                revert(0, 0)
            }
        }
    }
}

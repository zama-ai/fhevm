// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { E2EFHEVMConfig } from "./E2EFHEVMConfig.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

/// @notice This contract implements an encrypted ERC20-like token with confidential balances using Zama's FHE (Fully Homomorphic Encryption) library.
/// @dev It supports typical ERC20 functionality such as transferring tokens, minting, and setting allowances, but uses encrypted data types.
contract UserDecryptBenchmark is Ownable2Step, E2EFHEVMConfig {
    /// @dev A mapping from address to an encrypted balance - tracks encrypted balances of each address
    euint64[] internal values;

    constructor() Ownable(msg.sender) {}

    function getValue(uint256 index) public view virtual returns (euint64) {
        return values[index];
    }

    function getValuesCount() public view virtual returns (uint256) {
        return values.length;
    }

    function getAllValues() public view virtual returns (euint64[] memory) {
        return values;
    }

    function getValuesRange(uint256 start, uint256 end) public view virtual returns (euint64[] memory) {
        require(start < end, "Invalid range");
        require(end <= values.length, "End index out of bounds");

        euint64[] memory result = new euint64[](end - start);
        for (uint256 i = start; i < end; i++) {
            result[i - start] = values[i];
        }
        return result;
    }

    function refresh(uint256 nValues) public virtual onlyOwner {
        // Override existing values
        for (uint256 index = 0; index < values.length; index++) {
            values[index] = FHE.randEuint64();
            FHE.allowThis(values[index]);
            FHE.allow(values[index], owner());
        }
        // Append new values
        for (uint256 index = values.length; index < nValues; index++) {
            euint64 value = FHE.randEuint64();
            values.push() = value;
            FHE.allowThis(value);
            FHE.allow(value, owner());
        }
    }
}

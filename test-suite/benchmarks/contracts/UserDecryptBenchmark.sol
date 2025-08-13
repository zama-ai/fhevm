// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { E2ECoprocessorConfig } from "./E2ECoprocessorConfig.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

/// @notice This contract implements an encrypted ERC20-like token with confidential balances using Zama's FHE (Fully Homomorphic Encryption) library.
/// @dev It supports typical ERC20 functionality such as transferring tokens, minting, and setting allowances, but uses encrypted data types.
contract UserDecryptBenchmark is Ownable2Step, E2ECoprocessorConfig {
    mapping(uint256 => euint64[]) public valuesFromBatch;

    constructor() Ownable(msg.sender) {}

    function getValuesFromBatch(uint256 batchIndex) public view virtual returns (euint64[] memory) {
        return valuesFromBatch[batchIndex];
    }

    function refresh(uint256 nValues, uint256 batchIndex) public virtual onlyOwner {
        euint64[] memory result = new euint64[](nValues);
        for (uint256 index = 0; index < nValues; index++) {
            euint64 value = FHE.randEuint64();
            FHE.allowThis(value);
            FHE.allow(value, owner());
            result[index] = value;
        }
        valuesFromBatch[batchIndex] = result;
    }
}

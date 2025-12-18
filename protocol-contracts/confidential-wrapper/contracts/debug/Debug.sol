// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import "hardhat/console.sol";
import "@fhevm/solidity/lib/FHE.sol";
import {ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";

library Debug {
    event Debug(string varType, string varName, bytes value);

    function asEuint64(
        string memory varName,
        euint64 value
    ) internal {
        FHE.makePubliclyDecryptable(value);
        emit Debug("euint64", varName, abi.encode(euint64.unwrap(value)));
    }

    function asAddress(
        string memory varName,
        address value
    ) internal {
        emit Debug("address", varName, abi.encode(value));
    }
}

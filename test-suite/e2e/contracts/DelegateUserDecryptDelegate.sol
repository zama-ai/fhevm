// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";


/// @notice Contract for demonstrating user decryption with delegation
contract DelegateUserDecryptDelegate is E2ECoprocessorConfig {
    ebool public storedZBool;

    function useBool(ebool xBool) public returns (ebool zBool) {
        ebool yBool = FHE.asEbool(true);
        zBool = FHE.and(xBool, yBool);
        FHE.allowThis(zBool);
        FHE.allow(zBool, msg.sender);
        storedZBool = zBool;
    }

    function getResult() public view returns (ebool) {
        return storedZBool;
    }
}

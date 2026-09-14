// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, euint32} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

/**
 * This trivial example demonstrates the FHE decryption mechanism
 * and highlights a common pitfall developers may encounter.
 */
contract DecryptSingleValue is ZamaEthereumConfig {
    euint32 private _trivialEuint32;

    // solhint-disable-next-line no-empty-blocks
    constructor() {}

    function initializeUint32(uint32 value) external {
        // Compute a trivial FHE formula _trivialEuint32 = value + 1
        _trivialEuint32 = FHE.add(FHE.asEuint32(value), FHE.asEuint32(1));

        // Grant FHE permissions to:
        // ✅ The contract caller (`msg.sender`): allows them to decrypt `_trivialEuint32`.
        // ✅ The contract itself (`address(this)`): allows it to operate on `_trivialEuint32` and
        //    also enables the caller to perform decryption.
        //
        // Note: If you forget to call `FHE.allowThis(_trivialEuint32)`, the user will NOT be able
        //       to decrypt the value! Both the contract and the caller must have FHE permissions
        //       for decryption to succeed.
        FHE.allowThis(_trivialEuint32);
        FHE.allow(_trivialEuint32, msg.sender);
    }

    function initializeUint32Wrong(uint32 value) external {
        // Compute a trivial FHE formula _trivialEuint32 = value + 1
        _trivialEuint32 = FHE.add(FHE.asEuint32(value), FHE.asEuint32(1));

        // ❌ Common FHE permission mistake:
        // ================================================================
        // We grant FHE permissions to the contract caller (`msg.sender`),
        // expecting they will be able to decrypt the encrypted value later.
        //
        // However, this will fail! 💥
        // The contract itself (`address(this)`) also needs FHE permissions to allow decryption.
        // Without granting the contract access using `FHE.allowThis(...)`,
        // the decryption attempt by the user will not succeed.
        FHE.allow(_trivialEuint32, msg.sender);
    }

    function encryptedUint32() public view returns (euint32) {
        return _trivialEuint32;
    }
}

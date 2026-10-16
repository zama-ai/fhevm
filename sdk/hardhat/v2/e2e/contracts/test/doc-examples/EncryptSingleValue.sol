// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, externalEuint32, euint32} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

/**
 * This trivial example demonstrates the FHE encryption mechanism.
 */
contract EncryptSingleValue is ZamaEthereumConfig {
    euint32 private _encryptedEuint32;

    // solhint-disable-next-line no-empty-blocks
    constructor() {}

    function initialize(externalEuint32 inputEuint32, bytes calldata inputProof) external {
        _encryptedEuint32 = FHE.fromExternal(inputEuint32, inputProof);

        // Grant FHE permission to both the contract itself (`address(this)`) and the caller (`msg.sender`),
        // to allow future decryption by the caller (`msg.sender`).
        FHE.allowThis(_encryptedEuint32);
        FHE.allow(_encryptedEuint32, msg.sender);
    }

    function encryptedUint32() public view returns (euint32) {
        return _encryptedEuint32;
    }
}

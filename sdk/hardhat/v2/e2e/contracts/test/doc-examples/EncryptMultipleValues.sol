// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {
    FHE,
    externalEbool,
    externalEuint32,
    externalEaddress,
    ebool,
    euint32,
    eaddress
} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

/**
 * This trivial example demonstrates the FHE encryption mechanism.
 */
contract EncryptMultipleValues is ZamaEthereumConfig {
    ebool private _encryptedEbool;
    euint32 private _encryptedEuint32;
    eaddress private _encryptedEaddress;

    // solhint-disable-next-line no-empty-blocks
    constructor() {}

    function initialize(
        externalEbool inputEbool,
        externalEuint32 inputEuint32,
        externalEaddress inputEaddress,
        bytes calldata inputProof
    ) external {
        _encryptedEbool = FHE.fromExternal(inputEbool, inputProof);
        _encryptedEuint32 = FHE.fromExternal(inputEuint32, inputProof);
        _encryptedEaddress = FHE.fromExternal(inputEaddress, inputProof);

        // For each of the 3 values:
        // Grant FHE permission to both the contract itself (`address(this)`) and the caller (`msg.sender`),
        // to allow future decryption by the caller (`msg.sender`).

        FHE.allowThis(_encryptedEbool);
        FHE.allow(_encryptedEbool, msg.sender);

        FHE.allowThis(_encryptedEuint32);
        FHE.allow(_encryptedEuint32, msg.sender);

        FHE.allowThis(_encryptedEaddress);
        FHE.allow(_encryptedEaddress, msg.sender);
    }

    function encryptedBool() public view returns (ebool) {
        return _encryptedEbool;
    }

    function encryptedUint32() public view returns (euint32) {
        return _encryptedEuint32;
    }

    function encryptedAddress() public view returns (eaddress) {
        return _encryptedEaddress;
    }
}

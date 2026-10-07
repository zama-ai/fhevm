// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, ebool, euint32, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

contract DecryptMultipleValues is ZamaEthereumConfig {
    ebool private _encryptedBool; // = 0 (uninitizalized)
    euint32 private _encryptedUint32; // = 0 (uninitizalized)
    euint64 private _encryptedUint64; // = 0 (uninitizalized)

    // solhint-disable-next-line no-empty-blocks
    constructor() {}

    function initialize(bool a, uint32 b, uint64 c) external {
        // Compute 3 trivial FHE formulas

        // _encryptedBool = a ^ false
        _encryptedBool = FHE.xor(FHE.asEbool(a), FHE.asEbool(false));

        // _encryptedUint32 = b + 1
        _encryptedUint32 = FHE.add(FHE.asEuint32(b), FHE.asEuint32(1));

        // _encryptedUint64 = c + 1
        _encryptedUint64 = FHE.add(FHE.asEuint64(c), FHE.asEuint64(1));

        // see `DecryptSingleValue.sol` for more detailed explanations
        // about FHE permissions and asynchronous decryption requests.
        FHE.allowThis(_encryptedBool);
        FHE.allowThis(_encryptedUint32);
        FHE.allowThis(_encryptedUint64);

        FHE.allow(_encryptedBool, msg.sender);
        FHE.allow(_encryptedUint32, msg.sender);
        FHE.allow(_encryptedUint64, msg.sender);
    }

    function encryptedBool() public view returns (ebool) {
        return _encryptedBool;
    }

    function encryptedUint32() public view returns (euint32) {
        return _encryptedUint32;
    }

    function encryptedUint64() public view returns (euint64) {
        return _encryptedUint64;
    }
}

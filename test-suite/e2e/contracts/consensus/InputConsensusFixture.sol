// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// Expose verified inputs without manufacturing a computation-output row.
contract InputConsensusFixture is E2ECoprocessorConfig {
    function accept(bytes32[] calldata handles, uint8 kind, bytes calldata proof) external {
        require(handles.length > 0, "empty input");
        for (uint256 i = 0; i < handles.length; i++) {
            if (kind == 0) {
                ebool value = FHE.fromExternal(externalEbool.wrap(handles[i]), proof);
                FHE.allowThis(value);
                FHE.allow(value, msg.sender);
                FHE.makePubliclyDecryptable(value);
            }
            else if (kind == 2) {
                euint8 value = FHE.fromExternal(externalEuint8.wrap(handles[i]), proof);
                FHE.allowThis(value);
                FHE.allow(value, msg.sender);
                FHE.makePubliclyDecryptable(value);
            }
            else if (kind == 3) {
                euint16 value = FHE.fromExternal(externalEuint16.wrap(handles[i]), proof);
                FHE.allowThis(value);
                FHE.allow(value, msg.sender);
                FHE.makePubliclyDecryptable(value);
            }
            else if (kind == 4) {
                euint32 value = FHE.fromExternal(externalEuint32.wrap(handles[i]), proof);
                FHE.allowThis(value);
                FHE.allow(value, msg.sender);
                FHE.makePubliclyDecryptable(value);
            }
            else if (kind == 5) {
                euint64 value = FHE.fromExternal(externalEuint64.wrap(handles[i]), proof);
                FHE.allowThis(value);
                FHE.allow(value, msg.sender);
                FHE.makePubliclyDecryptable(value);
            }
            else if (kind == 6) {
                euint128 value = FHE.fromExternal(externalEuint128.wrap(handles[i]), proof);
                FHE.allowThis(value);
                FHE.allow(value, msg.sender);
                FHE.makePubliclyDecryptable(value);
            }
            else if (kind == 8) {
                euint256 value = FHE.fromExternal(externalEuint256.wrap(handles[i]), proof);
                FHE.allowThis(value);
                FHE.allow(value, msg.sender);
                FHE.makePubliclyDecryptable(value);
            }
            else { revert("unsupported input type"); }
        }
    }
}

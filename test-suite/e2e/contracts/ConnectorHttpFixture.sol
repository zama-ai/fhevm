// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

/// @notice Fixture for the kms-connector HTTP endpoint negative and re-arm tests.
/// @dev `xPrivate` is never made publicly decryptable nor allowed to any user, so
///      a public decryption of it is denied by the ACL on every KMS party.
///      `xLater` starts private too; `makeLaterPubliclyDecryptable` flips it so a
///      request that was denied can be re-submitted and succeed.
contract ConnectorHttpFixture is E2ECoprocessorConfig {
    euint32 public xPrivate;
    euint32 public xLater;

    constructor() {
        xPrivate = FHE.asEuint32(7);
        FHE.allowThis(xPrivate);

        xLater = FHE.asEuint32(1337);
        FHE.allowThis(xLater);
    }

    function makeLaterPubliclyDecryptable() external {
        FHE.makePubliclyDecryptable(xLater);
    }
}

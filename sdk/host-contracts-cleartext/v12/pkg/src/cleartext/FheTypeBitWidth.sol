// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "../contracts/shared/FheType.sol";

library FheTypeBitWidth {
    error UnsupportedType();

    function bitWidthForType(FheType fheType) internal pure returns (uint256) {
        if (fheType == FheType.Bool) return 1;
        if (fheType == FheType.Uint8) return 8;
        if (fheType == FheType.Uint16) return 16;
        if (fheType == FheType.Uint32) return 32;
        if (fheType == FheType.Uint64) return 64;
        if (fheType == FheType.Uint128) return 128;
        if (fheType == FheType.Uint160) return 160;
        if (fheType == FheType.Uint256) return 256;

        revert UnsupportedType();
    }
}

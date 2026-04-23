// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";

library FheTypeBitWidth {
    error UnsupportedType();

    function bitWidthForType(uint8 fheType) internal pure returns (uint256) {
        // 0
        if (fheType == uint8(FheType.Bool)) return 1;
        // 2
        if (fheType == uint8(FheType.Uint8)) return 8;
        // 3
        if (fheType == uint8(FheType.Uint16)) return 16;
        // 4
        if (fheType == uint8(FheType.Uint32)) return 32;
        // 5
        if (fheType == uint8(FheType.Uint64)) return 64;
        // 6
        if (fheType == uint8(FheType.Uint128)) return 128;
        // 7
        if (fheType == uint8(FheType.Uint160)) return 160;
        // 8
        if (fheType == uint8(FheType.Uint256)) return 256;

        // Make sure
        assert(fheType > uint8(FheType.Uint256));

        // Not used
        if (fheType == uint8(FheType.Uint2)) return 2;
        if (fheType == uint8(FheType.Uint4)) return 4;
        if (fheType == uint8(FheType.Uint6)) return 6;
        if (fheType == uint8(FheType.Uint10)) return 10;
        if (fheType == uint8(FheType.Uint12)) return 12;
        if (fheType == uint8(FheType.Uint14)) return 14;
        if (fheType == uint8(FheType.Uint24)) return 24;
        if (fheType == uint8(FheType.Uint40)) return 40;
        if (fheType == uint8(FheType.Uint48)) return 48;
        if (fheType == uint8(FheType.Uint56)) return 56;
        if (fheType == uint8(FheType.Uint72)) return 72;
        if (fheType == uint8(FheType.Uint80)) return 80;
        if (fheType == uint8(FheType.Uint88)) return 88;
        if (fheType == uint8(FheType.Uint96)) return 96;
        if (fheType == uint8(FheType.Uint104)) return 104;
        if (fheType == uint8(FheType.Uint112)) return 112;
        if (fheType == uint8(FheType.Uint120)) return 120;
        if (fheType == uint8(FheType.Uint136)) return 136;
        if (fheType == uint8(FheType.Uint144)) return 144;
        if (fheType == uint8(FheType.Uint152)) return 152;
        if (fheType == uint8(FheType.Uint168)) return 168;
        if (fheType == uint8(FheType.Uint176)) return 176;
        if (fheType == uint8(FheType.Uint184)) return 184;
        if (fheType == uint8(FheType.Uint192)) return 192;
        if (fheType == uint8(FheType.Uint200)) return 200;
        if (fheType == uint8(FheType.Uint208)) return 208;
        if (fheType == uint8(FheType.Uint216)) return 216;
        if (fheType == uint8(FheType.Uint224)) return 224;
        if (fheType == uint8(FheType.Uint232)) return 232;
        if (fheType == uint8(FheType.Uint240)) return 240;
        if (fheType == uint8(FheType.Uint248)) return 248;
        if (fheType == uint8(FheType.Uint512)) return 512;
        if (fheType == uint8(FheType.Uint1024)) return 1024;
        if (fheType == uint8(FheType.Uint2048)) return 2048;

        if (fheType == uint8(FheType.Int2)) return 2;
        if (fheType == uint8(FheType.Int4)) return 4;
        if (fheType == uint8(FheType.Int6)) return 6;
        if (fheType == uint8(FheType.Int8)) return 8;
        if (fheType == uint8(FheType.Int10)) return 10;
        if (fheType == uint8(FheType.Int12)) return 12;
        if (fheType == uint8(FheType.Int14)) return 14;
        if (fheType == uint8(FheType.Int16)) return 16;
        if (fheType == uint8(FheType.Int24)) return 24;
        if (fheType == uint8(FheType.Int32)) return 32;
        if (fheType == uint8(FheType.Int40)) return 40;
        if (fheType == uint8(FheType.Int48)) return 48;
        if (fheType == uint8(FheType.Int56)) return 56;
        if (fheType == uint8(FheType.Int64)) return 64;
        if (fheType == uint8(FheType.Int72)) return 72;
        if (fheType == uint8(FheType.Int80)) return 80;
        if (fheType == uint8(FheType.Int88)) return 88;
        if (fheType == uint8(FheType.Int96)) return 96;
        if (fheType == uint8(FheType.Int104)) return 104;
        if (fheType == uint8(FheType.Int112)) return 112;
        if (fheType == uint8(FheType.Int120)) return 120;
        if (fheType == uint8(FheType.Int128)) return 128;
        if (fheType == uint8(FheType.Int136)) return 136;
        if (fheType == uint8(FheType.Int144)) return 144;
        if (fheType == uint8(FheType.Int152)) return 152;
        if (fheType == uint8(FheType.Int160)) return 160;
        if (fheType == uint8(FheType.Int168)) return 168;
        if (fheType == uint8(FheType.Int176)) return 176;
        if (fheType == uint8(FheType.Int184)) return 184;
        if (fheType == uint8(FheType.Int192)) return 192;
        if (fheType == uint8(FheType.Int200)) return 200;
        if (fheType == uint8(FheType.Int208)) return 208;
        if (fheType == uint8(FheType.Int216)) return 216;
        if (fheType == uint8(FheType.Int224)) return 224;
        if (fheType == uint8(FheType.Int232)) return 232;
        if (fheType == uint8(FheType.Int240)) return 240;
        if (fheType == uint8(FheType.Int248)) return 248;
        if (fheType == uint8(FheType.Int256)) return 256;
        if (fheType == uint8(FheType.Int512)) return 512;
        if (fheType == uint8(FheType.Int1024)) return 1024;
        if (fheType == uint8(FheType.Int2048)) return 2048;

        revert UnsupportedType();
    }
}

// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "../../host-contracts/contracts/shared/FheType.sol";

/// ----------------------------------------------------------------------------
///   ⚠️ Private Library:
///     - Host-internal, shipped with the forge payload
///     - Should not be exposed to the public
///     - not maintained
///     - can change at any time
/// ----------------------------------------------------------------------------

library LibFheType {
    /// @notice A type outside the set this stack implements.
    error CleartextErrorUnsupportedType();

    /// @notice How many bits a value of this type occupies — the modulus every cleartext operation
    ///         wraps at, and the width every result is clamped to.
    /// @dev    Reverts rather than defaulting: a width guessed for an unsupported type would silently
    ///         produce arithmetic the coprocessor never performs.
    function bitWidthForType(FheType fheType) internal pure returns (uint256) {
        if (fheType == FheType.Bool) return 1;
        if (fheType == FheType.Uint8) return 8;
        if (fheType == FheType.Uint16) return 16;
        if (fheType == FheType.Uint32) return 32;
        if (fheType == FheType.Uint64) return 64;
        if (fheType == FheType.Uint128) return 128;
        if (fheType == FheType.Uint160) return 160;
        if (fheType == FheType.Uint256) return 256;

        revert CleartextErrorUnsupportedType();
    }

    /// @notice The name of an FHE type, like "euint32" or "eaddress".
    /// @dev A type this stack does not implement comes back as "FheType(unsupported)" — not as its enum
    ///      index. `FheType` has 84 members and this stack implements 8, so a numeric fallback would need
    ///      an integer-to-string routine, and THIS FILE IS COMPILED INTO THE DEPLOYED CONTRACTS (rules.md
    ///      2.1): it cannot reach for `vm.toString`, and hand-rolling one on chain to decorate an error
    ///      message is not worth its bytecode. The caller knows the handle; the name of a type it cannot
    ///      use adds nothing the error does not already say.
    function toString(FheType t) internal pure returns (string memory) {
        if (t == FheType.Bool) return "ebool";
        if (t == FheType.Uint8) return "euint8";
        if (t == FheType.Uint16) return "euint16";
        if (t == FheType.Uint32) return "euint32";
        if (t == FheType.Uint64) return "euint64";
        if (t == FheType.Uint128) return "euint128";
        if (t == FheType.Uint160) return "eaddress";
        if (t == FheType.Uint256) return "euint256";
        return "FheType(unsupported)";
    }
}

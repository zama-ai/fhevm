// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/FheType.sol";

library FHETypeBitSizes {
    // Cleartext bit size for each FHE type.
    // Note that ebool have a bit size of 2.
    uint16 internal constant EBOOL_SIZE = 2;
    uint16 internal constant EUINT8_SIZE = 8;
    uint16 internal constant EUINT16_SIZE = 16;
    uint16 internal constant EUINT32_SIZE = 32;
    uint16 internal constant EUINT64_SIZE = 64;
    uint16 internal constant EUINT128_SIZE = 128;
    uint16 internal constant EUINT160_SIZE = 160;
    uint16 internal constant EUINT256_SIZE = 256;
    uint16 internal constant EBYTES64_SIZE = 512;
    uint16 internal constant EBYTES128_SIZE = 1024;
    uint16 internal constant EBYTES256_SIZE = 2048;

    /// @notice Error thrown when FHE type is not supported.
    /// @param fheType The unsupported FHE type.
    error UnsupportedFHEType(FheType fheType);

    /// @notice Get the bit size for a given FHE type.
    /// @param fheType The FHE type.
    /// @return The bit size for the given FHE type.
    // solhint-disable-next-line code-complexity
    function getBitSize(FheType fheType) internal pure returns (uint16) {
        /// @dev The following normally triggers a "cyclomatic complexity" error from solhint.
        /// @dev This could be avoid by, for example, using fixed array that maps types with sizes.
        /// @dev Still, we keep considering if-else branches as it will be easier to maintain if we
        /// @dev need to consider or remove other types in the future.
        /// @dev The full list of FHE types is available in the `FheType` enum and comes from the
        /// @dev `fhevm-solidity` repository, which is directly based on TFHE-rs' list of supported
        /// @dev types.
        /// @dev Note that only a subset of them are currently supported (in particular, Uint4 is not
        /// @dev supported). This list is also defined in the `fhevm-solidity` repository.
        if (fheType == FheType.Bool) {
            return EBOOL_SIZE;
        } else if (fheType == FheType.Uint8) {
            return EUINT8_SIZE;
        } else if (fheType == FheType.Uint16) {
            return EUINT16_SIZE;
        } else if (fheType == FheType.Uint32) {
            return EUINT32_SIZE;
        } else if (fheType == FheType.Uint64) {
            return EUINT64_SIZE;
        } else if (fheType == FheType.Uint128) {
            return EUINT128_SIZE;
        } else if (fheType == FheType.Uint160) {
            return EUINT160_SIZE;
        } else if (fheType == FheType.Uint256) {
            return EUINT256_SIZE;
        } else if (fheType == FheType.Uint512) {
            return EBYTES64_SIZE;
        } else if (fheType == FheType.Uint1024) {
            return EBYTES128_SIZE;
        } else if (fheType == FheType.Uint2048) {
            return EBYTES256_SIZE;
        } else {
            revert UnsupportedFHEType(fheType);
        }
    }
}

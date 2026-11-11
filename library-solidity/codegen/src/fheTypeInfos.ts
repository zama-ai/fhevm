import type { FheTypeInfo } from './common.js';
import fheTypeInfos from './fheTypeInfos.json' with { type: 'json' };

/**
 * A constant array containing all Fully Homomorphic Encryption (FHE) types, loaded from `fheTypeInfos.json`.
 * Each type is represented as an object with the following properties:
 *
 * - `type`: The name of the FHE type.
 * - `value`: A unique numeric identifier for the FHE type.
 * - `supportedOperators`: An array of strings representing the operators supported by the FHE type.
 * - `bitLength`: The bit length of the FHE type.
 * - `clearMatchingType`: The corresponding clear (non-encrypted) type in Solidity.
 * - `aliases`: An optional array of alias types that are associated with this FHE type.
 *
 * The FHE types included that are currently implemented in the Solidity code generator are:
 *
 * - `Bool`: Boolean type with a bit length of 1.
 * - `Uint8`: Unsigned integer type with a bit length of 8.
 * - `Uint16`: Unsigned integer type with a bit length of 16.
 * - `Uint32`: Unsigned integer type with a bit length of 32.
 * - `Uint64`: Unsigned integer type with a bit length of 64.
 * - `Uint128`: Unsigned integer type with a bit length of 128.
 * - `Uint160`: Unsigned integer type with a bit length of 160.
 * - `Uint256`: Unsigned integer type with a bit length of 256.
 *
 * The data lives in `fheTypeInfos.json`, which both ES modules (codegen, the Hardhat 3 tests) and CommonJS modules (the
 * Hardhat 2 tests, through `lib-js/`) can load. JSON has no union types, hence the cast: the values are checked at
 * runtime by `validate()` (see `validate.ts`).
 */
export const ALL_FHE_TYPE_INFOS = fheTypeInfos as FheTypeInfo[];

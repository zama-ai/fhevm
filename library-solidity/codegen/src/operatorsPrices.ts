import type { PriceData } from './common.js';
import operatorsPrices from './operatorsPrices.json' with { type: 'json' };

/**
 * The HCU price of each FHE operator, per FHE type.
 *
 * `fheMulDiv[T] = fheMul[2T] + fheDiv[2T]` (tfhe-rs widens to 2T internally).
 *
 * The data lives in `operatorsPrices.json`, which both ES modules (codegen, the Hardhat 3 tests) and CommonJS modules
 * (the Hardhat 2 tests, through `lib-js/`) can load. JSON has no union types, hence the cast: the values are checked at
 * runtime by `validate()` (see `validate.ts`).
 */
export const ALL_OPERATORS_PRICES = operatorsPrices as PriceData;

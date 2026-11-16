// CommonJS facade over the codegen data, for the Hardhat 2 tests of host-contracts and test-suite/e2e (CommonJS, ts-node):
// they cannot load the codegen ES modules. Hence this package.json ("type": "commonjs"), the type-only imports of codegen
// and the data read from JSON. library-solidity itself imports codegen directly. To be deleted once both consumers are
// on Hardhat 3.
import type { PriceData } from '../codegen/src/common.js' with { 'resolution-mode': 'import' };
import operatorsPrices from '../codegen/src/operatorsPrices.json';

// Same data and type as codegen's ALL_OPERATORS_PRICES (codegen/src/operatorsPrices.ts), validated by the codegen.
export const ALL_OPERATORS_PRICES = operatorsPrices as PriceData;

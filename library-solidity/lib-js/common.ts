// CommonJS facade over the codegen data, for the Hardhat 2 tests of host-contracts and test-suite/e2e (CommonJS, ts-node):
// they cannot load the codegen ES modules. Hence this package.json ("type": "commonjs"), the type-only imports of codegen
// and the data read from JSON. library-solidity itself imports codegen directly. To be deleted once both consumers are
// on Hardhat 3.
export type * from '../codegen/src/common.js' with { 'resolution-mode': 'import' };

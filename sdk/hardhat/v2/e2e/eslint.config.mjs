// The `hardhat/v2/e2e` layout, applied to the workspace rules in sdk/eslint.base.mjs.
//
// `.mjs`, not `.js`: this package is `"type": "commonjs"` because Hardhat v2 requires it, so a `.js`
// config here would be parsed as CommonJS and could not `import` the shared base.
import eslintBase from '../../../eslint.base.mjs';

export default eslintBase({
  packageDir: import.meta.dirname,
  // Nothing here is published: the whole package is node-side test code.
  publicFiles: [],
  nodeFiles: ['hardhat.config.ts', 'test/**/*.ts'],
  returnTypeFiles: ['hardhat.config.ts', 'test/**/*.ts'],
  ignores: [
    // Build output: `hardhat compile`, typechain, forge.
    'artifacts/**',
    'cache/**',
    'out/**',
    'typechain-types/**',
    'dependencies/**',
    // 36k lines mirroring library-solidity/test/fhevmOperations, itself codegen output. Linting a
    // generated file only asks for edits the next regeneration discards.
    'test/hardhat-mock-engine/operators/**',
    'test/hardhat-mock-engine/operators-manual/**',
    'test/hardhat-mock-engine/operators-public-decrypt/**',
  ],
});

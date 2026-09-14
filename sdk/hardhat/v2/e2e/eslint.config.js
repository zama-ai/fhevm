// The `hardhat/v2/e2e` layout, applied to the workspace rules in sdk/eslint.base.mjs.
//
// This package is CommonJS because Hardhat v2 requires it. ESLint awaits the exported promise, which
// lets this conventional `.js` config load the shared ESM base without introducing another filename.
module.exports = import('../../../eslint.base.mjs').then(({ default: eslintBase }) =>
  eslintBase({
    packageDir: __dirname,
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
  }),
);

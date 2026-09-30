// The `hardhat/v3/e2e` layout, applied to the workspace rules in sdk/eslint.base.mjs.
import eslintBase from '../../../eslint.base.mjs';

export default [
  ...eslintBase({
    packageDir: import.meta.dirname,
    // Nothing here is published: the whole package is node-side test code.
    publicFiles: [],
    nodeFiles: ['hardhat.config.ts', 'test/**/*.ts'],
    returnTypeFiles: ['hardhat.config.ts', 'test/**/*.ts'],
    browserSafeFiles: [],
    // Build output: `hardhat build` (artifacts, typechain `types`), forge.
    ignores: [
      'artifacts/**',
      'cache/**',
      'types/**',
      'out/**',
      'dependencies/**',
      // 36k lines mirroring library-solidity/test/fhevmOperations, itself codegen output (mocha-context
      // typed `any`). Linting a generated file only asks for edits the next regeneration discards.
      'test/operators/**',
    ],
  }),
];

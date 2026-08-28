// The `hardhat/v2` layout, applied to the workspace rules in sdk/eslint.base.mjs.
import eslintBase from '../../eslint.base.mjs';

export default [
  ...eslintBase({
    packageDir: import.meta.dirname,
    publicFiles: ['pkg/src/**/*.ts'],
    nodeFiles: ['internal/**/*.ts', 'test/**/*.ts'],
    returnTypeFiles: ['pkg/src/**/*.ts'],
    // Published, but Node-only: a Hardhat v2 plugin never runs in a browser.
    browserSafeFiles: [],
    // `vendored/` stays byte-identical to upstream, which an autofix would break.
    ignores: ['pkg/_cjs/**', 'pkg/_types/**', 'pkg/src/internal/vendored/**'],
  }),

  {
    // `require` is this package's module system: Hardhat v2 loads plugins with it, and only a call that
    // can throw where it stands can load an optional companion plugin that may not be installed.
    files: ['pkg/src/**/*.ts'],
    rules: {
      '@typescript-eslint/no-require-imports': 'off',
    },
  },
];

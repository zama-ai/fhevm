// The `hardhat/v3/plugin` layout, applied to the workspace rules in sdk/eslint.base.mjs.
import eslintBase from '../../../eslint.base.mjs';

export default [
  ...eslintBase({
    packageDir: import.meta.dirname,
    publicFiles: ['pkg/src/**/*.ts'],
    nodeFiles: ['test/**/*.ts'],
    returnTypeFiles: ['pkg/src/**/*.ts'],
    // Published, but Node-only: a Hardhat plugin never runs in a browser.
    browserSafeFiles: [],
    ignores: ['pkg/_esm/**'],
  }),
];

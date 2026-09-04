import eslintBase from '../../../eslint.base.mjs';

export default [
  ...eslintBase({
    packageDir: import.meta.dirname,
    publicFiles: [],
    nodeFiles: [],
    returnTypeFiles: [],
    browserSafeFiles: [],
    ignores: ['pkg/**'],
  }),
];

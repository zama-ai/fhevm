import eslintBase from '../../../../eslint.base.mjs';

export default [
  ...eslintBase({
    packageDir: import.meta.dirname,
    publicFiles: [],
    nodeFiles: ['hardhat.config.ts', 'ignition/**/*.ts', 'tasks/**/*.ts', 'test/**/*.ts'],
    returnTypeFiles: [],
    browserSafeFiles: [],
    ignores: ['artifacts/**', 'cache/**', 'dist/**', 'ignition/deployments/**'],
  }),
];

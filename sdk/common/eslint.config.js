import eslintBase from '../eslint.base.mjs';

export default eslintBase({
  packageDir: import.meta.dirname,
  publicFiles: [],
  nodeFiles: ['src/**/*.ts'],
  // Explicit return types required: these helpers are shared across the sdk workspace.
  returnTypeFiles: ['src/**/*.ts'],
});

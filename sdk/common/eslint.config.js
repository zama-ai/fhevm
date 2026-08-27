import eslintBase from '../eslint.base.mjs';

export default eslintBase({
  packageDir: import.meta.dirname,
  publicFiles: [],
  nodeFiles: ['src/**/*.ts'],
  // Explicit return types required: these helpers are imported by every harness.
  returnTypeFiles: ['src/**/*.ts'],
});

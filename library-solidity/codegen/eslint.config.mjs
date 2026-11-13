import { createConfig } from '../eslint.config.mjs';

export default createConfig({
  files: ['src/**/*.ts'],
  ignores: ['dist/**', 'node_modules/**', '**/*.js', '**/*.cjs', '**/*.mjs'],
  tsconfigRootDir: import.meta.dirname,
});

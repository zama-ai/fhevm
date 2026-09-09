import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

const typescriptFiles = ['hardhat.config.ts', 'ignition/**/*.ts', 'tasks/**/*.ts', 'test/**/*.ts'];

export default [
  {
    ignores: ['artifacts/**', 'cache/**', 'dist/**', 'ignition/deployments/**', 'node_modules/**', '**/*.js'],
  },
  { ...eslint.configs.recommended, files: typescriptFiles },
  ...tseslint.configs.strictTypeChecked.map((config) => ({ ...config, files: typescriptFiles })),
  ...tseslint.configs.stylisticTypeChecked.map((config) => ({ ...config, files: typescriptFiles })),
  {
    files: typescriptFiles,
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    rules: {
      'no-undef': 'off',
      'no-unused-vars': 'off',
      '@typescript-eslint/consistent-type-definitions': 'off',
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_',
        },
      ],
    },
  },
];

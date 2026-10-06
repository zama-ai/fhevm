import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

const targetFiles = ['src/**/*.ts'];

/** @type {import('typescript-eslint').ConfigArray} */
export default [
  // Global ignores
  {
    ignores: [
      'src/_cjs/**',
      'src/_esm/**',
      'src/_types/**',
      'src/wasm/**',
      'node_modules/**',
      'coverage/**',
      '**/*.test.ts',
      '**/*.test-d.ts',
      'src/vitest.config.ts',
      '**/*.js',
      '**/*.cjs',
      '**/*.mjs',
      'test/standalone/relayer-sdk-test/**',
    ],
  },

  // Base ESLint recommended config
  {
    ...eslint.configs.recommended,
    files: targetFiles,
  },

  // TypeScript ESLint strict type-checked configs
  ...tseslint.configs.strictTypeChecked.map((config) => ({
    ...config,
    files: targetFiles,
  })),

  // TypeScript ESLint stylistic type-checked configs
  ...tseslint.configs.stylisticTypeChecked.map((config) => ({
    ...config,
    files: targetFiles,
  })),

  // TypeScript ESLint strict rules for the target folders only
  {
    files: targetFiles,
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    rules: {
      // Strict type safety
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unsafe-assignment': 'error',
      '@typescript-eslint/no-unsafe-call': 'error',
      '@typescript-eslint/no-unsafe-member-access': 'error',
      '@typescript-eslint/no-unsafe-return': 'error',
      '@typescript-eslint/no-unsafe-argument': 'error',
      '@typescript-eslint/no-inferrable-types': 'off',
      '@typescript-eslint/restrict-template-expressions': 'off',
      '@typescript-eslint/prefer-nullish-coalescing': 'off',
      '@typescript-eslint/prefer-for-of': 'off',
      '@typescript-eslint/prefer-nullish-coalescing': 'off',

      // Require explicit return types
      '@typescript-eslint/explicit-function-return-type': [
        'error',
        {
          allowExpressions: true,
          allowTypedFunctionExpressions: true,
          allowHigherOrderFunctions: true,
          allowDirectConstAssertionInArrowFunctions: true,
        },
      ],
      '@typescript-eslint/explicit-module-boundary-types': 'error',

      // Strict null checks
      '@typescript-eslint/no-non-null-assertion': 'error',
      '@typescript-eslint/strict-boolean-expressions': [
        'error',
        {
          allowString: false,
          allowNumber: false,
          allowNullableObject: true,
          allowNullableBoolean: false,
          allowNullableString: false,
          allowNullableNumber: false,
          allowAny: false,
        },
      ],

      // Prevent common mistakes
      '@typescript-eslint/no-floating-promises': 'error',
      '@typescript-eslint/no-misused-promises': 'error',
      '@typescript-eslint/await-thenable': 'error',
      '@typescript-eslint/require-await': 'error',
      '@typescript-eslint/no-unnecessary-condition': 'error',
      '@typescript-eslint/no-unnecessary-type-assertion': 'error',
      '@typescript-eslint/no-redundant-type-constituents': 'error',

      // Code quality
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_',
        },
      ],
      '@typescript-eslint/no-shadow': 'error',
      '@typescript-eslint/prefer-nullish-coalescing': 'error',
      '@typescript-eslint/prefer-optional-chain': 'error',
      '@typescript-eslint/prefer-readonly': 'error',
      '@typescript-eslint/prefer-as-const': 'error',

      // Consistent code style
      '@typescript-eslint/consistent-type-imports': [
        'error',
        { prefer: 'type-imports', fixStyle: 'separate-type-imports' },
      ],
      '@typescript-eslint/consistent-type-exports': ['error', { fixMixedExportsWithInlineTypeSpecifier: false }],
      '@typescript-eslint/consistent-type-definitions': 'off', // Allow both interface and type
      '@typescript-eslint/array-type': ['error', { default: 'array-simple' }],
      '@typescript-eslint/consistent-indexed-object-style': ['error', 'record'],

      // Naming conventions
      '@typescript-eslint/naming-convention': [
        'error',
        {
          selector: 'interface',
          format: ['PascalCase'],
        },
        {
          selector: 'typeAlias',
          format: ['PascalCase'],
        },
        {
          selector: 'enum',
          format: ['PascalCase'],
        },
        {
          selector: 'enumMember',
          format: ['UPPER_CASE', 'PascalCase'],
        },
        {
          selector: 'variable',
          modifiers: ['const'],
          format: ['camelCase', 'UPPER_CASE', 'PascalCase'],
        },
        {
          selector: 'function',
          format: ['camelCase', 'PascalCase'],
          leadingUnderscore: 'allow',
        },
        {
          selector: 'parameter',
          format: ['camelCase'],
          leadingUnderscore: 'allow',
        },
        {
          selector: 'class',
          format: ['PascalCase'],
        },
      ],

      // Prevent problematic patterns
      '@typescript-eslint/no-confusing-void-expression': 'error',
      '@typescript-eslint/no-meaningless-void-operator': 'error',
      '@typescript-eslint/no-mixed-enums': 'error',
      '@typescript-eslint/no-useless-empty-export': 'error',
      '@typescript-eslint/prefer-enum-initializers': 'error',
      '@typescript-eslint/prefer-includes': 'error',
      '@typescript-eslint/prefer-string-starts-ends-with': 'error',
      '@typescript-eslint/switch-exhaustiveness-check': 'error',
      '@typescript-eslint/unified-signatures': 'error',

      // Disable base ESLint rules that conflict with TypeScript
      'no-unused-vars': 'off',
      'no-shadow': 'off',
      'no-undef': 'off',

      // Prevent Node.js-specific globals in browser code
      'no-restricted-globals': [
        'error',
        {
          name: 'process',
          message: 'process is Node.js specific and not available in browsers. Use a browser-compatible alternative.',
        },
        {
          name: 'Buffer',
          message: 'Buffer is Node.js specific. Use Uint8Array or TextEncoder/TextDecoder instead.',
        },
        {
          name: '__dirname',
          message: '__dirname is Node.js specific. Use import.meta.url instead.',
        },
        {
          name: '__filename',
          message: '__filename is Node.js specific. Use import.meta.url instead.',
        },
        {
          name: 'global',
          message: 'global is Node.js specific. Use globalThis for cross-platform compatibility.',
        },
      ],
    },
  },

  // Suppress "unused eslint-disable directive" in files where CLI and IDE
  // disagree on @typescript-eslint/no-unnecessary-type-arguments (TS version mismatch).
  {
    files: ['src/core/base/trustedValue.ts', 'src/core/base/isomorphicWorker.ts'],
    linterOptions: {
      reportUnusedDisableDirectives: 'off',
    },
  },
];

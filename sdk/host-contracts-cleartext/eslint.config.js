import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

const publicFiles = ['ts/**/*.ts'];
const nodeFiles = ['internal/**/*.ts', 'test/**/*.ts'];
const targetFiles = [...publicFiles, ...nodeFiles];
const generatedFiles = ['ts/_cjs/**', 'ts/_esm/**', 'ts/_types/**'];

const browserRestrictedGlobals = [
  {
    name: 'process',
    message: 'process is Node.js specific and not available in browsers. Use an injected adapter instead.',
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
];

/** @type {import('typescript-eslint').ConfigArray} */
export default [
  {
    ignores: [
      ...generatedFiles,
      'node_modules/**',
      'coverage/**',
      'cache/**',
      'out/**',
      'test/ts/.consumer/**',
      'test/ts/.tarballs/**',
      '**/*.js',
      '**/*.cjs',
      '**/*.mjs',
    ],
  },

  {
    ...eslint.configs.recommended,
    files: targetFiles,
  },

  ...tseslint.configs.strictTypeChecked.map((config) => ({
    ...config,
    files: targetFiles,
  })),

  ...tseslint.configs.stylisticTypeChecked.map((config) => ({
    ...config,
    files: targetFiles,
  })),

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
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unsafe-assignment': 'error',
      '@typescript-eslint/no-unsafe-call': 'error',
      '@typescript-eslint/no-unsafe-member-access': 'error',
      '@typescript-eslint/no-unsafe-return': 'error',
      '@typescript-eslint/no-unsafe-argument': 'error',
      '@typescript-eslint/no-inferrable-types': 'off',
      '@typescript-eslint/no-unnecessary-type-parameters': 'off',
      '@typescript-eslint/restrict-template-expressions': 'off',
      '@typescript-eslint/prefer-for-of': 'off',
      '@typescript-eslint/prefer-nullish-coalescing': 'error',

      '@typescript-eslint/explicit-module-boundary-types': 'error',

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

      '@typescript-eslint/no-floating-promises': 'error',
      '@typescript-eslint/no-misused-promises': 'error',
      '@typescript-eslint/await-thenable': 'error',
      '@typescript-eslint/require-await': 'error',
      '@typescript-eslint/no-unnecessary-condition': 'error',
      '@typescript-eslint/no-unnecessary-type-assertion': 'error',
      '@typescript-eslint/no-redundant-type-constituents': 'error',

      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_',
        },
      ],
      '@typescript-eslint/no-shadow': 'error',
      '@typescript-eslint/prefer-optional-chain': 'error',
      '@typescript-eslint/prefer-readonly': 'error',
      '@typescript-eslint/prefer-as-const': 'error',

      '@typescript-eslint/consistent-type-imports': [
        'error',
        { prefer: 'type-imports', fixStyle: 'separate-type-imports' },
      ],
      '@typescript-eslint/consistent-type-exports': ['error', { fixMixedExportsWithInlineTypeSpecifier: false }],
      '@typescript-eslint/consistent-type-definitions': 'off',
      '@typescript-eslint/array-type': ['error', { default: 'array-simple' }],
      '@typescript-eslint/consistent-indexed-object-style': ['error', 'record'],

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

      '@typescript-eslint/no-confusing-void-expression': 'error',
      '@typescript-eslint/no-meaningless-void-operator': 'error',
      '@typescript-eslint/no-mixed-enums': 'error',
      '@typescript-eslint/no-useless-empty-export': 'error',
      '@typescript-eslint/no-require-imports': 'error',
      '@typescript-eslint/prefer-enum-initializers': 'error',
      '@typescript-eslint/prefer-includes': 'error',
      '@typescript-eslint/prefer-string-starts-ends-with': 'error',
      '@typescript-eslint/switch-exhaustiveness-check': 'error',
      '@typescript-eslint/unified-signatures': 'error',

      'no-unused-vars': 'off',
      'no-shadow': 'off',
      'no-undef': 'off',
    },
  },

  {
    files: [...publicFiles, 'internal/**/*.ts'],
    rules: {
      '@typescript-eslint/explicit-function-return-type': [
        'error',
        {
          allowExpressions: true,
          allowTypedFunctionExpressions: true,
          allowHigherOrderFunctions: true,
          allowDirectConstAssertionInArrowFunctions: true,
        },
      ],
    },
  },

  {
    files: publicFiles,
    rules: {
      'no-restricted-globals': ['error', ...browserRestrictedGlobals],
    },
  },

  {
    files: nodeFiles,
    rules: {
      'no-restricted-globals': 'off',
      '@typescript-eslint/explicit-module-boundary-types': 'off',
    },
  },
];

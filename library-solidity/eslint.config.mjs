import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

/**
 * Same rule set as `sdk/js-sdk/eslint.config.js`, minus the browser-only
 * `no-restricted-globals` block (library-solidity and codegen are Node-only).
 *
 * Shared with `codegen/eslint.config.mjs`.
 *
 * @param {object} options
 * @param {string[]} options.files Files to lint.
 * @param {string[]} options.ignores Global ignores.
 * @param {string} options.tsconfigRootDir Root directory used by the TypeScript project service.
 * @param {string[]} [options.allowDefaultProject] Files outside any tsconfig, type-checked with the default project.
 * @returns {import('typescript-eslint').ConfigArray}
 */
export function createConfig({ files, ignores, tsconfigRootDir, allowDefaultProject }) {
  return [
    // Global ignores
    { ignores },

    // Base ESLint recommended config
    {
      ...eslint.configs.recommended,
      files,
    },

    // TypeScript ESLint strict type-checked configs
    ...tseslint.configs.strictTypeChecked.map((config) => ({
      ...config,
      files,
    })),

    // TypeScript ESLint stylistic type-checked configs
    ...tseslint.configs.stylisticTypeChecked.map((config) => ({
      ...config,
      files,
    })),

    // TypeScript ESLint strict rules for the target folders only
    {
      files,
      languageOptions: {
        parser: tseslint.parser,
        parserOptions: {
          projectService: allowDefaultProject === undefined ? true : { allowDefaultProject },
          tsconfigRootDir,
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
        '@typescript-eslint/prefer-for-of': 'off',

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
      },
    },
  ];
}

export default createConfig({
  // Hardhat 3 migration: only the files already ported to Hardhat 3 are linted (they are also the only files in
  // tsconfig.json). The Hardhat 2 tests (test/) are added back as they get ported.
  files: [
    'hardhat.config.ts',
    'lib-js/common.ts',
    'lib-js/fheTypeInfos.ts',
    'lib-js/operatorsPrices.ts',
    'tasks/compileAll.ts',
    'tasks/env.ts',
    'tasks/hostContracts.ts',
    'tasks/hostDeployment.ts',
    'tasks/prepareTestContracts.ts',
    'tasks/zamaConfigAddresses.ts',
    'test/coprocessorUtils.ts',
    'test/encryptedERC20/EncryptedERC20.HCU.ts',
    'test/encryptedERC20/EncryptedERC20.fixture.ts',
    'test/encryptedERC20/EncryptedERC20.ts',
    'test/fhevmjsMocked.ts',
    'test/hardhat.ts',
    'test/instance.ts',
    'test/legacyRelayerSdk/eip712.ts',
    'test/legacyRelayerSdk/types.ts',
    'test/multiSig/MultiSig.fixture.ts',
    'test/multiSig/MultiSig.ts',
    'test/networkHooks.ts',
    'test/rand/Rand.fixture.ts',
    'test/rand/Rand.ts',
    'test/signers.ts',
    'test/types.ts',
    'test/utils.ts',
  ],
  ignores: [
    // codegen is a separate workspace with its own eslint.config.mjs
    'codegen/**',
    'ci/**',
    'config/**',
    'node_modules/**',
    'artifacts/**',
    'cache/**',
    'coverage/**',
    'types/**',
    'typechain-types/**',
    'lib/**',
    'fhevmTemp/**',
    // TODO: temporarily skipped. Generated by codegen (testgen.ts); fix the templates, then lint.
    'test/fhevmOperations/**',
    '**/*.js',
    '**/*.cjs',
    '**/*.mjs',
  ],
  tsconfigRootDir: import.meta.dirname,
  // Root-level files not covered by tsconfig.json
  allowDefaultProject: ['commitlint.config.ts'],
});

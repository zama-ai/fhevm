import jsdoc from 'eslint-plugin-jsdoc';
import tseslint from 'typescript-eslint';

const targetFiles = ['src/core/actions/**/*.ts'];

/** @type {import('typescript-eslint').ConfigArray} */
export default [
  {
    ignores: ['src/_cjs/**', 'src/_esm/**', 'src/_types/**', '**/*.d.ts'],
  },

  {
    files: targetFiles,
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    plugins: { jsdoc },
    rules: {
      'jsdoc/require-jsdoc': [
        'error',
        {
          require: {
            FunctionDeclaration: true,
            ArrowFunctionExpression: false,
            FunctionExpression: false,
          },
          publicOnly: true,
        },
      ],
    },
  },
];

import eslintCleartext from '../eslint.cleartext.mjs';

export default [
  ...eslintCleartext(import.meta.dirname),
  {
    ignores: [
      // Copied from common-vendored/src by `fhevm-npm sync-vendored`; an autofix would show up as drift.
      // See common-vendored/manifest.json.
      'pkg/ts/types/ethereumLibTypes.ts',
      'pkg/ts/cleartext-config.ts',
    ],
  },
];

import eslintCleartext from '../eslint.cleartext.mjs';

export default [
  ...eslintCleartext(import.meta.dirname),
  {
    ignores: [
      // `test/ts` is the tarball-consumer fixture's project, not this one. Its files import the payload
      // by its PUBLISHED name, which only resolves once `prepare:tarball-consumer` has run — and `build`
      // deletes that fixture in its first step. `lint:tarball-consumer` is what lints them, with
      // eslint.config.with-tarball-consumer.js, at the point in `build` where the fixture exists.
      //
      // A directory, not a file list: the same rule stated once. The enumeration this replaces had
      // drifted from its twin in test/tsconfig.json and named files that no longer existed.
      'test/ts/**',
      // Copied from vendored/src by scripts/sync-vendored-ts.ts; an autofix would show up as drift.
      // See vendored/manifest.json.
      'pkg/ts/types/ethereumLibTypes.ts',
    ],
  },
];

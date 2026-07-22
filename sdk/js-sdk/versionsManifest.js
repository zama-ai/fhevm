// Single source of truth for supported WASM versions and their build profiles.
//
// This is build metadata, so it lives outside src/wasm/ and is not copied into
// the published runtime WASM trees.
//
// Read by:
//   - scripts/build/codegen-loaders.mjs (regenerates the source loaders and shared
//     API declarations)
//   - scripts/build/build-cjs-wasm.mjs / build-esm-wasm.mjs (decide which versioned
//     subdirectories to copy/transpile and which to drop from the published
//     artifact, based on BUILD_PROFILE)
//
// Profiles:
//   - 'prod' - included in the npm-published build (BUILD_PROFILE=prod)
//   - 'dev'  - included in the dev/local build (BUILD_PROFILE=dev, default)
//
// A version tagged with both 'prod' and 'dev' ships in both. A version tagged
// only with 'dev' is excluded from the prod publish (its subdirectory is not
// copied to src/_cjs/wasm or src/_esm/wasm, and the runtime loader generated
// into those output trees does not reference it, so bundlers will not include
// the dev .wasm payload).
//
// Optional per-version fields:
//   - `source` — npm install spec used by `npm run wasm:install`.
//     Defaults to `<package-name>@<version>` (`tfhe@...` or `tkms@...`).
//     Supports local packages via `file:` URLs relative to sdk/js-sdk, e.g.
//     `source: 'file:./src/wasm/tfhe/dev.local'`.
//
// `WASM_DEFAULT_VERSIONS` controls the generated DEFAULT_* loader constants.
// Each default must be present in the active profile's manifest-listed versions.
//
// To add a new version:
//   1. Add a row here with the appropriate `tags`.
//   2. Run `npm run wasm:install -- --lib <tfhe|tkms> --force` to install
//      manifest-listed WASM packages.
//   3. Run `npm run codegen:loaders` to regenerate the source loader.

export const TFHE_MANIFEST = Object.freeze([
  Object.freeze({ version: '1.5.3', tags: Object.freeze(['prod', 'dev']) }),
  Object.freeze({ version: '1.6.2', tags: Object.freeze(['prod', 'dev']) }),
  Object.freeze({
    version: '1.6.0-dev',
    tags: Object.freeze(['dev']),
    source: 'file:../../../tfhe-dev-wasm/',
  }),
]);

export const KMS_MANIFEST = Object.freeze([
  Object.freeze({ version: '0.13.10', tags: Object.freeze(['prod', 'dev']) }),
  Object.freeze({ version: '0.13.20-0', tags: Object.freeze(['prod', 'dev']) }),
]);

export const WASM_DEFAULT_VERSIONS = Object.freeze({
  prod: Object.freeze({
    tfhe: '1.6.2',
    tkms: '0.13.20-0',
  }),
  dev: Object.freeze({
    tfhe: '1.6.2',
    tkms: '0.13.20-0',
  }),
});

export const BUILD_PROFILES = Object.freeze(['prod', 'dev']);

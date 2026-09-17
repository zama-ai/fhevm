// Single source of truth for the supported WASM versions.
//
// This is build metadata, so it lives outside src/wasm/ and is not copied into
// the published runtime WASM trees.
//
// Read by:
//   - scripts/build/codegen-loaders.mjs (regenerates the source loaders and shared
//     API declarations)
//   - scripts/build/build-cjs-wasm.mjs / build-esm-wasm.mjs (decide which versioned
//     subdirectories to copy/transpile into the published artifact)
//
// Each manifest lists every version shipped by the SDK. A version not listed
// here (e.g. a local experimental folder under src/wasm/<lib>/) is excluded
// from the build: its subdirectory is not copied to src/_cjs/wasm or
// src/_esm/wasm, and the runtime loader generated into those output trees
// does not reference it.
//
// Optional per-version fields:
//   - `source` — npm install spec used by `npm run wasm:install`.
//     Defaults to `<package-name>@<version>` (`tfhe@...` or `tkms@...`).
//     Supports local packages via `file:` URLs relative to sdk/js-sdk, e.g.
//     `source: 'file:./src/wasm/tfhe/dev.local'`.
//
// `WASM_DEFAULT_VERSIONS` controls the generated DEFAULT_* loader constants.
// Each default must be present in the manifest-listed versions.
//
// To add a new version:
//   1. Add a row here.
//   2. Run `npm run wasm:install -- --lib <tfhe|tkms|all> --force` to install
//      manifest-listed WASM packages.
//   3. Run `npm run codegen:loaders` to regenerate the source loader.

export const TFHE_MANIFEST = Object.freeze([Object.freeze({ version: '1.8.1' })]);

export const KMS_MANIFEST = Object.freeze([Object.freeze({ version: '0.15.0-0' })]);

export const WASM_DEFAULT_VERSIONS = Object.freeze({
  tfhe: '1.8.1',
  tkms: '0.15.0-0',
});

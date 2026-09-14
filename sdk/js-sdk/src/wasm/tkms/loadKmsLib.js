// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
//
// Version-selective TKMS loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadKmsLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const KMS_VERSIONS = Object.freeze(['0.14.0-1']);
export const DEFAULT_TKMS_VERSION = '0.14.0-1';

const _loaders = {
  '0.14.0-1': () => import('./v0.14.0-1/kms_lib.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '0.14.0-1': () => import('./v0.14.0-1/kms_lib_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '0.14.0-1': Object.freeze({
    wasm: Object.freeze({
      filename: 'kms_lib_bg.v0.14.0-1.wasm',
      localRelativePath: './v0.14.0-1/kms_lib_bg.wasm',
      sha256: 'd4c2a1ed2567e7a3b2dfefb4c24e8a521110dd7c7648b533316647c06f6a9252',
    }),
  }),
});

////////////////////////////////////////////////////////////////////////////////

export function kmsAssetsWithVersion(version) {
  const assets = _assets[version];
  if (!assets) throw new Error(`unsupported tkms version: ${version}`);
  return assets;
}

////////////////////////////////////////////////////////////////////////////////

export async function loadKmsLib(version) {
  const loader = _loaders[version];
  if (!loader) throw new Error(`unsupported tkms version: ${version}`);
  return loader();
}

////////////////////////////////////////////////////////////////////////////////

export async function loadKmsWasmBase64(version) {
  const loader = _wasmBase64Loaders[version];
  if (!loader) throw new Error(`unsupported tkms version: ${version}`);
  return loader();
}

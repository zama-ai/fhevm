// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
// Profile:   prod
//
// Version-selective TKMS loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadKmsLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const KMS_VERSIONS = Object.freeze(['0.13.10', '0.13.20-0']);
export const DEFAULT_TKMS_VERSION = '0.13.20-0';

const _loaders = {
  '0.13.10': () => import('./v0.13.10/kms_lib.js'),
  '0.13.20-0': () => import('./v0.13.20-0/kms_lib.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '0.13.10': () => import('./v0.13.10/kms_lib_bg.wasm.base64.js'),
  '0.13.20-0': () => import('./v0.13.20-0/kms_lib_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '0.13.10': Object.freeze({
    wasm: Object.freeze({
      filename: 'kms_lib_bg.v0.13.10.wasm',
      localRelativePath: './v0.13.10/kms_lib_bg.wasm',
      sha256: '31cfb31392445ae2630f560b961edd21c72e40d58cb20c455f9fc4d4750bb9a0',
    }),
  }),
  '0.13.20-0': Object.freeze({
    wasm: Object.freeze({
      filename: 'kms_lib_bg.v0.13.20-0.wasm',
      localRelativePath: './v0.13.20-0/kms_lib_bg.wasm',
      sha256: 'be54c8f11daf048b897b41cf6e6735895490cdca77f2a01c01be0d6fbf369c81',
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

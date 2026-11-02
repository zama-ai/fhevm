// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
//
// Version-selective TKMS loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadKmsLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const KMS_VERSIONS = Object.freeze(['0.16.0-0-solana.177a2302']);
export const DEFAULT_TKMS_VERSION = '0.16.0-0-solana.177a2302';

const _loaders = {
  '0.16.0-0-solana.177a2302': () => import('./v0.16.0-0-solana.177a2302/kms_lib.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '0.16.0-0-solana.177a2302': () => import('./v0.16.0-0-solana.177a2302/kms_lib_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '0.16.0-0-solana.177a2302': Object.freeze({
    wasm: Object.freeze({
      filename: 'kms_lib_bg.v0.16.0-0-solana.177a2302.wasm',
      localRelativePath: './v0.16.0-0-solana.177a2302/kms_lib_bg.wasm',
      sha256: '06cbc418f986293d04373cf4dcfd628d0b62baec3c18fca79cae748e0932f9cf',
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

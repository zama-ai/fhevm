// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
//
// Version-selective TKMS loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadKmsLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const KMS_VERSIONS = Object.freeze(['0.16.0-0-solana.14c63e6d']);
export const DEFAULT_TKMS_VERSION = '0.16.0-0-solana.14c63e6d';

const _loaders = {
  '0.16.0-0-solana.14c63e6d': () => import('./v0.16.0-0-solana.14c63e6d/kms_lib.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '0.16.0-0-solana.14c63e6d': () => import('./v0.16.0-0-solana.14c63e6d/kms_lib_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '0.16.0-0-solana.14c63e6d': Object.freeze({
    wasm: Object.freeze({
      filename: 'kms_lib_bg.v0.16.0-0-solana.14c63e6d.wasm',
      localRelativePath: './v0.16.0-0-solana.14c63e6d/kms_lib_bg.wasm',
      sha256: '37e92b77203dc0311a7b4434c4a00326b71516e6914f9959e558860e04542c03',
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

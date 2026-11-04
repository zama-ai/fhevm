// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
//
// Version-selective TKMS loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadKmsLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const KMS_VERSIONS = Object.freeze(['0.16.0-0-solana.08f5f65f']);
export const DEFAULT_TKMS_VERSION = '0.16.0-0-solana.08f5f65f';

const _loaders = {
  '0.16.0-0-solana.08f5f65f': () => import('./v0.16.0-0-solana.08f5f65f/kms_lib.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '0.16.0-0-solana.08f5f65f': () => import('./v0.16.0-0-solana.08f5f65f/kms_lib_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '0.16.0-0-solana.08f5f65f': Object.freeze({
    wasm: Object.freeze({
      filename: 'kms_lib_bg.v0.16.0-0-solana.08f5f65f.wasm',
      localRelativePath: './v0.16.0-0-solana.08f5f65f/kms_lib_bg.wasm',
      sha256: '025c864cd699d833813640030d2826baef08be6ba3576039c93c03289dc3096e',
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

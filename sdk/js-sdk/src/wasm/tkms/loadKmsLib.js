// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
//
// Version-selective TKMS loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadKmsLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const KMS_VERSIONS = Object.freeze(['0.15.0-1-solana.650e2a4a']);
export const DEFAULT_TKMS_VERSION = '0.15.0-1-solana.650e2a4a';

const _loaders = {
  '0.15.0-1-solana.650e2a4a': () => import('./v0.15.0-1-solana.650e2a4a/kms_lib.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '0.15.0-1-solana.650e2a4a': () => import('./v0.15.0-1-solana.650e2a4a/kms_lib_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '0.15.0-1-solana.650e2a4a': Object.freeze({
    wasm: Object.freeze({
      filename: 'kms_lib_bg.v0.15.0-1-solana.650e2a4a.wasm',
      localRelativePath: './v0.15.0-1-solana.650e2a4a/kms_lib_bg.wasm',
      sha256: '772fe14b65a14e4608128efc5ec45925b2a43ef145a21937160b7774b62b8c4e',
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

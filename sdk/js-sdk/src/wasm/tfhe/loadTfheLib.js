// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
//
// Version-selective TFHE loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadTfheLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const TFHE_VERSIONS = Object.freeze(['1.8.1']);
export const DEFAULT_TFHE_VERSION = '1.8.1';

const _loaders = {
  '1.8.1': () => import('./v1.8.1/tfhe.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '1.8.1': () => import('./v1.8.1/tfhe_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '1.8.1': Object.freeze({
    wasm: Object.freeze({
      filename: 'tfhe_bg.v1.8.1.wasm',
      localRelativePath: './v1.8.1/tfhe_bg.wasm',
      sha256: 'e7811b7a5cd2d3b32dadd1e84addb55882bca0a13df1b7d5f8671bee1d9fc6b6',
    }),
    worker: Object.freeze({
      filename: 'tfhe-worker.v1.8.1.mjs',
      localRelativePath: './v1.8.1/tfhe-worker.mjs',
      sha256: 'd67a2b2c52175bce3d32f0033b3e2fbc8044717728089b90945688c4270aaa6f',
    }),
  }),
});

////////////////////////////////////////////////////////////////////////////////

export function tfheAssetsWithVersion(version) {
  const assets = _assets[version];
  if (!assets) throw new Error(`unsupported tfhe version: ${version}`);
  return assets;
}

////////////////////////////////////////////////////////////////////////////////

export async function loadTfheLib(version) {
  const loader = _loaders[version];
  if (!loader) throw new Error(`unsupported tfhe version: ${version}`);
  return loader();
}

////////////////////////////////////////////////////////////////////////////////

export async function loadTfheWasmBase64(version) {
  const loader = _wasmBase64Loaders[version];
  if (!loader) throw new Error(`unsupported tfhe version: ${version}`);
  return loader();
}

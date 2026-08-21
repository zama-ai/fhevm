// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
// Profile:   prod
//
// Version-selective TFHE loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadTfheLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const TFHE_VERSIONS = Object.freeze(['1.5.3', '1.6.2', '1.7.0']);
export const DEFAULT_TFHE_VERSION = '1.7.0';

const _loaders = {
  '1.5.3': () => import('./v1.5.3/tfhe.js'),
  '1.6.2': () => import('./v1.6.2/tfhe.js'),
  '1.7.0': () => import('./v1.7.0/tfhe.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '1.5.3': () => import('./v1.5.3/tfhe_bg.wasm.base64.js'),
  '1.6.2': () => import('./v1.6.2/tfhe_bg.wasm.base64.js'),
  '1.7.0': () => import('./v1.7.0/tfhe_bg.wasm.base64.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  '1.5.3': Object.freeze({
    wasm: Object.freeze({
      filename: 'tfhe_bg.v1.5.3.wasm',
      localRelativePath: './v1.5.3/tfhe_bg.wasm',
      sha256: 'dd349c2e34834527890a80e1b70bf5ee57a02aabb7f65e32a1bca654db9201ec',
    }),
    worker: Object.freeze({
      filename: 'tfhe-worker.v1.5.3.mjs',
      localRelativePath: './v1.5.3/tfhe-worker.mjs',
      sha256: '3f93fe86a8dfa6e25ae5fcfe28d19833219cba8f45f81c6dd05c2f3cc5323c52',
    }),
  }),
  '1.6.2': Object.freeze({
    wasm: Object.freeze({
      filename: 'tfhe_bg.v1.6.2.wasm',
      localRelativePath: './v1.6.2/tfhe_bg.wasm',
      sha256: 'ecd1841ad42226629c1a665ba784e073f2e780137f7986b00088c9227acb9760',
    }),
    worker: Object.freeze({
      filename: 'tfhe-worker.v1.6.2.mjs',
      localRelativePath: './v1.6.2/tfhe-worker.mjs',
      sha256: '2c648dd89132bb63d37e8b47c6fe1f53b06abb1389faeb9b3f671eea9a0db5dd',
    }),
  }),
  '1.7.0': Object.freeze({
    wasm: Object.freeze({
      filename: 'tfhe_bg.v1.7.0.wasm',
      localRelativePath: './v1.7.0/tfhe_bg.wasm',
      sha256: '9e6e067c5e8df560e307f8fbbc4da42df97ae02e4523b55531f3626e88c50ba6',
    }),
    worker: Object.freeze({
      filename: 'tfhe-worker.v1.7.0.mjs',
      localRelativePath: './v1.7.0/tfhe-worker.mjs',
      sha256: 'b37a7aced3bad48aea10d80ef5c3efafd35c0665d5bc8cf5f6b66708f5606c69',
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

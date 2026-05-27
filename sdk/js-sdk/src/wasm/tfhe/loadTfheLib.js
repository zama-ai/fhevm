// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
// Profile:   prod
//
// Version-selective TFHE loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadTfheLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const TFHE_VERSIONS = Object.freeze(['1.5.3', '1.6.1']);
export const DEFAULT_TFHE_VERSION = '1.6.1';

const _loaders = {
  '1.5.3': () => import('./v1.5.3/tfhe.js'),
  '1.6.1': () => import('./v1.6.1/tfhe.js'),
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  '1.5.3': () => import('./v1.5.3/tfhe_bg.wasm.base64.js'),
  '1.6.1': () => import('./v1.6.1/tfhe_bg.wasm.base64.js'),
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
  '1.6.1': Object.freeze({
    wasm: Object.freeze({
      filename: 'tfhe_bg.v1.6.1.wasm',
      localRelativePath: './v1.6.1/tfhe_bg.wasm',
      sha256: 'd470dc00347cdc83135e29700abe54c6a7ee9ba4ad58449bfc2494dfa4423f38',
    }),
    worker: Object.freeze({
      filename: 'tfhe-worker.v1.6.1.mjs',
      localRelativePath: './v1.6.1/tfhe-worker.mjs',
      sha256: '348fe6c2e77bafe0cbfb9c0a512af99b5f97de1119ceadf07cc34620dbc5690e',
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

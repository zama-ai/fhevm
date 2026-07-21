// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
// Profile:   __BUILD_PROFILE__
//
// Version-selective TFHE loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadTfheLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const TFHE_VERSIONS = Object.freeze([__TFHE_VERSIONS__]);
export const DEFAULT_TFHE_VERSION = __TFHE_DEFAULT_VERSION__;

const _loaders = {
  /* __TFHE_LIB_LOADERS__ */
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  /* __TFHE_WASM_BASE64_LOADERS__ */
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  /* __TFHE_ASSETS__ */
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

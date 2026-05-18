// AUTO-GENERATED FROM versionsManifest.js - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs
// Profile:   __BUILD_PROFILE__
//
// Version-selective TKMS loader. Authored as plain ESM .js (not .ts) so the
// published artifact is byte-identical to source. Types live in the sibling
// loadKmsLib.d.ts.

////////////////////////////////////////////////////////////////////////////////

export const KMS_VERSIONS = Object.freeze([__KMS_VERSIONS__]);
export const DEFAULT_TKMS_VERSION = __TKMS_DEFAULT_VERSION__;

const _loaders = {
  /* __KMS_LIB_LOADERS__ */
};

////////////////////////////////////////////////////////////////////////////////

const _wasmBase64Loaders = {
  /* __KMS_WASM_BASE64_LOADERS__ */
};

////////////////////////////////////////////////////////////////////////////////

const _assets = Object.freeze({
  /* __KMS_ASSETS__ */
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

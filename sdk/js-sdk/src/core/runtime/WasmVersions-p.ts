import type { TfheVersion, TkmsVersion } from '../types/moduleVersions.js';

////////////////////////////////////////////////////////////////////////////////

type WasmModuleVersionByKey = {
  readonly tfhe: TfheVersion;
  readonly kms: TkmsVersion;
};

/**
 * The single WASM version shipped per module in this SDK release.
 *
 * The SDK targets one protocol line per release, so it bundles exactly one
 * tfhe and one tkms WASM module and always loads these versions.
 */
export const CANONICAL_WASM_VERSIONS: WasmModuleVersionByKey = Object.freeze({
  tfhe: '1.6.2',
  kms: '0.14.0-1',
});

import { loadTfheLib, DEFAULT_TFHE_VERSION } from '../../../src/wasm/tfhe/loadTfheLib.js';
import { loadKmsLib, DEFAULT_TKMS_VERSION } from '../../../src/wasm/tkms/loadKmsLib.js';
import type { WasmMemoryInfo } from './memorySampler.js';

// `loadTfheLib`/`loadKmsLib` just return the already-initialized singleton glue
// module (see src/core/modules/*/module/init-p.ts — init is cached forever,
// per version, for the life of the process). Resolving it once after a
// client's `.ready` has fired gives a reference whose `getWasmInfo()` reads
// the WASM instance's *current* linear-memory size on every call — unlike the
// SDK's internal `getTfheModuleInfo()`, which freezes a snapshot at
// first-init time and would not show growth across later operations.

/**
 * Builds a synchronous, live WASM-memory reader for the tfhe module.
 * Only call after a client using this exact version has resolved `.ready` at
 * least once — this does not itself trigger initialization.
 */
export async function createTfheMemoryReader(
  version: string = DEFAULT_TFHE_VERSION,
): Promise<() => WasmMemoryInfo | undefined> {
  const tfheLib = await loadTfheLib(version as Parameters<typeof loadTfheLib>[0]);
  return () => {
    const info = tfheLib.getWasmInfo();
    return info.memory === undefined ? undefined : { byteLength: info.memory.byteLength, pages: info.memory.pages };
  };
}

/**
 * Builds a synchronous, live WASM-memory reader for the tkms module.
 *
 * NOTE: v1 targets the `localstack` chain only, whose protocol resolves the
 * (unset) `kms` module version to `DEFAULT_TKMS_VERSION` — see
 * `TFHE_VERSION_BY_CHAIN`/`FHE_ENCRYPTION_KEY_TFHE_VERSION_BY_CHAIN` in
 * `test/fheTest/setupCommon.ts`, where `localstack` has no explicit kms
 * override. If a scenario is ever pointed at a different chain, this default
 * must be revisited.
 */
export async function createTkmsMemoryReader(
  version: string = DEFAULT_TKMS_VERSION,
): Promise<() => WasmMemoryInfo | undefined> {
  const kmsLib = await loadKmsLib(version as Parameters<typeof loadKmsLib>[0]);
  return () => {
    const info = kmsLib.getWasmInfo();
    return info.memory === undefined ? undefined : { byteLength: info.memory.byteLength, pages: info.memory.pages };
  };
}

// AUTO-GENERATED FROM scripts/wasm/loaders/TfheApi.template.d.ts - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs

// Shared opaque types from tfhe. We pick v1.6.1 as the canonical source.
// The per-version `type-check.test.ts` files enforce that every supported
// version exposes the same public shape, so any of them would do.
//
// `export type` makes these declaration-only: no JS import statement is
// emitted and `noEmit`/`isolatedModules` builds produce zero runtime code
// from this file.
export type {
  CompactCiphertextList,
  CompactCiphertextListBuilder,
  CompactPkeCrs,
  ProvenCompactCiphertextList,
  TfheCompactPublicKey,
  ZkComputeLoad,
} from './v1.6.1/tfhe.js';

/** The subset you actually use - the runtime contract callers depend on. */
export interface TfheLibApi {
  initAsync: typeof import('./v1.6.1/tfhe.js').initAsync;

  // --- Free functions used by encrypt/init-p.ts -----------------------------
  init_panic_hook: typeof import('./v1.6.1/tfhe.js').init_panic_hook;
  initThreadPool: typeof import('./v1.6.1/tfhe.js').initThreadPool;
  setWorkerUrlConfig: typeof import('./v1.6.1/tfhe.js').setWorkerUrlConfig;
  getWasmInfo: typeof import('./v1.6.1/tfhe.js').getWasmInfo;

  // --- Class constructors used as values in encrypt/api-p.ts ----------------
  // These are imported as runtime values: instanceof checks, static methods
  // (safe_deserialize), factory calls (CompactCiphertextList.builder()).
  CompactCiphertextList: typeof import('./v1.6.1/tfhe.js').CompactCiphertextList;
  CompactPkeCrs: typeof import('./v1.6.1/tfhe.js').CompactPkeCrs;
  ProvenCompactCiphertextList: typeof import('./v1.6.1/tfhe.js').ProvenCompactCiphertextList;
  TfheCompactPublicKey: typeof import('./v1.6.1/tfhe.js').TfheCompactPublicKey;

  // --- Enum used as a value (e.g. ZkComputeLoad.Verify) ---------------------
  ZkComputeLoad: typeof import('./v1.6.1/tfhe.js').ZkComputeLoad;
}

// Default version
export type TfheVersion = '1.5.3' | '1.6.1' | '1.6.2';

export type TfheWasmBase64 = {
  readonly tfheWasmBase64: string;
  readonly tfheWasmBase64IsGzipped: boolean;
  readonly tfheWasmBase64CompressionFormat: 'gzip' | 'deflate' | 'deflate-raw' | undefined;
};

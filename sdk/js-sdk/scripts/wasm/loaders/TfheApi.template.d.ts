// AUTO-GENERATED FROM scripts/wasm/loaders/TfheApi.template.d.ts - DO NOT EDIT.
// Generator: scripts/build/codegen-loaders.mjs

// Shared opaque types from tfhe. We pick v1.7.0 as the canonical source: it's
// the narrowest shape among supported versions (1.7.0 dropped the legacy
// serialize()/deserialize() methods in favor of safe_serialize()/
// safe_deserialize(), which is all the SDK ever calls). Older supported
// versions are supersets, so they still satisfy this contract.
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
} from '../../../src/wasm/tfhe/v1.7.0/tfhe.js';

/** The subset you actually use - the runtime contract callers depend on. */
export interface TfheLibApi {
  initAsync: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').initAsync;

  // --- Free functions used by encrypt/init-p.ts -----------------------------
  init_panic_hook: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').init_panic_hook;
  initThreadPool: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').initThreadPool;
  setWorkerUrlConfig: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').setWorkerUrlConfig;
  getWasmInfo: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').getWasmInfo;

  // --- Class constructors used as values in encrypt/api-p.ts ----------------
  // These are imported as runtime values: instanceof checks, static methods
  // (safe_deserialize), factory calls (CompactCiphertextList.builder()).
  CompactCiphertextList: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').CompactCiphertextList;
  CompactPkeCrs: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').CompactPkeCrs;
  ProvenCompactCiphertextList: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').ProvenCompactCiphertextList;
  TfheCompactPublicKey: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').TfheCompactPublicKey;

  // --- Enum used as a value (e.g. ZkComputeLoad.Verify) ---------------------
  ZkComputeLoad: typeof import('../../../src/wasm/tfhe/v1.7.0/tfhe.js').ZkComputeLoad;
}

// Default version
export type TfheVersion = '1.7.0';

export type TfheWasmBase64 = {
  readonly tfheWasmBase64: string;
  readonly tfheWasmBase64IsGzipped: boolean;
  readonly tfheWasmBase64CompressionFormat: 'gzip' | 'deflate' | 'deflate-raw' | undefined;
};

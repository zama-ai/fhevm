// sdk/js-sdk/src/wasm/tfhe/v1.6.2/type-check.test.ts
//
// Verifies that this version's generated tfhe module conforms to the canonical
// API contract declared in ../TfheApi.d.ts. The check is one-way assignability:
// a version must provide AT LEAST the members the contract requires. Newer
// versions are allowed to add members without breaking the check.
//
// This file intentionally ends with `.test.ts`:
// - it is included by the repository test TypeScript project,
// - it is excluded from the published package.

import type * as Mod from './tfhe.js';
import type { TfheLibApi } from '../TfheApi.js';
import type { WasmAssetLoadMode as CoreWasmAssetLoadMode } from '../../../core/types/coreFhevmRuntime.js';
import type { WasmAssetLoadMode as TfheWasmAssetLoadMode } from './tfhe.js';

type Assert<T extends true> = T;
type IsAssignable<A, B> = [A] extends [B] ? true : false;
type IsEqual<A, B> = IsAssignable<A, B> extends true ? (IsAssignable<B, A> extends true ? true : false) : false;
type InstanceWithoutEq<C extends { prototype: object }> = Omit<C['prototype'], 'eq'>;

// --- Class constructors / instances ---
// Note: these use one-way assignability, not strict equality. A version may
// add methods without breaking the contract, as long as every member the
// contract requires is present. `eq` is ignored because wasm-bindgen may expose
// it on some versions only, and the SDK does not use it.
type _CompactCiphertextList = Assert<
  IsAssignable<
    InstanceWithoutEq<typeof Mod.CompactCiphertextList>,
    InstanceWithoutEq<TfheLibApi['CompactCiphertextList']>
  >
>;
type _CompactPkeCrs = Assert<IsAssignable<typeof Mod.CompactPkeCrs, TfheLibApi['CompactPkeCrs']>>;
type _ProvenCompactCiphertextList = Assert<
  IsAssignable<
    InstanceWithoutEq<typeof Mod.ProvenCompactCiphertextList>,
    InstanceWithoutEq<TfheLibApi['ProvenCompactCiphertextList']>
  >
>;
type _TfheCompactPublicKey = Assert<IsAssignable<typeof Mod.TfheCompactPublicKey, TfheLibApi['TfheCompactPublicKey']>>;

// --- Enum value (ZkComputeLoad.Verify is read as a value) ---
type _ZkComputeLoad = Assert<IsAssignable<typeof Mod.ZkComputeLoad, TfheLibApi['ZkComputeLoad']>>;

// --- Free functions matching the TfheLibApi runtime contract ---
// Note: `default` (wasm-bindgen __wbg_init) is intentionally excluded — its
// InitInput / InitOutput shapes are version-specific by design.
type _init_panic_hook = Assert<IsAssignable<typeof Mod.init_panic_hook, TfheLibApi['init_panic_hook']>>;
type _initThreadPool = Assert<IsAssignable<typeof Mod.initThreadPool, TfheLibApi['initThreadPool']>>;
type _setWorkerUrlConfig = Assert<IsAssignable<typeof Mod.setWorkerUrlConfig, TfheLibApi['setWorkerUrlConfig']>>;
type _getWasmInfo = Assert<IsAssignable<typeof Mod.getWasmInfo, TfheLibApi['getWasmInfo']>>;

// --- Cross-check: tfhe's WasmAssetLoadMode equals the core SDK's ---
// Pre-existing invariant unrelated to TfheLibApi.
type _WasmAssetLoadModeMatches = Assert<IsEqual<TfheWasmAssetLoadMode, CoreWasmAssetLoadMode>>;

// Bundle everything so noUnusedLocals doesn't flag them.
export type _NeverUse = _CompactCiphertextList &
  _CompactPkeCrs &
  _ProvenCompactCiphertextList &
  _TfheCompactPublicKey &
  _ZkComputeLoad &
  _init_panic_hook &
  _initThreadPool &
  _setWorkerUrlConfig &
  _getWasmInfo &
  _WasmAssetLoadModeMatches &
  never;

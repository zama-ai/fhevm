import type { WasmAssetLoadMode as CoreWasmAssetLoadMode } from '../../src/core/types/coreFhevmRuntime.js';
import type { WasmAssetLoadMode as TfheWasmAssetLoadMode } from '../../src/wasm/tfhe/v1.5.3/tfhe.js';

type AssertAssignable<_A extends B, B> = true;

type _CoreCoversTfhe = AssertAssignable<CoreWasmAssetLoadMode, TfheWasmAssetLoadMode>;
type _TfheCoversCore = AssertAssignable<TfheWasmAssetLoadMode, CoreWasmAssetLoadMode>;

export type NeverUse = _CoreCoversTfhe & _TfheCoversCore & never;

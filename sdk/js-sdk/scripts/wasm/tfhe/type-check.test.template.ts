/*
 * Type-only checks for generated TFHE SDK declarations.
 *
 * This file intentionally ends with `.test.ts`:
 * - it is included by the repository test TypeScript project,
 * - it is excluded from the published package by `src/package.json`.
 */
// Template-local imports: `build-tfhe.ts` replaces these with imports relative to the generated file.
// Generated import placeholder: __CORE_TYPES_FHEVM_RUNTIME_IMPORT__
import type { WasmAssetLoadMode as CoreWasmAssetLoadMode } from '../../../src/core/types/coreFhevmRuntime.js';
// Generated import placeholder: __TFHE_JS_IMPORT__ for `WasmAssetLoadMode` from generated `tfhe.js`.
import type { WasmAssetLoadMode as TfheWasmAssetLoadMode } from '../../../src/core/types/coreFhevmRuntime.js';

type Assert<T extends true> = T;
type IsAssignable<A, B> = [A] extends [B] ? true : false;
type IsEqual<A, B> = IsAssignable<A, B> extends true ? (IsAssignable<B, A> extends true ? true : false) : false;

export type _WasmAssetLoadModeMatches = Assert<IsEqual<CoreWasmAssetLoadMode, TfheWasmAssetLoadMode>>;

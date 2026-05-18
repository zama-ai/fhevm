/*
 * Type-only checks for generated TFHE SDK declarations.
 *
 * This file intentionally ends with `.test.ts`:
 * - it is included by the repository test TypeScript project,
 * - it is excluded from the published package by `src/package.json`.
 */

import type { WasmAssetLoadMode as CoreWasmAssetLoadMode } from '../../../core/types/coreFhevmRuntime.js';
import type { WasmAssetLoadMode as TfheWasmAssetLoadMode } from './tfhe.js';

type Assert<T extends true> = T;
type IsAssignable<A, B> = [A] extends [B] ? true : false;
type IsEqual<A, B> = IsAssignable<A, B> extends true ? (IsAssignable<B, A> extends true ? true : false) : false;

export type _WasmAssetLoadModeMatches = Assert<IsEqual<CoreWasmAssetLoadMode, TfheWasmAssetLoadMode>>;

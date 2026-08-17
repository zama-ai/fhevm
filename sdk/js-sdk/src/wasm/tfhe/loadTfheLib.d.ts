import type { TfheLibApi, TfheVersion, TfheWasmBase64 } from './TfheApi.js';

export type { TfheVersion } from './TfheApi.js';

export type TfheAssetMetadata = {
  readonly filename: string;
  readonly localRelativePath: string;
  readonly sha256: string;
};

export type TfheAssets = {
  readonly wasm: TfheAssetMetadata;
  readonly worker: TfheAssetMetadata;
};

export declare const DEFAULT_TFHE_VERSION: TfheVersion;
export declare function tfheAssetsWithVersion(version: TfheVersion): TfheAssets;
export declare function loadTfheLib(version: TfheVersion): Promise<TfheLibApi>;
export declare function loadTfheWasmBase64(version: TfheVersion): Promise<TfheWasmBase64>;

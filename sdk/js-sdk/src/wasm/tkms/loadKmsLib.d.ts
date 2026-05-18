import type { KmsLibApi, TkmsVersion, TkmsWasmBase64 } from './KmsLibApi.js';

export type { TkmsVersion } from './KmsLibApi.js';

export type KmsAssetMetadata = {
  readonly filename: string;
  readonly localRelativePath: string;
  readonly sha256: string;
};

export type KmsAssets = {
  readonly wasm: KmsAssetMetadata;
};

export declare const DEFAULT_TKMS_VERSION: TkmsVersion;
export declare function kmsAssetsWithVersion(version: TkmsVersion): KmsAssets;
export declare function loadKmsLib(version: TkmsVersion): Promise<KmsLibApi>;
export declare function loadKmsWasmBase64(version: TkmsVersion): Promise<TkmsWasmBase64>;

import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';

export declare const TkmsPrivateKeyBrand: unique symbol;

export type TkmsPrivateKey = {
  readonly [TkmsPrivateKeyBrand]: never;
  readonly tkmsVersion: TkmsVersion;
  free(): void;
};

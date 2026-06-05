import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';

export declare const TkmsPrivateKeyBrand: unique symbol;

export type TkmsPrivateKey = {
  readonly tkmsVersion: TkmsVersion;
  readonly [TkmsPrivateKeyBrand]: never;
};

import type { FheType } from './fheType.js';
import type { Bytes32Hex } from './primitives.js';

////////////////////////////////////////////////////////////////////////////////
// Public types
////////////////////////////////////////////////////////////////////////////////

declare const etype: unique symbol;
declare const eValueBrand: unique symbol;

export type EValueBrand = { readonly [eValueBrand]: never };

type EValue<T extends FheType> = Bytes32Hex &
  EValueBrand & {
    readonly [etype]: T;
  };

export type ETypedValue<T extends FheType = FheType> = {
  [K in T]: EValue<K>;
}[T];

export type Ebool = ETypedValue<'ebool'>;
export type Euint8 = ETypedValue<'euint8'>;
export type Euint16 = ETypedValue<'euint16'>;
export type Euint32 = ETypedValue<'euint32'>;
export type Euint64 = ETypedValue<'euint64'>;
export type Euint128 = ETypedValue<'euint128'>;
export type Euint256 = ETypedValue<'euint256'>;
export type Eaddress = ETypedValue<'eaddress'>;

export type EncryptedValue = ETypedValue;

/**
 * Any value that can be interpreted as an encrypted value (bytes32 handle).
 *
 * - `Uint8Array` — raw 32-byte handle (`Bytes32`)
 * - `string` — 0x-prefixed hex-encoded 32-byte handle (`Bytes32Hex`, e.g. `"0xabcd..."`)
 * - `{ bytes32Hex: string }` — object with a hex-encoded handle property
 * - `EncryptedValue` — an already-parsed encrypted value
 */
export type EncryptedValueLike = Uint8Array | string | { readonly bytes32Hex: string };

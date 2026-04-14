import type { FheType, FheTypeToValueTypeNameMap } from './fheType.js';
import type { TypedValueOfBase } from './primitives.js';
import type { ComputedEncryptedValueOfTypeBase, ExternalEncryptedValueOfTypeBase } from './encryptedTypes-p.js';

////////////////////////////////////////////////////////////////////////////////
// Public types
////////////////////////////////////////////////////////////////////////////////

/**
 * An encrypted FHE value (`handle` in `FHE.sol` / FHEVM whitepaper).
 * Either a {@link ComputedEncryptedValue} (verified, on-chain) or an
 * {@link ExternalEncryptedValue} (unverified input). Narrowable via `isExternal`.
 */
export type EncryptedValue<etype extends FheType = FheType> =
  | ComputedEncryptedValue<etype>
  | ExternalEncryptedValue<etype>;

/** A computed encrypted value — verified on-chain, result of an FHE operation. */
export type ComputedEncryptedValue<etype extends FheType = FheType> = {
  [K in etype]: ComputedEncryptedValueOfTypeBase<K>;
}[etype];

/** An unverified encrypted value (`inputHandle` in `FHE.sol`). */
export type ExternalEncryptedValue<etype extends FheType = FheType> = {
  [K in etype]: ExternalEncryptedValueOfTypeBase<K>;
}[etype];

/**
 * Alias for {@link EncryptedValue} using `FHE.sol` terminology.
 * In `FHE.sol`, a `handle` is the `bytes32` reference to any encrypted value.
 */
export type Handle<etype extends FheType = FheType> = EncryptedValue<etype>;

/**
 * Alias for {@link ExternalEncryptedValue} using `FHE.sol` terminology.
 * In `FHE.sol`, an `inputHandle` is an encrypted value that has not yet been
 * verified on-chain via `InputVerifier.sol`.
 */
export type InputHandle<etype extends FheType = FheType> = ExternalEncryptedValue<etype>;

/** Alias for {@link ComputedEncryptedValue} using `FHE.sol` terminology. */
export type ComputedHandle<etype extends FheType = FheType> = ComputedEncryptedValue<etype>;

////////////////////////////////////////////////////////////////////////////////
// Typed shortcuts
////////////////////////////////////////////////////////////////////////////////

/** Encrypted boolean (`ebool` in Solidity). */
export type Ebool = EncryptedValue<'ebool'>;
/** Encrypted unsigned 8-bit integer (`euint8` in Solidity). */
export type Euint8 = EncryptedValue<'euint8'>;
/** Encrypted unsigned 16-bit integer (`euint16` in Solidity). */
export type Euint16 = EncryptedValue<'euint16'>;
/** Encrypted unsigned 32-bit integer (`euint32` in Solidity). */
export type Euint32 = EncryptedValue<'euint32'>;
/** Encrypted unsigned 64-bit integer (`euint64` in Solidity). */
export type Euint64 = EncryptedValue<'euint64'>;
/** Encrypted unsigned 128-bit integer (`euint128` in Solidity). */
export type Euint128 = EncryptedValue<'euint128'>;
/** Encrypted unsigned 256-bit integer (`euint256` in Solidity). */
export type Euint256 = EncryptedValue<'euint256'>;
/** Encrypted address (`eaddress` in Solidity). */
export type Eaddress = EncryptedValue<'eaddress'>;

/** Unverified encrypted boolean (`externalEbool` in Solidity). Requires on-chain verification before use. */
export type ExternalEbool = ExternalEncryptedValue<'ebool'>;
/** Unverified encrypted unsigned 8-bit integer (`externalEuint8` in Solidity). */
export type ExternalEuint8 = ExternalEncryptedValue<'euint8'>;
/** Unverified encrypted unsigned 16-bit integer (`externalEuint16` in Solidity). */
export type ExternalEuint16 = ExternalEncryptedValue<'euint16'>;
/** Unverified encrypted unsigned 32-bit integer (`externalEuint32` in Solidity). */
export type ExternalEuint32 = ExternalEncryptedValue<'euint32'>;
/** Unverified encrypted unsigned 64-bit integer (`externalEuint64` in Solidity). */
export type ExternalEuint64 = ExternalEncryptedValue<'euint64'>;
/** Unverified encrypted unsigned 128-bit integer (`externalEuint128` in Solidity). */
export type ExternalEuint128 = ExternalEncryptedValue<'euint128'>;
/** Unverified encrypted unsigned 256-bit integer (`externalEuint256` in Solidity). */
export type ExternalEuint256 = ExternalEncryptedValue<'euint256'>;
/** Unverified encrypted address (`externalEaddress` in Solidity). */
export type ExternalEaddress = ExternalEncryptedValue<'eaddress'>;

////////////////////////////////////////////////////////////////////////////////

/**
 * Any value that can be interpreted as an encrypted value (bytes32 handle).
 *
 * - `Uint8Array` — raw 32-byte handle (`Bytes32`)
 * - `string` — 0x-prefixed hex-encoded 32-byte handle (`Bytes32Hex`, e.g. `"0xabcd..."`)
 * - `{ bytes32Hex: string }` — object with a hex-encoded handle property
 * - `EncryptedValue` — an already-parsed encrypted value
 */
export type EncryptedValueLike = Uint8Array | string | { readonly bytes32Hex: string } | EncryptedValue;

export type HandleLike = EncryptedValueLike;

/**
 * Any value that can be interpreted as an external encrypted value (bytes32 input handle).
 * An input handle is a user-encrypted value that has not yet been verified on-chain via `InputVerifier.sol`.
 *
 * - `Uint8Array` — raw 32-byte handle (`Bytes32`)
 * - `string` — 0x-prefixed hex-encoded 32-byte handle (`Bytes32Hex`, e.g. `"0xabcd..."`)
 * - `{ bytes32Hex: string }` — object with a hex-encoded handle property
 * - `ExternalEncryptedValue` — an already-parsed external encrypted value
 */
export type ExternalEncryptedValueLike = Uint8Array | string | { readonly bytes32Hex: string } | ExternalEncryptedValue;

export type InputHandleLike = ExternalEncryptedValueLike;

////////////////////////////////////////////////////////////////////////////////

export type ClearValueOfFheType<etype extends FheType> = TypedValueOfBase<ClearValueTypeName<etype>> & {
  readonly encryptedValue: EncryptedValue<etype>;
};

/**
 * The decrypted clear value of an FHE encrypted value.
 * @typeParam T - The FHE type (e.g. `"euint8"`, `"ebool"`). Defaults to all types.
 */
export type ClearValue<etype extends FheType = FheType> = {
  [K in etype]: ClearValueOfFheType<K>;
}[etype];

export type ClearValueTypeName<etype extends FheType = FheType> = FheTypeToValueTypeNameMap[etype];

export type ClearBool = ClearValue<'ebool'>;
export type ClearUint8 = ClearValue<'euint8'>;
export type ClearUint16 = ClearValue<'euint16'>;
export type ClearUint32 = ClearValue<'euint32'>;
export type ClearUint64 = ClearValue<'euint64'>;
export type ClearUint128 = ClearValue<'euint128'>;
export type ClearUint256 = ClearValue<'euint256'>;
export type ClearAddress = ClearValue<'eaddress'>;

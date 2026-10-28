import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type {
  Bytes21Hex,
  Bytes32,
  Bytes32Hex,
  BytesHex,
  Uint64BigInt,
  Uint8Number,
  UintNumber,
  ValueTypeName,
} from '../types/primitives.js';
import type { EncryptionBits, FheType, FheTypeId, SolidityPrimitiveTypeName } from '../types/fheType.js';
import type { EncryptedValueLike } from '../types/encryptedTypes.js';
import type {
  Handle,
  HandleBaseV0,
  HandleBytes32,
  HandleBytes32Hex,
  HandleBytes32HexNo0x,
  ExternalHandle,
  ExternalHandleBaseV0,
  ExternalHandleBytes32,
  ExternalHandleBytes32Hex,
  ExternalHandleBytes32HexNo0x,
  InputHandle,
  InputHandleBaseV0,
  InputHandleBytes32,
  InputHandleBytes32Hex,
  InputHandleBytes32HexNo0x,
} from '../types/encryptedTypes-p.js';
import { FhevmHandleError } from '../errors/FhevmHandleError.js';
import {
  assertIsBytes32,
  assertIsBytes32Hex,
  bytes32ToHex,
  bytesHexSlice,
  bytesHexUint64At,
  bytesHexUint8At,
  bytesUint8At,
  hexToBytes,
  hexToBytes32,
  isBytes32,
  isBytes32Hex,
} from '../base/bytes.js';
import {
  encryptionBitsFromFheTypeId,
  fheTypeNameFromId,
  isFheTypeId,
  solidityPrimitiveTypeNameFromFheTypeId,
  typeNameFromFheTypeName,
} from './FheType.js';
import { remove0x } from '../base/string.js';
import { asUint8Number, uint64ToBytes32 } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('FhevmHandle.token');

////////////////////////////////////////////////////////////////////////////////

export const FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION = 0;

////////////////////////////////////////////////////////////////////////////////

// Computed handle: hash21 (bytes 0..20) | 0xFF (byte 21) | chainId (bytes 22..29) | fheTypeId (30) | version (31).
const FHEVM_HANDLE_HASH21_BYTE_OFFSET = 0 as Uint8Number;
const FHEVM_HANDLE_CHAINID_BYTE_OFFSET = 22 as Uint8Number;
const FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET = 30 as Uint8Number;
const FHEVM_HANDLE_VERSION_BYTE_OFFSET = 31 as Uint8Number;

// RFC-030 external handle: packageDigest with bytes [29..31] rewritten as
// (inputIndex, fheTypeId, version). Bytes 30 and 31 share the computed layout.
const FHEVM_EXTERNAL_HANDLE_HASH29_BYTE_OFFSET = 0 as Uint8Number;
const FHEVM_EXTERNAL_HANDLE_INDEX_BYTE_OFFSET = 29 as Uint8Number;

// ZKPoK input handle: the computed layout with the input index at byte 21
// instead of the 0xFF marker.
const FHEVM_INPUT_HANDLE_INDEX_BYTE_OFFSET = 21 as Uint8Number;
const FHEVM_HANDLE_COMPUTED_MARKER = 0xff;

////////////////////////////////////////////////////////////////////////////////
// Handle (EncryptedValue) implementation
////////////////////////////////////////////////////////////////////////////////

class FhevmHandleImpl implements HandleBaseV0 {
  //////////////////////////////////////////////////////////////////////////////
  // Instance Properties
  //////////////////////////////////////////////////////////////////////////////

  readonly #handleBytes32Hex: HandleBytes32Hex;
  #handleBytes32: HandleBytes32 | undefined;

  constructor(
    privateToken: symbol,
    parameters: {
      handleBytes32Hex: HandleBytes32Hex;
      handleBytes32?: HandleBytes32;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#handleBytes32Hex = parameters.handleBytes32Hex;
    this.#handleBytes32 = parameters.handleBytes32; // takes ownership, no copy
  }

  //////////////////////////////////////////////////////////////////////////////
  // Instance Getters
  //////////////////////////////////////////////////////////////////////////////

  public get bytes32Hex(): HandleBytes32Hex {
    return this.#handleBytes32Hex;
  }

  public get bytes32HexNo0x(): HandleBytes32HexNo0x {
    return remove0x(this.#handleBytes32Hex) as HandleBytes32HexNo0x;
  }

  public get bytes32(): HandleBytes32 {
    if (this.#handleBytes32 === undefined) {
      this.#handleBytes32 = hexToBytes32(this.#handleBytes32Hex) as HandleBytes32;
    }
    return new Uint8Array(this.#handleBytes32) as HandleBytes32;
  }

  public get hash21(): Bytes21Hex {
    // Extract hash21 (bytes 0-20)
    return bytesHexSlice(this.#handleBytes32Hex, FHEVM_HANDLE_HASH21_BYTE_OFFSET, 21);
  }

  public get chainId(): Uint64BigInt {
    // Extract chainId (bytes 22-29, 8 bytes as big-endian uint64)
    return bytesHexUint64At(this.#handleBytes32Hex, FHEVM_HANDLE_CHAINID_BYTE_OFFSET);
  }

  public get fheTypeId(): FheTypeId {
    // Extract fheTypeId (byte 30)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET) as FheTypeId;
  }

  public get fheType(): FheType {
    return fheTypeNameFromId(this.fheTypeId);
  }

  public get clearType(): ValueTypeName {
    return typeNameFromFheTypeName(this.fheType);
  }

  public get version(): Uint8Number {
    // Extract version (byte 31)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_HANDLE_VERSION_BYTE_OFFSET);
  }

  public get encryptionBits(): EncryptionBits {
    return encryptionBitsFromFheTypeId(this.fheTypeId);
  }

  public get solidityPrimitiveTypeName(): SolidityPrimitiveTypeName {
    return solidityPrimitiveTypeNameFromFheTypeId(this.fheTypeId);
  }

  public toString(): string {
    return this.#handleBytes32Hex;
  }

  public toJSON(): string {
    return this.#handleBytes32Hex;
  }
}

Object.freeze(FhevmHandleImpl);
Object.freeze(FhevmHandleImpl.prototype);

////////////////////////////////////////////////////////////////////////////////
// External handle (RFC-030) implementation
////////////////////////////////////////////////////////////////////////////////

class FhevmExternalHandleImpl implements ExternalHandleBaseV0 {
  //////////////////////////////////////////////////////////////////////////////
  // Instance Properties
  //////////////////////////////////////////////////////////////////////////////

  readonly #handleBytes32Hex: ExternalHandleBytes32Hex;
  #handleBytes32: ExternalHandleBytes32 | undefined;

  constructor(
    privateToken: symbol,
    parameters: {
      handleBytes32Hex: ExternalHandleBytes32Hex;
      handleBytes32?: ExternalHandleBytes32;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#handleBytes32Hex = parameters.handleBytes32Hex;
    this.#handleBytes32 = parameters.handleBytes32; // takes ownership, no copy
  }

  //////////////////////////////////////////////////////////////////////////////
  // Instance Getters
  //////////////////////////////////////////////////////////////////////////////

  public get bytes32Hex(): ExternalHandleBytes32Hex {
    return this.#handleBytes32Hex;
  }

  public get bytes32HexNo0x(): ExternalHandleBytes32HexNo0x {
    return remove0x(this.#handleBytes32Hex);
  }

  public get bytes32(): ExternalHandleBytes32 {
    if (this.#handleBytes32 === undefined) {
      this.#handleBytes32 = hexToBytes32(this.#handleBytes32Hex) as ExternalHandleBytes32;
    }
    return new Uint8Array(this.#handleBytes32) as ExternalHandleBytes32;
  }

  public get hash29(): BytesHex {
    // Extract hash29 (bytes 0-28): the first 29 bytes of the package digest
    return bytesHexSlice(this.#handleBytes32Hex, FHEVM_EXTERNAL_HANDLE_HASH29_BYTE_OFFSET, 29 as UintNumber);
  }

  public get index(): Uint8Number {
    // Extract the input index (byte 29)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_EXTERNAL_HANDLE_INDEX_BYTE_OFFSET);
  }

  public get fheTypeId(): FheTypeId {
    // Extract fheTypeId (byte 30)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET) as FheTypeId;
  }

  public get fheType(): FheType {
    return fheTypeNameFromId(this.fheTypeId);
  }

  public get clearType(): ValueTypeName {
    return typeNameFromFheTypeName(this.fheType);
  }

  public get version(): Uint8Number {
    // Extract version (byte 31)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_HANDLE_VERSION_BYTE_OFFSET);
  }

  public get encryptionBits(): EncryptionBits {
    return encryptionBitsFromFheTypeId(this.fheTypeId);
  }

  public get solidityPrimitiveTypeName(): SolidityPrimitiveTypeName {
    return solidityPrimitiveTypeNameFromFheTypeId(this.fheTypeId);
  }

  public toString(): string {
    return this.#handleBytes32Hex;
  }

  public toJSON(): string {
    return this.#handleBytes32Hex;
  }
}

Object.freeze(FhevmExternalHandleImpl);
Object.freeze(FhevmExternalHandleImpl.prototype);

////////////////////////////////////////////////////////////////////////////////
// Input handle (ZKPoK) implementation
////////////////////////////////////////////////////////////////////////////////

class FhevmInputHandleImpl implements InputHandleBaseV0 {
  //////////////////////////////////////////////////////////////////////////////
  // Instance Properties
  //////////////////////////////////////////////////////////////////////////////

  readonly #handleBytes32Hex: InputHandleBytes32Hex;
  #handleBytes32: InputHandleBytes32 | undefined;

  constructor(
    privateToken: symbol,
    parameters: {
      handleBytes32Hex: InputHandleBytes32Hex;
      handleBytes32?: InputHandleBytes32;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#handleBytes32Hex = parameters.handleBytes32Hex;
    this.#handleBytes32 = parameters.handleBytes32; // takes ownership, no copy
  }

  //////////////////////////////////////////////////////////////////////////////
  // Instance Getters
  //////////////////////////////////////////////////////////////////////////////

  public get bytes32Hex(): InputHandleBytes32Hex {
    return this.#handleBytes32Hex;
  }

  public get bytes32HexNo0x(): InputHandleBytes32HexNo0x {
    return remove0x(this.#handleBytes32Hex);
  }

  public get bytes32(): InputHandleBytes32 {
    if (this.#handleBytes32 === undefined) {
      this.#handleBytes32 = hexToBytes32(this.#handleBytes32Hex) as InputHandleBytes32;
    }
    return new Uint8Array(this.#handleBytes32) as InputHandleBytes32;
  }

  public get hash21(): Bytes21Hex {
    // Extract hash21 (bytes 0-20)
    return bytesHexSlice(this.#handleBytes32Hex, FHEVM_HANDLE_HASH21_BYTE_OFFSET, 21);
  }

  public get index(): Uint8Number {
    // Extract the input index (byte 21)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_INPUT_HANDLE_INDEX_BYTE_OFFSET);
  }

  public get chainId(): Uint64BigInt {
    // Extract chainId (bytes 22-29, 8 bytes as big-endian uint64)
    return bytesHexUint64At(this.#handleBytes32Hex, FHEVM_HANDLE_CHAINID_BYTE_OFFSET);
  }

  public get fheTypeId(): FheTypeId {
    // Extract fheTypeId (byte 30)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET) as FheTypeId;
  }

  public get fheType(): FheType {
    return fheTypeNameFromId(this.fheTypeId);
  }

  public get clearType(): ValueTypeName {
    return typeNameFromFheTypeName(this.fheType);
  }

  public get version(): Uint8Number {
    // Extract version (byte 31)
    return bytesHexUint8At(this.#handleBytes32Hex, FHEVM_HANDLE_VERSION_BYTE_OFFSET);
  }

  public get encryptionBits(): EncryptionBits {
    return encryptionBitsFromFheTypeId(this.fheTypeId);
  }

  public get solidityPrimitiveTypeName(): SolidityPrimitiveTypeName {
    return solidityPrimitiveTypeNameFromFheTypeId(this.fheTypeId);
  }

  public toString(): string {
    return this.#handleBytes32Hex;
  }

  public toJSON(): string {
    return this.#handleBytes32Hex;
  }
}

Object.freeze(FhevmInputHandleImpl);
Object.freeze(FhevmInputHandleImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

export function assertIsHandleBytes32Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is HandleBytes32Hex {
  if (!isBytes32Hex(value)) {
    throw new FhevmHandleError({
      ...options,
      message: `FHEVM Handle is not a valid bytes32 hex.`,
    });
  }
  _assertIsHandleBytes32Hex(value, options);
}

/**
 * Asserts that `value` is a valid external handle bytes32 hex: a bytes32 hex
 * with a known fheTypeId (byte 30) and version (byte 31). External and computed
 * handles cannot be told apart by their bytes alone.
 */
export function assertIsExternalHandleBytes32Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is ExternalHandleBytes32Hex {
  assertIsHandleBytes32Hex(value, options);
}

export function assertIsHandleBytes32(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is HandleBytes32 {
  if (!isBytes32(value)) {
    throw new FhevmHandleError({
      ...options,
      message: `FHEVM Handle is not a valid bytes32.`,
    });
  }

  _assertIsHandleBytes32(value, options);
}

/**
 * Asserts that `value` is a valid external handle bytes32: 32 bytes with a
 * known fheTypeId (byte 30) and version (byte 31). External and computed
 * handles cannot be told apart by their bytes alone.
 */
export function assertIsExternalHandleBytes32(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is ExternalHandleBytes32 {
  assertIsHandleBytes32(value, options);
}

export function assertIsHandle(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is Handle {
  if (!isHandle(value)) {
    throw new FhevmHandleError({
      ...options,
      message: `Value is not a valid Handle.`,
    });
  }
}

export function assertIsExternalHandle(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is ExternalHandle {
  if (!isExternalHandle(value)) {
    throw new FhevmHandleError({
      ...options,
      message: `Value is not a valid ExternalHandle.`,
    });
  }
}

export function assertIsInputHandle(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is InputHandle {
  if (!isInputHandle(value)) {
    throw new FhevmHandleError({
      ...options,
      message: `Value is not a valid InputHandle.`,
    });
  }
}

export function assertIsEncryptedValueLike(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is EncryptedValueLike {
  if (value === null || value === undefined) {
    throw new FhevmHandleError({
      ...options,
      message: `FHEVM Handle is null or undefined.`,
    });
  }

  if (value instanceof FhevmHandleImpl) {
    return;
  }

  if (typeof value === 'object' && 'bytes32Hex' in value) {
    assertIsHandleBytes32Hex(value.bytes32Hex, options);
    return;
  }

  if (typeof value === 'string') {
    assertIsHandleBytes32Hex(value, options);
    return;
  }

  assertIsHandleBytes32(value, options);
}

////////////////////////////////////////////////////////////////////////////////

export function isHandleBytes32(value: unknown): value is HandleBytes32 {
  try {
    assertIsHandleBytes32(value);
    return true;
  } catch {
    return false;
  }
}

export function isHandleBytes32Hex(value: unknown): value is HandleBytes32Hex {
  try {
    assertIsHandleBytes32Hex(value);
    return true;
  } catch {
    return false;
  }
}

/**
 * Checks if a value is a `Handle` (EncryptedValue) instance.
 *
 * **Same-realm only**: Uses `instanceof` which only works when the value
 * was created in the same JavaScript realm (same module instance).
 * Will return `false` for handles from:
 * - Different package versions (duplicate node_modules)
 * - Different bundler outputs
 * - Cross-realm contexts (iframes, workers)
 *
 * @param value - The value to check
 * @returns `true` if value is a `Handle` instance from the same realm
 */
export function isHandle(value: unknown): value is Handle {
  return value instanceof FhevmHandleImpl;
}

/**
 * Checks if a value is an {@link ExternalHandle} instance.
 *
 * **Same-realm only**: like {@link isHandle}, it uses `instanceof`. An external
 * handle is recognized by how it was created, never by its bytes: in the
 * RFC-030 layout byte 21 is package digest data, so it may equal the computed
 * marker `0xFF`.
 *
 * @param value - The value to check
 * @returns `true` if value is an `ExternalHandle` instance from the same realm
 */
export function isExternalHandle(value: unknown): value is ExternalHandle {
  return value instanceof FhevmExternalHandleImpl;
}

/**
 * Checks if a value is an {@link InputHandle} (ZKPoK) instance.
 *
 * **Same-realm only**: like {@link isHandle}, it uses `instanceof`.
 *
 * @param value - The value to check
 * @returns `true` if value is an `InputHandle` instance from the same realm
 */
export function isInputHandle(value: unknown): value is InputHandle {
  return value instanceof FhevmInputHandleImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function asHandle(value: unknown): Handle {
  assertIsHandle(value);
  return value;
}

export function asHandleBytes32(value: unknown): HandleBytes32 {
  assertIsHandleBytes32(value);
  return value;
}

export function asHandleBytes32Hex(value: unknown): HandleBytes32Hex {
  assertIsHandleBytes32Hex(value);
  return value;
}

/**
 * [Trusted] Converts a `HandleBytes32Hex` to a `Handle`.
 *
 * Trusts the type system for hex format validation.
 *
 * @param handleBytes32Hex - A valid 32-byte hex string (typed as `HandleBytes32Hex`)
 * @returns A `Handle` instance
 */
export function handleBytes32HexToHandle(handleBytes32Hex: HandleBytes32Hex): Handle {
  return new FhevmHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex,
  }) as Handle;
}

/**
 * [Trusted] Converts a `Bytes32Hex` to a `Handle`.
 *
 * Trusts the type system for hex format validation.
 * Still validates FHEVM-specific fields (fheTypeId, version).
 *
 * @param handleBytes32Hex - A valid 32-byte hex string (typed as `Bytes32Hex`)
 * @returns A `Handle` instance
 * @throws A {@link FhevmHandleError} If fheTypeId or version is invalid
 */
export function bytes32HexToHandle(handleBytes32Hex: Bytes32Hex): Handle {
  _assertIsHandleBytes32Hex(handleBytes32Hex);

  return new FhevmHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex,
  }) as Handle;
}

/**
 * [Trusted] Converts a `Bytes32Hex` to an `ExternalHandle`.
 *
 * Trusts the type system for hex format validation.
 * Still validates FHEVM-specific fields (fheTypeId, version).
 *
 * @param handleBytes32Hex - A valid 32-byte hex string (typed as `Bytes32Hex`)
 * @returns An `ExternalHandle` instance
 * @throws A {@link FhevmHandleError} If fheTypeId or version is invalid
 */
export function bytes32HexToExternalHandle(handleBytes32Hex: Bytes32Hex): ExternalHandle {
  _assertIsHandleBytes32Hex(handleBytes32Hex);

  return new FhevmExternalHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex: handleBytes32Hex as ExternalHandleBytes32Hex,
  }) as ExternalHandle;
}

/**
 * [Trusted] Converts a `Bytes32` to an `ExternalHandle`.
 *
 * Trusts the type system for bytes format validation.
 * Still validates FHEVM-specific fields (fheTypeId, version).
 *
 * @param bytes - A valid 32-byte array (typed as `Bytes32`)
 * @returns An `ExternalHandle` instance
 * @throws A {@link FhevmHandleError} If fheTypeId or version is invalid
 */
export function bytes32ToExternalHandle(bytes: Bytes32): ExternalHandle {
  _assertIsHandleBytes32(bytes);
  return new FhevmExternalHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex: bytes32ToHex(bytes) as ExternalHandleBytes32Hex,
    handleBytes32: new Uint8Array(bytes) as ExternalHandleBytes32,
  }) as ExternalHandle;
}

/**
 * [Trusted] Converts a `Bytes32Hex` to an `InputHandle`.
 *
 * Trusts the type system for hex format validation.
 * Still validates FHEVM-specific fields (input index, fheTypeId, version).
 *
 * @param handleBytes32Hex - A valid 32-byte hex string (typed as `Bytes32Hex`)
 * @returns An `InputHandle` instance
 * @throws A {@link FhevmHandleError} If the input index, fheTypeId or version is invalid
 */
export function bytes32HexToInputHandle(handleBytes32Hex: Bytes32Hex): InputHandle {
  _assertIsHandleBytes32Hex(handleBytes32Hex);
  _assertIsInputHandleIndex(bytesHexUint8At(handleBytes32Hex, FHEVM_INPUT_HANDLE_INDEX_BYTE_OFFSET));

  return new FhevmInputHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex: handleBytes32Hex as InputHandleBytes32Hex,
  }) as InputHandle;
}

/**
 * [Trusted] Converts a `Bytes32` to an `InputHandle`.
 *
 * Trusts the type system for bytes format validation.
 * Still validates FHEVM-specific fields (input index, fheTypeId, version).
 *
 * @param bytes - A valid 32-byte array (typed as `Bytes32`)
 * @returns An `InputHandle` instance
 * @throws A {@link FhevmHandleError} If the input index, fheTypeId or version is invalid
 */
export function bytes32ToInputHandle(bytes: Bytes32): InputHandle {
  _assertIsHandleBytes32(bytes);
  _assertIsInputHandleIndex(bytesUint8At(bytes, FHEVM_INPUT_HANDLE_INDEX_BYTE_OFFSET));

  return new FhevmInputHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex: bytes32ToHex(bytes) as InputHandleBytes32Hex,
    handleBytes32: new Uint8Array(bytes) as InputHandleBytes32,
  }) as InputHandle;
}

/**
 * [Trusted] Converts a `Bytes32` to a `Handle`.
 *
 * Trusts the type system for bytes format validation.
 * Still validates FHEVM-specific fields (fheTypeId, version).
 *
 * @param bytes - A valid 32-byte array (typed as `Bytes32`)
 * @returns A `Handle` instance
 * @throws A {@link FhevmHandleError} If fheTypeId or version is invalid
 */
export function bytes32ToHandle(bytes: Bytes32): Handle {
  // bytes is validated as a Bytes32
  const hex = bytes32ToHex(bytes);

  _assertIsHandleBytes32(bytes);
  return new FhevmHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex: hex as HandleBytes32Hex,
    handleBytes32: new Uint8Array(bytes as Bytes32) as HandleBytes32,
  }) as Handle;
}

////////////////////////////////////////////////////////////////////////////////
// Conversion
////////////////////////////////////////////////////////////////////////////////

/**
 * [Validated] Converts an unknown value to a `Handle`.
 *
 * Performs full runtime validation of input format and FHEVM-specific fields.
 *
 * @param value - An unknown value (string, Uint8Array, or object with bytes32Hex)
 * @returns A `Handle` instance
 * @throws {InvalidTypeError} If value is not a valid bytes32 hex or bytes32
 * @throws {FhevmHandleError} If fheTypeId or version is invalid
 */
export function toFhevmHandle(value: unknown): Handle {
  if (value instanceof FhevmHandleImpl) {
    return value as Handle;
  }

  // Object with bytes32Hex property (FhevmHandle-like)
  if (value !== null && typeof value === 'object' && 'bytes32Hex' in value) {
    assertIsBytes32Hex(value.bytes32Hex, {});
    return bytes32HexToHandle(value.bytes32Hex);
  }

  if (typeof value === 'string') {
    assertIsBytes32Hex(value, {});
    return bytes32HexToHandle(value);
  }

  assertIsBytes32(value, {});
  return bytes32ToHandle(value);
}

/**
 * [Validated] Converts an unknown value to an `ExternalHandle`.
 *
 * The bytes are read with the RFC-030 external handle layout; nothing in them
 * distinguishes an external handle from a computed one, so the caller decides.
 *
 * @param value - An unknown value (string, Uint8Array, or object with bytes32Hex)
 * @returns An `ExternalHandle` instance
 * @throws {InvalidTypeError} If value is not a valid bytes32 hex or bytes32
 * @throws {FhevmHandleError} If fheTypeId or version is invalid
 */
export function toExternalHandle(value: unknown): ExternalHandle {
  if (value instanceof FhevmExternalHandleImpl) {
    return value as ExternalHandle;
  }

  // Object with bytes32Hex property (FhevmHandle-like)
  if (value !== null && typeof value === 'object' && 'bytes32Hex' in value) {
    assertIsBytes32Hex(value.bytes32Hex, {});
    return bytes32HexToExternalHandle(value.bytes32Hex);
  }

  if (typeof value === 'string') {
    assertIsBytes32Hex(value, {});
    return bytes32HexToExternalHandle(value);
  }

  assertIsBytes32(value, {});
  return bytes32ToExternalHandle(value);
}

/**
 * [Validated] Converts an unknown value to an `InputHandle`.
 *
 * The bytes are read with the ZKPoK input handle layout: byte 21 is the input
 * index, so the computed marker `0xFF` is rejected.
 *
 * @param value - An unknown value (string, Uint8Array, or object with bytes32Hex)
 * @returns An `InputHandle` instance
 * @throws {InvalidTypeError} If value is not a valid bytes32 hex or bytes32
 * @throws {FhevmHandleError} If the input index, fheTypeId or version is invalid
 */
export function toInputHandle(value: unknown): InputHandle {
  if (value instanceof FhevmInputHandleImpl) {
    return value as InputHandle;
  }

  // Object with bytes32Hex property (FhevmHandle-like)
  if (value !== null && typeof value === 'object' && 'bytes32Hex' in value) {
    assertIsBytes32Hex(value.bytes32Hex, {});
    return bytes32HexToInputHandle(value.bytes32Hex);
  }

  if (typeof value === 'string') {
    assertIsBytes32Hex(value, {});
    return bytes32HexToInputHandle(value);
  }

  assertIsBytes32(value, {});
  return bytes32ToInputHandle(value);
}

/**
 * [Trusted] Compares two `Handle` instances for equality.
 *
 * @param a - First handle
 * @param b - Second handle
 * @returns `true` if both handles have the same `bytes32Hex` value
 */
export function handleEquals(a: Handle, b: Handle): boolean {
  return a.bytes32Hex === b.bytes32Hex;
}

export function assertHandleArrayEquals(
  actual: ReadonlyArray<Handle | InputHandle>,
  expected: ReadonlyArray<Handle | InputHandle>,
  options?: {
    actualName?: string | undefined;
    expectedName?: string | undefined;
  },
): void {
  const actualTxt = options?.actualName !== undefined ? ` (${options.actualName})` : '';
  const expectedTxt = options?.expectedName !== undefined ? ` (${options.expectedName})` : '';

  if (actual.length !== expected.length) {
    throw new FhevmHandleError({
      message: `Unexpected handles list sizes: ${actual.length}${actualTxt} != ${expected.length}${expectedTxt}`,
    });
  }

  const expectedHandles = expected.map((h) => toFhevmHandle(h));

  for (let i = 0; i < actual.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const a = actual[i]!;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const e = expectedHandles[i]!;
    if (a.bytes32Hex !== e.bytes32Hex) {
      throw new FhevmHandleError({
        message: `Unexpected handle[${i}]: ${a.bytes32Hex}${actualTxt} != ${e.bytes32Hex}${expectedTxt}`,
      });
    }
  }
}

/**
 * Builds an RFC-030 external handle:
 * hash29 (bytes 0..28) | index (29) | fheTypeId (30) | version (31),
 * where `hash29` is the first 29 bytes of the input package digest.
 */
export function buildExternalHandle({
  hash29,
  index,
  fheTypeId,
  version,
}: {
  hash29: string;
  index: number;
  fheTypeId: number;
  version?: number;
}): ExternalHandle {
  const theVersion = asUint8Number(version ?? FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION);
  const handleHash29 = hexToBytes(hash29);
  if (handleHash29.length !== 29) {
    throw new FhevmHandleError({
      message: `Invalid external handle hash29: expected 29 bytes, got ${handleHash29.length}.`,
    });
  }

  // A package holds at most 255 values, so input indices are 0..254.
  const theIndex = asUint8Number(index);
  if (theIndex === 255) {
    throw new FhevmHandleError({
      message: `Invalid external handle index: 255 (an input package holds at most 255 values, indexed 0..254).`,
    });
  }

  const handleBytes32AsBytes = new Uint8Array(32) as Bytes32;
  handleBytes32AsBytes.set(handleHash29, FHEVM_EXTERNAL_HANDLE_HASH29_BYTE_OFFSET);
  handleBytes32AsBytes[FHEVM_EXTERNAL_HANDLE_INDEX_BYTE_OFFSET] = theIndex;
  handleBytes32AsBytes[FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET] = fheTypeId;
  handleBytes32AsBytes[FHEVM_HANDLE_VERSION_BYTE_OFFSET] = theVersion;

  return bytes32ToExternalHandle(handleBytes32AsBytes);
}

/**
 * Builds a ZKPoK input handle:
 * hash21 (bytes 0..20) | index (21) | chainId (22..29) | fheTypeId (30) | version (31).
 */
export function buildInputHandle({
  hash21,
  index,
  chainId,
  fheTypeId,
  version,
}: {
  hash21: string;
  index: number;
  chainId: number | bigint;
  fheTypeId: number;
  version?: number;
}): InputHandle {
  const theVersion = asUint8Number(version ?? FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION);
  const handleHash21 = hexToBytes(hash21);
  if (handleHash21.length !== 21) {
    throw new FhevmHandleError({
      message: `Invalid input handle hash21: expected 21 bytes, got ${handleHash21.length}.`,
    });
  }

  const theIndex = asUint8Number(index);
  _assertIsInputHandleIndex(theIndex);

  const chainId8Bytes = uint64ToBytes32(chainId).subarray(24, 32);

  const handleBytes32AsBytes = new Uint8Array(32) as Bytes32;
  handleBytes32AsBytes.set(handleHash21, FHEVM_HANDLE_HASH21_BYTE_OFFSET);
  handleBytes32AsBytes[FHEVM_INPUT_HANDLE_INDEX_BYTE_OFFSET] = theIndex;
  handleBytes32AsBytes.set(chainId8Bytes, FHEVM_HANDLE_CHAINID_BYTE_OFFSET);
  handleBytes32AsBytes[FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET] = fheTypeId;
  handleBytes32AsBytes[FHEVM_HANDLE_VERSION_BYTE_OFFSET] = theVersion;

  return bytes32ToInputHandle(handleBytes32AsBytes);
}

export function assertHandlesBelongToSameChainId(fhevmHandles: readonly Handle[], chainId?: Uint64BigInt): void {
  if (fhevmHandles.length === 0) {
    return;
  }
  const theChainId = chainId ?? fhevmHandles[0]?.chainId;
  for (const handle of fhevmHandles) {
    if (handle.chainId !== theChainId) {
      throw new FhevmHandleError({
        message: `Handle (${handle.bytes32Hex}) has chainId ${handle.chainId}, expected ${chainId}`,
      });
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// Private Helpers
////////////////////////////////////////////////////////////////////////////////

function _assertIsValidFhevmHandleFields(
  fheTypeId: number,
  version: number,
  options?: { subject?: string } & ErrorMetadataParams,
): void {
  if (!isFheTypeId(fheTypeId)) {
    throw new FhevmHandleError({
      ...options,
      message: `FHEVM Handle is invalid. Unknown FheType: ${fheTypeId}`,
    });
  }
  if (version !== FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION) {
    throw new FhevmHandleError({
      ...options,
      message: `FHEVM Handle is invalid. Unknown version: ${version}`,
    });
  }
}

function _assertIsInputHandleIndex(index: number): void {
  if (index === FHEVM_HANDLE_COMPUTED_MARKER) {
    throw new FhevmHandleError({
      message: `Invalid input handle index: 255 is the computed handle marker (input indices are 0..254).`,
    });
  }
}

function _assertIsHandleBytes32Hex(
  value: Bytes32Hex,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is HandleBytes32Hex {
  _assertIsValidFhevmHandleFields(
    bytesHexUint8At(value, FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET),
    bytesHexUint8At(value, FHEVM_HANDLE_VERSION_BYTE_OFFSET),
    options,
  );
}

function _assertIsHandleBytes32(
  value: Bytes32,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is HandleBytes32 {
  _assertIsValidFhevmHandleFields(
    bytesUint8At(value, FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET),
    bytesUint8At(value, FHEVM_HANDLE_VERSION_BYTE_OFFSET),
    options,
  );
}

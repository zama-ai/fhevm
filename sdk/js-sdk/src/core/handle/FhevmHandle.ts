import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { Bytes21Hex, Bytes32, Bytes32Hex, Uint64BigInt, Uint8Number, ValueTypeName } from '../types/primitives.js';
import type { EncryptionBits, FheType, FheTypeId, SolidityPrimitiveTypeName } from '../types/fheType.js';
import type { EncryptedValueLike } from '../types/encryptedTypes.js';
import type {
  Handle,
  HandleBaseV0,
  HandleBytes32,
  HandleBytes32Hex,
  HandleBytes32HexNo0x,
  InputHandle,
  InputHandleBytes32,
  InputHandleBytes32Hex,
} from '../types/encryptedTypes-p.js';
import { FhevmHandleError } from '../errors/FhevmHandleError.js';
import {
  asBytes21,
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

const FHEVM_HANDLE_HASH21_BYTE_OFFSET = 0 as Uint8Number;
const FHEVM_HANDLE_INDEX_BYTE_OFFSET = 21 as Uint8Number;
const FHEVM_HANDLE_CHAINID_BYTE_OFFSET = 22 as Uint8Number;
const FHEVM_HANDLE_FHETYPEID_BYTE_OFFSET = 30 as Uint8Number;
const FHEVM_HANDLE_VERSION_BYTE_OFFSET = 31 as Uint8Number;

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

  public get isComputed(): boolean {
    return this.index === 255;
  }

  public get index(): Uint8Number {
    // Extract index (byte 21) - 255 means computed
    const indexUint8: Uint8Number = bytesHexUint8At(this.#handleBytes32Hex, FHEVM_HANDLE_INDEX_BYTE_OFFSET);
    return indexUint8;
  }

  public get isExternal(): boolean {
    return !this.isComputed;
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

export function assertIsInputHandleBytes32Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is InputHandleBytes32Hex {
  assertIsHandleBytes32Hex(value, options);
  _assertIsInputHandleBytes32Hex(value, options);
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

export function assertIsInputHandleBytes32(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is InputHandleBytes32 {
  assertIsHandleBytes32(value, options);
  _assertIsInputHandleBytes32(value, options);
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

// export function assertIsEncryptedValueLike(
//   value: unknown,
//   options?: { subject?: string } & ErrorMetadataParams,
// ): asserts value is EncryptedValueLike {
//   assertIsHandleLike(value, options);
// }

// export function assertIsInputHandleLike(
//   value: unknown,
//   options?: { subject?: string } & ErrorMetadataParams,
// ): asserts value is InputHandleLike {
//   if (value instanceof FhevmHandleImpl) {
//     if (!value.isExternal) {
//       throw new FhevmHandleError({ message: 'Expected an input handle' });
//     }
//     return;
//   }

//   if (value !== null && typeof value === 'object' && 'bytes32Hex' in value) {
//     assertIsInputHandleBytes32Hex(value.bytes32Hex, options);
//     return;
//   }

//   if (typeof value === 'string') {
//     assertIsInputHandleBytes32Hex(value, options);
//     return;
//   }

//   assertIsInputHandleBytes32(value, options);
// }

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

// export function isHandleLike(value: unknown): value is HandleLike {
//   try {
//     assertIsHandleLike(value);
//     return true;
//   } catch {
//     return false;
//   }
// }

// export function isInputHandleLike(value: unknown): value is InputHandleLike {
//   try {
//     assertIsInputHandleLike(value);
//     return true;
//   } catch {
//     return false;
//   }
// }

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

export function isInputHandle(value: unknown): value is InputHandle {
  return isHandle(value) && value.isExternal && value.index !== 255 && !value.isComputed;
}

////////////////////////////////////////////////////////////////////////////////

export function asHandle(value: unknown): Handle {
  assertIsHandle(value);
  return value;
}

// export function asHandleLike(value: unknown): HandleLike {
//   assertIsHandleLike(value);
//   return value;
// }

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

export function bytes32HexToInputHandle(handleBytes32Hex: Bytes32Hex): InputHandle {
  _assertIsInputHandleBytes32Hex(handleBytes32Hex);

  return new FhevmHandleImpl(PRIVATE_TOKEN, {
    handleBytes32Hex,
  }) as unknown as InputHandle;
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

// /**
//  * [Trusted] Converts a `HandleLike` to a `Handle`.
//  *
//  * Trusts the type system for input validation.
//  * Still validates FHEVM-specific fields (fheTypeId, version).
//  *
//  * @param handleLike - A `HandleLike` (Bytes32, Bytes32Hex, Bytes32HexAble, or Handle)
//  * @returns A `Handle` instance
//  * @throws A {@link FhevmHandleError} If fheTypeId or version is invalid
//  */
// export function handleLikeToHandle(handleLike: HandleLike): Handle {
//   // Already a Handle
//   if (handleLike instanceof FhevmHandleImpl) {
//     return handleLike as Handle;
//   }

//   // Bytes32Hex (string)
//   if (typeof handleLike === 'string') {
//     return bytes32HexToHandle(asBytes32Hex(handleLike));
//   }

//   // Bytes32HexAble (object with bytes32Hex property)
//   if ('bytes32Hex' in handleLike) {
//     return bytes32HexToHandle(asBytes32Hex(handleLike.bytes32Hex));
//   }

//   // Bytes32 (Uint8Array)
//   return bytes32ToHandle(asBytes32(handleLike));
// }

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

export function toInputHandle(value: unknown): InputHandle {
  const h = toFhevmHandle(value);
  if (!isInputHandle(h)) {
    throw new FhevmHandleError({
      message: 'Invalid input handle',
    });
  }
  return h;
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
  actual: readonly Handle[],
  expected: readonly Handle[],
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

export function buildHandle(parameters: {
  index: number;
  chainId: number | bigint;
  hash21: string;
  fheTypeId: number;
  version?: number;
}): InputHandle;
export function buildHandle(parameters: {
  index?: undefined;
  chainId: number | bigint;
  hash21: string;
  fheTypeId: number;
  version?: number;
}): Handle;
export function buildHandle({
  index,
  chainId,
  hash21,
  fheTypeId,
  version,
}: {
  index?: number | undefined;
  chainId: number | bigint;
  hash21: string;
  fheTypeId: number;
  version?: number;
}): Handle | InputHandle {
  const theVersion = asUint8Number(version ?? FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION);
  const chainId32Bytes = uint64ToBytes32(chainId);
  const chainId8Bytes = chainId32Bytes.subarray(24, 32);

  const handleHash21 = asBytes21(hexToBytes(hash21));

  const handleBytes32AsBytes = new Uint8Array(32) as Bytes32;
  handleBytes32AsBytes.set(handleHash21, 0);
  handleBytes32AsBytes[21] = asUint8Number(index ?? 255);
  handleBytes32AsBytes.set(chainId8Bytes, 22);
  handleBytes32AsBytes[30] = fheTypeId;
  handleBytes32AsBytes[31] = theVersion;

  return bytes32ToHandle(handleBytes32AsBytes);
}

// export function assertIsHandleLikeArray(
//   value: unknown,
//   options?: { subject?: string } & ErrorMetadataParams,
// ): asserts value is HandleLike[] {
//   if (!Array.isArray(value)) {
//     throw new InvalidTypeError(
//       {
//         subject: options?.subject,
//         type: typeof value,
//         expectedType: 'HandleLike[]',
//       },
//       options ?? {},
//     );
//   }
//   for (let i = 0; i < value.length; ++i) {
//     if (!isHandleLike(value[i])) {
//       throw new InvalidTypeError(
//         {
//           subject: options?.subject,
//           index: i,
//           type: typeof value[i],
//           expectedType: 'HandleLike',
//         },
//         options ?? {},
//       );
//     }
//   }
// }

// export function assertIsInputHandleLikeArray(
//   value: unknown,
//   options?: { subject?: string } & ErrorMetadataParams,
// ): asserts value is InputHandleLike[] {
//   if (!Array.isArray(value)) {
//     throw new InvalidTypeError(
//       {
//         subject: options?.subject,
//         type: typeof value,
//         expectedType: 'InputHandleLike[]',
//       },
//       options ?? {},
//     );
//   }
//   for (let i = 0; i < value.length; ++i) {
//     if (!isInputHandleLike(value[i])) {
//       throw new InvalidTypeError(
//         {
//           subject: options?.subject,
//           index: i,
//           type: typeof value[i],
//           expectedType: 'InputHandleLike',
//         },
//         options ?? {},
//       );
//     }
//   }
// }

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

function _assertIsInputHandleBytes32Hex(
  value: Bytes32Hex,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is InputHandleBytes32Hex {
  _assertIsHandleBytes32Hex(value, options);
  // InputHandle (external) must have index < 255.
  // index === 255 means computed (not external).
  const index = bytesHexUint8At(value, FHEVM_HANDLE_INDEX_BYTE_OFFSET);
  if (index === 255) {
    throw new FhevmHandleError({
      message: `Expected an input handle (index < 255) but got a computed handle (index = 255)`,
      handle: value,
    });
  }
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

function _assertIsInputHandleBytes32(
  value: Bytes32,
  options?: { subject?: string } & ErrorMetadataParams,
): asserts value is InputHandleBytes32 {
  _assertIsHandleBytes32(value, options);
  // InputHandle (external) must have index < 255.
  // index === 255 means computed (not external).
  const index = bytesUint8At(value, FHEVM_HANDLE_INDEX_BYTE_OFFSET);
  if (index === 255) {
    throw new FhevmHandleError({
      message: `Expected an input handle (index < 255) but got a computed handle (index = 255)`,
    });
  }
}

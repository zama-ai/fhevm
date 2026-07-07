import { describe, it, expect } from 'vitest';
import {
  assertIsHandleBytes32Hex,
  assertIsInputHandleBytes32Hex,
  assertIsHandleBytes32,
  assertIsInputHandleBytes32,
  assertIsHandle,
  assertIsInputHandle,
  assertIsEncryptedValueLike,
  isHandleBytes32,
  isHandleBytes32Hex,
  isHandle,
  isInputHandle,
  asHandle,
  asHandleBytes32,
  asHandleBytes32Hex,
  handleBytes32HexToHandle,
  bytes32HexToHandle,
  bytes32HexToInputHandle,
  bytes32ToHandle,
  toFhevmHandle,
  toInputHandle,
  handleEquals,
  assertHandleArrayEquals,
  buildHandle,
  assertHandlesBelongToSameChainId,
  FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION,
} from './FhevmHandle.js';
import { FhevmHandleError } from '../errors/FhevmHandleError.js';
import { asBytes32Hex, asBytes32 } from '../base/bytes.js';
import { asUint64BigInt } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/handle/FhevmHandle.test.ts
////////////////////////////////////////////////////////////////////////////////

const HASH21 = `0x${'ab'.repeat(21)}`;
const CHAIN_ID = 12345n;
const EUINT8 = 2;
const EADDRESS = 7;

describe('FhevmHandle', () => {
  //////////////////////////////////////////////////////////////////////////////
  // buildHandle / instance getters
  //////////////////////////////////////////////////////////////////////////////

  it('buildHandle builds a computed handle by default (no index)', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(handle.hash21.toLowerCase()).toBe(HASH21);
    expect(handle.chainId).toBe(CHAIN_ID);
    expect(handle.fheTypeId).toBe(EUINT8);
    expect(handle.fheType).toBe('euint8');
    expect(handle.clearType).toBe('uint8');
    expect(handle.version).toBe(FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION);
    expect(handle.index).toBe(255);
    expect(handle.isComputed).toBe(true);
    expect(handle.isExternal).toBe(false);
    expect(handle.encryptionBits).toBe(8);
    expect(handle.solidityPrimitiveTypeName).toBe('uint256');
    expect(handle.bytes32Hex).toBe(handle.toString());
    expect(JSON.stringify(handle)).toBe(JSON.stringify(handle.bytes32Hex));
    expect(handle.bytes32HexNo0x).toBe(handle.bytes32Hex.slice(2));
    expect(handle.bytes32).toBeInstanceOf(Uint8Array);
    expect(handle.bytes32.length).toBe(32);
  });

  it('buildHandle builds an external/input handle when index is provided', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EADDRESS, index: 3 });

    expect(handle.index).toBe(3);
    expect(handle.isComputed).toBe(false);
    expect(handle.isExternal).toBe(true);
    expect(handle.fheType).toBe('eaddress');
    expect(isInputHandle(handle)).toBe(true);
  });

  it('buildHandle.bytes32 returns a defensive copy each time', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    const a = handle.bytes32;
    const b = handle.bytes32;
    expect(a).not.toBe(b);
    expect(a).toEqual(b);
  });

  //////////////////////////////////////////////////////////////////////////////
  // isHandleBytes32Hex / assertIsHandleBytes32Hex
  //////////////////////////////////////////////////////////////////////////////

  it('isHandleBytes32Hex / assertIsHandleBytes32Hex accept a valid handle hex', () => {
    const validHex = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32Hex;

    expect(isHandleBytes32Hex(validHex)).toBe(true);
    expect(() => assertIsHandleBytes32Hex(validHex)).not.toThrow();
  });

  it('isHandleBytes32Hex / assertIsHandleBytes32Hex reject a non bytes32-hex value', () => {
    expect(isHandleBytes32Hex('not-a-hex')).toBe(false);
    expect(() => assertIsHandleBytes32Hex('not-a-hex')).toThrow(FhevmHandleError);
    expect(() => assertIsHandleBytes32Hex(123)).toThrow(FhevmHandleError);
    expect(() => assertIsHandleBytes32Hex(null)).toThrow(FhevmHandleError);
  });

  it('isHandleBytes32Hex / assertIsHandleBytes32Hex reject an unknown fheTypeId', () => {
    // byte 30 (fheTypeId) = 0x63 (99), which is not a known FheTypeId
    const invalidFheTypeHex = `0x${'00'.repeat(30)}6300`;

    expect(isHandleBytes32Hex(invalidFheTypeHex)).toBe(false);
    expect(() => assertIsHandleBytes32Hex(invalidFheTypeHex)).toThrow(FhevmHandleError);
  });

  it('isHandleBytes32Hex / assertIsHandleBytes32Hex reject an unknown version', () => {
    // byte 30 = 0x02 (euint8), byte 31 (version) = 0x01, only version 0 is supported
    const invalidVersionHex = `0x${'00'.repeat(30)}0201`;

    expect(isHandleBytes32Hex(invalidVersionHex)).toBe(false);
    expect(() => assertIsHandleBytes32Hex(invalidVersionHex)).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertIsInputHandleBytes32Hex
  //////////////////////////////////////////////////////////////////////////////

  it('assertIsInputHandleBytes32Hex accepts an external handle and rejects a computed one', () => {
    const externalHex = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 }).bytes32Hex;
    const computedHex = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32Hex;

    expect(() => assertIsInputHandleBytes32Hex(externalHex)).not.toThrow();
    expect(() => assertIsInputHandleBytes32Hex(computedHex)).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // isHandleBytes32 / assertIsHandleBytes32 / assertIsInputHandleBytes32
  //////////////////////////////////////////////////////////////////////////////

  it('isHandleBytes32 / assertIsHandleBytes32 accept a valid handle bytes32', () => {
    const validBytes = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32;

    expect(isHandleBytes32(validBytes)).toBe(true);
    expect(() => assertIsHandleBytes32(validBytes)).not.toThrow();
  });

  it('isHandleBytes32 / assertIsHandleBytes32 reject invalid values', () => {
    expect(isHandleBytes32(new Uint8Array(31))).toBe(false);
    expect(() => assertIsHandleBytes32(new Uint8Array(31))).toThrow(FhevmHandleError);
    expect(() => assertIsHandleBytes32('not-bytes')).toThrow(FhevmHandleError);
  });

  it('assertIsInputHandleBytes32 accepts an external handle and rejects a computed one', () => {
    const externalBytes = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 }).bytes32;
    const computedBytes = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32;

    expect(() => assertIsInputHandleBytes32(externalBytes)).not.toThrow();
    expect(() => assertIsInputHandleBytes32(computedBytes)).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // isHandle / assertIsHandle / isInputHandle / assertIsInputHandle
  //////////////////////////////////////////////////////////////////////////////

  it('isHandle / assertIsHandle only accept genuine Handle instances', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(isHandle(handle)).toBe(true);
    expect(() => assertIsHandle(handle)).not.toThrow();

    expect(isHandle({ bytes32Hex: handle.bytes32Hex })).toBe(false);
    expect(isHandle(handle.bytes32Hex)).toBe(false);
    expect(() => assertIsHandle({ bytes32Hex: handle.bytes32Hex })).toThrow(FhevmHandleError);
  });

  it('isInputHandle / assertIsInputHandle distinguish external from computed handles', () => {
    const external = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 5 });
    const computed = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(isInputHandle(external)).toBe(true);
    expect(() => assertIsInputHandle(external)).not.toThrow();

    expect(isInputHandle(computed)).toBe(false);
    expect(() => assertIsInputHandle(computed)).toThrow(FhevmHandleError);

    expect(isInputHandle('not-a-handle')).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertIsEncryptedValueLike
  //////////////////////////////////////////////////////////////////////////////

  it('assertIsEncryptedValueLike accepts a Handle, a bytes32Hex string, a {bytes32Hex} object, and Bytes32', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(() => assertIsEncryptedValueLike(handle)).not.toThrow();
    expect(() => assertIsEncryptedValueLike(handle.bytes32Hex)).not.toThrow();
    expect(() => assertIsEncryptedValueLike({ bytes32Hex: handle.bytes32Hex })).not.toThrow();
    expect(() => assertIsEncryptedValueLike(handle.bytes32)).not.toThrow();
  });

  it('assertIsEncryptedValueLike rejects null, undefined, and malformed values', () => {
    expect(() => assertIsEncryptedValueLike(null)).toThrow(FhevmHandleError);
    expect(() => assertIsEncryptedValueLike(undefined)).toThrow(FhevmHandleError);
    expect(() => assertIsEncryptedValueLike('not-a-hex')).toThrow();
    expect(() => assertIsEncryptedValueLike({ bytes32Hex: 'not-a-hex' })).toThrow();
    expect(() => assertIsEncryptedValueLike(123)).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////
  // asHandle / asHandleBytes32 / asHandleBytes32Hex
  //////////////////////////////////////////////////////////////////////////////

  it('asHandle / asHandleBytes32 / asHandleBytes32Hex return the value when valid', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });
    const bytes32 = handle.bytes32;
    const bytes32Hex = handle.bytes32Hex;

    expect(asHandle(handle)).toBe(handle);
    expect(asHandleBytes32(bytes32)).toBe(bytes32);
    expect(asHandleBytes32Hex(bytes32Hex)).toBe(bytes32Hex);
  });

  it('asHandle / asHandleBytes32 / asHandleBytes32Hex throw on invalid values', () => {
    expect(() => asHandle('not-a-handle')).toThrow(FhevmHandleError);
    expect(() => asHandleBytes32('not-bytes')).toThrow(FhevmHandleError);
    expect(() => asHandleBytes32Hex('not-a-hex')).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // handleBytes32HexToHandle / bytes32HexToHandle / bytes32HexToInputHandle / bytes32ToHandle
  //////////////////////////////////////////////////////////////////////////////

  it('handleBytes32HexToHandle wraps a hex string without validation (trusted)', () => {
    const validHex = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32Hex;
    const handle = handleBytes32HexToHandle(validHex);

    expect(isHandle(handle)).toBe(true);
    expect(handle.bytes32Hex).toBe(validHex);
  });

  it('bytes32HexToHandle validates fheTypeId/version and builds a Handle', () => {
    const validHex = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32Hex;
    const handle = bytes32HexToHandle(validHex);

    expect(isHandle(handle)).toBe(true);
    expect(handle.fheType).toBe('euint8');

    const invalidFheTypeHex = asBytes32Hex(`0x${'00'.repeat(30)}6300`);
    expect(() => bytes32HexToHandle(invalidFheTypeHex)).toThrow(FhevmHandleError);
  });

  it('bytes32HexToInputHandle validates the index and rejects computed handles', () => {
    const externalHex = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 }).bytes32Hex;
    const computedHex = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32Hex;

    const inputHandle = bytes32HexToInputHandle(externalHex);
    expect(isInputHandle(inputHandle)).toBe(true);

    expect(() => bytes32HexToInputHandle(computedHex)).toThrow(FhevmHandleError);
  });

  it('bytes32ToHandle validates fheTypeId/version and builds a Handle from raw bytes', () => {
    const validBytes = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 }).bytes32;
    const handle = bytes32ToHandle(validBytes);

    expect(isHandle(handle)).toBe(true);
    expect(handle.bytes32Hex.toLowerCase()).toBe(
      `0x${Array.from(validBytes)
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('')}`,
    );

    const invalidBytesRaw = new Uint8Array(32);
    invalidBytesRaw[30] = 99; // unknown fheTypeId
    const invalidBytes = asBytes32(invalidBytesRaw);
    expect(() => bytes32ToHandle(invalidBytes)).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // toFhevmHandle / toInputHandle
  //////////////////////////////////////////////////////////////////////////////

  it('toFhevmHandle accepts a Handle instance, a {bytes32Hex} object, a hex string, and Bytes32', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(toFhevmHandle(handle)).toBe(handle);
    expect(handleEquals(toFhevmHandle({ bytes32Hex: handle.bytes32Hex }), handle)).toBe(true);
    expect(handleEquals(toFhevmHandle(handle.bytes32Hex), handle)).toBe(true);
    expect(handleEquals(toFhevmHandle(handle.bytes32), handle)).toBe(true);
  });

  it('toFhevmHandle throws on invalid input', () => {
    expect(() => toFhevmHandle('not-a-hex')).toThrow();
    expect(() => toFhevmHandle({ bytes32Hex: 'not-a-hex' })).toThrow();
    expect(() => toFhevmHandle(123)).toThrow();
    expect(() => toFhevmHandle(null)).toThrow();
  });

  it('toInputHandle returns an input handle and rejects computed handles', () => {
    const external = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 });
    const computed = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(handleEquals(toInputHandle(external.bytes32Hex), external)).toBe(true);
    expect(() => toInputHandle(computed.bytes32Hex)).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // handleEquals
  //////////////////////////////////////////////////////////////////////////////

  it('handleEquals compares handles by bytes32Hex', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 });
    const b = bytes32HexToHandle(a.bytes32Hex);
    const c = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 1 });

    expect(handleEquals(a, b)).toBe(true);
    expect(handleEquals(a, c)).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertHandleArrayEquals
  //////////////////////////////////////////////////////////////////////////////

  it('assertHandleArrayEquals passes for matching arrays', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 });
    const b = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 1 });

    expect(() => assertHandleArrayEquals([a, b], [a, b])).not.toThrow();
    // `expected` items are converted via toFhevmHandle, so raw hex strings work too
    expect(() => assertHandleArrayEquals([a, b], [a.bytes32Hex, b.bytes32Hex] as never)).not.toThrow();
  });

  it('assertHandleArrayEquals throws on length mismatch', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 });

    expect(() => assertHandleArrayEquals([a], [])).toThrow(FhevmHandleError);
    expect(() => assertHandleArrayEquals([], [a])).toThrow(FhevmHandleError);
  });

  it('assertHandleArrayEquals throws on content mismatch', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 });
    const b = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 1 });

    expect(() => assertHandleArrayEquals([a], [b])).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertHandlesBelongToSameChainId
  //////////////////////////////////////////////////////////////////////////////

  it('assertHandlesBelongToSameChainId passes for an empty array', () => {
    expect(() => assertHandlesBelongToSameChainId([])).not.toThrow();
  });

  it('assertHandlesBelongToSameChainId passes when all handles share a chainId', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 });
    const b = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 1 });

    expect(() => assertHandlesBelongToSameChainId([a, b])).not.toThrow();
    expect(() => assertHandlesBelongToSameChainId([a, b], asUint64BigInt(CHAIN_ID))).not.toThrow();
  });

  it('assertHandlesBelongToSameChainId throws when a handle has a different chainId', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8, index: 0 });
    const otherChainId = CHAIN_ID + 1n;
    const b = buildHandle({ chainId: otherChainId, hash21: HASH21, fheTypeId: EUINT8, index: 1 });

    expect(() => assertHandlesBelongToSameChainId([a, b])).toThrow(FhevmHandleError);
    expect(() => assertHandlesBelongToSameChainId([a], asUint64BigInt(otherChainId))).toThrow(FhevmHandleError);
  });
});

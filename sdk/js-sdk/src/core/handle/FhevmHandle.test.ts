import type { Handle } from '../types/encryptedTypes-p.js';
import { describe, it, expect } from 'vitest';
import {
  assertIsHandleBytes32Hex,
  assertIsExternalHandleBytes32Hex,
  assertIsHandleBytes32,
  assertIsExternalHandleBytes32,
  assertIsHandle,
  assertIsExternalHandle,
  assertIsEncryptedValueLike,
  isHandleBytes32,
  isHandleBytes32Hex,
  isHandle,
  isExternalHandle,
  asHandle,
  asHandleBytes32,
  asHandleBytes32Hex,
  handleBytes32HexToHandle,
  bytes32HexToHandle,
  bytes32HexToExternalHandle,
  bytes32ToHandle,
  bytes32ToExternalHandle,
  toFhevmHandle,
  toExternalHandle,
  handleEquals,
  assertHandleArrayEquals,
  buildExternalHandle,
  buildInputHandle,
  isInputHandle,
  assertIsInputHandle,
  bytes32HexToInputHandle,
  toInputHandle,
  assertHandlesBelongToSameChainId,
  FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION,
} from './FhevmHandle.js';
import { FhevmHandleError } from '../errors/FhevmHandleError.js';
import { asBytes32Hex, asBytes32, hexToBytes } from '../base/bytes.js';
import { asUint64BigInt } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/handle/FhevmHandle.test.ts
////////////////////////////////////////////////////////////////////////////////

const HASH21 = `0x${'ab'.repeat(21)}`;
const OTHER_HASH21 = `0x${'cd'.repeat(21)}`;
const HASH29 = `0x${'ef'.repeat(29)}`;
const CHAIN_ID = 12345n;
const EUINT8 = 2;
const EADDRESS = 7;

/**
 * Test helper: builds a computed handle
 * hash21 (bytes 0..20) | 0xFF (byte 21) | chainId (bytes 22..29) | fheTypeId (30) | version (31).
 */
function buildHandle({
  chainId,
  hash21,
  fheTypeId,
  version = FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION,
}: {
  chainId: bigint;
  hash21: string;
  fheTypeId: number;
  version?: number;
}): Handle {
  const bytes = new Uint8Array(32);
  bytes.set(hexToBytes(hash21), 0);
  bytes[21] = 0xff;
  new DataView(bytes.buffer).setBigUint64(22, chainId);
  bytes[30] = fheTypeId;
  bytes[31] = version;
  return bytes32ToHandle(asBytes32(bytes));
}

describe('FhevmHandle', () => {
  //////////////////////////////////////////////////////////////////////////////
  // Computed / external handle instance getters
  //////////////////////////////////////////////////////////////////////////////

  it('a computed handle exposes its fields and the 0xFF marker at byte 21', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(handle.hash21.toLowerCase()).toBe(HASH21);
    expect(handle.chainId).toBe(CHAIN_ID);
    expect(handle.fheTypeId).toBe(EUINT8);
    expect(handle.fheType).toBe('euint8');
    expect(handle.clearType).toBe('uint8');
    expect(handle.version).toBe(FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION);
    expect(handle.bytes32[21]).toBe(255);
    expect(isHandle(handle)).toBe(true);
    expect(isExternalHandle(handle)).toBe(false);
    expect(handle.encryptionBits).toBe(8);
    expect(handle.solidityPrimitiveTypeName).toBe('uint256');
    expect(handle.bytes32Hex).toBe(handle.toString());
    expect(JSON.stringify(handle)).toBe(JSON.stringify(handle.bytes32Hex));
    expect(handle.bytes32HexNo0x).toBe(handle.bytes32Hex.slice(2));
    expect(handle.bytes32).toBeInstanceOf(Uint8Array);
    expect(handle.bytes32.length).toBe(32);
  });

  it('buildExternalHandle builds an RFC-030 external handle (hash29 | index | type | version)', () => {
    const handle = buildExternalHandle({ hash29: HASH29, index: 3, fheTypeId: EADDRESS });

    expect(handle.hash29.toLowerCase()).toBe(HASH29);
    expect(handle.index).toBe(3);
    expect(handle.fheTypeId).toBe(EADDRESS);
    expect(handle.fheType).toBe('eaddress');
    expect(handle.clearType).toBe('address');
    expect(handle.version).toBe(FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION);
    expect(handle.encryptionBits).toBe(160);
    expect(handle.bytes32Hex.toLowerCase()).toBe(`${HASH29}03070${FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION}`);
    expect(handle.bytes32HexNo0x).toBe(handle.bytes32Hex.slice(2));
    expect(JSON.stringify(handle)).toBe(JSON.stringify(handle.bytes32Hex));
    expect(handle.bytes32[29]).toBe(3);

    expect(isExternalHandle(handle)).toBe(true);
    expect(isHandle(handle)).toBe(false);
    // No chain id in the external layout.
    expect('chainId' in handle).toBe(false);
  });

  it('buildExternalHandle accepts index 254 and rejects index 255', () => {
    expect(buildExternalHandle({ hash29: HASH29, index: 254, fheTypeId: EUINT8 }).index).toBe(254);
    expect(() => buildExternalHandle({ hash29: HASH29, index: 255, fheTypeId: EUINT8 })).toThrow(FhevmHandleError);
  });

  it('buildExternalHandle rejects a hash29 that is not 29 bytes', () => {
    expect(() => buildExternalHandle({ hash29: HASH21, index: 0, fheTypeId: EUINT8 })).toThrow(FhevmHandleError);
  });

  it('buildInputHandle builds a ZKPoK input handle (hash21 | index | chainId | type | version)', () => {
    const handle = buildInputHandle({ hash21: HASH21, index: 3, chainId: CHAIN_ID, fheTypeId: EADDRESS });

    expect(handle.hash21.toLowerCase()).toBe(HASH21);
    expect(handle.index).toBe(3);
    expect(handle.chainId).toBe(CHAIN_ID);
    expect(handle.fheTypeId).toBe(EADDRESS);
    expect(handle.fheType).toBe('eaddress');
    expect(handle.clearType).toBe('address');
    expect(handle.version).toBe(FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION);
    expect(handle.encryptionBits).toBe(160);
    expect(handle.bytes32Hex.toLowerCase()).toBe(
      `${HASH21}03${CHAIN_ID.toString(16).padStart(16, '0')}070${FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION}`,
    );
    expect(handle.bytes32HexNo0x).toBe(handle.bytes32Hex.slice(2));
    expect(JSON.stringify(handle)).toBe(JSON.stringify(handle.bytes32Hex));

    expect(isInputHandle(handle)).toBe(true);
    expect(isHandle(handle)).toBe(false);
    expect(isExternalHandle(handle)).toBe(false);
    expect(() => assertIsInputHandle(handle)).not.toThrow();
  });

  it('buildInputHandle accepts index 254 and rejects index 255 (computed marker)', () => {
    expect(buildInputHandle({ hash21: HASH21, index: 254, chainId: CHAIN_ID, fheTypeId: EUINT8 }).index).toBe(254);
    expect(() => buildInputHandle({ hash21: HASH21, index: 255, chainId: CHAIN_ID, fheTypeId: EUINT8 })).toThrow(
      FhevmHandleError,
    );
  });

  it('buildInputHandle rejects a hash21 that is not 21 bytes', () => {
    expect(() => buildInputHandle({ hash21: HASH29, index: 0, chainId: CHAIN_ID, fheTypeId: EUINT8 })).toThrow(
      FhevmHandleError,
    );
  });

  it('toInputHandle parses an input handle and rejects a computed handle', () => {
    const input = buildInputHandle({ hash21: HASH21, index: 1, chainId: CHAIN_ID, fheTypeId: EUINT8 });
    expect(toInputHandle(input)).toBe(input);
    expect(toInputHandle(input.bytes32Hex).bytes32Hex).toBe(input.bytes32Hex);
    expect(toInputHandle(input.bytes32).bytes32Hex).toBe(input.bytes32Hex);
    expect(toInputHandle({ bytes32Hex: input.bytes32Hex }).index).toBe(1);

    const computed = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });
    expect(() => toInputHandle(computed.bytes32Hex)).toThrow(FhevmHandleError);
    expect(() => bytes32HexToInputHandle(computed.bytes32Hex)).toThrow(FhevmHandleError);
    expect(() => assertIsInputHandle(computed)).toThrow(FhevmHandleError);
  });

  it('Handle.bytes32 returns a defensive copy each time', () => {
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
  // assertIsExternalHandleBytes32Hex
  //////////////////////////////////////////////////////////////////////////////

  it('assertIsExternalHandleBytes32Hex checks fheTypeId and version only', () => {
    const externalHex = buildExternalHandle({ hash29: HASH29, index: 0, fheTypeId: EUINT8 }).bytes32Hex;

    expect(() => assertIsExternalHandleBytes32Hex(externalHex)).not.toThrow();
    // Byte 21 carries no meaning: an external handle may hold 0xFF there.
    expect(() => assertIsExternalHandleBytes32Hex(asBytes32Hex(`0x${'ff'.repeat(30)}0200`))).not.toThrow();
    expect(() => assertIsExternalHandleBytes32Hex(asBytes32Hex(`0x${'00'.repeat(30)}6300`))).toThrow(FhevmHandleError);
    expect(() => assertIsExternalHandleBytes32Hex(asBytes32Hex(`0x${'00'.repeat(30)}0263`))).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // isHandleBytes32 / assertIsHandleBytes32 / assertIsExternalHandleBytes32
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

  it('assertIsExternalHandleBytes32 checks fheTypeId and version only', () => {
    const externalBytes = buildExternalHandle({ hash29: HASH29, index: 0, fheTypeId: EUINT8 }).bytes32;

    expect(() => assertIsExternalHandleBytes32(externalBytes)).not.toThrow();
    const invalidBytesRaw = new Uint8Array(32);
    invalidBytesRaw[30] = 99; // unknown fheTypeId
    expect(() => assertIsExternalHandleBytes32(asBytes32(invalidBytesRaw))).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // isHandle / assertIsHandle / isExternalHandle / assertIsExternalHandle
  //////////////////////////////////////////////////////////////////////////////

  it('isHandle / assertIsHandle only accept genuine Handle instances', () => {
    const handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(isHandle(handle)).toBe(true);
    expect(() => assertIsHandle(handle)).not.toThrow();

    expect(isHandle({ bytes32Hex: handle.bytes32Hex })).toBe(false);
    expect(isHandle(handle.bytes32Hex)).toBe(false);
    expect(() => assertIsHandle({ bytes32Hex: handle.bytes32Hex })).toThrow(FhevmHandleError);
  });

  it('isExternalHandle / assertIsExternalHandle distinguish external from computed handles', () => {
    const external = buildExternalHandle({ hash29: HASH29, index: 5, fheTypeId: EUINT8 });
    const computed = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(isExternalHandle(external)).toBe(true);
    expect(() => assertIsExternalHandle(external)).not.toThrow();

    expect(isExternalHandle(computed)).toBe(false);
    expect(() => assertIsExternalHandle(computed)).toThrow(FhevmHandleError);

    expect(isExternalHandle('not-a-handle')).toBe(false);
    expect(isExternalHandle({ bytes32Hex: external.bytes32Hex })).toBe(false);
  });

  it('isExternalHandle recognizes instances, not bytes: byte 21 = 0xFF is valid digest data', () => {
    // The same bytes, read as either kind, depending on how they are parsed.
    const hex = asBytes32Hex(`0x${'ff'.repeat(29)}000200`);

    expect(isExternalHandle(bytes32HexToExternalHandle(hex))).toBe(true);
    expect(isExternalHandle(bytes32HexToHandle(hex))).toBe(false);
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
  // handleBytes32HexToHandle / bytes32HexToHandle / bytes32HexToExternalHandle / bytes32ToHandle
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

  it('bytes32HexToExternalHandle / bytes32ToExternalHandle validate fheTypeId/version and build an ExternalHandle', () => {
    const external = buildExternalHandle({ hash29: HASH29, index: 4, fheTypeId: EUINT8 });

    const fromHex = bytes32HexToExternalHandle(external.bytes32Hex);
    expect(isExternalHandle(fromHex)).toBe(true);
    expect(fromHex.index).toBe(4);
    expect(fromHex.fheType).toBe('euint8');

    const fromBytes = bytes32ToExternalHandle(external.bytes32);
    expect(isExternalHandle(fromBytes)).toBe(true);
    expect(fromBytes.bytes32Hex).toBe(external.bytes32Hex);

    expect(() => bytes32HexToExternalHandle(asBytes32Hex(`0x${'00'.repeat(30)}6300`))).toThrow(FhevmHandleError);
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
  // toFhevmHandle / toExternalHandle
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

  it('toExternalHandle accepts an ExternalHandle instance, a {bytes32Hex} object, a hex string, and Bytes32', () => {
    const external = buildExternalHandle({ hash29: HASH29, index: 0, fheTypeId: EUINT8 });

    expect(toExternalHandle(external)).toBe(external);
    expect(toExternalHandle({ bytes32Hex: external.bytes32Hex }).bytes32Hex).toBe(external.bytes32Hex);
    expect(toExternalHandle(external.bytes32Hex).bytes32Hex).toBe(external.bytes32Hex);
    expect(toExternalHandle(external.bytes32).bytes32Hex).toBe(external.bytes32Hex);
    expect(isExternalHandle(toExternalHandle(external.bytes32Hex))).toBe(true);
  });

  it('toExternalHandle throws on invalid input', () => {
    expect(() => toExternalHandle('not-a-hex')).toThrow();
    expect(() => toExternalHandle({ bytes32Hex: 'not-a-hex' })).toThrow();
    expect(() => toExternalHandle(asBytes32Hex(`0x${'00'.repeat(30)}6300`))).toThrow(FhevmHandleError);
    expect(() => toExternalHandle(123)).toThrow();
    expect(() => toExternalHandle(null)).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////
  // handleEquals
  //////////////////////////////////////////////////////////////////////////////

  it('handleEquals compares handles by bytes32Hex', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });
    const b = bytes32HexToHandle(a.bytes32Hex);
    const c = buildHandle({ chainId: CHAIN_ID, hash21: OTHER_HASH21, fheTypeId: EUINT8 });

    expect(handleEquals(a, b)).toBe(true);
    expect(handleEquals(a, c)).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertHandleArrayEquals
  //////////////////////////////////////////////////////////////////////////////

  it('assertHandleArrayEquals passes for matching arrays', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });
    const b = buildHandle({ chainId: CHAIN_ID, hash21: OTHER_HASH21, fheTypeId: EUINT8 });

    expect(() => assertHandleArrayEquals([a, b], [a, b])).not.toThrow();
    // `expected` items are converted via toFhevmHandle, so raw hex strings work too
    expect(() => assertHandleArrayEquals([a, b], [a.bytes32Hex, b.bytes32Hex] as never)).not.toThrow();
  });

  it('assertHandleArrayEquals throws on length mismatch', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });

    expect(() => assertHandleArrayEquals([a], [])).toThrow(FhevmHandleError);
    expect(() => assertHandleArrayEquals([], [a])).toThrow(FhevmHandleError);
  });

  it('assertHandleArrayEquals throws on content mismatch', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });
    const b = buildHandle({ chainId: CHAIN_ID, hash21: OTHER_HASH21, fheTypeId: EUINT8 });

    expect(() => assertHandleArrayEquals([a], [b])).toThrow(FhevmHandleError);
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertHandlesBelongToSameChainId
  //////////////////////////////////////////////////////////////////////////////

  it('assertHandlesBelongToSameChainId passes for an empty array', () => {
    expect(() => assertHandlesBelongToSameChainId([])).not.toThrow();
  });

  it('assertHandlesBelongToSameChainId passes when all handles share a chainId', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });
    const b = buildHandle({ chainId: CHAIN_ID, hash21: OTHER_HASH21, fheTypeId: EUINT8 });

    expect(() => assertHandlesBelongToSameChainId([a, b])).not.toThrow();
    expect(() => assertHandlesBelongToSameChainId([a, b], asUint64BigInt(CHAIN_ID))).not.toThrow();
  });

  it('assertHandlesBelongToSameChainId throws when a handle has a different chainId', () => {
    const a = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: EUINT8 });
    const otherChainId = CHAIN_ID + 1n;
    const b = buildHandle({ chainId: otherChainId, hash21: OTHER_HASH21, fheTypeId: EUINT8 });

    expect(() => assertHandlesBelongToSameChainId([a, b])).toThrow(FhevmHandleError);
    expect(() => assertHandlesBelongToSameChainId([a], asUint64BigInt(otherChainId))).toThrow(FhevmHandleError);
  });
});

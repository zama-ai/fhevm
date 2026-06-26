import type { Bytes32Hex, BytesHex, Uint256BigInt, Uint8Number } from '../types/primitives.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import { asUint256BigInt, asUint8Number } from '../base/uint.js';
import { asBytes32Hex, assertIsBytesHex } from '../base/bytes.js';

export const EXTRA_DATA_V0: Uint8Number = 0x00 as Uint8Number; // 0x00
export const EXTRA_DATA_V1: Uint8Number = 0x01 as Uint8Number; // 1 version byte + 32-byte big-endian context ID = 33 bytes
export const EXTRA_DATA_V2: Uint8Number = 0x02 as Uint8Number; // 1 version byte + 32-byte big-endian context ID + 32-byte big-endian epoch ID = 65 bytes

export function fromKmsExtraData(extraData: BytesHex): {
  version: Uint8Number;
  kmsContextId: Uint256BigInt;
  kmsEpochId: Uint256BigInt;
} {
  if ((extraData as string) === '0x00' || (extraData as string) === '0x') {
    return {
      version: EXTRA_DATA_V0,
      kmsContextId: 0n as Uint256BigInt,
      kmsEpochId: 0n as Uint256BigInt,
    };
  }

  if (extraData.length <= 4) {
    throw new Error(`Unsupported extraData length ${extraData.length}: must be more than 4 bytes`);
  }

  // First byte = version (characters 2-3 after '0x')
  const version = asUint8Number(Number('0x' + extraData.slice(2, 4)));

  if (version === EXTRA_DATA_V1) {
    // ExtraData v1 format: 1 version byte + 32-byte big-endian context ID = 33 bytes = 66 hex chars + 2 for '0x' = 68
    // 0x01aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    // ↑   ↑                                                               ↑
    // 0   4                                                               68
    // 0         1         2         3         4         5         6
    // 01234567890123456789012345678901234567890123456789012345678901234567

    if (extraData.length != 68) {
      throw new Error(`Invalid extraData length for v1: expected 68, got ${extraData.length}`);
    }
    // 32-byte contextId starts at byte 1 (hex chars 4..67)
    const contextIdBytes32Hex: Bytes32Hex = asBytes32Hex(`0x${extraData.slice(4, 68)}`);
    const kmsContextId = asUint256BigInt(BigInt(contextIdBytes32Hex));
    return { version, kmsContextId, kmsEpochId: 0n as Uint256BigInt };
  }

  if (version === EXTRA_DATA_V2) {
    // ExtraData v2 format: 1 version byte + 32-byte big-endian context ID + 32-byte big-endian epoch ID = 65 bytes = 130 hex chars + 2 for '0x' = 132
    // 0x01aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
    // ↑   ↑                                                               ↑                                                               ↑
    // 0   4                                                               68                                                              132
    // 0         1         2         3         4         5         6         7         8         9         0         1         2         3
    // 012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901

    if (extraData.length != 132) {
      throw new Error(`Invalid extraData length for v2: expected 132, got ${extraData.length}`);
    }
    // 32-byte contextId starts at byte 1 (hex chars 4..67)
    const contextIdBytes32Hex: Bytes32Hex = asBytes32Hex(`0x${extraData.slice(4, 68)}`);
    const kmsContextId = asUint256BigInt(BigInt(contextIdBytes32Hex));

    // 32-byte epochId starts at byte 33 (hex chars 68..131)
    const epochIdBytes32Hex: Bytes32Hex = asBytes32Hex(`0x${extraData.slice(68, 132)}`);
    const kmsEpochId = asUint256BigInt(BigInt(epochIdBytes32Hex));

    return { version, kmsContextId, kmsEpochId };
  }

  throw new Error(`Unsupported extraData version ${version}`);
}

/**
 * Encodes a KMS context into an extraData format.
 * v0: `0x00`
 * v1: `0x01` + 32-byte big-endian context ID.
 * v2: `0x02` + 32-byte big-endian context ID + 32-byte big-endian epoch ID.
 */
export function toKmsExtraData(args: {
  readonly version: Uint8Number;
  readonly kmsContextId: Uint256BigInt;
  readonly kmsEpochId: Uint256BigInt;
}): BytesHex {
  const v = args.version.toString(16).padStart(2, '0');

  switch (args.version) {
    case EXTRA_DATA_V0: {
      return `0x${v}` as BytesHex;
    }
    case EXTRA_DATA_V1: {
      if (args.kmsContextId === 0n) {
        throw new Error('kmsContextId cannot be 0 for v1 extraData');
      }
      if (args.kmsEpochId !== 0n) {
        throw new Error('kmsEpochId must be 0 for v1 extraData');
      }
      const contextId = args.kmsContextId.toString(16).padStart(64, '0');
      return `0x${v}${contextId}` as BytesHex;
    }
    case EXTRA_DATA_V2: {
      if (args.kmsContextId === 0n) {
        throw new Error('kmsContextId cannot be 0 for v2 extraData');
      }
      if (args.kmsEpochId === 0n) {
        throw new Error('kmsEpochId cannot be 0 for v2 extraData');
      }
      const contextId = args.kmsContextId.toString(16).padStart(64, '0');
      const epochId = args.kmsEpochId.toString(16).padStart(64, '0');
      return `0x${v}${contextId}${epochId}` as BytesHex;
    }
    default:
      throw new Error(`Unsupported extraData version ${args.version}`);
  }
}

export function assertIsKmsExtraData(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is BytesHex {
  if (value === '0x00' || value === '0x') {
    return;
  }

  assertIsBytesHex(value, options);

  // Will valid extraData length too
  fromKmsExtraData(value);
}

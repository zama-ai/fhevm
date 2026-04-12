import { asUint256BigInt, asUint8Number } from '../base/uint.js';
import type {
  ByteLength,
  Bytes32Hex,
  BytesHex,
  Uint256BigInt,
  Uint8Number,
} from '../types/primitives.js';
import { asBytes32Hex, assertIsBytesHex } from '../base/bytes.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';

const EXTRA_DATA_V1 = 0x01;

export function fromKmsExtraData(extraData: BytesHex): {
  version: Uint8Number;
  kmsContextId: Uint256BigInt;
} {
  // First byte = version (characters 2-3 after '0x')
  const version = asUint8Number(parseInt(extraData.slice(2, 4), 16));

  if (version === EXTRA_DATA_V1) {
    // 1 version byte + 32 contextId bytes = 33 bytes = 66 hex chars + 2 for '0x' = 68
    if (extraData.length < 68) {
      throw new Error(
        `extraData too short for v1: expected at least 33 bytes, got ${(extraData.length - 2) / 2}`,
      );
    }
    // 32-byte contextId starts at byte 1 (hex chars 4..67)
    const contextIdBytes32Hex: Bytes32Hex = asBytes32Hex(
      `0x${extraData.slice(4, 68)}`,
    );
    const kmsContextId = asUint256BigInt(BigInt(contextIdBytes32Hex));
    return { version, kmsContextId };
  }

  throw new Error(`Unsupported extraData version ${version}`);
}

/**
 * Encodes a KMS context ID into the v1 extraData format.
 * Layout: `0x` + `01` (version byte) + 32-byte big-endian context ID.
 */
export function toKmsExtraData(args: {
  readonly version: Uint8Number;
  readonly kmsContextId: Uint256BigInt;
}): BytesHex {
  const v = args.version.toString(16).padStart(2, '0');
  const id = args.kmsContextId.toString(16).padStart(64, '0');
  return `0x${v}${id}` as BytesHex;
}

export function assertIsKmsExtraData(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is BytesHex {
  if (value === '0x00') {
    return;
  }
  assertIsBytesHex(value, { ...options, byteLength: 33 as ByteLength });
  fromKmsExtraData(value);
}

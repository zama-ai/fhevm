import type {
  Address,
  Bytes20,
  Bytes20HexNo0x,
  ChecksummedAddress,
} from "../types/primitives.js";
import type {
  RecordAddressPropertyType,
  RecordChecksummedAddressArrayPropertyType,
  RecordChecksummedAddressPropertyType,
} from "../types/record-p.js";
import type { ErrorMetadataParams } from "./errors/ErrorBase.js";
import { keccak_256 } from "@noble/hashes/sha3.js";
import {
  assertRecordArrayProperty,
  isRecordNonNullableProperty,
  typeofProperty,
} from "./record.js";
import { remove0x } from "./string.js";
import { AddressError } from "./errors/AddressError.js";
import { ChecksummedAddressError } from "./errors/ChecksummedAddressError.js";
import { InvalidTypeError } from "./errors/InvalidTypeError.js";
import { bytesToHex, isBytes20Hex } from "./bytes.js";
import { InvalidPropertyError } from "./errors/InvalidPropertyError.js";

////////////////////////////////////////////////////////////////////////////////

export const ZERO_ADDRESS =
  "0x0000000000000000000000000000000000000000" as ChecksummedAddress;

////////////////////////////////////////////////////////////////////////////////

/**
 * Converts a checksummed Ethereum address to its raw 20-byte representation.
 *
 * @param address - A valid EIP-55 checksummed address
 * @returns The 20-byte Uint8Array representation
 */
export function checksummedAddressToBytes20(
  address: ChecksummedAddress,
): Bytes20 {
  const hex = remove0x(address);
  const bytes = new Uint8Array(20);
  for (let i = 0; i < 20; i++) {
    bytes[i] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
  }

  return bytes as Bytes20;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Type guard that checks if a value is a valid EIP-55 checksummed Ethereum address.
 *
 * @param value - The value to check
 * @returns True if the value is a valid checksummed address
 */
export function isChecksummedAddress(
  value: unknown,
): value is ChecksummedAddress {
  try {
    const a = toChecksummedAddress(value);
    return a === value;
  } catch {
    return false;
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsChecksummedAddress(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is ChecksummedAddress {
  if (!isChecksummedAddress(value)) {
    if (typeof value === "string") {
      throw new ChecksummedAddressError(
        {
          address: value,
        },
        options,
      );
    } else {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          type: typeof value,
          expectedType: "checksummedAddress",
        },
        options,
      );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

export function asChecksummedAddress(
  value: unknown,
  options?: ErrorMetadataParams,
): ChecksummedAddress {
  assertIsChecksummedAddress(value, options ?? {});
  return value;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsChecksummedAddressArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is ChecksummedAddress[] {
  if (!Array.isArray(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: "checksummedAddress[]",
      },
      options,
    );
  }
  for (let i = 0; i < value.length; ++i) {
    if (!isChecksummedAddress(value[i])) {
      throw new ChecksummedAddressError({ address: String(value) }, options);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

export function isAddress(value: unknown): value is Address {
  if (!isBytes20Hex(value)) {
    return false;
  }

  const hexNo0x = remove0x(value) as Bytes20HexNo0x;
  const hexNo0xLowerCase = hexNo0x.toLowerCase();

  if (hexNo0x === hexNo0xLowerCase) {
    return true;
  }

  // Has uppercase letters - must match EIP-55 checksum exactly
  return _toChecksummedAddress(hexNo0xLowerCase) === value;
}

////////////////////////////////////////////////////////////////////////////////

export function asAddress(
  value: unknown,
  options?: ErrorMetadataParams,
): Address {
  assertIsAddress(value, options ?? {});
  return value;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsAddress(
  value: unknown,
  options: ErrorMetadataParams,
): asserts value is Address {
  if (!isAddress(value)) {
    throw new AddressError({ address: String(value) }, options);
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsAddressArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Address[] {
  if (!Array.isArray(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: "address[]",
      },
      options,
    );
  }
  for (let i = 0; i < value.length; ++i) {
    if (!isAddress(value[i])) {
      throw new AddressError({ address: String(value) }, options);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

export function isRecordAddressProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordAddressPropertyType<K> {
  if (!isRecordNonNullableProperty(record, property)) {
    return false;
  }
  return isAddress(record[property]);
}

////////////////////////////////////////////////////////////////////////////////

export function assertRecordAddressProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordAddressPropertyType<K> {
  if (!isRecordAddressProperty(record, property)) {
    const type = typeofProperty(record, property);
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        type,
        expectedType: "address",
        value:
          type === "string"
            ? String((record as Record<string, unknown>)[property])
            : undefined,
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function isRecordChecksummedAddressProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordChecksummedAddressPropertyType<K> {
  if (!isRecordNonNullableProperty(record, property)) {
    return false;
  }
  return isChecksummedAddress(record[property]);
}

////////////////////////////////////////////////////////////////////////////////

export function assertRecordChecksummedAddressProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordChecksummedAddressPropertyType<K> {
  if (!isRecordChecksummedAddressProperty(record, property)) {
    const type = typeofProperty(record, property);
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        type,
        expectedType: "checksummedAddress",
        value:
          type === "string"
            ? String((record as Record<string, unknown>)[property])
            : undefined,
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertRecordChecksummedAddressArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordChecksummedAddressArrayPropertyType<K> {
  assertRecordArrayProperty(record, property, recordName, options);
  const arr = record[property];
  for (let i = 0; i < arr.length; ++i) {
    assertIsChecksummedAddress(arr[i], options);
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Converts an Ethereum address to its EIP-55 checksummed format.
 *
 * @param value - The 0x-prefixed address to checksum
 * @returns The checksummed address, or undefined if invalid
 */
export function toChecksummedAddress(
  value: unknown,
): ChecksummedAddress | undefined {
  if (!isBytes20Hex(value)) {
    return undefined;
  }

  return _toChecksummedAddress(remove0x(value).toLowerCase());
}

/**
 * Converts an Ethereum address to its EIP-55 checksummed format.
 *
 * @param address - The 0x-prefixed address to checksum
 * @returns The checksummed address, or undefined if invalid
 */
export function addressToChecksummedAddress(
  address: Address,
): ChecksummedAddress {
  return _toChecksummedAddress(remove0x(address).toLowerCase());
}

////////////////////////////////////////////////////////////////////////////////

export function _toChecksummedAddress(
  bytes20No0xLowerCase: string,
): ChecksummedAddress {
  // Hash the lowercase hex string as UTF-8 bytes (EIP-55)
  const hash = bytesToHex(
    keccak_256(new TextEncoder().encode(bytes20No0xLowerCase)),
  );

  // Apply checksum: uppercase if hash nibble >= 8
  let checksummed = "0x";
  for (let i = 0; i < 40; i++) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const char = bytes20No0xLowerCase[i]!;
    // Check if it's a letter (a-f)
    if (char >= "a" && char <= "f") {
      // Get corresponding nibble from hash (hash has 0x prefix, so offset by 2)
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const hashNibble = parseInt(hash[i + 2]!, 16);
      checksummed += hashNibble >= 8 ? char.toUpperCase() : char;
    } else {
      checksummed += char;
    }
  }

  return checksummed as ChecksummedAddress;
}

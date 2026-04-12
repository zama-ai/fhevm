import type {
  RecordStringArrayPropertyType,
  RecordStringPropertyType,
} from '../types/record-p.js';
import type { BytesHex, BytesHexNo0x } from '../types/primitives.js';
import type { ErrorMetadataParams } from './errors/ErrorBase.js';
import {
  assertRecordArrayProperty,
  isRecordNonNullableProperty,
  typeofProperty,
} from './record.js';
import { InternalError } from './errors/InternalError.js';
import { InvalidPropertyError } from './errors/InvalidPropertyError.js';

export function removeSuffix(s: string | undefined, suffix: string): string {
  if (s === undefined) {
    return '';
  }
  if (suffix.length === 0) {
    return s;
  }
  return s.endsWith(suffix) ? s.slice(0, -suffix.length) : s;
}

export function is0x(s: unknown): s is `0x${string}` {
  return typeof s === 'string' && s.startsWith('0x');
}

export function isNo0x(s: unknown): s is string {
  return typeof s === 'string' && !s.startsWith('0x');
}

/**
 * Prepends `0x` prefix. When the input is a branded `BytesHexNo0x` (or a sized
 * variant like `Bytes65HexNo0x`), the return type preserves the size brand via
 * generic inference — no per-size overload needed.
 */
export function ensure0x<T extends BytesHexNo0x>(
  s: T,
): BytesHex & Omit<T, keyof BytesHexNo0x>;
export function ensure0x(s: string): `0x${string}`;
export function ensure0x(s: string): `0x${string}` {
  return !s.startsWith('0x') ? `0x${s}` : (s as `0x${string}`);
}

/**
 * Strips `0x` prefix. When the input is a branded `BytesHex` (or a sized
 * variant like `Bytes65Hex`), the return type preserves the size brand via
 * generic inference — no per-size overload needed.
 */
export function remove0x<T extends BytesHex>(
  s: T,
): BytesHexNo0x & Omit<T, keyof BytesHex>;
export function remove0x(s: string): string;
export function remove0x(s: string): string {
  return s.startsWith('0x') ? s.substring(2) : s;
}

export function assertIs0xString(s: unknown): asserts s is `0x${string}` {
  if (!(typeof s === 'string' && s.startsWith('0x'))) {
    throw new InternalError({ message: 'value is not a `0x${string}`' });
  }
}

export function isNonEmptyString(s: unknown): s is string {
  if (s === undefined || s === null || typeof s !== 'string') {
    return false;
  }
  return s.length > 0;
}

export function assertIsNonEmptyString(s: unknown): asserts s is string {
  if (!isNonEmptyString(s)) {
    throw new InternalError({
      message: `Expected a non-empty string, got ${typeof s === 'string' ? 'an empty string' : typeof s}`,
    });
  }
}

/**
 * Type guard that checks if a property exists on an object and is a string.
 *
 * @template K - The property key type (string literal)
 * @param o - The value to check (can be any type)
 * @param property - The property name to check for
 * @returns True if `o` is an object with the specified property that is a non-null string
 *
 * @example
 * ```typescript
 * const data: unknown = { status: "active", count: 42 };
 * if (isRecordStringProperty(data, 'status')) {
 *   console.log(data.status.toUpperCase()); // OK
 * }
 * ```
 */
export function isRecordStringProperty<K extends string>(
  o: unknown,
  property: K,
): o is RecordStringPropertyType<K> {
  if (!isRecordNonNullableProperty(o, property)) {
    return false;
  }
  return typeof o[property] === 'string';
}

/**
 * Assertion function that validates a property exists on an object, is a string,
 * and optionally matches specific expected value(s).
 * Throws an `InvalidPropertyError` if validation fails.
 *
 * @template K - The property key type (string literal)
 * @param record - The value to validate (can be any type)
 * @param property - The property name to check for
 * @param recordName - The name of the object being validated (used in error messages)
 * @param expectedValue - Optional specific string value or array of allowed values to match against
 * @throws {InvalidPropertyError} When the property is missing, not a string, or doesn't match expectedValue
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * // Check property is a string (any value)
 * assertRecordStringProperty(data, 'name', 'user');
 *
 * // Check property equals a specific value
 * assertRecordStringProperty(data, 'status', 'response', 'active');
 *
 * // Check property is one of multiple allowed values
 * assertRecordStringProperty(data, 'status', 'response', ['queued', 'processing', 'completed']);
 * ```
 */
export function assertRecordStringProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: { expectedValue?: string | string[] } & ErrorMetadataParams,
): asserts record is RecordStringPropertyType<K> {
  if (!isRecordStringProperty(record, property)) {
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        expectedType: 'string',
        expectedValue: options.expectedValue,
        type: typeofProperty(record, property),
      },
      options,
    );
  }

  if (options.expectedValue !== undefined) {
    if (Array.isArray(options.expectedValue)) {
      // Check if value matches any of the allowed values
      for (let i = 0; i < options.expectedValue.length; ++i) {
        if (record[property] === options.expectedValue[i]) {
          return;
        }
      }

      throw new InvalidPropertyError(
        {
          subject: recordName,
          property,
          expectedType: 'string',
          expectedValue: options.expectedValue,
          type: typeof record[property], // === "string"
          value: record[property],
        },
        options,
      );
    } else {
      if (record[property] !== options.expectedValue) {
        throw new InvalidPropertyError(
          {
            subject: recordName,
            property,
            expectedType: 'string',
            expectedValue: options.expectedValue,
            type: typeof record[property], // === "string"
            value: record[property],
          },
          options,
        );
      }
    }
  }
}

export function assertRecordStringArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordStringArrayPropertyType<K> {
  assertRecordArrayProperty(record, property, recordName, options);
  const arr = record[property];
  for (let i = 0; i < arr.length; ++i) {
    if (typeof arr[i] !== 'string') {
      throw new InvalidPropertyError(
        {
          subject: recordName,
          index: i,
          property,
          expectedType: 'string',
          type: typeof arr[i],
        },
        options,
      );
    }
  }
}

/**
 * Capitalizes the first letter of a string.
 */
export function capitalizeFirstLetter(s: string): string {
  if (s.length === 0) {
    return s;
  }
  return s.charAt(0).toUpperCase() + s.slice(1);
}

export function safeJSONstringify(o: unknown, space?: string | number): string {
  try {
    return JSON.stringify(
      o,
      (_, v: unknown) => (typeof v === 'bigint' ? v.toString() : v),
      space,
    );
  } catch {
    return '';
  }
}

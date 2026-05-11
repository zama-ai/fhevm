import type { EncryptedValue } from '../types/encryptedTypes.js';
import { assertIsNonEmptyString } from '../base/string.js';
import { isHandle, toFhevmHandle } from './FhevmHandle.js';

export function assertIsEncryptedValue(value: unknown): asserts value is EncryptedValue {
  assertIsNonEmptyString(value);
  toFhevmHandle(value);
}

export function isEncryptedValue(value: unknown): value is EncryptedValue {
  try {
    assertIsEncryptedValue(value);
    return true;
  } catch {
    return false;
  }
}

export function asEncryptedValue(value: unknown): EncryptedValue {
  if (isHandle(value)) {
    return value.bytes32Hex as unknown as EncryptedValue;
  }
  assertIsEncryptedValue(value);
  return value;
}

export function toEncryptedValue(value: unknown): EncryptedValue {
  return toFhevmHandle(value).bytes32Hex as unknown as EncryptedValue;
}

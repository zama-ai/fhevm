import { assertRecordChecksummedAddressProperty } from '../base/address.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import { assertRecordStringProperty } from '../base/string.js';
import { assertRecordUintBigIntProperty } from '../base/uint.js';
import type { CoprocessorEIP712Domain } from '../types/coprocessor.js';

export function assertIsCoprocessorEIP712Domain(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is CoprocessorEIP712Domain {
  type T = CoprocessorEIP712Domain;
  assertRecordStringProperty(value, 'name' satisfies keyof T, name, {
    expectedValue: 'InputVerification' satisfies T['name'],
    ...options,
  });
  assertRecordStringProperty(value, 'version' satisfies keyof T, name, {
    expectedValue: '1' satisfies T['version'],
    ...options,
  });
  assertRecordUintBigIntProperty(
    value,
    'chainId' satisfies keyof T,
    name,
    options,
  );
  assertRecordChecksummedAddressProperty(
    value,
    'verifyingContract' satisfies keyof T,
    name,
    options,
  );
}

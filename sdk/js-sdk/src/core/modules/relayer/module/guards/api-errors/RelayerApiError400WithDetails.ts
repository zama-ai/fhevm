import type { ErrorMetadataParams } from '../../../../../base/errors/ErrorBase.js';
import type { RelayerApiError400WithDetails } from '../../../../../types/relayer-p.js';
import { InvalidPropertyError } from '../../../../../base/errors/InvalidPropertyError.js';
import {
  assertRecordArrayProperty,
  isRecordArrayProperty,
} from '../../../../../base/record.js';
import {
  assertRecordStringProperty,
  isRecordStringProperty,
} from '../../../../../base/string.js';

/** @see {@link assertIsRelayerApiError400WithDetails} */
export function isRelayerApiError400WithDetails(
  error: unknown,
): error is RelayerApiError400WithDetails {
  type T = RelayerApiError400WithDetails;
  type DetailItem = T['details'][number];
  if (!isRecordStringProperty(error, 'label' satisfies keyof T)) {
    return false;
  }
  if (
    !(
      error.label === ('missing_fields' satisfies T['label']) ||
      error.label === ('validation_failed' satisfies T['label'])
    )
  ) {
    return false;
  }
  if (!isRecordStringProperty(error, 'message' satisfies keyof T)) {
    return false;
  }
  if (!isRecordArrayProperty(error, 'details' satisfies keyof T)) {
    return false;
  }
  const arr = error.details;
  for (let i = 0; i < arr.length; ++i) {
    const detail = arr[i];
    if (!isRecordStringProperty(detail, 'field' satisfies keyof DetailItem)) {
      return false;
    }
    if (!isRecordStringProperty(detail, 'issue' satisfies keyof DetailItem)) {
      return false;
    }
  }
  return true;
}

/**
 * Asserts that a value matches the {@link RelayerApiError400WithDetails} schema:
 * ```json
 * {
 *   "label": "missing_fields" | "validation_failed",
 *   "message": "string",
 *   "details": [
 *     { "field": "string", "issue": "string" }
 *   ]
 * }
 * ```
 */
export function assertIsRelayerApiError400WithDetails(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerApiError400WithDetails {
  type T = RelayerApiError400WithDetails;
  type DetailItem = T['details'][number];
  assertRecordStringProperty(value, 'label' satisfies keyof T, name, options);
  if (
    !(
      value.label === ('missing_fields' satisfies T['label']) ||
      value.label === ('validation_failed' satisfies T['label'])
    )
  ) {
    throw new InvalidPropertyError(
      {
        subject: name,
        property: 'label' satisfies keyof T,
        expectedType: 'string',
        expectedValue: [
          'missing_fields' satisfies T['label'],
          'validation_failed' satisfies T['label'],
        ],
        type: typeof value.label,
        value: value.label,
      },
      options,
    );
  }
  assertRecordStringProperty(value, 'message' satisfies keyof T, name, options);
  assertRecordArrayProperty(value, 'details' satisfies keyof T, name, options);
  const arr = value.details;
  for (let i = 0; i < arr.length; ++i) {
    const detail = arr[i];
    assertRecordStringProperty(
      detail,
      'field' satisfies keyof DetailItem,
      `${name}.details[${i}]`,
      options,
    );
    assertRecordStringProperty(
      detail,
      'issue' satisfies keyof DetailItem,
      `${name}.details[${i}]`,
      options,
    );
  }
}

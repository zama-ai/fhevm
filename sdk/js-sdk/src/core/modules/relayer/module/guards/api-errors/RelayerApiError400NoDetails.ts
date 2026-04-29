import type { ErrorMetadataParams } from '../../../../../base/errors/ErrorBase.js';
import type { RelayerApiError400NoDetails } from '../../../../../types/relayer-p.js';
import { isRecordStringProperty } from '../../../../../base/string.js';
import { assertRecordStringProperty } from '../../../../../base/string.js';
import { InvalidPropertyError } from '../../../../../base/errors/InvalidPropertyError.js';

/** @see {@link assertIsRelayerApiError400NoDetails} */
export function isRelayerApiError400NoDetails(error: unknown): error is RelayerApiError400NoDetails {
  type T = RelayerApiError400NoDetails;
  if (!isRecordStringProperty(error, 'label' satisfies keyof T)) {
    return false;
  }
  if (
    !(
      error.label === ('malformed_json' satisfies T['label']) ||
      error.label === ('request_error' satisfies T['label']) ||
      error.label === ('not_ready_for_decryption' satisfies T['label'])
    )
  ) {
    return false;
  }
  return isRecordStringProperty(error, 'message' satisfies keyof T);
}

/**
 * Asserts that a value matches the {@link RelayerApiError400NoDetails} schema:
 * ```json
 * {
 *   "label": "malformed_json" | "request_error" | "not_ready_for_decryption",
 *   "message": "string"
 * }
 * ```
 */
export function assertIsRelayerApiError400NoDetails(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerApiError400NoDetails {
  type T = RelayerApiError400NoDetails;
  assertRecordStringProperty(value, 'label' satisfies keyof T, name, options);
  if (
    !(
      value.label === ('malformed_json' satisfies T['label']) ||
      value.label === ('request_error' satisfies T['label']) ||
      value.label === ('not_ready_for_decryption' satisfies T['label'])
    )
  ) {
    throw new InvalidPropertyError(
      {
        subject: name,
        property: 'label' satisfies keyof T,
        expectedType: 'string',
        expectedValue: [
          'malformed_json' satisfies T['label'],
          'request_error' satisfies T['label'],
          'not_ready_for_decryption' satisfies T['label'],
        ],
        type: typeof value.label, // === "string"
        value: value.label,
      },
      options,
    );
  }
  assertRecordStringProperty(value, 'message' satisfies keyof T, name, options);
}

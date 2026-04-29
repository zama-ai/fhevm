import type { ErrorMetadataParams } from '../../../../../base/errors/ErrorBase.js';
import type { RelayerApiError429 } from '../../../../../types/relayer-p.js';
import { assertRecordStringProperty } from '../../../../../base/string.js';

/**
 * Asserts that `error` matches the {@link RelayerApiError429} schema:
 * ```json
 * {
 *   "label": "rate_limited" | "protocol_overload",
 *   "message": "string"
 * }
 * ```
 */
export function assertIsRelayerApiError429(
  error: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts error is RelayerApiError429 {
  type T = RelayerApiError429;
  assertRecordStringProperty(error, 'label' satisfies keyof T, name, {
    expectedValue: ['rate_limited' satisfies T['label'], 'protocol_overload' satisfies T['label']],
    ...options,
  });
  assertRecordStringProperty(error, 'message' satisfies keyof T, name, options);
}

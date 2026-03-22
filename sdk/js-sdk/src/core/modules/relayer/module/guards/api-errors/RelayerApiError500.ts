import type { ErrorMetadataParams } from "../../../../../base/errors/ErrorBase.js";
import type { RelayerApiError500 } from "../../../../../types/relayer-p.js";
import { assertRecordStringProperty } from "../../../../../base/string.js";

/**
 * Asserts that `value` matches the {@link RelayerApiError500} schema:
 * ```json
 * {
 *   "label": "internal_server_error",
 *   "message": "string"
 * }
 * ```
 */
export function assertIsRelayerApiError500(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerApiError500 {
  type T = RelayerApiError500;
  assertRecordStringProperty(value, "label" satisfies keyof T, name, {
    expectedValue: "internal_server_error" satisfies T["label"],
    ...options,
  });
  assertRecordStringProperty(value, "message" satisfies keyof T, name, options);
}

import type { ErrorMetadataParams } from "../../../../../base/errors/ErrorBase.js";
import type { RelayerApiError404 } from "../../../../../types/relayer-p.js";
import { assertRecordStringProperty } from "../../../../../base/string.js";

/**
 * Asserts that `value` matches the {@link RelayerApiError404} schema:
 * ```json
 * {
 *   "label": "not_found",
 *   "message": "string"
 * }
 * ```
 */
export function assertIsRelayerApiError404(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerApiError404 {
  type T = RelayerApiError404;
  assertRecordStringProperty(value, "label" satisfies keyof T, name, {
    expectedValue: "not_found" satisfies T["label"],
    ...options,
  });
  assertRecordStringProperty(value, "message" satisfies keyof T, name, options);
}

import type { ErrorMetadataParams } from "../../../../../base/errors/ErrorBase.js";
import type { RelayerApiError503 } from "../../../../../types/relayer-p.js";
import { assertRecordStringProperty } from "../../../../../base/string.js";

/**
 * Asserts that `value` matches the {@link RelayerApiError503} schema:
 * ```json
 * {
 *   "label": "protocol_paused" | "gateway_not_reachable" | "readiness_check_timed_out" | "response_timed_out",
 *   "message": "string"
 * }
 * ```
 */
export function assertIsRelayerApiError503(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerApiError503 {
  type T = RelayerApiError503;
  assertRecordStringProperty(value, "label" satisfies keyof T, name, {
    expectedValue: [
      "protocol_paused" satisfies T["label"],
      "gateway_not_reachable" satisfies T["label"],
      "readiness_check_timed_out" satisfies T["label"],
      "response_timed_out" satisfies T["label"],
    ],
    ...options,
  });
  assertRecordStringProperty(value, "message" satisfies keyof T, name, options);
}

import type {
  RelayerPublicDecryptSucceeded,
  RelayerResult200PublicDecrypt,
} from '../../../../types/relayer-p.js';
import type { ErrorMetadataParams } from '../../../../base/errors/ErrorBase.js';
import { assertRecordNonNullableProperty } from '../../../../base/record.js';
import { assertRecordStringProperty } from '../../../../base/string.js';
import {
  assertRecordBytesHexNo0xArrayProperty,
  assertRecordBytesHexNo0xProperty,
  assertRecordBytesHexProperty,
} from '../../../../base/bytes.js';

/**
 * Asserts that `value` matches the {@link RelayerPublicDecryptSucceeded} schema:
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "string",
 *   "result": {
 *     "signatures": ["hexNo0x..."],
 *     "decryptedValue": "hexNo0x...",
 *     "extraData": "0x..."
 *   }
 * }
 * ```
 */
export function assertIsRelayerPublicDecryptSucceeded(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerPublicDecryptSucceeded {
  type T = RelayerPublicDecryptSucceeded;
  assertRecordStringProperty(value, 'status' satisfies keyof T, name, {
    expectedValue: 'succeeded' satisfies T['status'],
    ...options,
  });
  assertRecordStringProperty(
    value,
    'requestId' satisfies keyof T,
    name,
    options,
  );
  assertRecordNonNullableProperty(
    value,
    'result' satisfies keyof T,
    name,
    options,
  );
  _assertIsRelayerResult200PublicDecrypt(
    value.result,
    `${name}.result`,
    options,
  );
}

/**
 * Asserts that `value` matches the {@link RelayerResult200PublicDecrypt} schema:
 * ```json
 * {
 *   "signatures": ["hexNo0x..."],
 *   "decryptedValue": "hexNo0x...",
 *   "extraData": "0x..."
 * }
 * ```
 */
function _assertIsRelayerResult200PublicDecrypt(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerResult200PublicDecrypt {
  type T = RelayerResult200PublicDecrypt;
  assertRecordBytesHexNo0xArrayProperty(
    value,
    'signatures' satisfies keyof T,
    name,
    options,
  );
  assertRecordBytesHexNo0xProperty(
    value,
    'decryptedValue' satisfies keyof T,
    name,
    options,
  );
  assertRecordBytesHexProperty(
    value,
    'extraData' satisfies keyof T,
    name,
    options,
  );
}

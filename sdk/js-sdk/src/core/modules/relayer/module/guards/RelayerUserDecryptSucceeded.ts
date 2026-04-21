import type { ErrorMetadataParams } from '../../../../base/errors/ErrorBase.js';
import type { RelayerResult200UserDecrypt, RelayerUserDecryptSucceeded } from '../../../../types/relayer-p.js';
import { assertRecordBytesHexNo0xProperty, assertRecordBytesHexProperty } from '../../../../base/bytes.js';
import { assertRecordArrayProperty, assertRecordNonNullableProperty } from '../../../../base/record.js';
import { assertRecordStringProperty } from '../../../../base/string.js';

/**
 * Asserts that `value` matches the {@link RelayerUserDecryptSucceeded} schema:
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "string",
 *   "result": {
 *     "result": [{
 *       "payload": "hexNo0x...",
 *       "signature": "hexNo0x...",
 *       "extraData": "hex_or_hexNo0x_?..."
 *     }]
 *   }
 * }
 * ```
 */
export function assertIsRelayerUserDecryptSucceeded(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerUserDecryptSucceeded {
  type T = RelayerUserDecryptSucceeded;
  assertRecordStringProperty(value, 'status' satisfies keyof T, name, {
    expectedValue: 'succeeded' satisfies T['status'],
    ...options,
  });
  assertRecordStringProperty(value, 'requestId' satisfies keyof T, name, options);
  assertRecordNonNullableProperty(value, 'result' satisfies keyof T, name, options);
  _assertIsRelayerResult200UserDecrypt(value.result, `${name}.result`, options);
}

/**
 * Asserts that `value` matches the {@link RelayerResult200UserDecrypt} schema:
 * ```json
 * {
 *   "result": [{
 *     "payload": "hexNo0x...",
 *     "signature": "hexNo0x...",
 *     "extraData": "hex_or_hexNo0x_?..."
 *   }]
 * }
 * ```
 */
function _assertIsRelayerResult200UserDecrypt(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerResult200UserDecrypt {
  type T = RelayerResult200UserDecrypt;
  type ResultItem = T['result'][number];

  assertRecordArrayProperty(value, 'result' satisfies keyof T, name, options);
  for (let i = 0; i < value.result.length; ++i) {
    assertRecordBytesHexNo0xProperty(
      value.result[i],
      'payload' satisfies keyof ResultItem,
      `${name}.result[${i}]`,
      options,
    );
    assertRecordBytesHexNo0xProperty(value.result[i], 'signature' satisfies keyof ResultItem, `${name}.result[${i}]`, {
      ...options,
      byteLength: 65,
    });
    assertRecordBytesHexProperty(
      value.result[i],
      'extraData' satisfies keyof ResultItem,
      `${name}.result[${i}]`,
      options,
    );
  }
}

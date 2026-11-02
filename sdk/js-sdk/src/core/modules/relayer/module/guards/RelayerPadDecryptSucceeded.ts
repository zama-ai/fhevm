import type { ErrorMetadataParams } from '../../../../base/errors/ErrorBase.js';
import type { RelayerPadDecryptSucceeded } from '../../../../types/relayer-p.js';
import { assertRecordBytes32HexProperty, assertRecordBytesHexProperty } from '../../../../base/bytes.js';
import { assertRecordArrayProperty, assertRecordNonNullableProperty } from '../../../../base/record.js';
import { InvalidPropertyError } from '../../../../base/errors/InvalidPropertyError.js';
import { assertRecordStringProperty } from '../../../../base/string.js';
import { isUuidV4 } from '../../../../base/uuidv4.js';

/**
 * Asserts that `value` matches the {@link RelayerPadDecryptSucceeded} schema, relayer-http's `POST /v4/pad-decrypt`
 * success body (RFC-030):
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "UUIDv4",
 *   "result": {
 *     "responses": [{
 *       "decryptionId": "0x…32 bytes",
 *       "encryptedPayload": "0x…",
 *       "signature": "0x…65 bytes"
 *     }]
 *   }
 * }
 * ```
 * The envelope's `requestId` is the relayer's correlation id; each response's `decryptionId` is the KMS request id its
 * node signed (the connector's name for it).
 */
export function assertIsRelayerPadDecryptSucceeded(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerPadDecryptSucceeded {
  type T = RelayerPadDecryptSucceeded;
  type Item = T['result']['responses'][number];
  assertRecordStringProperty(value, 'status' satisfies keyof T, name, {
    expectedValue: 'succeeded' satisfies T['status'],
    ...options,
  });
  assertRecordStringProperty(value, 'requestId' satisfies keyof T, name, options);
  if (!isUuidV4(value.requestId)) {
    throw new InvalidPropertyError(
      { subject: name, property: 'requestId', expectedType: 'UUIDv4', type: 'string', value: value.requestId },
      options,
    );
  }
  assertRecordNonNullableProperty(value, 'result' satisfies keyof T, name, options);

  const result: unknown = value.result;
  const resultName = `${name}.result`;
  assertRecordArrayProperty(result, 'responses' satisfies keyof T['result'], resultName, options);

  for (let i = 0; i < result.responses.length; ++i) {
    const item: unknown = result.responses[i];
    const itemName = `${resultName}.responses[${i}]`;
    assertRecordBytes32HexProperty(item, 'decryptionId' satisfies keyof Item, itemName, options);
    assertRecordBytesHexProperty(item, 'encryptedPayload' satisfies keyof Item, itemName, options);
    assertRecordBytesHexProperty(item, 'signature' satisfies keyof Item, itemName, {
      ...options,
      byteLength: 65,
    });
  }
}

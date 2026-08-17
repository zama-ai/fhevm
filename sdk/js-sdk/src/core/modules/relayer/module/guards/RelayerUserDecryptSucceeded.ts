import type { ErrorMetadataParams } from '../../../../base/errors/ErrorBase.js';
import type { RelayerResult200UserDecrypt, RelayerUserDecryptSucceeded } from '../../../../types/relayer-p.js';
import type { BytesHex } from '../../../../types/primitives.js';
import { assertRecordBytesHexNo0xProperty, assertRecordBytesHexProperty } from '../../../../base/bytes.js';
import { assertRecordArrayProperty, assertRecordNonNullableProperty } from '../../../../base/record.js';
import { assertRecordStringProperty } from '../../../../base/string.js';
import { InvalidPropertyError } from '../../../../base/errors/InvalidPropertyError.js';

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
    const item: unknown = value.result[i];
    const itemName = `${name}.result[${i}]`;

    assertRecordBytesHexNo0xProperty(item, 'payload' satisfies keyof ResultItem, itemName, options);
    assertRecordBytesHexNo0xProperty(item, 'signature' satisfies keyof ResultItem, itemName, {
      ...options,
      byteLength: 65,
    });
  }

  // In v11 extraData is optional.
  const requiredExtraData = false;
  _assertExtraData(value.result, name, requiredExtraData, options);

  _patchMissingExtraDataV11(value.result as Array<{ extraData?: BytesHex }>);
}

// In v11 the relayer does not include extraData in response.
// Put it to '0x' to match the assertion
function _patchMissingExtraDataV11(
  items: Array<{ extraData?: BytesHex }>,
): asserts items is Array<{ extraData: BytesHex }> {
  for (const item of items) {
    if (!Object.prototype.hasOwnProperty.call(item, 'extraData')) {
      item.extraData = '0x' as BytesHex;
    }
  }
}

/**
 * Asserts that every result item either consistently includes `extraData` or
 * consistently omits it, then validates present `extraData` values as `BytesHex`.
 *
 * When `required` is true, every item must include `extraData`.
 */
export function _assertExtraData(
  result: readonly unknown[],
  name: string,
  required: boolean,
  options: ErrorMetadataParams,
): void {
  let hasExtraData: boolean | undefined;

  for (let i = 0; i < result.length; ++i) {
    const item: unknown = result[i];
    const itemName = `${name}.result[${i}]`;

    const itemHasExtraData = Object.prototype.hasOwnProperty.call(item, 'extraData');

    if (required && !itemHasExtraData) {
      throw new InvalidPropertyError(
        {
          subject: itemName,
          property: 'extraData',
          expectedType: 'bytesHex',
          type: 'undefined',
        },
        options,
      );
    }

    if (hasExtraData === undefined) {
      hasExtraData = itemHasExtraData;
    } else if (itemHasExtraData !== hasExtraData) {
      throw new InvalidPropertyError(
        {
          subject: itemName,
          property: 'extraData',
          expectedType: hasExtraData ? 'bytesHex' : 'undefined',
          type: itemHasExtraData ? 'unknown' : 'undefined',
        },
        options,
      );
    }
  }

  if (hasExtraData === true) {
    for (let i = 0; i < result.length; ++i) {
      assertRecordBytesHexProperty(result[i], 'extraData', `${name}.result[${i}]`, options);
    }
  }
}

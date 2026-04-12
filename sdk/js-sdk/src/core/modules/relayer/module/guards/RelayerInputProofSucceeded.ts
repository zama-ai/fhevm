import type {
  RelayerInputProofSucceeded,
  RelayerResult200InputProofAccepted,
  RelayerResult200InputProofRejected,
} from '../../../../types/relayer-p.js';
import type { ErrorMetadataParams } from '../../../../base/errors/ErrorBase.js';
import { assertRecordNonNullableProperty } from '../../../../base/record.js';
import { assertRecordStringProperty } from '../../../../base/string.js';
import { assertRecordBooleanProperty } from '../../../../base/record.js';
import {
  assertRecordBytes32HexArrayProperty,
  assertRecordBytesHexArrayProperty,
  assertRecordBytesHexProperty,
} from '../../../../base/bytes.js';

/**
 * Asserts that `value` matches the {@link RelayerInputProofSucceeded} schema:
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "string",
 *   "result": {
 *     "accepted": true,
 *     "extraData": "0x...",
 *     "handles": ["0x..."],
 *     "signatures": ["0x..."]
 *   } | {
 *     "accepted": false,
 *     "extraData": "0x..."
 *   }
 * }
 * ```
 */
export function assertIsRelayerInputProofSucceeded(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerInputProofSucceeded {
  type T = RelayerInputProofSucceeded;
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

  type R = RelayerInputProofSucceeded['result'];

  assertRecordBooleanProperty(
    value.result,
    'accepted' satisfies keyof R,
    `${name}.result`,
    options,
  );

  if (value.result.accepted) {
    _assertIsRelayerResult200InputProofAccepted(
      value.result,
      `${name}.result`,
      options,
    );
  } else {
    _assertIsRelayerResult200InputProofRejected(
      value.result,
      `${name}.result`,
      options,
    );
  }
}

/**
 * Asserts that `value` matches the {@link RelayerResult200InputProofAccepted} schema:
 * ```json
 * {
 *   "accepted": true,
 *   "extraData": "0x...",
 *   "handles": ["0x..."],
 *   "signatures": ["0x..."]
 * }
 * ```
 */
function _assertIsRelayerResult200InputProofAccepted(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerResult200InputProofAccepted {
  type T = RelayerResult200InputProofAccepted;

  assertRecordBooleanProperty(value, 'accepted' satisfies keyof T, name, {
    expectedValue: true,
    ...options,
  });
  assertRecordBytesHexProperty(
    value,
    'extraData' satisfies keyof T,
    name,
    options,
  );
  assertRecordBytes32HexArrayProperty(
    value,
    'handles' satisfies keyof T,
    name,
    options,
  );
  assertRecordBytesHexArrayProperty(
    value,
    'signatures' satisfies keyof T,
    name,
    options,
  );
}

/**
 * Asserts that `value` matches the {@link RelayerResult200InputProofAccepted} schema:
 * ```json
 * {
 *   "accepted": false,
 *   "extraData": "0x...",
 * }
 * ```
 */
function _assertIsRelayerResult200InputProofRejected(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is RelayerResult200InputProofRejected {
  type T = RelayerResult200InputProofRejected;

  assertRecordBooleanProperty(value, 'accepted' satisfies keyof T, name, {
    expectedValue: false,
    ...options,
  });
  assertRecordBytesHexProperty(
    value,
    'extraData' satisfies keyof T,
    name,
    options,
  );
}

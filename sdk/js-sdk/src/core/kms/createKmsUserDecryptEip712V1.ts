import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { KmsEip712Domain, KmsUserDecryptEip712V1 } from '../types/kms.js';
import type { BytesHex } from '../types/primitives.js';
import type { KmsUserDecryptEip712V1Message } from '../types/kms.js';
import type { KmsExtraData } from '../types/kms-p.js';
import {
  addressToChecksummedAddress,
  assertIsAddress,
  assertIsAddressArray,
  assertRecordChecksummedAddressArrayProperty,
} from '../base/address.js';
import { assertIsKmsEip712Domain, createKmsEip712Domain } from './createKmsEip712Domain.js';
import { asBytesHex, assertRecordBytesHexProperty, bytesToHexLarge } from '../base/bytes.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import { assertRecordStringProperty, ensure0x } from '../base/string.js';
import { assertIsUint64, assertIsUintNumber, assertIsUintString, MAX_UINT256 } from '../base/uint.js';
import { kmsUserDecryptEip712V1Types } from './kmsUserDecryptEip712V1Types.js';
import { kmsDelegatedUserDecryptEip712V1Types } from './kmsDelegatedUserDecryptEip712V1Types.js';
import { isDeepEqual } from '../base/object.js';
import { assertIsKmsExtraData, assertIsKmsExtraDataBytesHex } from './kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsUserDecryptEip712V1Parameters = {
  readonly verifyingContractAddressDecryption: string;
  readonly chainId: number | bigint;
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: KmsExtraData;
};

////////////////////////////////////////////////////////////////////////////////
// createKmsUserDecryptEip712V1
////////////////////////////////////////////////////////////////////////////////

export function createKmsUserDecryptEip712V1(
  parameters: CreateKmsUserDecryptEip712V1Parameters,
): KmsUserDecryptEip712V1 {
  const {
    verifyingContractAddressDecryption,
    chainId,
    publicKey,
    contractAddresses,
    startTimestamp,
    durationDays,
    extraData,
  } = parameters;
  const publicKeyBytesHex = _verifyPublicKeyArg(publicKey);

  assertIsUint64(chainId, {});
  assertIsAddress(verifyingContractAddressDecryption, {});
  assertIsAddressArray(contractAddresses, {});
  assertIsUintNumber(startTimestamp, {});
  assertIsUintNumber(durationDays, {});
  assertIsKmsExtraData(extraData, {});

  const checksummedContractAddresses = contractAddresses.map(addressToChecksummedAddress);

  const primaryType: KmsUserDecryptEip712V1['primaryType'] = 'UserDecryptRequestVerification';

  const domain = createKmsEip712Domain({
    chainId,
    verifyingContractAddressDecryption,
  });

  const eip712 = {
    domain,
    types: kmsUserDecryptEip712V1Types,
    primaryType,
    message: {
      publicKey: publicKeyBytesHex,
      contractAddresses: checksummedContractAddresses,
      startTimestamp: startTimestamp.toString(),
      durationDays: durationDays.toString(),
      extraData: extraData.bytesHex,
    },
  };

  Object.freeze(eip712);
  Object.freeze(eip712.domain);
  Object.freeze(eip712.types);
  Object.freeze(eip712.types.EIP712Domain);
  Object.freeze(eip712.types.UserDecryptRequestVerification);
  Object.freeze(eip712.message);
  Object.freeze(eip712.message.contractAddresses);

  return eip712;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsKmsUserDecryptEip712V1(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is KmsUserDecryptEip712V1 {
  _assertIsKmsUserDecryptEip712V1Base(
    value,
    name,
    'UserDecryptRequestVerification' satisfies KmsUserDecryptEip712V1['primaryType'],
    options,
  );
}

/**
 * Validates the common structure shared by both
 * {@link KmsUserDecryptEip712V1} and {@link KmsDelegatedUserDecryptEip712V1}:
 * domain, types, primaryType, and base message fields.
 */
export function _assertIsKmsUserDecryptEip712V1Base(
  value: unknown,
  name: string,
  primaryType: string,
  options: ErrorMetadataParams,
): asserts value is {
  readonly domain: KmsEip712Domain;
  readonly types: object;
  readonly primaryType: string;
  readonly message: KmsUserDecryptEip712V1Message;
} {
  assertRecordNonNullableProperty(value, 'domain', name, options);
  assertIsKmsEip712Domain((value as Record<string, unknown>).domain, `${name}.domain`, options);

  assertRecordNonNullableProperty(value, 'types', name, options);

  const expectedTypes =
    primaryType === 'DelegatedUserDecryptRequestVerification'
      ? kmsDelegatedUserDecryptEip712V1Types
      : kmsUserDecryptEip712V1Types;

  if (!isDeepEqual(value.types, expectedTypes)) {
    throw new Error('Unexpected KmsUserDecryptEip712V1Types');
  }

  assertRecordStringProperty(value, 'primaryType', name, {
    expectedValue: primaryType,
    ...options,
  });

  assertRecordNonNullableProperty(value, 'message', name, options);
  const msg = (value as Record<string, unknown>).message as Record<string, unknown>;
  _assertIsKmsUserDecryptEip712V1Message(msg, `${name}.message`, options);
}

/**
 * Validates the common message fields shared by both
 * {@link KmsUserDecryptEip712V1} and {@link KmsDelegatedUserDecryptEip712V1}.
 */
function _assertIsKmsUserDecryptEip712V1Message(
  msg: unknown,
  msgName: string,
  options: ErrorMetadataParams,
): asserts msg is KmsUserDecryptEip712V1Message {
  type MessageType = KmsUserDecryptEip712V1['message'];
  assertRecordBytesHexProperty(msg, 'publicKey' satisfies keyof MessageType, msgName, options);
  assertRecordChecksummedAddressArrayProperty(msg, 'contractAddresses' satisfies keyof MessageType, msgName, options);
  assertRecordStringProperty(msg, 'startTimestamp' satisfies keyof MessageType, msgName, options);
  assertRecordStringProperty(msg, 'durationDays' satisfies keyof MessageType, msgName, options);
  assertRecordBytesHexProperty(msg, 'extraData' satisfies keyof MessageType, msgName, options);
  assertIsKmsExtraDataBytesHex(msg.extraData, options);
  assertIsUintString(msg.startTimestamp, { max: MAX_UINT256 });
  assertIsUintString(msg.durationDays, { max: MAX_UINT256 });
}

////////////////////////////////////////////////////////////////////////////////

function _verifyPublicKeyArg(value: unknown): BytesHex {
  if (value === null || value === undefined) {
    throw new Error(`Missing publicKey argument.`);
  }

  let publicKeyBytesHex: BytesHex;

  const pk = value;

  if (typeof pk === 'string') {
    publicKeyBytesHex = asBytesHex(ensure0x(pk));
  } else if (pk instanceof Uint8Array) {
    publicKeyBytesHex = bytesToHexLarge(pk);
  } else {
    throw new Error(`Invalid publicKey argument.`);
  }

  return publicKeyBytesHex;
}

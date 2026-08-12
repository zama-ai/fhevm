import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { KmsUserDecryptEip712V2 } from '../types/kms.js';
import type { BytesHex } from '../types/primitives.js';
import type { KmsUserDecryptEip712V2Message } from '../types/kms.js';
import type { KmsExtraData } from '../types/kms-p.js';
import {
  addressToChecksummedAddress,
  assertIsAddress,
  assertIsAddressArray,
  assertRecordAddressProperty,
  assertRecordChecksummedAddressArrayProperty,
} from '../base/address.js';
import { assertIsKmsEip712Domain, createKmsEip712Domain } from './createKmsEip712Domain.js';
import { asBytesHex, assertRecordBytesHexProperty, bytesToHexLarge } from '../base/bytes.js';
import { isDeepEqual } from '../base/object.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import { assertRecordStringProperty, ensure0x } from '../base/string.js';
import { assertIsUint256, assertIsUint64, assertIsUintString, MAX_UINT256 } from '../base/uint.js';
import { kmsUserDecryptEip712V2Types } from './kmsUserDecryptEip712V2Types.js';
import {
  assertIsKmsExtraData,
  assertIsKmsExtraDataBytesHex,
  createKmsExtraDataFromBytesHex,
  EXTRA_DATA_V2,
} from './kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsUserDecryptEip712V2Parameters = {
  readonly verifyingContractAddressDecryption: string;
  readonly chainId: number | bigint;
  readonly userAddress: string;
  readonly publicKey: string | Uint8Array;
  readonly allowedContracts: readonly string[];
  readonly startTimestamp: number | bigint;
  readonly durationSeconds: bigint;
  readonly extraData: KmsExtraData;
};

////////////////////////////////////////////////////////////////////////////////
// createKmsUserDecryptEip712V2 (RFC-016)
////////////////////////////////////////////////////////////////////////////////

export function createKmsUserDecryptEip712V2(
  parameters: CreateKmsUserDecryptEip712V2Parameters,
): KmsUserDecryptEip712V2 {
  const {
    verifyingContractAddressDecryption,
    chainId,
    userAddress,
    publicKey,
    allowedContracts,
    startTimestamp,
    durationSeconds,
    extraData,
  } = parameters;
  const publicKeyBytesHex = _verifyPublicKeyArg(publicKey);

  assertIsUint64(chainId, {});
  assertIsAddress(verifyingContractAddressDecryption, {});
  assertIsAddress(userAddress, {});
  assertIsAddressArray(allowedContracts, {});
  assertIsUint256(startTimestamp, {});
  assertIsUint256(durationSeconds, {});
  assertIsKmsExtraData(extraData, {});

  // A unified (V2) EIP-712 only exists on protocol >= v0.14. Reject the older
  // v0/v1 encodings; accept v2 and any newer (or unknown/future) version — an
  // unknown version is assumed to be newer than v2 and forwarded to the chain.
  if (extraData.lt(EXTRA_DATA_V2)) {
    throw new Error(
      `createKmsUserDecryptEip712V2: a unified (V2) EIP-712 requires a v2 or later extraData, got version ${extraData.version}.`,
    );
  }

  const checksummedUserAddress = addressToChecksummedAddress(userAddress);
  const checksummedContractAddresses = allowedContracts.map(addressToChecksummedAddress);

  const primaryType: KmsUserDecryptEip712V2['primaryType'] = 'UserDecryptRequestVerification';

  const domain = createKmsEip712Domain({
    chainId,
    verifyingContractAddressDecryption,
  });

  const eip712 = {
    domain,
    types: kmsUserDecryptEip712V2Types,
    primaryType,
    message: {
      userAddress: checksummedUserAddress,
      publicKey: publicKeyBytesHex,
      allowedContracts: checksummedContractAddresses,
      startTimestamp: startTimestamp.toString(),
      durationSeconds: durationSeconds.toString(),
      extraData: extraData.bytesHex,
    } satisfies KmsUserDecryptEip712V2Message,
  };

  Object.freeze(eip712);
  Object.freeze(eip712.domain);
  Object.freeze(eip712.types);
  Object.freeze(eip712.types.EIP712Domain);
  Object.freeze(eip712.types.UserDecryptRequestVerification);
  Object.freeze(eip712.message);
  Object.freeze(eip712.message.allowedContracts);

  return eip712;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsKmsUserDecryptEip712V2(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is KmsUserDecryptEip712V2 {
  assertRecordNonNullableProperty(value, 'domain', name, options);
  assertIsKmsEip712Domain((value as Record<string, unknown>).domain, `${name}.domain`, options);

  assertRecordNonNullableProperty(value, 'types', name, options);
  if (!isDeepEqual(value.types, kmsUserDecryptEip712V2Types)) {
    throw new Error('Unexpected KmsUserDecryptEip712V2Types');
  }

  assertRecordStringProperty(value, 'primaryType', name, {
    expectedValue: 'UserDecryptRequestVerification',
    ...options,
  });

  assertRecordNonNullableProperty(value, 'message', name, options);
  const msg = (value as Record<string, unknown>).message as Record<string, unknown>;
  _assertIsKmsUserDecryptEip712V2Message(msg, `${name}.message`, options);
}

function _assertIsKmsUserDecryptEip712V2Message(
  msg: unknown,
  msgName: string,
  options: ErrorMetadataParams,
): asserts msg is KmsUserDecryptEip712V2Message {
  type MessageType = KmsUserDecryptEip712V2['message'];
  assertRecordAddressProperty(msg, 'userAddress' satisfies keyof MessageType, msgName, options);
  assertRecordBytesHexProperty(msg, 'publicKey' satisfies keyof MessageType, msgName, options);
  assertRecordChecksummedAddressArrayProperty(msg, 'allowedContracts' satisfies keyof MessageType, msgName, options);
  assertRecordStringProperty(msg, 'startTimestamp' satisfies keyof MessageType, msgName, options);
  assertRecordStringProperty(msg, 'durationSeconds' satisfies keyof MessageType, msgName, options);
  assertRecordBytesHexProperty(msg, 'extraData' satisfies keyof MessageType, msgName, options);
  assertIsKmsExtraDataBytesHex(msg.extraData, options);
  // A unified (V2) permit must carry a v2-or-later extraData; reject only the
  // older v0/v1 encodings. A newer (or unknown/future) version is accepted.
  const extraData = createKmsExtraDataFromBytesHex(msg.extraData);
  if (extraData.lt(EXTRA_DATA_V2)) {
    throw new Error(
      `${msgName}.extraData: a unified (V2) permit requires a v2 or later extraData, got version ${extraData.version}.`,
    );
  }
  assertIsUintString(msg.startTimestamp, { max: MAX_UINT256 });
  assertIsUintString(msg.durationSeconds, { max: MAX_UINT256 });
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

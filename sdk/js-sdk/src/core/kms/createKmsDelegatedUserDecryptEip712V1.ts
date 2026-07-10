import type { KmsDelegatedUserDecryptEip712V1 } from '../types/kms.js';
import type { BytesHex } from '../types/primitives.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { KmsExtraData } from '../types/kms-p.js';
import {
  addressToChecksummedAddress,
  assertIsAddress,
  assertIsAddressArray,
  assertRecordChecksummedAddressProperty,
} from '../base/address.js';
import { asBytesHex, bytesToHexLarge } from '../base/bytes.js';
import { ensure0x } from '../base/string.js';
import { assertIsUintNumber } from '../base/uint.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { _assertIsKmsUserDecryptEip712V1Base } from './createKmsUserDecryptEip712V1.js';
import { kmsDelegatedUserDecryptEip712V1Types } from './kmsDelegatedUserDecryptEip712V1Types.js';
import { assertIsKmsExtraData } from './kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsDelegatedUserDecryptEip712V1Parameters = {
  readonly verifyingContractAddressDecryption: string;
  readonly chainId: number | bigint;
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: KmsExtraData;
  readonly delegatorAddress: string;
};

////////////////////////////////////////////////////////////////////////////////
// createKmsDelegatedUserDecryptEip712V1
////////////////////////////////////////////////////////////////////////////////

export function createKmsDelegatedUserDecryptEip712V1(
  parameters: CreateKmsDelegatedUserDecryptEip712V1Parameters,
): KmsDelegatedUserDecryptEip712V1 {
  const {
    verifyingContractAddressDecryption,
    chainId,
    publicKey,
    contractAddresses,
    startTimestamp,
    durationDays,
    extraData,
    delegatorAddress,
  } = parameters;
  const publicKeyBytesHex = _verifyPublicKeyArg(publicKey);

  assertIsAddressArray(contractAddresses, {});
  assertIsUintNumber(startTimestamp, {});
  assertIsUintNumber(durationDays, {});
  assertIsAddress(delegatorAddress, {});
  assertIsKmsExtraData(extraData, {});

  const checksummedContractAddresses = contractAddresses.map(addressToChecksummedAddress);

  const checksummedDelegatorAddress = addressToChecksummedAddress(delegatorAddress);

  const primaryType: KmsDelegatedUserDecryptEip712V1['primaryType'] = 'DelegatedUserDecryptRequestVerification';

  const domain = createKmsEip712Domain({
    chainId,
    verifyingContractAddressDecryption,
  });

  const eip712: KmsDelegatedUserDecryptEip712V1 = {
    types: kmsDelegatedUserDecryptEip712V1Types,
    primaryType,
    domain,
    message: {
      publicKey: publicKeyBytesHex,
      contractAddresses: checksummedContractAddresses,
      delegatorAddress: checksummedDelegatorAddress,
      startTimestamp: startTimestamp.toString(),
      durationDays: durationDays.toString(),
      extraData: extraData.toBytesHex(),
    },
  };

  Object.freeze(eip712);
  Object.freeze(eip712.domain);
  Object.freeze(eip712.types);
  Object.freeze(eip712.types.EIP712Domain);
  Object.freeze(eip712.types.DelegatedUserDecryptRequestVerification);
  Object.freeze(eip712.message);
  Object.freeze(eip712.message.contractAddresses);

  return eip712;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsKmsDelegatedUserDecryptEip712V1(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is KmsDelegatedUserDecryptEip712V1 {
  _assertIsKmsUserDecryptEip712V1Base(
    value,
    name,
    'DelegatedUserDecryptRequestVerification' satisfies KmsDelegatedUserDecryptEip712V1['primaryType'],
    options,
  );

  assertRecordChecksummedAddressProperty(
    value.message,
    'delegatorAddress' satisfies keyof KmsDelegatedUserDecryptEip712V1['message'],
    `${name}.message`,
    options,
  );
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

import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import {
  addressToChecksummedAddress,
  assertIsAddress,
  assertIsAddressArray,
  assertRecordChecksummedAddressProperty,
} from '../base/address.js';
import {
  asBytesHex,
  assertIsBytesHex,
  bytesToHexLarge,
} from '../base/bytes.js';
import { ensure0x } from '../base/string.js';
import { assertIsUintNumber } from '../base/uint.js';
import type { KmsDelegatedUserDecryptEIP712 } from '../types/kms.js';
import type { BytesHex } from '../types/primitives.js';
import { createKmsEIP712Domain } from './createKmsEIP712Domain.js';
import { _assertIsKmsUserDecryptEIP712Base } from './createKmsUserDecryptEIP712.js';
import { kmsDelegatedUserDecryptEIP712Types } from './kmsDelegatedUserDecryptEIP712Types.js';
import { assertIsKmsExtraData } from './kmsExtraData.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsDelegatedUserDecryptEIP712Parameters = {
  readonly verifyingContractAddressDecryption: string;
  readonly chainId: number | bigint;
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: string;
  readonly delegatorAddress: string;
};

////////////////////////////////////////////////////////////////////////////////
// createKmsDelegatedUserDecryptEIP712
////////////////////////////////////////////////////////////////////////////////

export function createKmsDelegatedUserDecryptEIP712({
  verifyingContractAddressDecryption,
  chainId,
  publicKey,
  contractAddresses,
  startTimestamp,
  durationDays,
  extraData,
  delegatorAddress,
}: CreateKmsDelegatedUserDecryptEIP712Parameters): KmsDelegatedUserDecryptEIP712 {
  const publicKeyBytesHex = _verifyPublicKeyArg(publicKey);

  assertIsAddressArray(contractAddresses, {});
  assertIsUintNumber(startTimestamp, {});
  assertIsUintNumber(durationDays, {});
  assertIsBytesHex(extraData, {});
  assertIsAddress(delegatorAddress, {});
  assertIsKmsExtraData(extraData, {});

  const checksummedContractAddresses = contractAddresses.map(
    addressToChecksummedAddress,
  );

  const checksummedDelegatorAddress =
    addressToChecksummedAddress(delegatorAddress);

  const primaryType: KmsDelegatedUserDecryptEIP712['primaryType'] =
    'DelegatedUserDecryptRequestVerification';

  const domain = createKmsEIP712Domain({
    chainId,
    verifyingContractAddressDecryption,
  });

  const eip712: KmsDelegatedUserDecryptEIP712 = {
    types: kmsDelegatedUserDecryptEIP712Types,
    primaryType,
    domain,
    message: {
      publicKey: publicKeyBytesHex,
      contractAddresses: checksummedContractAddresses,
      delegatorAddress: checksummedDelegatorAddress,
      startTimestamp: startTimestamp.toString(),
      durationDays: durationDays.toString(),
      extraData,
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

export function assertIsKmsDelegatedUserDecryptEIP712(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is KmsDelegatedUserDecryptEIP712 {
  _assertIsKmsUserDecryptEIP712Base(
    value,
    name,
    'DelegatedUserDecryptRequestVerification' satisfies KmsDelegatedUserDecryptEIP712['primaryType'],
    options,
  );

  assertRecordChecksummedAddressProperty(
    value.message,
    'delegatorAddress' satisfies keyof KmsDelegatedUserDecryptEIP712['message'],
    `${name}.message`,
    options,
  );
}

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

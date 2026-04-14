import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { KmsEip712Domain } from '../types/kms.js';
import type { Uint64BigInt } from '../types/primitives.js';
import {
  addressToChecksummedAddress,
  assertIsAddress,
  assertRecordChecksummedAddressProperty,
} from '../base/address.js';
import { assertRecordStringProperty } from '../base/string.js';
import { assertIsUint64, assertRecordUintBigIntProperty } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsEip712DomainParameters = {
  readonly chainId: number | bigint;
  readonly verifyingContractAddressDecryption: string;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * IMPORTANT:
 *
 * KmsEip712Domain.chainId depends on the primaryType:
 *    - `UserDecryptRequestVerification`: chainId is the **host chain**
 *    - `DelegatedUserDecryptRequestVerification`: chainId is the **host chain**
 *    - `PublicDecryptVerification`: chainId is the **gateway chain**
 */
export function createKmsEip712Domain(parameters: CreateKmsEip712DomainParameters): KmsEip712Domain {
  const {
    chainId, // Warning! any chainId could be host or gateway
    verifyingContractAddressDecryption,
  } = parameters;

  assertIsUint64(chainId, {});
  assertIsAddress(verifyingContractAddressDecryption, {});

  const domain = {
    name: 'Decryption',
    version: '1',
    chainId: BigInt(chainId) as Uint64BigInt,
    verifyingContract: addressToChecksummedAddress(verifyingContractAddressDecryption),
  } as const;
  Object.freeze(domain);

  return domain;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsKmsEip712Domain(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is KmsEip712Domain {
  type T = KmsEip712Domain;
  assertRecordStringProperty(value, 'name' satisfies keyof T, name, {
    expectedValue: 'Decryption' satisfies T['name'],
    ...options,
  });
  assertRecordStringProperty(value, 'version' satisfies keyof T, name, {
    expectedValue: '1' satisfies T['version'],
    ...options,
  });
  assertRecordUintBigIntProperty(value, 'chainId' satisfies keyof T, name, options);
  assertRecordChecksummedAddressProperty(value, 'verifyingContract' satisfies keyof T, name, options);
}

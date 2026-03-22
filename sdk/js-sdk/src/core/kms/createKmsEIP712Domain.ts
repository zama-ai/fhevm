import {
  addressToChecksummedAddress,
  assertIsAddress,
  assertRecordChecksummedAddressProperty,
} from "../base/address.js";
import type { ErrorMetadataParams } from "../base/errors/ErrorBase.js";
import { assertRecordStringProperty } from "../base/string.js";
import {
  assertIsUint64,
  assertRecordUintBigIntProperty,
} from "../base/uint.js";
import type { KmsEIP712Domain } from "../types/kms.js";
import type { Uint64BigInt } from "../types/primitives.js";

export function createKmsEIP712Domain({
  chainId, // any chainId could be host or gateway
  verifyingContractAddressDecryption,
}: {
  chainId: number | bigint;
  verifyingContractAddressDecryption: string;
}): KmsEIP712Domain {
  assertIsUint64(chainId, {});
  assertIsAddress(verifyingContractAddressDecryption, {});

  const domain = {
    name: "Decryption",
    version: "1",
    chainId: BigInt(chainId) as Uint64BigInt,
    verifyingContract: addressToChecksummedAddress(
      verifyingContractAddressDecryption,
    ),
  } as const;
  Object.freeze(domain);

  return domain;
}

export function assertIsKmsEIP712Domain(
  value: unknown,
  name: string,
  options: ErrorMetadataParams,
): asserts value is KmsEIP712Domain {
  type T = KmsEIP712Domain;
  assertRecordStringProperty(value, "name" satisfies keyof T, name, {
    expectedValue: "Decryption" satisfies T["name"],
    ...options,
  });
  assertRecordStringProperty(value, "version" satisfies keyof T, name, {
    expectedValue: "1" satisfies T["version"],
    ...options,
  });
  assertRecordUintBigIntProperty(
    value,
    "chainId" satisfies keyof T,
    name,
    options,
  );
  assertRecordChecksummedAddressProperty(
    value,
    "verifyingContract" satisfies keyof T,
    name,
    options,
  );
}

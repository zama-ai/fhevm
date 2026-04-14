import type { CoprocessorEip712Domain } from '../types/coprocessor.js';
import type { Uint64BigInt } from '../types/primitives.js';
import { addressToChecksummedAddress, assertIsAddress } from '../base/address.js';
import { assertIsUint64 } from '../base/uint.js';

export function createCoprocessorEip712Domain({
  gatewayChainId,
  verifyingContractAddressInputVerification,
}: {
  readonly gatewayChainId: number | bigint;
  readonly verifyingContractAddressInputVerification: string;
}): CoprocessorEip712Domain {
  assertIsUint64(gatewayChainId, {});
  assertIsAddress(verifyingContractAddressInputVerification, {});

  const domain = {
    name: 'InputVerification',
    version: '1',
    chainId: BigInt(gatewayChainId) as Uint64BigInt,
    verifyingContract: addressToChecksummedAddress(verifyingContractAddressInputVerification),
  } as const;
  Object.freeze(domain);

  return domain;
}

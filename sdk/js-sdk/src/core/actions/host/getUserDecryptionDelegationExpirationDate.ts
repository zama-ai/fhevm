import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { Uint64BigInt } from '../../types/primitives.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { getUserDecryptionDelegationExpirationDate as getUserDecryptionDelegationExpirationDate_ } from '../../host-contracts/getUserDecryptionDelegationExpirationDate.js';

////////////////////////////////////////////////////////////////////////////////

export type GetUserDecryptionDelegationExpirationDateParameters = {
  readonly address: string;
  readonly delegator: string;
  readonly delegate: string;
  readonly contractAddress: string;
};

export type GetUserDecryptionDelegationExpirationDateReturnType = Uint64BigInt;

////////////////////////////////////////////////////////////////////////////////

export async function getUserDecryptionDelegationExpirationDate(
  fhevm: Fhevm,
  parameters: GetUserDecryptionDelegationExpirationDateParameters,
): Promise<GetUserDecryptionDelegationExpirationDateReturnType> {
  const { address, delegate, delegator, contractAddress } = parameters;

  assertIsAddress(address, {});
  assertIsAddress(delegate, {});
  assertIsAddress(delegator, {});
  assertIsAddress(contractAddress, {});

  return getUserDecryptionDelegationExpirationDate_(fhevm, {
    address: addressToChecksummedAddress(address),
    delegate: addressToChecksummedAddress(delegate),
    delegator: addressToChecksummedAddress(delegator),
    contractAddress: addressToChecksummedAddress(contractAddress),
  });
}

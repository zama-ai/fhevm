import { isUint64 } from '../../base/uint.js';
import { getUserDecryptionDelegationExpirationDateAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type {
  ChecksummedAddress,
  Uint64BigInt,
} from '../../types/primitives.js';

export type GetUserDecryptionDelegationExpirationDateParameters = {
  readonly address: ChecksummedAddress;
};

export type GetUserDecryptionDelegationExpirationDateReturnType = Uint64BigInt;

export async function getUserDecryptionDelegationExpirationDate(
  fhevm: Fhevm,
  parameters: GetUserDecryptionDelegationExpirationDateParameters,
): Promise<GetUserDecryptionDelegationExpirationDateReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getUserDecryptionDelegationExpirationDateAbi,
    args: [],
    functionName: getUserDecryptionDelegationExpirationDateAbi[0].name,
  });

  if (!isUint64(res)) {
    throw new Error(`Invalid expiration date.`);
  }

  return BigInt(res) as Uint64BigInt;
}
//isHandleDelegatedForUserDecryption

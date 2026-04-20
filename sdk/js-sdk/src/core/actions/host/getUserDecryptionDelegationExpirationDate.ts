import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress, Uint64BigInt } from '../../types/primitives.js';
import { isUint64 } from '../../base/uint.js';
import { getUserDecryptionDelegationExpirationDateAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';

export type GetUserDecryptionDelegationExpirationDateParameters = {
  /** Address of the ACL contract. */
  readonly aclAddress: ChecksummedAddress;
  /** Address of the account that granted the delegation. */
  readonly delegatorAddress: ChecksummedAddress;
  /** Address of the account that received the delegation. */
  readonly delegateAddress: ChecksummedAddress;
  /**
   * Contract address the delegation covers, or {@link WILDCARD_CONTRACT} to
   * query a wildcard (all-contracts) delegation entry.
   */
  readonly contractAddress: ChecksummedAddress;
};

export type GetUserDecryptionDelegationExpirationDateReturnType = Uint64BigInt;

/**
 * Returns the expiration timestamp (seconds since epoch, as `bigint`) of the
 * delegation entry `(delegator, delegate, contractAddress)` on the ACL
 * contract. Returns `0n` when no delegation exists.
 *
 * Pass {@link WILDCARD_CONTRACT} as `contractAddress` to check whether the
 * delegator granted wildcard (all-contracts) access to the delegate.
 */
export async function getUserDecryptionDelegationExpirationDate(
  fhevm: Fhevm,
  parameters: GetUserDecryptionDelegationExpirationDateParameters,
): Promise<GetUserDecryptionDelegationExpirationDateReturnType> {
  const trustedClient = getTrustedClient(fhevm);

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: parameters.aclAddress,
    abi: getUserDecryptionDelegationExpirationDateAbi,
    args: [parameters.delegatorAddress, parameters.delegateAddress, parameters.contractAddress],
    functionName: getUserDecryptionDelegationExpirationDateAbi[0].name,
  });

  if (!isUint64(res)) {
    throw new Error(`Invalid expiration date.`);
  }

  return BigInt(res) as Uint64BigInt;
}

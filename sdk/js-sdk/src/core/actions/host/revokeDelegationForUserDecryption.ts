import type { ChecksummedAddress } from '../../types/primitives.js';
import { revokeDelegationForUserDecryptionAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';

export type RevokeDelegationForUserDecryptionParameters = {
  readonly aclAddress: string;
  readonly delegateAddress: string;
  /** One or more contract addresses to revoke. Pass {@link WILDCARD_CONTRACT} to revoke the wildcard entry. */
  readonly contractAddresses: readonly string[];
};

export type RevokeDelegationForUserDecryptionCallArgs = ReadonlyArray<{
  readonly address: ChecksummedAddress;
  readonly abi: typeof revokeDelegationForUserDecryptionAbi;
  readonly functionName: 'revokeDelegationForUserDecryption';
  readonly args: readonly [ChecksummedAddress, ChecksummedAddress];
}>;

/**
 * Validates revocation parameters and returns one call-args object
 * per `contractAddress` entry, ready to be executed via the caller's own signer.
 *
 * After revoking the wildcard entry, per-contract entries may still be active.
 * Pass all entries (wildcard + specific) to ensure a complete revocation.
 */
export function revokeDelegationForUserDecryption(
  parameters: RevokeDelegationForUserDecryptionParameters,
): RevokeDelegationForUserDecryptionCallArgs {
  const { aclAddress, delegateAddress, contractAddresses } = parameters;

  assertIsAddress(aclAddress, {});
  assertIsAddress(delegateAddress, {});

  if (contractAddresses.length === 0) {
    throw new Error('contractAddresses is empty');
  }

  const checksummedAcl = addressToChecksummedAddress(aclAddress);
  const checksummedDelegate = addressToChecksummedAddress(delegateAddress);

  return contractAddresses.map((contractAddress) => {
    assertIsAddress(contractAddress, {});
    return {
      address: checksummedAcl,
      abi: revokeDelegationForUserDecryptionAbi,
      functionName: revokeDelegationForUserDecryptionAbi[0].name,
      args: [checksummedDelegate, addressToChecksummedAddress(contractAddress)] as const,
    };
  });
}

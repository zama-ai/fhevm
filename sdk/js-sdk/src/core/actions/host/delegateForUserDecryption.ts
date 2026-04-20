import type { ChecksummedAddress } from '../../types/primitives.js';
import { delegateForUserDecryptionAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { assertIsUint64 } from '../../base/uint.js';

export type DelegateForUserDecryptionParameters = {
  readonly aclAddress: string;
  readonly delegateAddress: string;
  readonly contractAddress: string;
  readonly expirationDate: bigint;
};

export type DelegateForUserDecryptionCallArgs = {
  readonly address: ChecksummedAddress;
  readonly abi: typeof delegateForUserDecryptionAbi;
  readonly functionName: 'delegateForUserDecryption';
  readonly args: readonly [ChecksummedAddress, ChecksummedAddress, bigint];
};

/**
 * Validates delegation parameters and returns the call arguments needed to execute
 * `ACL.delegateForUserDecryption(delegate, contractAddress, expirationDate)`
 * via the caller's own signer (ethers, viem, or any other library).
 *
 * Pass {@link WILDCARD_CONTRACT} as `contractAddress` to grant the delegate
 * access to every handle the delegator owns across all contracts.
 */
export function delegateForUserDecryption(
  parameters: DelegateForUserDecryptionParameters,
): DelegateForUserDecryptionCallArgs {
  const { aclAddress, delegateAddress, contractAddress, expirationDate } = parameters;

  assertIsAddress(aclAddress, {});
  assertIsAddress(delegateAddress, {});
  assertIsAddress(contractAddress, {});
  assertIsUint64(expirationDate, {});

  return {
    address: addressToChecksummedAddress(aclAddress),
    abi: delegateForUserDecryptionAbi,
    functionName: delegateForUserDecryptionAbi[0].name,
    args: [addressToChecksummedAddress(delegateAddress), addressToChecksummedAddress(contractAddress), expirationDate],
  };
}

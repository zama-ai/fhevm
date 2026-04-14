import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { Bytes32Hex, ChecksummedAddress } from '../../types/primitives.js';
import { isAllowedForDecryptionAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';

export type IsAllowedForDecryptionParameters = {
  readonly address: ChecksummedAddress;
  readonly args: {
    readonly handle: Bytes32Hex;
  };
};

export type IsAllowedForDecryptionReturnType = boolean;

export async function isAllowedForDecryption(
  fhevm: Fhevm,
  parameters: IsAllowedForDecryptionParameters,
): Promise<IsAllowedForDecryptionReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: isAllowedForDecryptionAbi,
    args: [parameters.args.handle],
    functionName: isAllowedForDecryptionAbi[0].name,
  });

  if (typeof res !== 'boolean') {
    throw new Error(`Invalid isAllowedForDecryption result.`);
  }

  return res;
}

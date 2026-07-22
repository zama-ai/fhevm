import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { Bytes32Hex, ChecksummedAddress } from '../../types/primitives.js';
import { isAllowedForDecryptionAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient, initPublicAction } from '../../runtime/CoreFhevm-p.js';
import { assertIsAddress } from '../../base/address.js';
import { toFhevmHandle } from '../../handle/FhevmHandle.js';

export type IsAllowedForDecryptionParameters = {
  readonly address: ChecksummedAddress;
  readonly args: {
    readonly handle: Bytes32Hex;
  };
};

export type IsAllowedForDecryptionReturnType = boolean;

// to be removed from core/actions
export async function isAllowedForDecryption(
  fhevm: Fhevm,
  parameters: IsAllowedForDecryptionParameters,
): Promise<IsAllowedForDecryptionReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const handle = toFhevmHandle(parameters.args.handle);
  assertIsAddress(address, {});

  // no context needed
  await initPublicAction(fhevm);

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: isAllowedForDecryptionAbi,
    args: [handle.bytes32Hex],
    functionName: isAllowedForDecryptionAbi[0].name,
  });

  if (typeof res !== 'boolean') {
    throw new Error(`Invalid isAllowedForDecryption result.`);
  }

  return res;
}

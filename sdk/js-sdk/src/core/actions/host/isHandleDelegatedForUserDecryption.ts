import type { HandleLike } from '../../types/encryptedTypes.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { isHandleDelegatedForUserDecryptionAbi } from '../../host-contracts/abi-fragments/fragments.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import { toHandle } from '../../handle/FhevmHandle.js';

export type IsHandleDelegatedForUserDecryptionParameters = {
  readonly address: ChecksummedAddress;
  readonly delegator: ChecksummedAddress;
  readonly delegate: ChecksummedAddress;
  readonly contractAddress: ChecksummedAddress;
  readonly handle: HandleLike;
};

export type IsHandleDelegatedForUserDecryptionReturnType = boolean;

export async function isHandleDelegatedForUserDecryption(
  fhevm: Fhevm,
  parameters: IsHandleDelegatedForUserDecryptionParameters,
): Promise<IsHandleDelegatedForUserDecryptionReturnType> {
  const h = toHandle(parameters.handle);

  const trustedClient = getTrustedClient(fhevm);
  const address = parameters.address;

  const res = await fhevm.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: isHandleDelegatedForUserDecryptionAbi,
    args: [parameters.delegator, parameters.delegate, parameters.contractAddress, h.bytes32Hex],
    functionName: isHandleDelegatedForUserDecryptionAbi[0].name,
  });

  if (typeof res !== 'boolean') {
    throw new Error(`Invalid boolean return type.`);
  }

  return res;
}

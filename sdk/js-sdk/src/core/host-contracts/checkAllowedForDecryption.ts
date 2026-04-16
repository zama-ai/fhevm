import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { AclPublicDecryptionError } from '../errors/AclError.js';
import { isAllowedForDecryption } from './isAllowedForDecryption.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly handles: readonly Handle[];
};

////////////////////////////////////////////////////////////////////////////////

export async function checkAllowedForDecryption(context: Context, parameters: Parameters): Promise<void> {
  const { handles } = parameters;

  const results = await isAllowedForDecryption(context, parameters);

  const failedHandles = handles.filter((_, i) => results[i] !== true).map((h) => h.bytes32Hex);
  if (failedHandles.length > 0) {
    throw new AclPublicDecryptionError({
      contractAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
      handles: failedHandles,
    });
  }
}

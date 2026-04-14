import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { HandleLike } from '../../types/encryptedTypes.js';
import { assertIsHandleLikeArray, toHandle } from '../../handle/FhevmHandle.js';
import { executeWithBatching } from '../../base/promise.js';
import { isAllowedForDecryption as isAllowedForDecryption_ } from '../host/isAllowedForDecryption.js';

export type IsAllowedForDecryptionArrayParameters = {
  readonly handles: readonly HandleLike[];
  readonly options?: { readonly checkArguments?: boolean } | undefined;
};

export type IsAllowedForDecryptionArrayReturnType = boolean[];

export type IsAllowedForDecryptionSingleParameters = {
  readonly handles: HandleLike;
  readonly options?: { readonly checkArguments?: boolean } | undefined;
};

export type IsAllowedForDecryptionSingleReturnType = boolean;

/**
 * Returns whether each handle is allowed for decryption.
 *
 * @throws A {@link FhevmHandleError} If checkArguments is true and any handle is not a valid Bytes32Hex
 */
export async function isAllowedForDecryption(
  fhevm: Fhevm<FhevmChain>,
  parameters: IsAllowedForDecryptionArrayParameters,
): Promise<IsAllowedForDecryptionArrayReturnType>;

export async function isAllowedForDecryption(
  fhevm: Fhevm<FhevmChain>,
  parameters: IsAllowedForDecryptionSingleParameters,
): Promise<IsAllowedForDecryptionSingleReturnType>;

export async function isAllowedForDecryption(
  fhevm: Fhevm<FhevmChain>,
  parameters: IsAllowedForDecryptionArrayParameters | IsAllowedForDecryptionSingleParameters,
): Promise<IsAllowedForDecryptionArrayReturnType | IsAllowedForDecryptionSingleReturnType> {
  const { handles, options } = parameters;
  const isArray = Array.isArray(handles);
  const handlesArray = isArray ? handles : [handles];

  // By default, always check arguments
  if (options?.checkArguments !== false) {
    assertIsHandleLikeArray(handlesArray);
  }

  const rpcCalls = handlesArray.map(
    (h) => () =>
      isAllowedForDecryption_(fhevm, {
        address: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
        args: { handle: toHandle(h).bytes32Hex },
      }),
  );

  const results = await executeWithBatching(rpcCalls, fhevm.options.batchRpcCalls);

  return isArray ? results : (results[0] as unknown as boolean);
}

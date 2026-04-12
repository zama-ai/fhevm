import type { HandleLike } from '../../types/encryptedTypes.js';
import { assertIsChecksummedAddress } from '../../base/address.js';
import { executeWithBatching } from '../../base/promise.js';
import { assertIsHandleLike, toHandle } from '../../handle/FhevmHandle.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { persistAllowed as persistAllowed_ } from '../host/persistAllowed.js';

type HandleAddressPair = {
  readonly address: ChecksummedAddress;
  readonly handle: HandleLike;
};

export type PersistAllowedArrayParameters = {
  readonly handleAddressPairs: readonly HandleAddressPair[];
  readonly options?: { readonly checkArguments?: boolean } | undefined;
};

export type PersistAllowedArrayReturnType = boolean[];

export type PersistAllowedSingleParameters = {
  readonly handleAddressPairs: HandleAddressPair;
  readonly options?: { readonly checkArguments?: boolean } | undefined;
};

export type PersistAllowedSingleReturnType = boolean;

/**
 * Returns whether account is allowed to decrypt handle.
 *
 * @throws A {@link FhevmHandleError} If checkArguments is true and any handle is not a valid Bytes32Hex
 * @throws A {@link ChecksummedAddressError} If checkArguments is true and any address is not a valid checksummed address
 */
export async function persistAllowed(
  fhevm: Fhevm<FhevmChain>,
  parameters: PersistAllowedArrayParameters,
): Promise<PersistAllowedArrayReturnType>;

export async function persistAllowed(
  fhevm: Fhevm<FhevmChain>,
  parameters: PersistAllowedSingleParameters,
): Promise<PersistAllowedSingleReturnType>;

export async function persistAllowed(
  fhevm: Fhevm<FhevmChain>,
  parameters: PersistAllowedArrayParameters | PersistAllowedSingleParameters,
): Promise<PersistAllowedArrayReturnType | PersistAllowedSingleReturnType> {
  const { handleAddressPairs, options } = parameters;
  const isArray = Array.isArray(handleAddressPairs);
  const pairsArray: readonly HandleAddressPair[] = isArray
    ? handleAddressPairs
    : [handleAddressPairs];

  // By default, always check arguments
  if (options?.checkArguments !== false) {
    for (const p of pairsArray) {
      assertIsHandleLike(p.handle);
      assertIsChecksummedAddress(p.address, {});
    }
  }

  const rpcCalls = pairsArray.map(
    (p) => () =>
      persistAllowed_(fhevm, {
        address: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
        args: {
          handle: toHandle(p.handle).bytes32Hex,
          account: p.address,
        },
      }),
  );

  const results = await executeWithBatching(
    rpcCalls,
    fhevm.options.batchRpcCalls,
  );

  return isArray ? results : (results[0] as unknown as boolean);
}

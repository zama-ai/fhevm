import type { FhevmHandleLike } from "../../../types/fhevmHandle.js";
import {
  assertIsFhevmHandleLikeArray,
  toFhevmHandle,
} from "../../../handle/FhevmHandle.js";
import { executeWithBatching } from "../../../base/promise.js";
import type { Fhevm } from "../../../types/coreFhevmClient.js";
import { isAllowedForDecryption as isAllowedForDecryption_ } from "../../host/isAllowedForDecryption.js";
import type { FhevmChain } from "../../../types/fhevmChain.js";
import type { ChecksummedAddress } from "../../../types/primitives.js";

export type IsAllowedForDecryptionArrayParameters = {
  readonly handles: readonly FhevmHandleLike[];
  readonly options?: { readonly checkArguments?: boolean } | undefined;
};

export type IsAllowedForDecryptionArrayReturnType = boolean[];

export type IsAllowedForDecryptionSingleParameters = {
  readonly handles: FhevmHandleLike;
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
  parameters:
    | IsAllowedForDecryptionArrayParameters
    | IsAllowedForDecryptionSingleParameters,
): Promise<
  IsAllowedForDecryptionArrayReturnType | IsAllowedForDecryptionSingleReturnType
> {
  const { handles, options } = parameters;
  const isArray = Array.isArray(handles);
  const handlesArray = isArray ? handles : [handles];

  // By default, always check arguments
  if (options?.checkArguments !== false) {
    assertIsFhevmHandleLikeArray(handlesArray);
  }

  const rpcCalls = handlesArray.map(
    (h) => () =>
      isAllowedForDecryption_(fhevm, {
        address: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
        args: { handle: toFhevmHandle(h).bytes32Hex },
      }),
  );

  const results = await executeWithBatching(
    rpcCalls,
    fhevm.options?.batchRpcCalls,
  );

  return isArray ? results : (results[0] as unknown as boolean);
}

import { ACLPublicDecryptionError } from "../../../errors/ACLError.js";
import { toFhevmHandle } from "../../../handle/FhevmHandle.js";
import type { Fhevm } from "../../../types/coreFhevmClient.js";
import type { FhevmChain } from "../../../types/fhevmChain.js";
import type { FhevmHandleLike } from "../../../types/fhevmHandle.js";
import type { ChecksummedAddress } from "../../../types/primitives.js";
import { isAllowedForDecryption } from "./isAllowedForDecryption.js";

export type CheckAllowedForDecryptionParameters = {
  readonly handles: readonly FhevmHandleLike[] | FhevmHandleLike;
  readonly options?: { readonly checkArguments?: boolean };
};

/**
 * Throws ACLPublicDecryptionError if any handle is not allowed for decryption.
 *
 * @throws A {@link FhevmHandleError} If checkArguments is true and any handle is not a valid Bytes32Hex
 * @throws A {@link ACLPublicDecryptionError} If any handle is not allowed for public decryption
 */
export async function checkAllowedForDecryption(
  fhevm: Fhevm<FhevmChain>,
  parameters: CheckAllowedForDecryptionParameters,
): Promise<void> {
  const { handles, options } = parameters;

  const handlesArray = Array.isArray(handles) ? handles : [handles];
  const results = await isAllowedForDecryption(fhevm, {
    handles: handlesArray,
    options,
  });

  const failedHandles = handlesArray
    .filter((_, i) => results[i] !== true)
    .map((h) => toFhevmHandle(h).bytes32Hex);
  if (failedHandles.length > 0) {
    throw new ACLPublicDecryptionError({
      contractAddress: fhevm.chain.fhevm.contracts.acl
        .address as ChecksummedAddress,
      handles: failedHandles,
    });
  }
}

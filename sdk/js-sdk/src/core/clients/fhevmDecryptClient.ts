import type { WithDecryptAndRelayer } from "../types/coreFhevmRuntime.js";
import { decryptActions, type DecryptActions } from "./decorators/decrypt.js";
import {
  createCoreFhevm,
  extendCoreFhevm,
  type CreateCoreFhevmParameters,
} from "../runtime/CoreFhevm-p.js";
import type { FhevmChain } from "../types/fhevmChain.js";
import type { Fhevm, NativeClient } from "../types/coreFhevmClient.js";

export type FhevmDecryptClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithDecryptAndRelayer = WithDecryptAndRelayer,
  client extends NativeClient = NativeClient,
> = Fhevm<chain, runtime, client> & DecryptActions;

export function createFhevmDecryptClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithDecryptAndRelayer = WithDecryptAndRelayer,
  client extends NativeClient = NativeClient,
>(
  ownerToken: symbol,
  parameters: CreateCoreFhevmParameters<chain, runtime, client>,
): FhevmDecryptClient<chain, runtime, client> {
  const c = createCoreFhevm(ownerToken, parameters);
  return extendCoreFhevm(c, decryptActions);
}

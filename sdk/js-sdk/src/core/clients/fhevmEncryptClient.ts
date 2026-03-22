import type { WithEncryptAndRelayer } from "../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../types/fhevmChain.js";
import { encryptActions, type EncryptActions } from "./decorators/encrypt.js";
import {
  createCoreFhevm,
  extendCoreFhevm,
  type CreateCoreFhevmParameters,
} from "../runtime/CoreFhevm-p.js";
import {
  globalFhePkeActions,
  type GlobalFhePkeActions,
} from "./decorators/globalFhePke.js";
import type { Fhevm, NativeClient } from "../types/coreFhevmClient.js";

export type FhevmEncryptClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithEncryptAndRelayer = WithEncryptAndRelayer,
  client extends NativeClient = NativeClient,
> = Fhevm<chain, runtime, client> & EncryptActions & GlobalFhePkeActions;

export function createFhevmEncryptClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithEncryptAndRelayer = WithEncryptAndRelayer,
  client extends NativeClient = NativeClient,
>(
  ownerToken: symbol,
  parameters: CreateCoreFhevmParameters<chain, runtime, client>,
): FhevmEncryptClient<chain, runtime, client> {
  const c = createCoreFhevm(ownerToken, parameters);
  const cEnc = extendCoreFhevm(c, encryptActions);
  return extendCoreFhevm(cEnc, globalFhePkeActions);
}

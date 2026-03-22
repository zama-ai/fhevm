import type { WithAll } from "../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../types/fhevmChain.js";
import { encryptActions, type EncryptActions } from "./decorators/encrypt.js";
import {
  globalFhePkeActions,
  type GlobalFhePkeActions,
} from "./decorators/globalFhePke.js";
import { decryptActions, type DecryptActions } from "./decorators/decrypt.js";
import {
  createCoreFhevm,
  extendCoreFhevm,
  type CreateCoreFhevmParameters,
} from "../runtime/CoreFhevm-p.js";
import type { Fhevm, NativeClient } from "../types/coreFhevmClient.js";

export type FhevmClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithAll = WithAll,
  client extends NativeClient = NativeClient,
> = Fhevm<chain, runtime, client> &
  DecryptActions &
  EncryptActions &
  GlobalFhePkeActions;

export function createFhevmClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithAll = WithAll,
  client extends NativeClient = NativeClient,
>(
  ownerToken: symbol,
  parameters: CreateCoreFhevmParameters<chain, runtime, client>,
): FhevmClient<chain, runtime, client> {
  const c = createCoreFhevm(ownerToken, parameters);
  const cEnc = extendCoreFhevm(c, encryptActions);
  const cDec = extendCoreFhevm(cEnc, decryptActions);
  return extendCoreFhevm(cDec, globalFhePkeActions);
}

import type { PublicClient, Chain, Transport } from "viem";
import { encryptModule } from "../../core/modules/encrypt/module/index.js";
import { relayerModule } from "../../core/modules/relayer/module/index.js";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { WithEncryptAndRelayer } from "../../core/types/coreFhevmRuntime.js";
import {
  getViemRuntime,
  PRIVATE_VIEM_TOKEN,
} from "../internal/viem-p.js";
import {
  createFhevmEncryptClient as createFhevmEncryptClient_,
  type FhevmEncryptClient,
} from "../../core/clients/fhevmEncryptClient.js";

export function createFhevmEncryptClient<
  chain extends FhevmChain,
  provider extends PublicClient<Transport, Chain>,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmEncryptClient<chain, WithEncryptAndRelayer, provider> {
  const runtime: WithEncryptAndRelayer = getViemRuntime()
    .extend(encryptModule)
    .extend(relayerModule);

  return createFhevmEncryptClient_(PRIVATE_VIEM_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.provider,
  });
}

import type { PublicClient, Chain, Transport } from "viem";
import { decryptModule } from "../../core/modules/decrypt/module/index.js";
import { relayerModule } from "../../core/modules/relayer/module/index.js";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { WithDecryptAndRelayer } from "../../core/types/coreFhevmRuntime.js";
import {
  createFhevmDecryptClient as createFhevmDecryptClient_,
  type FhevmDecryptClient,
} from "../../core/clients/fhevmDecryptClient.js";
import {
  getViemRuntime,
  PRIVATE_VIEM_TOKEN,
} from "../internal/viem-p.js";

export function createFhevmDecryptClient<
  chain extends FhevmChain,
  provider extends PublicClient<Transport, Chain>,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmDecryptClient<chain, WithDecryptAndRelayer, provider> {
  const runtime: WithDecryptAndRelayer = getViemRuntime()
    .extend(decryptModule)
    .extend(relayerModule);

  return createFhevmDecryptClient_(PRIVATE_VIEM_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.provider,
  });
}

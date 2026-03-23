import type { PublicClient, Chain, Transport } from "viem";
import { encryptModule } from "../../core/modules/encrypt/module/index.js";
import { relayerModule } from "../../core/modules/relayer/module/index.js";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { WithAll } from "../../core/types/coreFhevmRuntime.js";
import {
  getViemRuntime,
  PRIVATE_VIEM_TOKEN,
} from "../internal/viem-p.js";
import {
  createFhevmClient as createFhevmClient_,
  type FhevmClient,
} from "../../core/clients/fhevmClient.js";
import { decryptModule } from "../../core/modules/decrypt/module/index.js";

export function createFhevmClient<
  chain extends FhevmChain,
  provider extends PublicClient<Transport, Chain>,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmClient<chain, WithAll, provider> {
  const runtime: WithAll = getViemRuntime()
    .extend(encryptModule)
    .extend(decryptModule)
    .extend(relayerModule);

  return createFhevmClient_(PRIVATE_VIEM_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.provider,
  });
}

import type { ethers as EthersT } from "ethers";
import { encryptModule } from "../../core/modules/encrypt/module/index.js";
import { relayerModule } from "../../core/modules/relayer/module/index.js";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { WithEncryptAndRelayer } from "../../core/types/coreFhevmRuntime.js";
import {
  getEthersRuntime,
  PRIVATE_ETHERS_TOKEN,
} from "../internal/ethers-p.js";
import {
  createFhevmEncryptClient as createFhevmEncryptClient_,
  type FhevmEncryptClient,
} from "../../core/clients/fhevmEncryptClient.js";

export function createFhevmEncryptClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmEncryptClient<chain, WithEncryptAndRelayer, provider> {
  const runtime: WithEncryptAndRelayer = getEthersRuntime()
    .extend(encryptModule)
    .extend(relayerModule);

  return createFhevmEncryptClient_(PRIVATE_ETHERS_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.provider,
  });
}

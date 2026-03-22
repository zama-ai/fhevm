import type { ethers as EthersT } from "ethers";
import { encryptModule } from "../../core/modules/encrypt/module/index.js";
import { relayerModule } from "../../core/modules/relayer/module/index.js";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { WithAll } from "../../core/types/coreFhevmRuntime.js";
import {
  getEthersRuntime,
  PRIVATE_ETHERS_TOKEN,
} from "../internal/ethers-p.js";
import {
  createFhevmClient as createFhevmClient_,
  type FhevmClient,
} from "../../core/clients/fhevmClient.js";
import { decryptModule } from "../../core/modules/decrypt/module/index.js";

export function createFhevmClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmClient<chain, WithAll, provider> {
  const runtime: WithAll = getEthersRuntime()
    .extend(encryptModule)
    .extend(decryptModule)
    .extend(relayerModule);

  return createFhevmClient_(PRIVATE_ETHERS_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.provider,
  });
}

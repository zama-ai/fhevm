import type { ethers as EthersT } from "ethers";
import { decryptModule } from "../../core/modules/decrypt/module/index.js";
import { relayerModule } from "../../core/modules/relayer/module/index.js";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { WithDecryptAndRelayer } from "../../core/types/coreFhevmRuntime.js";
import {
  createFhevmDecryptClient as createFhevmDecryptClient_,
  type FhevmDecryptClient,
} from "../../core/clients/fhevmDecryptClient.js";
import {
  getEthersRuntime,
  PRIVATE_ETHERS_TOKEN,
} from "../internal/ethers-p.js";

export function createFhevmDecryptClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmDecryptClient<chain, WithDecryptAndRelayer, provider> {
  const runtime: WithDecryptAndRelayer = getEthersRuntime()
    .extend(decryptModule)
    .extend(relayerModule);

  return createFhevmDecryptClient_(PRIVATE_ETHERS_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.provider,
  });
}

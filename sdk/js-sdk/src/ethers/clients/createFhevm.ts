import type { ethers as EthersT } from "ethers";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { FhevmRuntime } from "../../core/types/coreFhevmRuntime.js";
import { createFhevm as createFhevm_ } from "../../core/clients/fhevm.js";
import {
  getEthersRuntime,
  PRIVATE_ETHERS_TOKEN,
} from "../internal/ethers-p.js";
import type { Fhevm } from "../../core/types/coreFhevmClient.js";

export function createFhevm<
  chain extends FhevmChain | undefined = undefined,
  provider extends EthersT.ContractRunner | undefined = undefined,
>(parameters?: {
  readonly provider?: provider;
  readonly chain?: chain;
}): Fhevm<chain, FhevmRuntime, provider> {
  const runtime = getEthersRuntime();

  return createFhevm_(PRIVATE_ETHERS_TOKEN, {
    chain: parameters?.chain,
    runtime,
    client: parameters?.provider,
  });
}

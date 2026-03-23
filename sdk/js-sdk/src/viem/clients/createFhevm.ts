import type { PublicClient, Chain, Transport } from "viem";
import type { FhevmChain } from "../../core/types/fhevmChain.js";
import type { FhevmRuntime } from "../../core/types/coreFhevmRuntime.js";
import { createFhevm as createFhevm_ } from "../../core/clients/fhevm.js";
import {
  getViemRuntime,
  PRIVATE_VIEM_TOKEN,
} from "../internal/viem-p.js";
import type { Fhevm } from "../../core/types/coreFhevmClient.js";

export function createFhevm<
  chain extends FhevmChain | undefined = undefined,
  provider extends PublicClient<Transport, Chain> | undefined = undefined,
>(parameters?: {
  readonly provider?: provider;
  readonly chain?: chain;
}): Fhevm<chain, FhevmRuntime, provider> {
  const runtime = getViemRuntime();

  return createFhevm_(PRIVATE_VIEM_TOKEN, {
    chain: parameters?.chain,
    runtime,
    client: parameters?.provider,
  });
}

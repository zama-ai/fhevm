import type { FhevmChain } from "@fhevm/sdk/chains";
import type { Chain, Hex } from "viem";

import type { NetworkName } from "../types";

export type NetworkConfig = Readonly<{
  fhevmChain: FhevmChain;
  hostChain: Chain;
  defaultRpcUrl: string;
  envRpcUrl: string;
  fheTestAddress: Hex;
}>;

export type ClientOptions = Readonly<{
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
  contractAddress?: Hex;
}>;

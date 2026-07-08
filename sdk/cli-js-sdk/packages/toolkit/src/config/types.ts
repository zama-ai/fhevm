import type { FhevmChain } from "@fhevm/sdk/chains";
import type { setFhevmRuntimeConfig } from "@fhevm/sdk/viem";
import type { Chain, Hex } from "viem";

import type { NetworkName } from "../types";

type FhevmRuntimeConfig = Parameters<typeof setFhevmRuntimeConfig>[0];

export type NetworkConfig = Readonly<{
  fhevmChain: FhevmChain;
  hostChain: Chain;
  defaultRpcUrl: string;
  envRpcUrl: string;
  fheTestAddress: Hex;
  runtime?: Readonly<{
    moduleVersions?: FhevmRuntimeConfig["moduleVersions"];
  }>;
}>;

export type ClientOptions = Readonly<{
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
  contractAddress?: Hex;
}>;

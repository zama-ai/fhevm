import type { Command } from "@commander-js/extra-typings";

import { DEFAULT_NETWORK } from "../config";
import type { NetworkName } from "../types";

export type GlobalOptions = Readonly<{
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
}>;

export const getGlobalOptions = (command: Command): GlobalOptions => {
  const options = command.optsWithGlobals() as Partial<GlobalOptions>;
  return {
    network: options.network ?? DEFAULT_NETWORK,
    relayerUrl: options.relayerUrl,
    rpcUrl: options.rpcUrl,
  };
};

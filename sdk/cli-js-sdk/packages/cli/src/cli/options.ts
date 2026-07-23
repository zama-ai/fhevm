import type { Command } from "@commander-js/extra-typings";

import { DEFAULT_NETWORK, type NetworkName } from "@cli-fhevm-sdk/toolkit/types";

/**
 * Global options shared by every command.
 *
 * Commander exposes inherited options from the current action context, whether
 * users pass them before or after subcommands.
 */
export type GlobalOptions = Readonly<{
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
}>;

/** Reads resolved global options from a command action context. */
export const getGlobalOptions = (command: Command): GlobalOptions => {
  const options = command.optsWithGlobals() as Partial<GlobalOptions>;
  return {
    network: options.network ?? DEFAULT_NETWORK,
    relayerUrl: options.relayerUrl,
    rpcUrl: options.rpcUrl,
  };
};

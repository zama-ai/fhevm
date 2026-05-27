import type { Command } from "@commander-js/extra-typings";

import { initFheTest } from "../../flows";
import { FHE_VALUE_TYPES } from "../../types";
import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import { parseAddress, parsePrivateKey, parseValueType } from "../parsers";
import { createProgressReporter } from "../progress";

export const registerFheTestCommands = (program: Command): void => {
  const supportedValueTypes = FHE_VALUE_TYPES.join(", ");
  const fheTestCommand = program
    .command("fhe-test")
    .description("FHETest contract utilities");

  fheTestCommand
    .command("init")
    .description("Initialize publicly decryptable FHETest handles")
    .option(
      "-t, --type <type>",
      `initialize one type (${supportedValueTypes}); defaults to all`,
      parseValueType,
    )
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .option("--force", "overwrite existing handles", false)
    .option(
      "--private-key <privateKey>",
      "wallet private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
    .action(async (options, command) => {
      const globals = getGlobalOptions(command);
      const result = await initFheTest({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        type: options.type,
        contractAddress: options.contract,
        force: options.force,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });
};

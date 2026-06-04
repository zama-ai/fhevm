import type { Command } from "@commander-js/extra-typings";

import { getFheTestInfo, initFheTest, inspectFheTest } from "../../flows";
import { FHE_VALUE_TYPES } from "../../types";
import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import {
  parseAddress,
  parseBytes32,
  parsePrivateKey,
  parseValueType,
} from "../parsers";
import { createProgressReporter } from "../progress";

export const registerFheTestCommands = (program: Command): void => {
  const supportedValueTypes = FHE_VALUE_TYPES.join(", ");
  const fheTestCommand = program
    .command("fhe-test")
    .description("FHETest contract utilities");

  fheTestCommand
    .command("info")
    .description("Show FHETest contract and network metadata")
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .action(async (options, command) => {
      const globals = getGlobalOptions(command);
      const result = await getFheTestInfo({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        contractAddress: options.contract,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });

  fheTestCommand
    .command("inspect")
    .description("Inspect FHETest account/type state or a raw handle")
    .option(
      "-t, --type <type>",
      `value type for account inspection (${supportedValueTypes})`,
      parseValueType,
    )
    .option(
      "--account <address>",
      "account used for FHETest.getHandleOf; defaults to wallet address",
      parseAddress,
    )
    .option("--handle <handle>", "raw encrypted handle to inspect", parseBytes32)
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .option(
      "--private-key <privateKey>",
      "wallet private key for default account; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option(
      "--mnemonic <mnemonic>",
      "wallet mnemonic for default account; falls back to MNEMONIC",
    )
    .action(async (options, command) => {
      if (options.handle) {
        if (
          options.type ||
          options.account ||
          options.privateKey ||
          options.mnemonic
        ) {
          throw new Error(
            "Use either --handle, or account/type inspection options, not both.",
          );
        }
      } else if (!options.type) {
        command.outputHelp();
        return;
      }

      const globals = getGlobalOptions(command);
      const result = await inspectFheTest({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        type: options.type,
        account: options.account,
        handle: options.handle,
        contractAddress: options.contract,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });

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

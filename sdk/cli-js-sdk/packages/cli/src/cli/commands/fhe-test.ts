import type { Command } from "@commander-js/extra-typings";

import {
  FHE_TEST_OPERATIONS,
  getFheTestOperationType,
  FHE_VALUE_TYPES,
} from "@cli-fhevm-sdk/toolkit/types";
import { parseClearValue, serializeValue } from "@cli-fhevm-sdk/toolkit/values";
import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import {
  collectValueType,
  parseAddress,
  parseBytes32,
  parsePrivateKey,
  parseValueType,
} from "../parsers";
import { createProgressReporter } from "../progress";

/** Registers FHETest utility commands for info, inspect, init, and operations. */
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
      const { getFheTestInfo } = await import("@cli-fhevm-sdk/toolkit/flows/fhe-test/info");
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

      const { inspectFheTest } = await import("@cli-fhevm-sdk/toolkit/flows/fhe-test/inspect");
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
      `initialize one or more types; repeat for multiple (${supportedValueTypes}); defaults to all`,
      collectValueType,
    )
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .option(
      "--bulk",
      "initialize all types in one FHETest.initFheTest transaction",
      false,
    )
    .option("--force", "overwrite existing handles", false)
    .option(
      "--private-key <privateKey>",
      "wallet private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
    .action(async (options, command) => {
      if (options.bulk && options.type?.length) {
        throw new Error("fhe-test init --bulk cannot be used with --type.");
      }

      const { initFheTest } = await import("@cli-fhevm-sdk/toolkit/flows/fhe-test/init");
      const globals = getGlobalOptions(command);
      const result = await initFheTest({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        types: options.type,
        contractAddress: options.contract,
        bulk: options.bulk,
        force: options.force,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });

  const opCommand = fheTestCommand
    .command("op")
    .description("Run FHETest on-chain FHE operation demos");

  for (const operation of FHE_TEST_OPERATIONS) {
    const type = getFheTestOperationType(operation);
    opCommand
      .command(operation)
      .description(`Run FHETest ${operation} using the caller's stored ${type}`)
      .option(
        "--value <value>",
        "right-hand clear value to encrypt; defaults to a random value",
      )
      .option(
        "--contract <address>",
        "FHETest contract address override",
        parseAddress,
      )
      .option(
        "--public",
        "make the resulting handle publicly decryptable",
        false,
      )
      .option(
        "--private-key <privateKey>",
        "wallet private key; falls back to PRIVATE_KEY",
        parsePrivateKey,
      )
      .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
      .action(async (options, command) => {
        const { runFheTestOperation } = await import("@cli-fhevm-sdk/toolkit/flows/fhe-test/op");
        const globals = getGlobalOptions(command);
        const value =
          options.value === undefined
            ? undefined
            : parseClearValue(type, options.value);
        const result = await runFheTestOperation({
          network: globals.network,
          relayerUrl: globals.relayerUrl,
          rpcUrl: globals.rpcUrl,
          operation,
          value,
          contractAddress: options.contract,
          makePublic: options.public,
          privateKey: options.privateKey,
          mnemonic: options.mnemonic,
          onProgress: createProgressReporter(),
        });

        printJson({
          ...result,
          inputValues: result.inputValues.map(serializeValue),
        });
      });
  }
};

#!/usr/bin/env -S node --env-file=.env --import tsx
import { Command } from "@commander-js/extra-typings";
import { consola } from "consola";

import { DEFAULT_NETWORK } from "./src/config";
import { getGlobalOptions } from "./src/cli/options";
import {
  collectHandle,
  parseAddress,
  parseNetwork,
  parsePrivateKey,
  parseValueType,
} from "./src/cli/parsers";
import { printJson } from "./src/cli/output";
import { createProgressReporter } from "./src/cli/progress";
import {
  freshPublicDecrypt,
  initFheTest,
  makePublicAndDecrypt,
  publicDecrypt,
  requestInputProof,
} from "./src/flows";
import { FHE_VALUE_TYPES } from "./src/types";
import { parseClearValue, serializeValue } from "./src/values";

const program = new Command()
  .name("cli-relayer-sdk")
  .description("CLI for @fhevm/sdk flows against FHETest")
  .version("0.1.0")
  .option(
    "-n, --network <network>",
    "network to target",
    parseNetwork,
    DEFAULT_NETWORK,
  )
  .option(
    "--relayer-url <url>",
    "relayer base URL override, for example localhost:3000",
  )
  .option("--rpc-url <url>", "host chain RPC URL override");

const supportedValueTypes = FHE_VALUE_TYPES.join(", ");

program
  .command("input-proof")
  .description(
    "Generate encrypted inputs and request relayer verified input proof",
  )
  .option(
    "-t, --type <type>",
    `value type (${supportedValueTypes})`,
    parseValueType,
    "bool",
  )
  .option("--value <value>", "clear value to encrypt; defaults to a random value")
  .option(
    "--contract <address>",
    "contract address bound into the proof",
    parseAddress,
  )
  .option("--user <address>", "user address bound into the proof", parseAddress)
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const value =
      options.value === undefined
        ? undefined
        : parseClearValue(options.type, options.value);
    const result = await requestInputProof({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      type: options.type,
      value,
      contractAddress: options.contract,
      userAddress: options.user,
      onProgress: createProgressReporter(),
    });

    printJson({
      contractAddress: result.contractAddress,
      userAddress: result.userAddress,
      values: result.values.map(serializeValue),
      encryptedValues: result.encryptedValues,
      inputProof: result.inputProof,
    });
  });

const publicDecryptCommand = program
  .command("public-decrypt")
  .description(
    `Public decrypt flows. Supported types: ${supportedValueTypes}`,
  );

publicDecryptCommand
  .command("cached")
  .description(
    "Public decrypt an FHETest handle from account/type, or direct handles",
  )
  .option(
    "-t, --type <type>",
    `value type (${supportedValueTypes})`,
    parseValueType,
    "bool",
  )
  .option(
    "--account <address>",
    "account used for FHETest.getHandleOf",
    parseAddress,
  )
  .option(
    "--contract <address>",
    "FHETest contract address override",
    parseAddress,
  )
  .option(
    "--handle <handle>",
    "encrypted handle to decrypt directly; repeat for multiple",
    collectHandle,
    [],
  )
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
    parsePrivateKey,
  )
  .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const result = await publicDecrypt({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      type: options.type,
      contractAddress: options.contract,
      account: options.account,
      handles: options.handle,
      privateKey: options.privateKey,
      mnemonic: options.mnemonic,
      onProgress: createProgressReporter(),
    });

    printJson(result);
  });

publicDecryptCommand
  .command("fresh")
  .description(
    "Encrypt a new value, store it in FHETest as public, then public decrypt it",
  )
  .option(
    "-t, --type <type>",
    `value type to encrypt (${supportedValueTypes})`,
    parseValueType,
    "bool",
  )
  .option("--value <value>", "clear value to encrypt; defaults to random")
  .option(
    "--contract <address>",
    "FHETest contract address override",
    parseAddress,
  )
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
    parsePrivateKey,
  )
  .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const value =
      options.value === undefined
        ? undefined
        : parseClearValue(options.type, options.value);
    const result = await freshPublicDecrypt({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      type: options.type,
      value,
      contractAddress: options.contract,
      privateKey: options.privateKey,
      mnemonic: options.mnemonic,
      onProgress: createProgressReporter(),
    });

    printJson({
      transactionHash: result.transactionHash,
      inputValues: result.inputValues.map(serializeValue),
      inputProof: result.inputProof,
      handle: result.handle,
      encryptedValues: result.encryptedValues,
      clearValues: result.clearValues,
      abiEncodedCleartexts: result.abiEncodedCleartexts,
      decryptionProof: result.decryptionProof,
    });
  });

publicDecryptCommand
  .command("make-public")
  .description("Make the caller's stored FHETest handle public, then decrypt it")
  .option(
    "-t, --type <type>",
    `value type (${supportedValueTypes})`,
    parseValueType,
    "bool",
  )
  .option(
    "--contract <address>",
    "FHETest contract address override",
    parseAddress,
  )
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
    parsePrivateKey,
  )
  .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const result = await makePublicAndDecrypt({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      type: options.type,
      contractAddress: options.contract,
      privateKey: options.privateKey,
      mnemonic: options.mnemonic,
      onProgress: createProgressReporter(),
    });

    printJson(result);
  });

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

program.parseAsync().catch((error: unknown) => {
  consola.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});

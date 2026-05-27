#!/usr/bin/env -S node --env-file=.env --import tsx
import { Command, InvalidArgumentError } from "@commander-js/extra-typings";
import { consola } from "consola";
import type { Hex } from "viem";

import { DEFAULT_NETWORK } from "./src/config";
import {
  freshPublicDecrypt,
  initFheTest,
  makePublicAndDecrypt,
  publicDecrypt,
  requestInputProof,
} from "./src/flows";
import {
  FHE_VALUE_TYPES,
  NETWORKS,
  type FheValueType,
  type NetworkName,
} from "./src/types";
import { normalizeHexArray, parseClearValue, serializeValue } from "./src/values";

const parseNetwork = (value: string): NetworkName => {
  if (NETWORKS.includes(value as NetworkName)) return value as NetworkName;
  throw new InvalidArgumentError(
    `Unsupported network "${value}". Supported: ${NETWORKS.join(", ")}`,
  );
};

const parseValueType = (value: string): FheValueType => {
  if (FHE_VALUE_TYPES.includes(value as FheValueType)) {
    return value as FheValueType;
  }
  throw new InvalidArgumentError(
    `Unsupported type "${value}". Supported: ${FHE_VALUE_TYPES.join(", ")}`,
  );
};

const collectHex = (value: string, previous: string[] = []): string[] => [
  ...previous,
  value,
];

const printJson = (value: unknown) => {
  process.stdout.write(
    JSON.stringify(
      value,
      (_key, item) => (typeof item === "bigint" ? item.toString() : item),
      2,
    ) + "\n",
  );
};

const createProgressReporter = () => {
  const startedAt = performance.now();
  return (message: string) => {
    const elapsedSeconds = ((performance.now() - startedAt) / 1000).toFixed(1);
    process.stderr.write(`[${elapsedSeconds}s] ${message}\n`);
  };
};

type GlobalOptions = Readonly<{
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
}>;

const getGlobalOptions = (command: Command): GlobalOptions => {
  const options = command.optsWithGlobals() as Partial<GlobalOptions>;
  return {
    network: options.network ?? DEFAULT_NETWORK,
    relayerUrl: options.relayerUrl,
    rpcUrl: options.rpcUrl,
  };
};

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
  .option("--contract <address>", "contract address bound into the proof")
  .option("--user <address>", "user address bound into the proof")
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
      contractAddress: options.contract as Hex | undefined,
      userAddress: options.user as Hex | undefined,
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
  .option("--account <address>", "account used for FHETest.getHandleOf")
  .option("--contract <address>", "FHETest contract address override")
  .option(
    "--handle <handle>",
    "encrypted handle to decrypt directly; repeat for multiple",
    collectHex,
    [],
  )
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
  )
  .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const result = await publicDecrypt({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      type: options.type,
      contractAddress: options.contract as Hex | undefined,
      account: options.account as Hex | undefined,
      handles: normalizeHexArray(options.handle),
      privateKey: options.privateKey as Hex | undefined,
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
  .option("--contract <address>", "FHETest contract address override")
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
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
      contractAddress: options.contract as Hex | undefined,
      privateKey: options.privateKey as Hex | undefined,
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
  .option("--contract <address>", "FHETest contract address override")
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
  )
  .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
  .action(async (options, command) => {
  const globals = getGlobalOptions(command);
  const result = await makePublicAndDecrypt({
    network: globals.network,
    relayerUrl: globals.relayerUrl,
    rpcUrl: globals.rpcUrl,
    type: options.type,
    contractAddress: options.contract as Hex | undefined,
    privateKey: options.privateKey as Hex | undefined,
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
  .option("--contract <address>", "FHETest contract address override")
  .option("--force", "overwrite existing handles", false)
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
  )
  .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
  .action(async (options, command) => {
  const globals = getGlobalOptions(command);
  const result = await initFheTest({
    network: globals.network,
    relayerUrl: globals.relayerUrl,
    rpcUrl: globals.rpcUrl,
    type: options.type,
    contractAddress: options.contract as Hex | undefined,
    force: options.force,
    privateKey: options.privateKey as Hex | undefined,
    mnemonic: options.mnemonic,
    onProgress: createProgressReporter(),
  });

  printJson(result);
});

program.parseAsync().catch((error: unknown) => {
  consola.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});

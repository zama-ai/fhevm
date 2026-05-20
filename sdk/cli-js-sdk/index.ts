#!/usr/bin/env bun
import { Command, InvalidArgumentError } from "@commander-js/extra-typings";
import { consola } from "consola";
import type { Hex } from "viem";

import {
  DEFAULT_NETWORK,
  TESTNET_RELAYER_SDK_TEST_CONTRACT,
} from "./src/config";
import {
  freshPublicDecrypt,
  publicDecrypt,
  requestInputProof,
} from "./src/flows";
import {
  DECRYPT_TYPES,
  NETWORKS,
  type DecryptType,
  type NetworkName,
} from "./src/types";
import { normalizeHexArray, serializeValue } from "./src/values";

const parseNetwork = (value: string): NetworkName => {
  if (NETWORKS.includes(value as NetworkName)) return value as NetworkName;
  throw new InvalidArgumentError(
    `Unsupported network "${value}". Supported: ${NETWORKS.join(", ")}`,
  );
};

const parseDecryptType = (value: string): DecryptType => {
  if (DECRYPT_TYPES.includes(value as DecryptType)) return value as DecryptType;
  throw new InvalidArgumentError(
    `Unsupported type "${value}". Supported: ${DECRYPT_TYPES.join(", ")}`,
  );
};

const collectHex = (value: string, previous: string[] = []): string[] => [
  ...previous,
  value,
];

const printJson = (value: unknown) => {
  console.log(
    JSON.stringify(
      value,
      (_key, item) => (typeof item === "bigint" ? item.toString() : item),
      2,
    ),
  );
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
  .description("CLI for @fhevm/sdk input proof and public decrypt flows")
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

program
  .command("input-proof")
  .description(
    "Generate encrypted inputs and request relayer verified input proof",
  )
  .option("--contract <address>", "contract address bound into the proof")
  .option("--user <address>", "user address bound into the proof")
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const result = await requestInputProof({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      contractAddress: options.contract as Hex | undefined,
      userAddress: options.user as Hex | undefined,
    });

    printJson({
      contractAddress: result.contractAddress,
      userAddress: result.userAddress,
      values: result.values.map(serializeValue),
      encryptedValues: result.encryptedValues,
      inputProof: result.inputProof,
    });
  });

const supportedDecryptTypes = DECRYPT_TYPES.join(", ");
const publicDecryptCommand = program
  .command("public-decrypt")
  .description(
    `Public decrypt flows. Supported types: ${supportedDecryptTypes}`,
  );

publicDecryptCommand
  .command("cached")
  .description("Public decrypt existing publicly decryptable handles")
  .option(
    "-t, --type <type>",
    `value type (${supportedDecryptTypes})`,
    parseDecryptType,
    "bool",
  )
  .option(
    "--handle <handle>",
    "encrypted handle to decrypt; repeat for multiple",
    collectHex,
    [],
  )
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const result = await publicDecrypt({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      decryptType: options.type,
      handles: normalizeHexArray(options.handle),
    });

    printJson(result);
  });

publicDecryptCommand
  .command("fresh")
  .description(
    "Encrypt new values, make them publicly decryptable on-chain, then public decrypt them",
  )
  .option(
    "-t, --type <type>",
    `value type to encrypt (${supportedDecryptTypes})`,
    parseDecryptType,
    "bool",
  )
  .option(
    "--contract <address>",
    "contract to call",
    TESTNET_RELAYER_SDK_TEST_CONTRACT,
  )
  .option(
    "--private-key <privateKey>",
    "wallet private key; falls back to PRIVATE_KEY",
  )
  .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
  .action(async (options, command) => {
    const globals = getGlobalOptions(command);
    const result = await freshPublicDecrypt({
      network: globals.network,
      relayerUrl: globals.relayerUrl,
      rpcUrl: globals.rpcUrl,
      decryptType: options.type,
      contractAddress: options.contract as Hex,
      privateKey: options.privateKey as Hex | undefined,
      mnemonic: options.mnemonic,
    });

    printJson({
      transactionHash: result.transactionHash,
      inputValues: result.inputValues.map(serializeValue),
      inputProof: result.inputProof,
      encryptedValues: result.encryptedValues,
      clearValues: result.clearValues,
      abiEncodedCleartexts: result.abiEncodedCleartexts,
      decryptionProof: result.decryptionProof,
    });
  });

program.parseAsync().catch((error: unknown) => {
  consola.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});

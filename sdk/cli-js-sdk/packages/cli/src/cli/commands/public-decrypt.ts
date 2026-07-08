import type { Command } from "@commander-js/extra-typings";

import { FHE_VALUE_TYPES } from "@cli-fhevm-sdk/toolkit/types";
import { parseClearValue, serializeValue } from "@cli-fhevm-sdk/toolkit/values";
import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import {
  collectHandle,
  collectValueType,
  parseAddress,
  parsePrivateKey,
  parseValueType,
} from "../parsers";
import { createProgressReporter } from "../progress";

/** Registers public decrypt commands for cached, fresh, and make-public flows. */
export const registerPublicDecryptCommands = (program: Command): void => {
  const supportedValueTypes = FHE_VALUE_TYPES.join(", ");
  const publicDecryptCommand = program
    .command("public-decrypt")
    .description(
      `Public decrypt flows. Supported types: ${supportedValueTypes}`,
    );

  publicDecryptCommand
    .command("cached")
    .description(
      "Public decrypt FHETest handles from account/type slots, or direct handles",
    )
    .option(
      "-t, --type <type>",
      `stored value type to read; repeat for multiple (${supportedValueTypes})`,
      collectValueType,
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
      const { publicDecrypt } = await import("@cli-fhevm-sdk/toolkit/flows/public-decrypt/cached");
      const globals = getGlobalOptions(command);
      const result = await publicDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        types: options.type,
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
      const { freshPublicDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/public-decrypt/fresh"
      );
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
    .description(
      "Make the caller's stored FHETest handle public, then decrypt it",
    )
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
      const { makePublicAndDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/public-decrypt/make-public"
      );
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
};

import type { Command } from "@commander-js/extra-typings";

import { FHE_VALUE_TYPES } from "@cli-fhevm-sdk/toolkit/types";
import { parseClearValue, serializeValue } from "@cli-fhevm-sdk/toolkit/values";
import { getGlobalOptions } from "../options";
import { printJson, writeJsonFile } from "../output";
import {
  collectHandle,
  collectValueType,
  daysToSeconds,
  parseAddress,
  parsePositiveInteger,
  parsePrivateKey,
  parseValueType,
} from "../parsers";
import { createProgressReporter } from "../progress";

const DEFAULT_PERMIT_DURATION_DAYS = 1;

/** Registers owner-signed user decrypt commands. */
export const registerUserDecryptCommands = (program: Command): void => {
  const supportedValueTypes = FHE_VALUE_TYPES.join(", ");
  const userDecryptCommand = program
    .command("user-decrypt")
    .description(
      `Decrypt existing private handles as the signing wallet, from any contract. Supported types: ${supportedValueTypes}`,
    );

  userDecryptCommand
    .command("direct")
    .description(
      "Decrypt handles passed directly via --handle as the signing wallet",
    )
    .option(
      "--handle <handle>",
      "encrypted handle to decrypt directly; repeat for multiple",
      collectHandle,
      [],
    )
    .option(
      "--contract <address>",
      "contract address paired with the handles for ACL verification; defaults to the FHETest contract",
      parseAddress,
    )
    .option(
      "--duration-days <days>",
      "decryption permit duration in days",
      parsePositiveInteger,
      DEFAULT_PERMIT_DURATION_DAYS,
    )
    .option(
      "--artifact <path>",
      "write a sensitive user-decrypt validation artifact",
    )
    .option(
      "--private-key <privateKey>",
      "wallet private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
    .action(async (options, command) => {
      if (options.handle.length === 0) {
        command.help();
        return;
      }
      const { userDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/user-decrypt/direct"
      );
      const globals = getGlobalOptions(command);
      const result = await userDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        contractAddress: options.contract,
        handles: options.handle,
        durationSeconds: daysToSeconds(options.durationDays),
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        includeValidationArtifact: options.artifact !== undefined,
        onProgress: createProgressReporter(),
      });

      if (options.artifact !== undefined) {
        await writeJsonFile(options.artifact, result.validationArtifact);
      }
      const { validationArtifact: _validationArtifact, ...publicResult } = result;
      printJson(publicResult);
    });

  userDecryptCommand
    .command("stored")
    .description(
      "Demo: user decrypt FHETest handles stored in the wallet's type slots",
    )
    .option(
      "-t, --type <type>",
      `stored value type to read; repeat for multiple (${supportedValueTypes})`,
      collectValueType,
    )
    .option(
      "--contract <address>",
      "FHETest contract address override",
      parseAddress,
    )
    .option(
      "--duration-days <days>",
      "decryption permit duration in days",
      parsePositiveInteger,
      DEFAULT_PERMIT_DURATION_DAYS,
    )
    .option(
      "--artifact <path>",
      "write a sensitive user-decrypt validation artifact",
    )
    .option(
      "--private-key <privateKey>",
      "wallet private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
    .action(async (options, command) => {
      const { storedUserDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/user-decrypt/stored"
      );
      const globals = getGlobalOptions(command);
      const result = await storedUserDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        types: options.type,
        contractAddress: options.contract,
        durationSeconds: daysToSeconds(options.durationDays),
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        includeValidationArtifact: options.artifact !== undefined,
        onProgress: createProgressReporter(),
      });

      if (options.artifact !== undefined) {
        await writeJsonFile(options.artifact, result.validationArtifact);
      }
      const { validationArtifact: _validationArtifact, ...publicResult } = result;
      printJson(publicResult);
    });

  userDecryptCommand
    .command("fresh")
    .description("Demo: encrypt a new value, store it in FHETest, then user decrypt it")
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
      "--duration-days <days>",
      "decryption permit duration in days",
      parsePositiveInteger,
      DEFAULT_PERMIT_DURATION_DAYS,
    )
    .option(
      "--artifact <path>",
      "write a sensitive user-decrypt validation artifact",
    )
    .option(
      "--private-key <privateKey>",
      "wallet private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
    .action(async (options, command) => {
      const { freshUserDecrypt } = await import(
        "@cli-fhevm-sdk/toolkit/flows/user-decrypt/fresh"
      );
      const globals = getGlobalOptions(command);
      const value =
        options.value === undefined
          ? undefined
          : parseClearValue(options.type, options.value);
      const result = await freshUserDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        type: options.type,
        value,
        contractAddress: options.contract,
        durationSeconds: daysToSeconds(options.durationDays),
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        includeValidationArtifact: options.artifact !== undefined,
        onProgress: createProgressReporter(),
      });

      if (options.artifact !== undefined) {
        await writeJsonFile(options.artifact, result.validationArtifact);
      }
      printJson({
        transactionHash: result.transactionHash,
        inputValues: result.inputValues.map(serializeValue),
        inputProof: result.inputProof,
        handle: result.handle,
        contractAddress: result.contractAddress,
        ownerAddress: result.ownerAddress,
        signerAddress: result.signerAddress,
        isDelegated: result.isDelegated,
        encryptedValues: result.encryptedValues,
        clearValues: result.clearValues,
        permit: result.permit,
      });
    });
};

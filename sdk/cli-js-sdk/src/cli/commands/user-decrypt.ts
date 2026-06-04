import type { Command } from "@commander-js/extra-typings";

import { freshUserDecrypt, userDecrypt } from "../../flows";
import { FHE_VALUE_TYPES } from "../../types";
import { parseClearValue, serializeValue } from "../../values";
import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import {
  collectHandle,
  parseAddress,
  parsePositiveInteger,
  parsePrivateKey,
  parseValueType,
} from "../parsers";
import { createProgressReporter } from "../progress";

const DEFAULT_PERMIT_DURATION_DAYS = 1;

/** Registers owner-signed user decrypt commands for cached and fresh handles. */
export const registerUserDecryptCommands = (program: Command): void => {
  const supportedValueTypes = FHE_VALUE_TYPES.join(", ");
  const userDecryptCommand = program
    .command("user-decrypt")
    .description(`User decrypt flows. Supported types: ${supportedValueTypes}`);

  userDecryptCommand
    .command("cached")
    .description("User decrypt an FHETest handle from wallet/type, or direct handles")
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
      "--handle <handle>",
      "encrypted handle to decrypt directly; repeat for multiple",
      collectHandle,
      [],
    )
    .option(
      "--duration-days <days>",
      "decryption permit duration in days",
      parsePositiveInteger,
      DEFAULT_PERMIT_DURATION_DAYS,
    )
    .option(
      "--private-key <privateKey>",
      "wallet private key; falls back to PRIVATE_KEY",
      parsePrivateKey,
    )
    .option("--mnemonic <mnemonic>", "wallet mnemonic; falls back to MNEMONIC")
    .action(async (options, command) => {
      const globals = getGlobalOptions(command);
      const result = await userDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        type: options.type,
        contractAddress: options.contract,
        handles: options.handle,
        durationDays: options.durationDays,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        onProgress: createProgressReporter(),
      });

      printJson(result);
    });

  userDecryptCommand
    .command("fresh")
    .description("Encrypt a new value, store it in FHETest, then user decrypt it")
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
      const result = await freshUserDecrypt({
        network: globals.network,
        relayerUrl: globals.relayerUrl,
        rpcUrl: globals.rpcUrl,
        type: options.type,
        value,
        contractAddress: options.contract,
        durationDays: options.durationDays,
        privateKey: options.privateKey,
        mnemonic: options.mnemonic,
        onProgress: createProgressReporter(),
      });

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

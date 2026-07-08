import type { Command } from "@commander-js/extra-typings";

import { FHE_VALUE_TYPES } from "@cli-fhevm-sdk/toolkit/types";
import { parseClearValue, serializeValue } from "@cli-fhevm-sdk/toolkit/values";
import { getGlobalOptions } from "../options";
import { printJson } from "../output";
import { parseAddress, parseValueType } from "../parsers";
import { createProgressReporter } from "../progress";

/** Registers the JSON-producing input-proof smoke-test command. */
export const registerInputProofCommand = (program: Command): void => {
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
    .option(
      "--value <value>",
      "clear value to encrypt; defaults to a random value",
    )
    .option(
      "--contract <address>",
      "contract address bound into the proof",
      parseAddress,
    )
    .option(
      "--user <address>",
      "user address bound into the proof",
      parseAddress,
    )
    .action(async (options, command) => {
      const { requestInputProof } = await import("@cli-fhevm-sdk/toolkit/flows/input-proof");
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
};

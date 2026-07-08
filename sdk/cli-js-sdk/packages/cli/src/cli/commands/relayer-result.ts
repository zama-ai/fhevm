import type { Command } from "@commander-js/extra-typings";

import type { UserDecryptValidationArtifact } from "@cli-fhevm-sdk/toolkit/types";
import { getGlobalOptions } from "../options";
import { printJson, readJsonFile } from "../output";

const parseArtifact = (value: unknown): UserDecryptValidationArtifact => {
  if (
    typeof value === "object" &&
    value !== null &&
    !Array.isArray(value) &&
    (value as { schemaVersion?: unknown }).schemaVersion === 1
  ) {
    return value as UserDecryptValidationArtifact;
  }
  throw new Error("Invalid user-decrypt validation artifact");
};

/** Registers commands for validating previously returned relayer GET results. */
export const registerRelayerResultCommands = (program: Command): void => {
  const relayerResult = program
    .command("relayer-result")
    .description("Validate relayer GET results using saved request artifacts");

  relayerResult
    .command("verify-user-decrypt")
    .description("Decrypt and compare a user-decrypt GET result")
    .requiredOption("--url <url>", "relayer GET result URL")
    .requiredOption(
      "--artifact <path>",
      "sensitive validation artifact written by user-decrypt --artifact",
    )
    .action(async (options, command) => {
      const { verifyUserDecryptResult } = await import(
        "@cli-fhevm-sdk/toolkit/flows/relayer-result/user-decrypt"
      );
      const globals = getGlobalOptions(command);
      const artifact = parseArtifact(await readJsonFile(options.artifact));
      const result = await verifyUserDecryptResult({
        url: options.url,
        rpcUrl: globals.rpcUrl,
        artifact,
      });

      printJson(result);
    });
};

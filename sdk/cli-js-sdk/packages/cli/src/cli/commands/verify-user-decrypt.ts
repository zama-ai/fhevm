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

/**
 * Registers the top-level verify-user-decrypt command.
 *
 * The relayer GET URL is derived from the artifact (network base, flow path
 * segment, relayer.jobId). `--relayer-url` overrides the base, `--job-id` the
 * job, and `--url` the whole URL.
 */
export const registerVerifyUserDecryptCommand = (program: Command): void => {
  program
    .command("verify-user-decrypt")
    .description(
      "Decrypt and compare a relayer user-decrypt GET result using a saved validation artifact",
    )
    .requiredOption(
      "--artifact <path>",
      "sensitive validation artifact written by user-decrypt/delegated-user-decrypt --artifact",
    )
    .option(
      "--job-id <id>",
      "relayer job id override; defaults to the artifact's relayer.jobId",
    )
    .option(
      "--url <url>",
      "full relayer GET result URL override; wins over the derived URL",
    )
    .action(async (options, command) => {
      const { verifyUserDecryptResult } = await import(
        "@cli-fhevm-sdk/toolkit/flows/relayer-result/user-decrypt"
      );
      const globals = getGlobalOptions(command);
      const artifact = parseArtifact(await readJsonFile(options.artifact));
      // Devnet relayers are keyless; mainnet requires ZAMA_FHEVM_API_KEY. Never
      // print or persist the key.
      const apiKey = process.env.ZAMA_FHEVM_API_KEY;
      const result = await verifyUserDecryptResult({
        artifact,
        rpcUrl: globals.rpcUrl,
        relayerUrl: globals.relayerUrl,
        jobId: options.jobId,
        url: options.url,
        authHeaders: apiKey ? () => ({ "x-api-key": apiKey }) : undefined,
      });

      printJson(result);
    });
};

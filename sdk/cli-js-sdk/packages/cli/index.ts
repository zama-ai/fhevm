#!/usr/bin/env node
// Must stay first so the project .env is loaded before any module reads env.
import "./src/env";

import { Command, Option } from "@commander-js/extra-typings";
import { consola } from "consola";

import { registerCompletionCommands } from "./src/cli/commands/completion";
import { registerDelegatedUserDecryptCommands } from "./src/cli/commands/delegated-user-decrypt";
import { registerFheTestCommands } from "./src/cli/commands/fhe-test";
import { registerInputProofCommand } from "./src/cli/commands/input-proof";
import { registerPublicDecryptCommands } from "./src/cli/commands/public-decrypt";
import { registerRelayerResultCommands } from "./src/cli/commands/relayer-result";
import { registerUserDecryptCommands } from "./src/cli/commands/user-decrypt";
import { DEFAULT_NETWORK, NETWORKS } from "@cli-fhevm-sdk/toolkit/types";

const program = new Command()
  .name("fhevm-sdk")
  .description("CLI for @fhevm/sdk flows against FHETest")
  .version("0.1.0")
  .addOption(
    new Option("-n, --network <network>", "network to target")
      .choices(NETWORKS)
      .default(DEFAULT_NETWORK),
  )
  .option(
    "--relayer-url <url>",
    "relayer base URL override, for example localhost:3000",
  )
  .option("--rpc-url <url>", "host chain RPC URL override");

registerInputProofCommand(program);
registerPublicDecryptCommands(program);
registerUserDecryptCommands(program);
registerDelegatedUserDecryptCommands(program);
registerRelayerResultCommands(program);
registerFheTestCommands(program);
registerCompletionCommands(program);

program.parseAsync().catch((error: unknown) => {
  consola.error(error instanceof Error ? (error.stack ?? error.message) : error);
  process.exitCode = 1;
});

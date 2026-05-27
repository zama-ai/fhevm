#!/usr/bin/env -S node --env-file=.env --import tsx
import { Command } from "@commander-js/extra-typings";
import { consola } from "consola";

import { DEFAULT_NETWORK } from "./src/config";
import { registerFheTestCommands } from "./src/cli/commands/fhe-test";
import { registerInputProofCommand } from "./src/cli/commands/input-proof";
import { registerPublicDecryptCommands } from "./src/cli/commands/public-decrypt";
import { parseNetwork } from "./src/cli/parsers";

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

registerInputProofCommand(program);
registerPublicDecryptCommands(program);
registerFheTestCommands(program);

program.parseAsync().catch((error: unknown) => {
  consola.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});

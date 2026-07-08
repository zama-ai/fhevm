#!/usr/bin/env node
// Must stay first so the project .env is loaded before any module reads env.
import "./src/env";

import { Command, Option } from "@commander-js/extra-typings";
import { consola } from "consola";

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

program.parseAsync().catch((error: unknown) => {
  consola.error(error instanceof Error ? (error.stack ?? error.message) : error);
  process.exitCode = 1;
});

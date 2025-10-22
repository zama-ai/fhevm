#!/usr/bin/env node
//
// How to run:
// ===========
//
// ./codegen.mjs --debug --config ./codegen.e2e.config.json lib
//
import { generateAllFiles } from "#lib";
import { Command } from "commander";

const program = new Command();
program.option("--debug", "output extra debugging");
program.option("--dry-run", "Dry run mode");
program.option("--config <path to config.json>", "Config json file");

globalThis.program = program;

program.name("codegen").description("A tool to generate all kind of code.");

const libCmd = program.command("lib");

libCmd.description("Generates Solidity library.").action((options) => generateAllFiles(options));
libCmd.option("--base-dir <absolute path>", "output base directory (absolute path).");
libCmd.option(
  "--types-dir <relative path>",
  "types relative directory (relative to base directory). Default is './lib'.",
);
libCmd.option("--lib-dir <relative path>", "lib relative directory (relative to base directory). Default is './lib'.");
libCmd.option(
  "--contracts-dir <relative path>",
  "contracts relative directory (relative to base directory). Default is './contracts'.",
);
libCmd.option(
  "--overloads-dir <relative path>",
  "overloads relative directory (relative to base directory). Default is './overloads'.",
);
libCmd.option(
  "--overloads <relative path or absolute path to another overload.json file>",
  "overloads relative directory (relative to base directory). Default is './overloads'.",
);

// // Add "update" command
// program
//   .command("update")
//   .description("Update the mock contracts.")
//   .action(() => generateAllFiles());

program.parse(process.argv);

#!/usr/bin/env node
//
// How to run:
// ===========
//
// #use overloads defined in codegen config
// ./codegen.mjs lib --config ./codegen.e2e.config.json --verbose
//
// #use supersedes config overloads
// ./codegen.mjs lib --overloads ./overloads/e2e.json --config ./codegen.e2e.config.json --verbose
//
// #generate new overloads file
// ./codegen.mjs overloads ./overloads/e2e.json
//
import { commandGenerateAllFiles, commandRegenerateOverloads } from "#lib";
import { Command } from "commander";

const program = new Command();
program.option("--config <path to config.json>", "Config json file");
program.option("--verbose", "output extra debugging");
program.option("--dry-run", "Dry run mode");

globalThis.program = program;

program.name("codegen").description("A tool to generate all kind of code.");

const libCmd = program.command("lib");
libCmd.description("Generates FHEVM Solidity library and tests.").action((options) => commandGenerateAllFiles(options));
libCmd.option(
  "--overloads <path to overloads.json>",
  "Path to the overloads JSON file. This argument supersedes any 'overloads' entry defined within the codegen config file.",
);
libCmd.option(
  "--no-test",
  "Do not generate the tests. This argument supersedes any 'noTest' entry defined within the codegen config file.",
);
libCmd.option(
  "--no-lib",
  "Do not generate the FHEVM Solidity library. This argument supersedes any 'noLib' entry defined within the codegen config file.",
);

const overloadsCmd = program.command("overloads");
overloadsCmd
  .description("Generates test overloads.")
  .action((outputFile, options) => commandRegenerateOverloads(outputFile, options));
overloadsCmd.argument("[outputFile]", "The path to the output JSON file for overloads.");
overloadsCmd.option("--force", "Overwrite any existing file");
overloadsCmd.option(
  "--update",
  "Keep existing overloads, generate missing ones (for example: if a new operation is added).",
);

program.parse(process.argv);

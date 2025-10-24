#!/usr/bin/env node
//
// How to run:
// ===========
//
// ./codegen.mjs lib --overloads ./overloads/e2e.json --config ./codegen.e2e.config.json --verbose
// ./codegen.mjs overloads ./overloads/e2e.json
//
import { generateAllFiles, forceRegenerateOverloads } from "#lib";
import { Command } from "commander";

const program = new Command();
program.option("--config <path to config.json>", "Config json file");
program.option("--verbose", "output extra debugging");
program.option("--dry-run", "Dry run mode");

globalThis.program = program;

program.name("codegen").description("A tool to generate all kind of code.");

const libCmd = program.command("lib");
libCmd.description("Generates FHEVM Solidity library and tests.").action((options) => generateAllFiles(options));
libCmd.option("--overloads <path to overloads.json>", "Overloads json file");
libCmd.option("--no-test", "Do not generate the tests");
libCmd.option("--no-lib", "Do not generate the FHEVM Solidity library");

const overloadsCmd = program.command("overloads");
overloadsCmd
  .description("Generates test overloads.")
  .action((outputFile, options) => forceRegenerateOverloads(outputFile, options));
overloadsCmd.argument("<outputFile>", "The path to the output JSON file for overloads.");

program.parse(process.argv);

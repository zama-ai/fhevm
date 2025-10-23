#!/usr/bin/env node
//
// How to run:
// ===========
//
// ./codegen.mjs --overloads ./overloads/e2e.json --config ./codegen.e2e.config.json --verbose
//
import { generateAllFiles } from "#lib";
import { Command } from "commander";

const program = new Command();
program.option("--config <path to config.json>", "Config json file");
program.option("--overloads <path to overloads.json>", "Overloads json file");
program.option("--verbose", "output extra debugging");
program.option("--dry-run", "Dry run mode");

globalThis.program = program;

program.name("codegen").description("A tool to generate all kind of code.");
program.action(() => generateAllFiles());

program.parse(process.argv);

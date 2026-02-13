#!/usr/bin/env bun

import { Command } from "commander";
import chalk from "chalk";

const LOGO = `
${chalk.blueBright("  ______   _    _   ______  __      __  __  __")}
${chalk.blueBright(' |  ____| | |  | | |  ____| \\ \\    / / |  \\/  |')}
${chalk.blueBright(" | |__    | |__| | | |__     \\ \\  / /  | \\  / |")}
${chalk.blueBright(" |  __|   |  __  | |  __|     \\ \\/ /   | |\\/| |")}
${chalk.blueBright(" | |      | |  | | | |____     \\  /    | |  | |")}
${chalk.blueBright(" |_|      |_|  |_| |______|     \\/     |_|  |_|")}
`;

const program = new Command();

program
  .name("fhevm")
  .description("CLI for managing the local FHEVM Docker Compose stack")
  .version("0.1.0")
  .hook("preAction", () => {
    console.log(LOGO);
  });

// Commands are registered below — imported after all modules are defined
import { registerUpCommand } from "./commands/up.js";
import { registerTestCommand } from "./commands/test.js";
import { registerCleanCommand } from "./commands/clean.js";
import { registerLogsCommand } from "./commands/logs.js";
import { registerRestartCommand } from "./commands/restart.js";
import { registerUpgradeCommand } from "./commands/upgrade.js";
import { registerPauseCommand } from "./commands/pause.js";
import { registerUnpauseCommand } from "./commands/unpause.js";
import { registerStatusCommand } from "./commands/status.js";
import { registerDoctorCommand } from "./commands/doctor.js";

registerUpCommand(program);
registerTestCommand(program);
registerCleanCommand(program);
registerLogsCommand(program);
registerRestartCommand(program);
registerUpgradeCommand(program);
registerPauseCommand(program);
registerUnpauseCommand(program);
registerStatusCommand(program);
registerDoctorCommand(program);

program.parse();

import { Option } from "@commander-js/extra-typings";
import type { Command } from "@commander-js/extra-typings";
import {
  getShellFromEnv,
  install,
  log,
  parseEnv,
  SUPPORTED_SHELLS,
  uninstall,
} from "@pnpm/tabtab";

import { getCompletionItems } from "../completion";
import { printJson } from "../output";

/** Registers shell completion install/uninstall and hidden completion server. */
export const registerCompletionCommands = (program: Command): void => {
  const completionCommand = program
    .command("completion")
    .description("Manage shell tab completion for this CLI");

  completionCommand
    .command("install")
    .description("Install tab completion into the shell profile")
    .addOption(
      new Option(
        "--shell <shell>",
        "target shell, prompts when omitted",
      ).choices(SUPPORTED_SHELLS),
    )
    .action(async (options) => {
      const name = program.name();
      await install({ name, completer: name, shell: options.shell });
      printJson({ status: "installed", name, shell: options.shell });
    });

  completionCommand
    .command("uninstall")
    .description("Remove tab completion from the shell profile")
    .addOption(
      new Option(
        "--shell <shell>",
        "target shell, all supported shells when omitted",
      ).choices(SUPPORTED_SHELLS),
    )
    .action(async (options) => {
      const name = program.name();
      await uninstall({ name, shell: options.shell });
      printJson({ status: "uninstalled", name, shell: options.shell });
    });

  // Invoked by the tabtab shell templates on every Tab press; must print
  // completion items only.
  program
    .command("completion-server", { hidden: true })
    .description("Resolve completion items for the shell")
    .allowExcessArguments(true)
    .action(() => {
      const env = parseEnv(process.env);
      if (!env.complete) return;
      log(getCompletionItems(program, env), getShellFromEnv(process.env));
    });
};

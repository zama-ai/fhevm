/**
 * cli.ts — Entry point for the fhevm CLI.
 *
 * Uses @effect/cli for declarative command parsing, typed options,
 * and built-in help generation. Dispatches to command handlers.
 */
import { Command } from "@effect/cli";
import { BunContext } from "@effect/platform-bun";
import { Effect, Layer } from "effect";

import { LiveLayer } from "./services/layers";
import { StateManager } from "./services/StateManager";
import { formatCliError } from "./errors";

import { upCommand, deployCommand } from "./commands/up.cmd";
import { downCommand } from "./commands/down.cmd";
import { cleanCommand } from "./commands/clean.cmd";
import { statusCommand } from "./commands/status.cmd";
import { logsCommand } from "./commands/logs.cmd";
import { upgradeCommand } from "./commands/upgrade.cmd";
import { testCommand } from "./commands/test.cmd";
import { pauseCommand } from "./commands/pause.cmd";
import { unpauseCommand } from "./commands/unpause.cmd";

// ---------------------------------------------------------------------------
// Root command with subcommands
// ---------------------------------------------------------------------------

const rootCommand = Command.make("fhevm-cli").pipe(
  Command.withSubcommands([
    upCommand,
    deployCommand,
    downCommand,
    cleanCommand,
    statusCommand,
    logsCommand,
    upgradeCommand,
    testCommand,
    pauseCommand,
    unpauseCommand,
  ]),
);

// ---------------------------------------------------------------------------
// CLI runner
// ---------------------------------------------------------------------------

const cli = Command.run(rootCommand, {
  name: "fhevm-cli",
  version: "0.1.0",
});

// ---------------------------------------------------------------------------
// main — preserved signature for test compatibility
// ---------------------------------------------------------------------------

export const main = async (
  argv = process.argv,
  layerOverride?: Layer.Layer<any, never, never>,
) => {
  const layer = layerOverride ?? LiveLayer;
  let failure: unknown;

  const program = cli(argv).pipe(
    Effect.provide(layer),
    Effect.provide(BunContext.layer),
    Effect.catchAll((error) => {
      failure = error;
      const message = formatCliError(error);
      if (message) {
        console.error(message);
      }
      process.exitCode = 1;
      return Effect.void;
    }),
  );

  await Effect.runPromise(program as Effect.Effect<void, never, never>);

  // Show resume hint if 'up' failed
  if (
    process.exitCode === 1 &&
    !argv.includes("--dry-run") &&
    failure &&
    typeof failure === "object" &&
    "_tag" in failure &&
    !["PreflightError", "ResumeError", "SchemaGuardError"].includes(String((failure as { _tag?: string })._tag))
  ) {
    const command = argv[2];
    if (command === "up" || command === "deploy") {
      try {
        const state = await Effect.runPromise(
          Effect.gen(function* () {
            const stateManager = yield* StateManager;
            return yield* stateManager.load;
          }).pipe(Effect.provide(layer)),
        );
        if (state?.completedSteps.length) {
          console.error(
            "Hint: run with --resume to continue, or without to start fresh.",
          );
        }
      } catch {
        // Ignore errors checking state for the hint
      }
    }
  }
};

if (import.meta.main) {
  await main();
}

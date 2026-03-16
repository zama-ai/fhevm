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

import { upCommand, deployCommand } from "./commands/up.cmd";
import { downCommand } from "./commands/down.cmd";
import { cleanCommand } from "./commands/clean.cmd";
import { statusCommand } from "./commands/status.cmd";
import { logsCommand } from "./commands/logs.cmd";
import { upgradeCommand } from "./commands/upgrade.cmd";
import { testCommand } from "./commands/test.cmd";
import { pauseCommand } from "./commands/pause.cmd";
import { unpauseCommand } from "./commands/unpause.cmd";
import { compatDefaultsCommand } from "./commands/compat-defaults.cmd";
import { doctorCommand } from "./commands/doctor.cmd";

// Re-export parse helpers for backward compatibility
export {
  parseLocalOverride,
  parseKeyValue,
  parseInstanceKey,
  parseInstanceEnv,
  parseInstanceArgs,
  mergeInstanceOverrides,
} from "./options";

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
    compatDefaultsCommand,
    doctorCommand,
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

  const program = cli(argv).pipe(
    Effect.provide(layer),
    Effect.provide(BunContext.layer),
    Effect.catchAll((error) => {
      // Handler errors (PreflightError, etc.) have .message — print it.
      // @effect/cli framework errors (CommandMismatch, etc.) already printed
      // their user-friendly message; they have no .message property.
      if (error && typeof error === "object" && "message" in error) {
        console.error((error as { message: string }).message);
      }
      process.exitCode = 1;
      return Effect.void;
    }),
  );

  await Effect.runPromise(program as Effect.Effect<void, never, never>);

  // Show resume hint if 'up' failed
  if (process.exitCode === 1) {
    const command = argv[2];
    if (command === "up" || command === "deploy") {
      try {
        const hasState = await Effect.runPromise(
          Effect.gen(function* () {
            const stateManager = yield* StateManager;
            return yield* stateManager.load;
          }).pipe(Effect.provide(layer)),
        );
        if (hasState) {
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

export { main as default };

if (import.meta.main) {
  await main();
}

// ============================================================================
// STRANGLER SCAFFOLD — DELETE THIS FILE WHEN THE MIGRATION IS COMPLETE.
// ============================================================================
//
// Strangler-fig migration boundary between the OLD CLI (the ~10.7k-LoC imperative
// engine at test-suite/fhevm — the IMMUTABLE, human-gated oracle) and the NEW engine
// (stack/lib + stack/cli/main.ts — the thin declarative driver).
//
// `fhevm-cli <command>` enters here. Each command routes to exactly one engine:
//   - command in MIGRATED  → NEW engine (stack/cli/main.ts over a real Stack)
//   - otherwise            → OLD CLI, spawned untouched (the oracle keeps running)
// `FHEVM_ENGINE=new|old` force-overrides the routing for A/B comparison against the
// L2 behavioral golden (the real cross-engine equivalence check).
//
// MIGRATION PROTOCOL (the strangle):
//   1. A command's NEW-engine path reaches parity (its L2 erc20 pass-set golden is green
//      vs the OLD CLI oracle — see .github/workflows/acceptance.yml).
//   2. Add the command to MIGRATED. Traffic now flows to the new engine; the old path
//      stays available via FHEVM_ENGINE=old for one release as a safety net.
//   3. When MIGRATED covers every command, DELETE this file and the OLD CLI, and point
//      the `fhevm-cli` launcher straight at stack/cli/main.ts. Nothing else changes —
//      that deletability is the whole point of the scaffold.

import { spawn } from "node:child_process";

/**
 * Commands the NEW engine owns. EMPTY until each command's L2 golden is green vs the
 * oracle — do not migrate a command on faith. Add e.g. "up", "test", "down" here as
 * each reaches behavioral parity.
 */
const MIGRATED = new Set<string>([]);

/** The old CLI launcher (the oracle) — spawned verbatim, never modified. */
const OLD_CLI = process.env.FHEVM_OLD_CLI ?? "test-suite/fhevm/fhevm-cli";
/** The new CLI entrypoint. Spawned as a subprocess to avoid coupling to its internals. */
const NEW_CLI = (process.env.FHEVM_NEW_CLI ?? "bun stack/cli/main.ts").split(" ");

const spawnInherit = (cmd: string, args: string[]): Promise<number> =>
  new Promise((resolve, reject) => {
    const child = spawn(cmd, args, { stdio: "inherit" });
    child.on("error", reject);
    child.on("close", (code) => resolve(code ?? 0));
  });

/** Decide which engine handles `command`. */
export const routesToNewEngine = (command: string | undefined): boolean => {
  const forced = process.env.FHEVM_ENGINE;
  if (forced === "new") return true;
  if (forced === "old") return false;
  return command !== undefined && MIGRATED.has(command);
};

/** Route argv to the chosen engine; propagate its exit code. */
export const route = async (argv: string[] = process.argv.slice(2)): Promise<void> => {
  const command = argv[0];
  const [newBin, ...newPrefix] = NEW_CLI;
  const code = routesToNewEngine(command)
    ? await spawnInherit(newBin, [...newPrefix, ...argv])
    : await spawnInherit(OLD_CLI, argv);
  if (code !== 0) process.exitCode = code;
};

if (import.meta.main) {
  await route();
}

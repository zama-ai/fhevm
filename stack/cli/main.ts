// EXEMPLAR — thin arg-dispatch entrypoint; no orchestration logic of its own.
/**
 * fhevm CLI — main entrypoint.
 *
 * Dispatches argv tokens to subcommand handlers that each map parsed args
 * to exactly one Stack API call (or a small fixed sequence for compound
 * commands like `runbook run`).  No domain logic lives here.
 *
 * Command surface:
 *   fhevm up [--scenario X] [--kms centralized|threshold:N[/T]]
 *            [--chains N] [--build] [--values FILE] [--dry-run]
 *   fhevm test <suite> [--network NET] [--parallel]
 *   fhevm down
 *   fhevm clean [--keep-images]
 *   fhevm logs <svc> [--no-follow] [--tail N]
 *   fhevm runbook run <name>
 *   fhevm runbook receipt
 *
 * The Stack instance is injected so the module is testable without a real cluster.
 */

import { createStack } from "../lib/engine";
import type { Stack } from "../lib/stack";
import { runUp } from "./up";
import { runTest } from "./test";
import { runRunbook, runReceipt } from "./runbook";

// ---------------------------------------------------------------------------
// Minimal flag parser (no external dep — keeps the exemplar self-contained)
// ---------------------------------------------------------------------------

type Flags = Record<string, string | boolean | undefined>;

/**
 * Parse a flat argv slice into positionals and flags.
 *
 * Supports:
 *   --flag          → { flag: true }
 *   --no-flag       → { flag: false }
 *   --flag value    → { flag: "value" }
 *   --flag=value    → { flag: "value" }
 */
const parseArgv = (argv: string[]): { positionals: string[]; flags: Flags } => {
  const positionals: string[] = [];
  const flags: Flags = {};
  let i = 0;
  while (i < argv.length) {
    const token = argv[i];
    if (token === "--") {
      positionals.push(...argv.slice(i + 1));
      break;
    }
    if (token.startsWith("--")) {
      const eqIdx = token.indexOf("=");
      if (eqIdx !== -1) {
        flags[token.slice(2, eqIdx)] = token.slice(eqIdx + 1);
      } else if (token.startsWith("--no-")) {
        flags[token.slice(5)] = false;
      } else {
        const key = token.slice(2);
        const next = argv[i + 1];
        if (next !== undefined && !next.startsWith("-")) {
          flags[key] = next;
          i += 1;
        } else {
          flags[key] = true;
        }
      }
    } else {
      positionals.push(token);
    }
    i += 1;
  }
  return { positionals, flags };
};

const asString = (v: string | boolean | undefined): string | undefined =>
  typeof v === "string" ? v : undefined;

const asBool = (v: string | boolean | undefined): boolean | undefined =>
  typeof v === "boolean" ? v : v === "true" ? true : v === "false" ? false : undefined;

const asInt = (v: string | boolean | undefined): number | undefined => {
  const s = asString(v);
  if (s === undefined) return undefined;
  const n = parseInt(s, 10);
  return Number.isNaN(n) ? undefined : n;
};

// ---------------------------------------------------------------------------
// Usage
// ---------------------------------------------------------------------------

const USAGE = `\
fhevm <command> [options]

Commands:
  up          Boot or reconcile the stack
  test        Run a named test suite
  down        Tear down all stack releases
  clean       Tear down and delete generated state
  logs        Stream or print logs for a service
  runbook     Load and execute a TS runbook, or print the last receipt

Run \`fhevm <command> --help\` for per-command options.
`.trim();

const UP_USAGE = `\
fhevm up [options]

  --scenario X          Scenario preset name or path to a YAML file
  --kms <topology>      centralized | threshold:N | threshold:N/T
  --chains N            Number of host chains (default: 1)
  --build               Rebuild workspace-owned images before booting
  --values FILE         Extra Helm values overlay file
  --dry-run             Print resolved Helm plan; make no cluster changes
`.trim();

const TEST_USAGE = `\
fhevm test <suite> [options]

  --network NET         Hardhat network name (default: staging)
  --parallel            Run test files in parallel
`.trim();

const LOGS_USAGE = `\
fhevm logs <svc> [options]

  --no-follow           Print recent logs and exit (default: follow)
  --tail N              Number of lines from the end to show
`.trim();

const RUNBOOK_USAGE = `\
fhevm runbook <subcommand>

  run <name>            Execute a TypeScript runbook file
  receipt               Print the receipt of the most recent runbook run
`.trim();

// ---------------------------------------------------------------------------
// Command handlers
// ---------------------------------------------------------------------------

const handleUp = async (stack: Stack, argv: string[]): Promise<void> => {
  const { flags } = parseArgv(argv);
  if (flags.help) {
    console.log(UP_USAGE);
    return;
  }
  await runUp(stack, {
    scenario: asString(flags.scenario),
    kms: asString(flags.kms),
    chains: asInt(flags.chains),
    build: asBool(flags.build),
    values: asString(flags.values),
    dryRun: asBool(flags["dry-run"]),
    from: asString(flags.from),
    until: asString(flags.until),
  });
};

const handleTest = async (stack: Stack, argv: string[]): Promise<void> => {
  const { positionals, flags } = parseArgv(argv);
  if (flags.help || positionals.length === 0) {
    console.log(TEST_USAGE);
    if (positionals.length === 0 && !flags.help) {
      throw new Error("fhevm test requires a <suite> argument");
    }
    return;
  }
  await runTest(stack, {
    suite: positionals[0],
    network: asString(flags.network),
    parallel: asBool(flags.parallel),
  });
};

const handleDown = async (stack: Stack): Promise<void> => {
  await stack.down();
};

const handleClean = async (stack: Stack, argv: string[]): Promise<void> => {
  const { flags } = parseArgv(argv);
  await stack.clean({ keepImages: asBool(flags["keep-images"]) });
};

const handleLogs = async (stack: Stack, argv: string[]): Promise<void> => {
  const { positionals, flags } = parseArgv(argv);
  if (flags.help || positionals.length === 0) {
    console.log(LOGS_USAGE);
    if (positionals.length === 0 && !flags.help) {
      throw new Error("fhevm logs requires a <svc> argument");
    }
    return;
  }
  // --no-follow sets flags.follow = false via the "--no-X" parser rule above.
  const follow = flags.follow !== false;
  await stack.logs(positionals[0], { follow, tail: asInt(flags.tail) });
};

const handleRunbook = async (stack: Stack, argv: string[]): Promise<void> => {
  const { positionals, flags } = parseArgv(argv);
  const sub = positionals[0];
  if (!sub || flags.help) {
    console.log(RUNBOOK_USAGE);
    return;
  }
  if (sub === "run") {
    const name = positionals[1];
    if (!name) throw new Error("fhevm runbook run requires a <name> argument");
    await runRunbook(stack, { name });
    return;
  }
  if (sub === "receipt") {
    await runReceipt(stack);
    return;
  }
  throw new Error(`Unknown runbook subcommand "${sub}". Expected: run | receipt`);
};

// ---------------------------------------------------------------------------
// Main dispatch
// ---------------------------------------------------------------------------

/**
 * Entry point.  Accepts an injectable Stack instance so the dispatch table
 * can be unit-tested without a real cluster.
 */
export const main = async (stack: Stack, argv = process.argv.slice(2)): Promise<void> => {
  const [command, ...rest] = argv;

  if (!command || command === "--help" || command === "-h") {
    console.log(USAGE);
    return;
  }

  switch (command) {
    case "up":
      await handleUp(stack, rest);
      break;
    case "test":
      await handleTest(stack, rest);
      break;
    case "down":
      await handleDown(stack);
      break;
    case "clean":
      await handleClean(stack, rest);
      break;
    case "logs":
      await handleLogs(stack, rest);
      break;
    case "runbook":
      await handleRunbook(stack, rest);
      break;
    default:
      console.error(`Unknown command "${command}". Run \`fhevm --help\` for usage.`);
      process.exitCode = 1;
  }
};

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

// Run when invoked directly via `bun stack/cli/main.ts …` or the fhevm launcher.
// The KubectlStack constructor only stores config (no cluster contact), so `--help`
// branches still return before any Stack method is invoked.
if (import.meta.main) {
  await main(createStack(), process.argv.slice(2));
}

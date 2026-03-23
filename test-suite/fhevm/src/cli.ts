/**
 * Defines the fhevm CLI surface and maps user commands onto stack and test operations.
 */
import { defineCommand, renderUsage, runCommand } from "citty";

import { formatCliError, PreflightError } from "./errors";
import { test } from "./commands/test";
import {
  clean,
  down,
  listScenarios,
  logs,
  pause,
  showResumeHint,
  status,
  unpause,
  up,
  upDryRun,
  upgrade,
} from "./flow/up-flow";
import { parseUpInput } from "./input/up";
import { asBool, asString } from "./input/shared";
const HELP_FLAGS = new Set(["--help", "-h"]);

/** Shares the `up` command argument surface and execution for `deploy`. */
const upCommandDefinition = {
  args: {
    target: { type: "string", description: "Bundle source to boot." },
    sha: { type: "string", description: "Commit SHA to resolve when --target sha is used." },
    override: { type: "string", description: "Build selected workspace groups locally.", alias: "o" },
    "from-step": { type: "string", description: "Start from a specific pipeline step when resuming or previewing." },
    "lock-file": { type: "string", description: "Use an existing lock snapshot instead of resolving versions live." },
    scenario: { type: "string", description: "Scenario preset name or path." },
    resume: { type: "boolean", description: "Resume from persisted state." },
    "dry-run": { type: "boolean", description: "Print the resolved plan and stop before mutating state." },
    reset: { type: "boolean", description: "Discard cached resolution and regenerate from scratch." },
    "allow-schema-mismatch": { type: "boolean", description: "Bypass schema-coupled local override safety checks." },
    build: { type: "boolean", description: "Build every workspace-owned group locally." },
  },
  async run({ args }: { args: Record<string, unknown> }) {
    const parsed = parseUpInput(args);
    if (parsed.dryRun) {
      const { dryRun: _dryRun, ...rest } = parsed;
      await upDryRun(rest);
      return;
    }
    await up(parsed);
  },
} as const;

const root = defineCommand({
  meta: {
    name: "fhevm-cli",
    version: "0.1.0",
    description: "Local orchestration entrypoint for the fhEVM test stack.",
  },
  subCommands: {
    up: defineCommand({
      meta: { name: "up", description: "Boot the fhevm stack from a target, lock file, or persisted state." },
      ...upCommandDefinition,
    }),
    deploy: defineCommand({
      meta: { name: "deploy", description: "Alias of `up` kept for deployment-oriented workflows." },
      ...upCommandDefinition,
    }),
    down: defineCommand({
      meta: { name: "down", description: "Stop all stack containers in reverse order." },
      async run() {
        await down();
      },
    }),
    clean: defineCommand({
      meta: { name: "clean", description: "Stop containers, optionally remove CLI-owned images, and delete .fhevm." },
      args: {
        images: { type: "boolean", description: "Also remove locally built images." },
      },
      async run({ args }) {
        await clean({ images: asBool(args.images) });
      },
    }),
    status: defineCommand({
      meta: { name: "status", description: "Print persisted state and running containers." },
      async run() {
        await status();
      },
    }),
    logs: defineCommand({
      meta: { name: "logs", description: "Follow container logs for a specified or first container." },
      args: {
        service: { type: "positional", description: "Container alias or service name to target.", required: false },
        "no-follow": { type: "boolean", description: "Print recent logs and exit instead of following." },
      },
      async run({ args }) {
        await logs(asString(args.service), { follow: !asBool(args["no-follow"] ?? args.noFollow) });
      },
    }),
    upgrade: defineCommand({
      meta: { name: "upgrade", description: "Rebuild and restart an active local runtime override group." },
      args: {
        group: { type: "positional", description: "Local override group to rebuild in-place." },
      },
      async run({ args }) {
        await upgrade(asString(args.group));
      },
    }),
    test: defineCommand({
      meta: { name: "test", description: "Run e2e tests inside the fhevm test-suite container." },
      args: {
        testName: { type: "positional", description: "Named test profile to run.", required: false },
        grep: { type: "string", description: "Custom grep pattern passed through to the e2e runner." },
        network: { type: "string", description: "Hardhat network passed to the test suite.", default: "staging" },
        verbose: { type: "boolean", description: "Enable verbose output from the test command." },
        parallel: { type: "boolean", description: "Run supported test suites in parallel." },
      },
      async run({ args }) {
        await test(asString(args.testName), {
          grep: asString(args.grep),
          network: asString(args.network) ?? "staging",
          verbose: asBool(args.verbose),
          parallel: args.parallel === undefined ? undefined : asBool(args.parallel),
        });
      },
    }),
    pause: defineCommand({
      meta: { name: "pause", description: "Pause host or gateway contracts." },
      args: {
        scope: { type: "positional", description: "Pause target: host or gateway." },
      },
      async run({ args }) {
        await pause(asString(args.scope));
      },
    }),
    unpause: defineCommand({
      meta: { name: "unpause", description: "Unpause host or gateway contracts." },
      args: {
        scope: { type: "positional", description: "Pause target: host or gateway." },
      },
      async run({ args }) {
        await unpause(asString(args.scope));
      },
    }),
    scenario: defineCommand({
      meta: { name: "scenario", description: "Scenario utilities." },
      subCommands: {
        list: defineCommand({
          meta: { name: "list", description: "List bundled scenario presets." },
          async run() {
            await listScenarios();
          },
        }),
      },
    }),
  },
});

/** Finds the first positional token in a raw argv slice. */
const firstNonFlagIndex = (rawArgs: string[]) => rawArgs.findIndex((arg) => !arg.startsWith("-"));

/** Walks subcommands to find the command whose usage should be rendered. */
const resolveUsageTarget = async (
  cmd: any,
  rawArgs: string[],
  parent?: any,
): Promise<[any, any?]> => {
  const subCommands = typeof cmd.subCommands === "function" ? await cmd.subCommands() : cmd.subCommands;
  if (!subCommands || !Object.keys(subCommands).length) {
    return [cmd, parent];
  }
  const index = firstNonFlagIndex(rawArgs);
  if (index < 0) {
    return [cmd, parent];
  }
  const subCommand = subCommands[rawArgs[index] as keyof typeof subCommands];
  if (!subCommand) {
    return [cmd, parent];
  }
  return resolveUsageTarget(subCommand, rawArgs.slice(index + 1), cmd);
};

/** Runs the CLI entrypoint with custom help rendering and error formatting. */
export const main = async (argv = process.argv) => {
  const rawArgs = argv.slice(2);
  if (rawArgs.length === 0 || rawArgs.some((arg) => HELP_FLAGS.has(arg))) {
    const [cmd, parent] = await resolveUsageTarget(root, rawArgs);
    console.log(`${await renderUsage(cmd, parent)}\n`);
    return;
  }
  try {
    await runCommand(root, { rawArgs });
  } catch (error) {
    const message = formatCliError(error);
    if (message) {
      console.error(message);
    }
    process.exitCode = 1;
    await showResumeHint(argv.slice(2)).catch(() => undefined);
  }
};

if (import.meta.main) {
  await main();
}

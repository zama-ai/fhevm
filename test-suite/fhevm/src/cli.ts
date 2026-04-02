/**
 * Defines the fhevm CLI surface and maps user commands onto stack and test operations.
 */
import { defineCommand, renderUsage, runCommand } from "citty";

import { formatCliError, PreflightError } from "./errors";
import { listTestProfiles, test } from "./commands/test";
import { resolveBundle } from "./resolve/bundle-store";
import { STEP_NAMES } from "./types";
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
type CommandArg = { type?: string; alias?: string | string[] };
type CliCommand = {
  args?: Record<string, CommandArg> | (() => Record<string, CommandArg> | Promise<Record<string, CommandArg>>);
  subCommands?: Record<string, CliCommand> | (() => Record<string, CliCommand> | Promise<Record<string, CliCommand>>);
};

const commandArgs = async (cmd: CliCommand) =>
  typeof cmd.args === "function" ? await cmd.args() : (cmd.args ?? {});

/** Collects the accepted flag spellings for one command definition. */
const commandOptionForms = async (cmd: CliCommand) => {
  const args = await commandArgs(cmd);
  const forms = new Set<string>();
  for (const [name, def] of Object.entries(args as Record<string, { type?: string; alias?: string | string[] }>)) {
    if (def.type === "positional") {
      continue;
    }
    forms.add(`--${name}`);
    if (def.type === "boolean") {
      forms.add(`--no-${name}`);
    }
    const aliases = Array.isArray(def.alias) ? def.alias : def.alias ? [def.alias] : [];
    for (const alias of aliases) {
      forms.add(`${alias.length === 1 ? "-" : "--"}${alias}`);
    }
  }
  return forms;
};

const commandPositionalCount = async (cmd: CliCommand) =>
  Object.values(await commandArgs(cmd)).filter((arg) => arg.type === "positional").length;

const positionalTokens = async (cmd: CliCommand, rawArgs: string[]) => {
  const args = await commandArgs(cmd);
  const shortOptions = new Map<string, CommandArg>();
  for (const [name, def] of Object.entries(args)) {
    const aliases = Array.isArray(def.alias) ? def.alias : def.alias ? [def.alias] : [];
    for (const alias of aliases) {
      if (alias.length === 1) {
        shortOptions.set(alias, def);
      }
    }
    shortOptions.set(name, def);
  }
  const positionals: string[] = [];
  for (let index = 0; index < rawArgs.length; index += 1) {
    const token = rawArgs[index];
    if (token === "--") {
      positionals.push(...rawArgs.slice(index + 1));
      break;
    }
    if (!token.startsWith("-")) {
      positionals.push(token);
      continue;
    }
    if (token.startsWith("--")) {
      const name = token.slice(2).split("=")[0];
      const def = args[name];
      if (def && def.type !== "boolean" && !token.includes("=")) {
        index += 1;
      }
      continue;
    }
    const shorts = token.slice(1).split("=")[0];
    const last = shorts.at(-1);
    if (last && shortOptions.get(last)?.type !== "boolean" && !token.includes("=")) {
      index += 1;
    }
  }
  return positionals;
};

/** Rejects unsupported options before citty silently accepts them. */
const validateKnownArgs = async (cmd: CliCommand, rawArgs: string[]): Promise<void> => {
  const subCommands = typeof cmd.subCommands === "function" ? await cmd.subCommands() : cmd.subCommands;
  if (subCommands && Object.keys(subCommands).length) {
    const subIndex = rawArgs.findIndex((arg) => !arg.startsWith("-"));
    const optionSlice = subIndex >= 0 ? rawArgs.slice(0, subIndex) : rawArgs;
    const allowed = await commandOptionForms(cmd);
    for (const token of optionSlice) {
      if (!token.startsWith("-") || token === "--") {
        continue;
      }
      if (token.startsWith("--")) {
        const form = `--${token.slice(2).split("=")[0]}`;
        if (!allowed.has(form)) {
          throw new PreflightError(`Unknown option ${form}`);
        }
        continue;
      }
      for (const short of token.slice(1).split("=")[0]) {
        const form = `-${short}`;
        if (!allowed.has(form)) {
          throw new PreflightError(`Unknown option ${form}`);
        }
      }
    }
    if (subIndex < 0) return;
    const subName = rawArgs[subIndex];
    const subCommand = subCommands[subName as keyof typeof subCommands];
    if (!subCommand) {
      throw new PreflightError(`Unknown subcommand ${subName}`);
    }
    await validateKnownArgs(subCommand, rawArgs.slice(subIndex + 1));
    return;
  }

  const allowed = await commandOptionForms(cmd);
  for (const token of rawArgs) {
    if (!token.startsWith("-") || token === "--") {
      continue;
    }
    if (token.startsWith("--")) {
      const form = `--${token.slice(2).split("=")[0]}`;
      if (!allowed.has(form)) {
        throw new PreflightError(`Unknown option ${form}`);
      }
      continue;
    }
    for (const short of token.slice(1).split("=")[0]) {
      const form = `-${short}`;
      if (!allowed.has(form)) {
        throw new PreflightError(`Unknown option ${form}`);
      }
    }
  }
  const positionals = await positionalTokens(cmd, rawArgs);
  const allowedPositionals = await commandPositionalCount(cmd);
  if (positionals.length > allowedPositionals) {
    throw new PreflightError(`Unexpected positional argument ${positionals[allowedPositionals]}`);
  }
};

/** Resolves the effective follow mode for `logs`, including citty's `--no-follow` negation. */
export const resolveLogsFollow = (args: Record<string, unknown>) => args.follow !== false;

/** Shares the `up` command argument surface and execution for `deploy`. */
const upCommandDefinition = {
  args: {
    target: { type: "string", description: "Bundle source to boot." },
    sha: { type: "string", description: "Commit SHA to resolve when --target sha is used." },
    override: { type: "string", description: "Build selected workspace groups locally. Use --override test-suite to run local e2e test changes.", alias: "o" },
    "from-step": { type: "string", description: `Start from a specific pipeline step when resuming or previewing. Valid: ${STEP_NAMES.join(", ")}.` },
    "lock-file": { type: "string", description: "Use an existing lock snapshot instead of resolving versions live." },
    scenario: { type: "string", description: "Scenario preset name or path." },
    resume: { type: "boolean", description: "Resume from persisted state." },
    "dry-run": { type: "boolean", description: "Print the resolved plan and stop before mutating state." },
    reset: { type: "boolean", description: "Discard cached resolution and regenerate from scratch." },
    "allow-schema-mismatch": { type: "boolean", description: "Bypass schema-coupled local override safety checks." },
    build: { type: "boolean", description: "Build every workspace-owned group locally, including test-suite." },
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
      meta: { name: "clean", description: "Stop containers, remove CLI-owned images by default, and delete .fhevm." },
      args: {
        "keep-images": { type: "boolean", description: "Preserve locally built images instead of removing them." },
      },
      async run({ args }) {
        await clean({ keepImages: asBool(args["keep-images"]) });
      },
    }),
    status: defineCommand({
      meta: { name: "status", description: "Print persisted state and running containers." },
      async run() {
        await status();
      },
    }),
    logs: defineCommand({
      meta: { name: "logs", description: "Follow container logs for a specified service, or the first running fhevm container." },
      args: {
        service: { type: "positional", description: "Container alias or service name to target.", required: false },
        follow: { type: "boolean", default: true, description: "Follow logs; pass --no-follow to print recent logs and exit." },
      },
      async run({ args }) {
        await logs(asString(args.service), { follow: resolveLogsFollow(args) });
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
    resolve: defineCommand({
      meta: { name: "resolve", description: "Resolve a version target and print the resulting lock path." },
      args: {
        target: { type: "string", description: "Bundle source to resolve." },
        sha: { type: "string", description: "Commit SHA to resolve when --target sha is used." },
        "lock-file": { type: "string", description: "Use an existing lock snapshot instead of resolving versions live." },
        reset: { type: "boolean", description: "Discard cached resolution and regenerate from scratch." },
      },
      async run({ args }) {
        const parsed = parseUpInput(args);
        if (parsed.resume || parsed.dryRun || parsed.fromStep || parsed.overrides.length || parsed.scenarioPath) {
          throw new PreflightError("resolve only supports --target, --sha, --lock-file, and --reset");
        }
        const { lockPath } = await resolveBundle(parsed, process.env);
        console.log(lockPath);
      },
    }),
    test: defineCommand({
      meta: { name: "test", description: "Run e2e tests inside the fhevm test-suite container." },
      args: {
        testName: { type: "positional", description: "Named test profile to run.", required: false },
        grep: { type: "string", description: "Custom grep pattern passed through to the e2e runner." },
        network: { type: "string", description: "Hardhat network passed to the test suite.", default: "staging" },
        verbose: { type: "boolean", description: "Enable verbose output from the test command." },
        "no-hardhat-compile": { type: "boolean", description: "Skip the Hardhat compilation step inside the test runner." },
        parallel: { type: "boolean", description: "Run supported test suites in parallel." },
      },
      async run({ args }) {
        const testName = asString(args.testName);
        if (testName === "list") {
          listTestProfiles();
          return;
        }
        await test(testName, {
          grep: asString(args.grep),
          network: asString(args.network) ?? "staging",
          verbose: asBool(args.verbose),
          noHardhatCompile: asBool(args["no-hardhat-compile"]),
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
  cmd: CliCommand,
  rawArgs: string[],
  parent?: CliCommand,
): Promise<[CliCommand, CliCommand?]> => {
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
    const [cmd, parent] = await resolveUsageTarget(root as unknown as CliCommand, rawArgs);
    console.log(`${await renderUsage(cmd as never, parent as never)}\n`);
    return;
  }
  try {
    await validateKnownArgs(root as unknown as CliCommand, rawArgs);
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

import { ExitCode, FhevmCliError } from "../errors";
import { exec, type ShellResult } from "../utils/shell";

import { parseComposePsOutput } from "./containers";
import { toLogLines } from "./logs";
import type {
  ComposeDownOptions,
  ComposeLogsOptions,
  ComposeOptions,
  ComposeUpOptions,
  ContainerInfo,
} from "./types";

function shellResultToLogLines(result: ShellResult): string[] {
  const output = result.stderr || result.stdout;
  return toLogLines(output);
}

function dockerFailure(message: string, result: ShellResult, service?: string): FhevmCliError {
  return new FhevmCliError({
    exitCode: ExitCode.DOCKER,
    message: `${message} (exit ${result.exitCode})`,
    service,
    logLines: shellResultToLogLines(result),
  });
}

export function buildComposeArgs(options: ComposeOptions): string[] {
  const args = ["docker", "compose", "-p", options.project];

  for (const file of options.files) {
    args.push("-f", file);
  }

  if (options.envFile) {
    args.push("--env-file", options.envFile);
  }

  return args;
}

async function composeExec(
  args: string[],
  options: ComposeOptions,
  execOptions: { timeoutMs?: number } = {},
): Promise<ShellResult> {
  return exec(args, {
    cwd: options.cwd,
    timeoutMs: execOptions.timeoutMs,
    env: options.envVars,
  });
}

function buildUpArgs(options: ComposeUpOptions): string[] {
  const args = buildComposeArgs(options);
  args.push("up", "-d");

  if (options.build) {
    args.push("--build");
  }
  if (options.noBuild) {
    args.push("--no-build");
  }
  if (options.noCache) {
    args.push("--no-cache");
  }
  if (options.wait) {
    args.push("--wait");
    if (typeof options.waitTimeout === "number") {
      args.push("--timeout", String(options.waitTimeout));
    }
  }
  if (options.services?.length) {
    args.push(...options.services);
  }

  return args;
}

function buildLogsArgs(options: ComposeLogsOptions): string[] {
  const args = buildComposeArgs(options);
  args.push("logs");

  if (options.follow) {
    args.push("-f");
  }
  if (typeof options.tail === "number") {
    args.push("--tail", String(options.tail));
  }
  if (options.noColor) {
    args.push("--no-color");
  }
  if (options.format) {
    args.push("--format", options.format);
  }
  if (options.services?.length) {
    args.push(...options.services);
  }

  return args;
}

export async function composeUp(options: ComposeUpOptions): Promise<void> {
  const result = await composeExec(buildUpArgs(options), options);
  if (result.exitCode !== 0) {
    throw dockerFailure("docker compose up failed", result);
  }
}

export async function composeDown(options: ComposeDownOptions): Promise<void> {
  const args = buildComposeArgs(options);
  args.push("down");

  if (options.volumes) {
    args.push("-v");
  }
  if (options.removeImages) {
    args.push("--rmi", options.removeImages);
  }
  if (typeof options.timeout === "number") {
    args.push("--timeout", String(options.timeout));
  }

  const result = await composeExec(args, options);
  if (result.exitCode !== 0) {
    throw dockerFailure("docker compose down failed", result);
  }
}

export async function composeStop(services: string[], options: ComposeOptions): Promise<void> {
  const args = buildComposeArgs(options);
  args.push("stop", ...services);

  const result = await composeExec(args, options);
  if (result.exitCode !== 0) {
    throw dockerFailure("docker compose stop failed", result);
  }
}

export async function composeStart(services: string[], options: ComposeOptions): Promise<void> {
  const args = buildComposeArgs(options);
  args.push("start", ...services);

  const result = await composeExec(args, options);
  if (result.exitCode !== 0) {
    throw dockerFailure("docker compose start failed", result);
  }
}

export async function composeLogs(options: ComposeLogsOptions): Promise<void> {
  const args = buildLogsArgs(options);

  const process = Bun.spawn(args, {
    cwd: options.cwd,
    env: { ...Bun.env, ...options.envVars },
    stdout: "inherit",
    stderr: "inherit",
  });

  const exitCode = await process.exited;
  if (exitCode !== 0) {
    throw new FhevmCliError({
      exitCode: ExitCode.DOCKER,
      message: `docker compose logs failed (exit ${exitCode})`,
    });
  }
}

export async function composeExecStreaming(
  service: string,
  command: string[],
  options: ComposeOptions & { interactive?: boolean },
): Promise<number> {
  const args = buildComposeArgs(options);
  args.push("exec");

  if (!options.interactive) {
    args.push("-T");
  }

  args.push(service, ...command);

  const process = Bun.spawn(args, {
    cwd: options.cwd,
    env: { ...Bun.env, ...options.envVars },
    stdout: "inherit",
    stderr: "inherit",
  });

  return process.exited;
}

export async function composePs(
  options: ComposeOptions & { all?: boolean },
): Promise<ContainerInfo[]> {
  const args = buildComposeArgs(options);
  args.push("ps", "--format", "json");
  if (options.all !== false) {
    args.push("-a");
  }

  const result = await composeExec(args, options);
  if (result.exitCode !== 0) {
    throw dockerFailure("docker compose ps failed", result);
  }

  return parseComposePsOutput(result.stdout);
}

export async function composeExecIn(
  service: string,
  command: string[],
  options: ComposeOptions & { interactive?: boolean },
): Promise<ShellResult> {
  const args = buildComposeArgs(options);
  args.push("exec");

  if (!options.interactive) {
    args.push("-T");
  }

  args.push(service, ...command);

  const result = await composeExec(args, options);
  if (result.exitCode !== 0) {
    throw dockerFailure("docker compose exec failed", result, service);
  }

  return result;
}

export async function composeWait(
  services: string[],
  options: ComposeOptions & { timeoutMs?: number },
): Promise<number> {
  const args = buildComposeArgs(options);
  args.push("wait", ...services);

  const result = await composeExec(args, options, { timeoutMs: options.timeoutMs });
  return result.exitCode;
}

export async function composePull(services: string[], options: ComposeOptions): Promise<void> {
  const args = buildComposeArgs(options);
  args.push("pull");

  if (services.length > 0) {
    args.push(...services);
  }

  const result = await composeExec(args, options);
  if (result.exitCode !== 0) {
    throw dockerFailure("docker compose pull failed", result);
  }
}

export const __internal = {
  buildUpArgs,
  buildLogsArgs,
};

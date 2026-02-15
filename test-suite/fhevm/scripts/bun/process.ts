import { spawnSync, type SpawnSyncOptionsWithStringEncoding } from "node:child_process";

export interface RunResult {
  status: number;
  stdout: string;
  stderr: string;
}

export interface RunOptions {
  cwd?: string;
  env?: NodeJS.ProcessEnv;
  capture?: boolean;
  check?: boolean;
  allowFailure?: boolean;
}

export function runCommand(command: string[], options: RunOptions = {}): RunResult {
  const [bin, ...args] = command;
  const spawnOptions: SpawnSyncOptionsWithStringEncoding = {
    cwd: options.cwd,
    env: options.env,
    encoding: "utf8",
    stdio: options.capture ? "pipe" : "inherit",
  };

  const result = spawnSync(bin, args, spawnOptions);
  const status = result.status ?? 1;
  const stdout = result.stdout ?? "";
  const stderr = result.stderr ?? "";

  if (result.error) {
    throw result.error;
  }

  if (options.check && status !== 0 && !options.allowFailure) {
    const cmd = command.join(" ");
    const errOut = [stdout.trim(), stderr.trim()].filter(Boolean).join("\n");
    throw new Error(`Command failed (${status}): ${cmd}${errOut ? `\n${errOut}` : ""}`);
  }

  return { status, stdout, stderr };
}

export function sleep(seconds: number): void {
  const milliseconds = Math.max(0, Math.floor(seconds * 1000));
  if (milliseconds === 0) {
    return;
  }

  const buffer = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT);
  const blocker = new Int32Array(buffer);
  Atomics.wait(blocker, 0, 0, milliseconds);
}

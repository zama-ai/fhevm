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

export type RunCommandFn = (command: string[], options?: RunOptions) => RunResult;
export type SleepFn = (seconds: number) => void;

export function runCommand(command: string[], options: RunOptions = {}): RunResult {
  const [bin, ...args] = command;
  const spawnOptions: SpawnSyncOptionsWithStringEncoding = {
    cwd: options.cwd,
    // Bun does not reliably inherit process.env mutations unless env is explicit.
    env: options.env ?? process.env,
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
    const conflictHint = buildPortConflictHint(errOut, spawnOptions);
    const suffix = conflictHint ? `${errOut ? `${errOut}\n` : ""}${conflictHint}` : errOut;
    throw new Error(`Command failed (${status}): ${cmd}${suffix ? `\n${suffix}` : ""}`);
  }

  return { status, stdout, stderr };
}

function buildPortConflictHint(errorOutput: string, spawnOptions: SpawnSyncOptionsWithStringEncoding): string {
  if (!errorOutput.includes("port is already allocated")) {
    return "";
  }

  const ports = new Set<string>();
  for (const match of errorOutput.matchAll(/0\.0\.0\.0:(\d+)/g)) {
    if (match[1]) {
      ports.add(match[1]);
    }
  }

  if (ports.size === 0) {
    return "Port conflict detected. Another local stack is likely already using required ports.";
  }

  const lines: string[] = [];
  for (const port of ports) {
    const holder = spawnSync(
      "docker",
      ["ps", "--filter", `publish=${port}`, "--format", "{{.Names}}"],
      {
        cwd: spawnOptions.cwd,
        env: spawnOptions.env,
        encoding: "utf8",
        stdio: "pipe",
      },
    );
    const holders = (holder.stdout ?? "")
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    lines.push(`Port ${port} is already in use by: ${holders.length > 0 ? holders.join(", ") : "unknown container"}`);
  }

  lines.push("Stop conflicting containers or run this CLI with a clean host before retrying.");
  return lines.join("\n");
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

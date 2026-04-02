/**
 * Wraps process execution, streaming output, compose env loading, and heartbeat reporting for external commands.
 */
import { STATE_DIR, envPath, versionsEnvPath } from "../layout";
import { exists, readEnvFileIfExists, type RunOptions, type RunResult } from "./fs";
import { CommandError } from "../errors";

/** Reads a spawned process pipe into a string when one exists. */
const readPipe = async (stream: ReadableStream<Uint8Array> | number | null | undefined) =>
  stream && typeof stream !== "number" ? new Response(stream).text() : "";

/** Runs a command and captures stdout, stderr, and exit code. */
export const run = async (argv: string[], options: RunOptions = {}): Promise<RunResult> => {
  let proc: ReturnType<typeof Bun.spawn>;
  try {
    proc = Bun.spawn(argv, {
      cwd: options.cwd,
      env: { ...process.env, ...options.env },
      stdin: options.input ? new Blob([options.input]) : undefined,
      stdout: "pipe",
      stderr: "pipe",
    });
  } catch (error) {
    if (options.allowFailure) {
      return {
        stdout: "",
        stderr: error instanceof Error ? error.message : String(error),
        code: 1,
      };
    }
    throw new CommandError(argv, 1, error instanceof Error ? error.message : String(error));
  }
  let timedOut = false;
  const timer =
    options.timeoutMs && options.timeoutMs > 0
      ? setTimeout(() => {
          timedOut = true;
          proc.kill();
        }, options.timeoutMs)
      : undefined;
  const [stdout, stderr, code] = await Promise.all([
    readPipe(proc.stdout),
    readPipe(proc.stderr),
    proc.exited,
  ]);
  if (timer) {
    clearTimeout(timer);
  }
  if (timedOut) {
    const message = `${argv.join(" ")} timed out after ${options.timeoutMs}ms`;
    if (options.allowFailure) {
      return { stdout, stderr: message, code: 124 };
    }
    throw new CommandError(argv, 124, message);
  }
  if (code !== 0 && !options.allowFailure) {
    throw new CommandError(argv, code, (stderr || stdout).trim());
  }
  return { stdout, stderr, code };
};

/** Runs a command with inherited stdio for direct user-facing streaming. */
export const runStreaming = async (
  argv: string[],
  options: Omit<RunOptions, "input"> = {},
): Promise<number> => {
  let proc: ReturnType<typeof Bun.spawn>;
  try {
    proc = Bun.spawn(argv, {
      cwd: options.cwd,
      env: { ...process.env, ...options.env },
      stdout: "inherit",
      stderr: "inherit",
      stdin: "inherit",
    });
  } catch (error) {
    if (options.allowFailure) {
      return 1;
    }
    throw new CommandError(argv, 1, error instanceof Error ? error.message : String(error));
  }
  const code = await proc.exited;
  if (code !== 0 && !options.allowFailure) {
    throw new CommandError(argv, code, "see output above");
  }
  return code;
};

/** Runs a command with live output and periodic heartbeat messages when silent. */
export const runWithHeartbeat = async (
  argv: string[],
  label: string,
  options: Omit<RunOptions, "input"> = {},
) => {
  let stdout = "";
  let stderr = "";
  let sawOutput = false;
  const readLive = async (
    stream: ReadableStream<Uint8Array> | number | null | undefined,
    writer: NodeJS.WriteStream,
    onOutput: () => void,
    capture: "stdout" | "stderr",
  ) => {
    if (!stream || typeof stream === "number") {
      return;
    }
    const reader = stream.getReader();
    try {
      while (true) {
        const { done, value } = await reader.read();
        if (done) {
          return;
        }
        if (value?.length) {
          sawOutput = true;
          onOutput();
          const chunk = Buffer.from(value);
          if (capture === "stdout") {
            stdout += chunk.toString();
          } else {
            stderr += chunk.toString();
          }
          writer.write(chunk);
        }
      }
    } finally {
      reader.releaseLock();
    }
  };

  let proc: ReturnType<typeof Bun.spawn>;
  try {
    proc = Bun.spawn(argv, {
      cwd: options.cwd,
      env: { ...process.env, ...options.env },
      stdin: "inherit",
      stdout: "pipe",
      stderr: "pipe",
    });
  } catch (error) {
    throw new CommandError(argv, 1, error instanceof Error ? error.message : String(error));
  }

  let lastOutput = Date.now();
  let announced = 0;
  const timer = setInterval(() => {
    const silent = Date.now() - lastOutput;
    if (silent >= 15_000 && silent - announced >= 15_000) {
      announced = silent;
      console.log(`[wait] ${label} still running (${Math.floor(silent / 1000)}s)`);
    }
  }, 5_000);

  try {
    const [code] = await Promise.all([
      proc.exited,
      readLive(proc.stdout, process.stdout, () => {
        lastOutput = Date.now();
      }, "stdout"),
      readLive(proc.stderr, process.stderr, () => {
        lastOutput = Date.now();
      }, "stderr"),
    ]);
    if (code !== 0 && !options.allowFailure) {
      throw new CommandError(argv, code, sawOutput ? "" : `${argv.join(" ")} failed (${code})`);
    }
    return { stdout, stderr, code };
  } finally {
    clearInterval(timer);
  }
};

/** Assembles the environment used for docker compose commands for one component. */
export const composeEnv = async (component: string, extra?: Record<string, string>) => {
  const env = (await exists(versionsEnvPath))
    ? { ...(await readEnvFileIfExists(versionsEnvPath)), COMPOSE_IGNORE_ORPHANS: "true", FHEVM_STATE_DIR: STATE_DIR }
    : { COMPOSE_IGNORE_ORPHANS: "true", FHEVM_STATE_DIR: STATE_DIR };
  return { ...env, ...(await readEnvFileIfExists(envPath(component))), ...extra };
};

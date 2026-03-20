/**
 * Wraps process execution, streaming output, compose env loading, and heartbeat reporting for external commands.
 */
import { envPath, versionsEnvPath } from "./layout";
import { exists, readEnvFileIfExists, type RunOptions, type RunResult } from "./utils";
import { CommandError } from "./errors";

const readPipe = async (stream: ReadableStream<Uint8Array> | number | null | undefined) =>
  stream && typeof stream !== "number" ? new Response(stream).text() : "";

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
  const [stdout, stderr, code] = await Promise.all([
    readPipe(proc.stdout),
    readPipe(proc.stderr),
    proc.exited,
  ]);
  if (code !== 0 && !options.allowFailure) {
    throw new CommandError(argv, code, (stderr || stdout).trim());
  }
  return { stdout, stderr, code };
};

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
    throw new CommandError(argv, code, "");
  }
  return code;
};

export const runWithHeartbeat = async (
  argv: string[],
  label: string,
  options: Omit<RunOptions, "input"> = {},
) => {
  let sawOutput = false;
  const readLive = async (
    stream: ReadableStream<Uint8Array> | number | null | undefined,
    writer: NodeJS.WriteStream,
    onOutput: () => void,
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
          writer.write(Buffer.from(value));
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
      }),
      readLive(proc.stderr, process.stderr, () => {
        lastOutput = Date.now();
      }),
    ]);
    if (code !== 0 && !options.allowFailure) {
      throw new CommandError(argv, code, sawOutput ? "" : `${argv.join(" ")} failed (${code})`);
    }
  } finally {
    clearInterval(timer);
  }
};

export const composeEnv = async (component: string, extra?: Record<string, string>) => {
  const env = (await exists(versionsEnvPath))
    ? { ...(await readEnvFileIfExists(versionsEnvPath)), COMPOSE_IGNORE_ORPHANS: "true" }
    : { COMPOSE_IGNORE_ORPHANS: "true" };
  return { ...env, ...(await readEnvFileIfExists(envPath(component))), ...extra };
};

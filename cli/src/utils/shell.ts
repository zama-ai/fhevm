export interface ShellResult {
  exitCode: number;
  stdout: string;
  stderr: string;
}

export interface ExecOptions {
  cwd?: string;
  timeoutMs?: number;
  env?: Record<string, string | undefined>;
}

const SIGKILL_GRACE_MS = 1_000;

export async function exec(command: string[], options: ExecOptions = {}): Promise<ShellResult> {
  try {
    const proc = Bun.spawn(command, {
      cwd: options.cwd,
      env: options.env ? { ...Bun.env, ...options.env } : undefined,
      stdout: "pipe",
      stderr: "pipe",
    });

    let timedOut = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let killTimer: ReturnType<typeof setTimeout> | undefined;

    if (options.timeoutMs && options.timeoutMs > 0) {
      timer = setTimeout(() => {
        timedOut = true;
        try {
          proc.kill("SIGTERM");
        } catch {}
        killTimer = setTimeout(() => {
          try {
            proc.kill("SIGKILL");
          } catch {}
        }, SIGKILL_GRACE_MS);
      }, options.timeoutMs);
    }

    const [stdout, stderr, exitCode] = await Promise.all([
      streamText(proc.stdout),
      streamText(proc.stderr),
      proc.exited,
    ]);

    if (timer) {
      clearTimeout(timer);
    }
    if (killTimer) {
      clearTimeout(killTimer);
    }

    return {
      exitCode: timedOut ? (exitCode === 0 ? 124 : exitCode) : exitCode,
      stdout: stdout.trim(),
      stderr: stderr.trim(),
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return { exitCode: 127, stdout: "", stderr: message };
  }
}

export async function execOk(command: string[], options?: ExecOptions): Promise<boolean> {
  const result = await exec(command, options);
  return result.exitCode === 0;
}

async function streamText(stream: ReadableStream | null): Promise<string> {
  if (!stream) {
    return "";
  }
  return new Response(stream).text();
}

/**
 * Executor abstraction for Bun.spawn.
 *
 * In normal mode: delegates to Bun.spawn.
 * In dry-run mode: records every command as a JSON line to stdout,
 * returns a fake process that exits with 0 and produces no output.
 */

export interface SpawnOpts {
  stdio?: ("inherit" | "pipe")[] | ["inherit", "inherit", "inherit"] | ["pipe", "pipe", "pipe"];
  stdout?: "pipe" | "inherit";
  stderr?: "pipe" | "inherit";
  env?: Record<string, string | undefined>;
}

export interface SpawnResult {
  stdout: ReadableStream<Uint8Array> | null;
  stderr: ReadableStream<Uint8Array> | null;
  exited: Promise<number>;
}

/** A single recorded command from dry-run mode */
export interface DryRunEntry {
  argv: string[];
  env?: Record<string, string | undefined>;
}

let _dryRun = false;
const _recorded: DryRunEntry[] = [];

export function setDryRun(enabled: boolean): void {
  _dryRun = enabled;
  if (enabled) _recorded.length = 0;
}

export function isDryRun(): boolean {
  return _dryRun;
}

/** Get all recorded commands (only meaningful in dry-run mode) */
export function getRecordedCommands(): DryRunEntry[] {
  return _recorded;
}

/** Clear recorded commands */
export function clearRecordedCommands(): void {
  _recorded.length = 0;
}

/**
 * Spawn a process, or record the command in dry-run mode.
 * API mirrors the subset of Bun.spawn that docker.ts uses.
 */
export function spawn(args: string[], opts?: SpawnOpts): SpawnResult {
  if (_dryRun) {
    // Record the command
    const entry: DryRunEntry = { argv: args };
    // Only capture non-inherited env diffs (when env is explicitly passed)
    if (opts?.env) {
      // Only log the env keys that differ from process.env
      const diff: Record<string, string | undefined> = {};
      for (const [k, v] of Object.entries(opts.env)) {
        if (process.env[k] !== v) {
          diff[k] = v;
        }
      }
      if (Object.keys(diff).length > 0) {
        entry.env = diff;
      }
    }
    _recorded.push(entry);

    // Print the JSON line to stderr so it doesn't mix with stdout pipe consumers
    console.error(JSON.stringify(entry));

    // Return a fake result: empty stdout/stderr streams, exit 0
    const emptyStream = new ReadableStream<Uint8Array>({
      start(controller) {
        controller.close();
      },
    });
    const emptyStream2 = new ReadableStream<Uint8Array>({
      start(controller) {
        controller.close();
      },
    });

    return {
      stdout: emptyStream,
      stderr: emptyStream2,
      exited: Promise.resolve(0),
    };
  }

  // Real mode: delegate to Bun.spawn
  const proc = Bun.spawn(args, opts as any);
  return {
    stdout: (proc.stdout as unknown as ReadableStream<Uint8Array>) ?? null,
    stderr: (proc.stderr as unknown as ReadableStream<Uint8Array>) ?? null,
    exited: proc.exited,
  };
}

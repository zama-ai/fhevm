import { Context, Effect, Layer } from "effect";
import { CommandError } from "../errors";
export type { RunOptions, RunResult } from "../utils";

export class CommandRunner extends Context.Tag("CommandRunner")<
  CommandRunner,
  {
    readonly run: (argv: string[], options?: RunOptions) => Effect.Effect<RunResult, CommandError>;
    readonly runLive: (argv: string[], options?: Omit<RunOptions, "input">) => Effect.Effect<number, CommandError>;
  }
>() {
  static Live = Layer.succeed(CommandRunner, {
    run: (argv, options = {}) =>
      Effect.tryPromise({
        try: async () => {
          const proc = Bun.spawn(argv, {
            cwd: options.cwd,
            env: { ...process.env, ...options.env },
            stdin: options.input ? new Blob([options.input]) : undefined,
            stdout: "pipe",
            stderr: "pipe",
          });
          const [stdout, stderr, code] = await Promise.all([
            new Response(proc.stdout).text(),
            new Response(proc.stderr).text(),
            proc.exited,
          ]);
          if (code !== 0 && !options.allowFailure) {
            throw { argv, code, stderr: (stderr || stdout).trim() };
          }
          return { stdout, stderr, code };
        },
        catch: (error) => {
          if (error && typeof error === "object" && "argv" in error) {
            const e = error as { argv: string[]; code: number; stderr: string };
            return new CommandError({ argv: e.argv, code: e.code, stderr: e.stderr });
          }
          return new CommandError({
            argv,
            code: 1,
            stderr: error instanceof Error ? error.message : String(error),
          });
        },
      }),

    runLive: (argv, options = {}) =>
      Effect.tryPromise({
        try: async () => {
          const proc = Bun.spawn(argv, {
            cwd: options.cwd,
            env: { ...process.env, ...options.env },
            stdout: "inherit",
            stderr: "inherit",
            stdin: "inherit",
          });
          const code = await proc.exited;
          if (code !== 0 && !options.allowFailure) {
            throw { argv, code };
          }
          return code;
        },
        catch: (error) => {
          if (error && typeof error === "object" && "argv" in error) {
            const e = error as { argv: string[]; code: number };
            return new CommandError({ argv: e.argv, code: e.code, stderr: "" });
          }
          return new CommandError({
            argv,
            code: 1,
            stderr: error instanceof Error ? error.message : String(error),
          });
        },
      }),
  });
}

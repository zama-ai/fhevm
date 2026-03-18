import { Context, Effect, Layer } from "effect";
import { CommandError } from "../errors";
import type { RunOptions, RunResult } from "../utils";
export type { RunResult } from "../utils";

export class CommandRunner extends Context.Tag("CommandRunner")<
  CommandRunner,
  {
    readonly run: (argv: string[], options?: RunOptions) => Effect.Effect<RunResult, CommandError>;
    readonly runLive: (argv: string[], options?: Omit<RunOptions, "input">) => Effect.Effect<number, CommandError>;
    readonly runWithHeartbeat: (
      argv: string[],
      label: string,
      options?: Omit<RunOptions, "input">,
    ) => Effect.Effect<void, CommandError>;
  }
>() {
  static Live = Layer.succeed(CommandRunner, {
    run: (argv, options = {}) =>
      Effect.tryPromise({
        try: async () => {
          const readPipe = async (
            stream: ReadableStream<Uint8Array> | number | null | undefined,
          ) => (stream && typeof stream !== "number" ? new Response(stream).text() : "");
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
            throw error;
          }
          const [stdout, stderr, code] = await Promise.all([
            readPipe(proc.stdout),
            readPipe(proc.stderr),
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
            throw error;
          }
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

    runWithHeartbeat: (argv, label, options = {}) =>
      Effect.async<void, CommandError>((resume) => {
        const readPipe = async (
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
          resume(
            Effect.fail(
              new CommandError({
                argv,
                code: 1,
                stderr: error instanceof Error ? error.message : String(error),
              }),
            ),
          );
          return;
        }

        let lastOutput = Date.now();
        let announced = 0;
        const timer = setInterval(() => {
          const silent = Date.now() - lastOutput;
          if (silent >= 15_000 && silent - announced >= 15_000) {
            announced = silent;
            console.log(
              `[wait] ${label} still running (${Math.floor(silent / 1000)}s)`,
            );
          }
        }, 5_000);

        Promise.all([
          proc.exited,
          readPipe(proc.stdout, process.stdout, () => {
            lastOutput = Date.now();
          }),
          readPipe(proc.stderr, process.stderr, () => {
            lastOutput = Date.now();
          }),
        ])
          .then(([code]) => {
            clearInterval(timer);
            if (code !== 0 && !options.allowFailure) {
              resume(
                Effect.fail(
                  new CommandError({
                    argv,
                    code,
                    stderr: `${argv.join(" ")} failed (${code})`,
                  }),
                ),
              );
              return;
            }
            resume(Effect.succeed(undefined));
          })
          .catch((error) => {
            clearInterval(timer);
            proc.kill();
            if (options.allowFailure) {
              resume(Effect.succeed(undefined));
              return;
            }
            resume(
              Effect.fail(
                new CommandError({
                  argv,
                  code: 1,
                  stderr: error instanceof Error ? error.message : String(error),
                }),
              ),
            );
          });

        return Effect.sync(() => {
          clearInterval(timer);
          proc.kill();
        });
      }),
  });
}

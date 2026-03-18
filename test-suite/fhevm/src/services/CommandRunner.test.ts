import { describe, expect, test } from "bun:test";
import { Effect } from "effect";
import { CommandRunner } from "./CommandRunner";

describe("CommandRunner", () => {
  test("run captures stdout", async () => {
    const program = Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      const result = yield* cmd.run(["echo", "hello"]);
      return result.stdout.trim();
    });
    const result = await Effect.runPromise(program.pipe(Effect.provide(CommandRunner.Live)));
    expect(result).toBe("hello");
  });

  test("run returns non-zero exit with allowFailure", async () => {
    const program = Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      const result = yield* cmd.run(["sh", "-c", "exit 42"], { allowFailure: true });
      return result.code;
    });
    const code = await Effect.runPromise(program.pipe(Effect.provide(CommandRunner.Live)));
    expect(code).toBe(42);
  });

  test("run fails on non-zero exit without allowFailure", async () => {
    const program = Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      yield* cmd.run(["sh", "-c", "exit 1"]);
    });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(CommandRunner.Live), Effect.either),
    );
    expect(result._tag).toBe("Left");
  });

  test("run returns code 1 for missing commands when allowFailure is set", async () => {
    const program = Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      return yield* cmd.run(["definitely-not-a-real-command"], { allowFailure: true });
    });
    const result = await Effect.runPromise(program.pipe(Effect.provide(CommandRunner.Live)));
    expect(result.code).toBe(1);
    expect(result.stderr.length).toBeGreaterThan(0);
  });

  test("runWithHeartbeat streams successful commands", async () => {
    const program = Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      yield* cmd.runWithHeartbeat(["sh", "-c", "exit 0"], "heartbeat");
    });
    await Effect.runPromise(program.pipe(Effect.provide(CommandRunner.Live)));
  });

  test("runWithHeartbeat fails on non-zero exit", async () => {
    const program = Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      yield* cmd.runWithHeartbeat(["sh", "-c", "exit 1"], "heartbeat");
    });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(CommandRunner.Live), Effect.either),
    );
    expect(result._tag).toBe("Left");
  });
});

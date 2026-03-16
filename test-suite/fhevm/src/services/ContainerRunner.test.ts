import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { ContainerRunner } from "./ContainerRunner";
import { CommandRunner, type RunResult } from "./CommandRunner";

const calls: string[][] = [];
const TestCommandRunner = Layer.succeed(CommandRunner, {
  run: (argv) => {
    calls.push(argv);
    return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
  },
  runLive: (argv) => {
    calls.push(argv);
    return Effect.succeed(0);
  },
});

describe("ContainerRunner", () => {
  test("composeUp calls docker compose up -d", async () => {
    calls.length = 0;
    const program = Effect.gen(function* () {
      const runner = yield* ContainerRunner;
      yield* runner.composeUp("minio");
    });
    await Effect.runPromise(
      program.pipe(Effect.provide(ContainerRunner.Live), Effect.provide(TestCommandRunner)),
    );
    expect(calls.some((c) => c.includes("up") && c.includes("-d"))).toBe(true);
  });

  test("composeUp passes --no-deps when specified", async () => {
    calls.length = 0;
    const program = Effect.gen(function* () {
      const runner = yield* ContainerRunner;
      yield* runner.composeUp("minio", [], { noDeps: true });
    });
    await Effect.runPromise(
      program.pipe(Effect.provide(ContainerRunner.Live), Effect.provide(TestCommandRunner)),
    );
    expect(calls.some((c) => c.includes("--no-deps"))).toBe(true);
  });

  test("composeDown returns true when compose file missing", async () => {
    // composePath for a random name won't exist, so should return true without running anything
    calls.length = 0;
    const program = Effect.gen(function* () {
      const runner = yield* ContainerRunner;
      return yield* runner.composeDown("nonexistent-component");
    });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(ContainerRunner.Live), Effect.provide(TestCommandRunner)),
    );
    expect(result).toBe(true);
  });
});

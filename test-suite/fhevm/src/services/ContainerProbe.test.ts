import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { ContainerProbe } from "./ContainerProbe";
import { CommandRunner, type RunResult } from "./CommandRunner";
import { ContainerCrashed, ProbeTimeout } from "../errors";

const noopHeartbeat = () => Effect.void;

describe("ContainerProbe", () => {
  test("waitForHealthy retries until healthy", async () => {
    let attempt = 0;
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "inspect") {
          attempt++;
          const status = attempt >= 3 ? "healthy" : "starting";
          return Effect.succeed({
            stdout: JSON.stringify([
              {
                Name: "test",
                State: { Status: "running", ExitCode: 0, Health: { Status: status } },
                NetworkSettings: { Networks: {} },
              },
            ]),
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      yield* probe.waitForHealthy("test");
    });
    await Effect.runPromise(
      program.pipe(Effect.provide(ContainerProbe.Live), Effect.provide(TestCmd)),
    );
    expect(attempt).toBeGreaterThanOrEqual(3);
  });

  test("waitForRunning succeeds immediately when running", async () => {
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "inspect") {
          return Effect.succeed({
            stdout: JSON.stringify([
              {
                Name: "test",
                State: { Status: "running", ExitCode: 0 },
                NetworkSettings: { Networks: {} },
              },
            ]),
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      yield* probe.waitForRunning("test");
    });
    await Effect.runPromise(
      program.pipe(Effect.provide(ContainerProbe.Live), Effect.provide(TestCmd)),
    );
  });

  test("waitForHealthy fails with ContainerCrashed on exit code != 0", async () => {
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "inspect") {
          return Effect.succeed({
            stdout: JSON.stringify([
              {
                Name: "test",
                State: { Status: "exited", ExitCode: 1 },
                NetworkSettings: { Networks: {} },
              },
            ]),
            stderr: "",
            code: 0,
          } as RunResult);
        }
        if (argv[0] === "docker" && argv[1] === "logs") {
          return Effect.succeed({
            stdout: "some error log",
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      yield* probe.waitForHealthy("test");
    });
    const result = await Effect.runPromise(
      program.pipe(
        Effect.provide(ContainerProbe.Live),
        Effect.provide(TestCmd),
        Effect.either,
      ),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left).toBeInstanceOf(ContainerCrashed);
      expect((result.left as ContainerCrashed).exitCode).toBe(1);
    }
  });

  test("waitForComplete succeeds when container exits 0", async () => {
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "inspect") {
          return Effect.succeed({
            stdout: JSON.stringify([
              {
                Name: "test",
                State: { Status: "exited", ExitCode: 0 },
                NetworkSettings: { Networks: {} },
              },
            ]),
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      yield* probe.waitForComplete("test");
    });
    await Effect.runPromise(
      program.pipe(Effect.provide(ContainerProbe.Live), Effect.provide(TestCmd)),
    );
  });

  test("waitForComplete fails when container exits non-zero", async () => {
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "inspect") {
          return Effect.succeed({
            stdout: JSON.stringify([
              {
                Name: "test",
                State: { Status: "exited", ExitCode: 1 },
                NetworkSettings: { Networks: {} },
              },
            ]),
            stderr: "",
            code: 0,
          } as RunResult);
        }
        if (argv[0] === "docker" && argv[1] === "logs") {
          return Effect.succeed({
            stdout: "boom",
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      yield* probe.waitForComplete("test");
    });
    const result = await Effect.runPromise(
      program.pipe(
        Effect.provide(ContainerProbe.Live),
        Effect.provide(TestCmd),
        Effect.either,
      ),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left).toBeInstanceOf(ContainerCrashed);
      expect((result.left as ContainerCrashed).exitCode).toBe(1);
    }
  });

  test("waitForLog returns matched text", async () => {
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "logs") {
          return Effect.succeed({
            stdout: "some text handle abc123def done",
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      return yield* probe.waitForLog("test", /handle ([a-zA-Z0-9]+)/);
    });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(ContainerProbe.Live), Effect.provide(TestCmd)),
    );
    expect(result).toBe("handle abc123def");
  });

  test("postBootHealthGate succeeds when all containers running", async () => {
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "inspect") {
          return Effect.succeed({
            stdout: JSON.stringify([
              {
                Name: "test",
                State: { Status: "running", ExitCode: 0 },
                NetworkSettings: { Networks: {} },
              },
            ]),
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      yield* probe.postBootHealthGate(["test-container"], 0);
    });
    await Effect.runPromise(
      program.pipe(Effect.provide(ContainerProbe.Live), Effect.provide(TestCmd)),
    );
  });

  test("postBootHealthGate fails when container crashed", async () => {
    const TestCmd = Layer.succeed(CommandRunner, {
      run: (argv) => {
        if (argv[0] === "docker" && argv[1] === "inspect") {
          return Effect.succeed({
            stdout: JSON.stringify([
              {
                Name: "test",
                State: { Status: "exited", ExitCode: 137 },
                NetworkSettings: { Networks: {} },
              },
            ]),
            stderr: "",
            code: 0,
          } as RunResult);
        }
        if (argv[0] === "docker" && argv[1] === "logs") {
          return Effect.succeed({
            stdout: "OOM killed",
            stderr: "",
            code: 0,
          } as RunResult);
        }
        return Effect.succeed({ stdout: "", stderr: "", code: 0 } as RunResult);
      },
      runLive: () => Effect.succeed(0),
      runWithHeartbeat: noopHeartbeat,
    });

    const program = Effect.gen(function* () {
      const probe = yield* ContainerProbe;
      yield* probe.postBootHealthGate(["test-container"], 0);
    });
    const result = await Effect.runPromise(
      program.pipe(
        Effect.provide(ContainerProbe.Live),
        Effect.provide(TestCmd),
        Effect.either,
      ),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left).toBeInstanceOf(ContainerCrashed);
    }
  });
});

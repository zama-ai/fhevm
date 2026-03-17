import { Context, Effect, Layer, Schedule } from "effect";
import { CommandRunner } from "./CommandRunner";
import { ProbeTimeout, ContainerCrashed } from "../errors";

type ContainerInspect = {
  Name: string;
  State: { Status: string; ExitCode: number; Health?: { Status: string } };
  NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
};

const dockerInspect = (
  cmd: Context.Tag.Service<typeof CommandRunner>,
  name: string,
): Effect.Effect<ContainerInspect[], never, never> =>
  Effect.gen(function* () {
    const result = yield* cmd.run(["docker", "inspect", name], { allowFailure: true });
    if (result.code !== 0) return [];
    return JSON.parse(result.stdout) as ContainerInspect[];
  }).pipe(Effect.catchAll(() => Effect.succeed([] as ContainerInspect[])));

export class ContainerProbe extends Context.Tag("ContainerProbe")<
  ContainerProbe,
  {
    readonly waitForHealthy: (
      container: string,
    ) => Effect.Effect<void, ProbeTimeout | ContainerCrashed>;
    readonly waitForRunning: (
      container: string,
    ) => Effect.Effect<void, ProbeTimeout | ContainerCrashed>;
    readonly waitForComplete: (
      container: string,
    ) => Effect.Effect<void, ProbeTimeout | ContainerCrashed>;
    readonly waitForLog: (
      container: string,
      pattern: RegExp,
    ) => Effect.Effect<string, ProbeTimeout>;
    readonly waitForRpc: (url: string) => Effect.Effect<void, ProbeTimeout>;
    readonly postBootHealthGate: (
      containers: string[],
      delayMs?: number,
    ) => Effect.Effect<void, ContainerCrashed>;
  }
>() {
  static Live = Layer.effect(
    ContainerProbe,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;

      const checkContainer = (
        container: string,
        want: "running" | "healthy" | "complete",
      ): Effect.Effect<void, "not-ready" | ContainerCrashed> =>
        Effect.gen(function* () {
          const [inspect] = yield* dockerInspect(cmd, container);
          if (!inspect) {
            return yield* Effect.fail("not-ready" as const);
          }

          if (want === "healthy" && inspect.State.Health?.Status === "healthy") return;
          if (want === "running" && inspect.State.Status === "running") return;
          if (
            want === "complete" &&
            inspect.State.Status === "exited" &&
            inspect.State.ExitCode === 0
          )
            return;

          if (inspect.State.Status === "exited" && inspect.State.ExitCode !== 0) {
            const logs = yield* cmd.run(["docker", "logs", container], {
              allowFailure: true,
            });
            return yield* Effect.fail(
              new ContainerCrashed({
                container,
                exitCode: inspect.State.ExitCode,
                logs: (logs.stdout + logs.stderr).trim(),
              }),
            );
          }

          return yield* Effect.fail("not-ready" as const);
        }).pipe(
          Effect.catchTag("CommandError", () => Effect.fail("not-ready" as const)),
        );

      const containerRetryPolicy = Schedule.spaced("2 seconds").pipe(
        Schedule.compose(Schedule.recurs(90)),
      );
      const rpcRetryPolicy = Schedule.spaced("1 second").pipe(
        Schedule.compose(Schedule.recurs(60)),
      );
      const logRetryPolicy = Schedule.spaced("2 seconds").pipe(
        Schedule.compose(Schedule.recurs(90)),
      );

      const isNotReady = (error: "not-ready" | ContainerCrashed): error is "not-ready" =>
        error === "not-ready";

      const withContainerRetry = (
        container: string,
        want: "running" | "healthy" | "complete",
      ): Effect.Effect<void, ProbeTimeout | ContainerCrashed> =>
        checkContainer(container, want).pipe(
          Effect.retry({
            while: isNotReady,
            schedule: containerRetryPolicy,
          }),
          Effect.catchAll((error) =>
            isNotReady(error)
              ? Effect.fail(new ProbeTimeout({ container, elapsed: 180 }) as ProbeTimeout | ContainerCrashed)
              : Effect.fail(error as ProbeTimeout | ContainerCrashed),
          ),
          Effect.asVoid,
        );

      return {
        waitForHealthy: (container) => withContainerRetry(container, "healthy"),

        waitForRunning: (container) => withContainerRetry(container, "running"),

        waitForComplete: (container) => withContainerRetry(container, "complete"),

        waitForLog: (container, pattern) =>
          Effect.gen(function* () {
            const logs = yield* cmd.run(["docker", "logs", container], {
              allowFailure: true,
            });
            const combined = logs.stdout + logs.stderr;
            const match = combined.match(pattern);
            if (!match) return yield* Effect.fail("not-ready" as const);
            return match[0];
          }).pipe(
            Effect.catchTag("CommandError", () =>
              Effect.fail("not-ready" as const),
            ),
            Effect.retry({
              while: (error: "not-ready") => error === "not-ready",
              schedule: logRetryPolicy,
            }),
            Effect.mapError(() => new ProbeTimeout({ container, elapsed: 180 })),
          ),

        waitForRpc: (url) =>
          Effect.tryPromise({
            try: async () => {
              const response = await fetch(url, {
                method: "POST",
                headers: { "content-type": "application/json" },
                body: JSON.stringify({
                  jsonrpc: "2.0",
                  id: 1,
                  method: "eth_chainId",
                  params: [],
                }),
              });
              if (!response.ok) throw new Error(`HTTP ${response.status}`);
            },
            catch: () => "not-ready" as const,
          }).pipe(
            Effect.retry({
              while: (error: "not-ready") => error === "not-ready",
              schedule: rpcRetryPolicy,
            }),
            Effect.mapError(() => new ProbeTimeout({ container: url, elapsed: 60 })),
          ),

        postBootHealthGate: (containers, delayMs = 5000) =>
          Effect.gen(function* () {
            if (delayMs > 0) yield* Effect.sleep(`${delayMs} millis`);
            const crashed: { name: string; exitCode: number; logs: string }[] = [];
            for (const name of containers) {
              const [inspect] = yield* dockerInspect(cmd, name);
              if (!inspect) {
                crashed.push({ name, exitCode: -1, logs: "(container not found)" });
                continue;
              }
              if (inspect.State.Status === "exited" && inspect.State.ExitCode !== 0) {
                const result = yield* cmd.run(
                  ["docker", "logs", "--tail", "30", name],
                  { allowFailure: true },
                ).pipe(Effect.catchAll(() => Effect.succeed({ stdout: "", stderr: "", code: 1 })));
                crashed.push({
                  name,
                  exitCode: inspect.State.ExitCode,
                  logs: (result.stdout + result.stderr).trim(),
                });
              }
            }
            if (crashed.length) {
              const details = crashed
                .map(
                  (c) =>
                    `  ${c.name} (exit ${c.exitCode}):\n    ${c.logs.split("\n").join("\n    ")}`,
                )
                .join("\n");
              return yield* Effect.fail(
                new ContainerCrashed({
                  container: crashed[0].name,
                  exitCode: crashed[0].exitCode,
                  logs: `Post-boot health gate: ${crashed.length} container(s) crashed:\n${details}`,
                }),
              );
            }
          }),
      };
    }),
  );
}

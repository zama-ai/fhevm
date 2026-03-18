import { Effect, Layer } from "effect";
import { BunContext } from "@effect/platform-bun";
import type { RunOptions, RunResult, Runner } from "./utils";
import type { LocalOverride, State, VersionBundle } from "./types";
import { PORTS } from "./layout";
import { CommandRunner } from "./services/CommandRunner";
import { ContainerRunner } from "./services/ContainerRunner";
import { ContainerProbe } from "./services/ContainerProbe";
import { ImageBuilder } from "./services/ImageBuilder";
import { RpcClient } from "./services/RpcClient";
import { MinioClient } from "./services/MinioClient";
import { GitHubClient } from "./services/GitHubClient";
import { EnvWriter } from "./services/EnvWriter";
import { StateManager } from "./services/StateManager";
import { CommandError } from "./errors";
import { defaultCoprocessorScenario } from "./scenario";

const STUB_VERSION_ENV: Record<string, string> = {
  GATEWAY_VERSION: "v0.11.0",
  HOST_VERSION: "v0.11.0",
  COPROCESSOR_DB_MIGRATION_VERSION: "v0.11.0",
  COPROCESSOR_HOST_LISTENER_VERSION: "v0.11.0",
  COPROCESSOR_GW_LISTENER_VERSION: "v0.11.0",
  COPROCESSOR_TX_SENDER_VERSION: "v0.11.0",
  COPROCESSOR_TFHE_WORKER_VERSION: "v0.11.0",
  COPROCESSOR_ZKPROOF_WORKER_VERSION: "v0.11.0",
  COPROCESSOR_SNS_WORKER_VERSION: "v0.11.0",
  CONNECTOR_DB_MIGRATION_VERSION: "v0.11.0",
  CONNECTOR_GW_LISTENER_VERSION: "v0.11.0",
  CONNECTOR_KMS_WORKER_VERSION: "v0.11.0",
  CONNECTOR_TX_SENDER_VERSION: "v0.11.0",
  CORE_VERSION: "v0.13.0",
  RELAYER_VERSION: "v0.9.0",
  RELAYER_MIGRATE_VERSION: "v0.9.0",
  TEST_SUITE_VERSION: "v0.11.0",
};

export const stubBundle = (overrides?: {
  env?: Record<string, string>;
  lockName?: string;
  sources?: string[];
}): VersionBundle => ({
  target: "latest-supported",
  lockName: overrides?.lockName ?? "test.json",
  sources: overrides?.sources ?? ["test"],
  env: { ...STUB_VERSION_ENV, ...overrides?.env },
});

export const stubState = (overrides?: {
  envOverrides?: Record<string, string>;
  count?: number;
  threshold?: number;
  overrides?: LocalOverride[];
  discovery?: State["discovery"];
  completedSteps?: State["completedSteps"];
}): State => ({
  target: "latest-supported",
  lockPath: "/tmp/test.json",
  versions: stubBundle({ env: overrides?.envOverrides }),
  overrides: overrides?.overrides ?? [],
  scenario: {
    ...defaultCoprocessorScenario(),
    topology: {
      count: overrides?.count ?? 1,
      threshold: overrides?.threshold ?? overrides?.count ?? 1,
    },
    instances: Array.from({ length: overrides?.count ?? 1 }, (_, index) => ({
      index,
      source: { mode: "inherit" as const },
      env: {},
      args: {},
    })),
  },
  discovery: overrides?.discovery,
  completedSteps: overrides?.completedSteps ?? [],
  updatedAt: "2026-03-09T00:00:00.000Z",
});

export const fakeRunner = (responses: Record<string, string | RunResult>): Runner =>
  async (argv: string[], _options?: RunOptions) => {
    const key = argv.join(" ");
    const value = responses[key];
    if (value === undefined) {
      throw new Error(`Missing fake response for ${key}`);
    }
    if (typeof value === "string") {
      return { stdout: value, stderr: "", code: 0 };
    }
    return value;
  };

export const portCheckResponses = Object.fromEntries(
  PORTS.map((p) => [`lsof -nP -iTCP:${p} -sTCP:LISTEN`, { stdout: "", stderr: "", code: 1 }]),
) as Record<string, RunResult>;

export const captureConsole = (method: "log" | "error" | "warn") => {
  const logs: string[] = [];
  const orig = console[method];
  console[method] = (msg: string) => logs.push(msg);
  return { logs, restore: () => { console[method] = orig; } };
};

export const noopDeps = {
  runner: async (_argv?: string[], _options?: RunOptions) => ({ stdout: "", stderr: "", code: 0 }) as RunResult,
  liveRunner: async () => 0,
  now: () => "2026-03-06T00:00:00.000Z",
  fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
  env: {},
};

/**
 * Converts old-style deps object to an Effect Layer for testing.
 * This bridges the old test pattern `main(argv, deps)` to the new
 * Effect-based `main(argv, layer)`.
 */
export const depsToLayer = (deps: {
  runner?: Runner;
  liveRunner?: (...args: any[]) => Promise<number>;
  fetch?: typeof globalThis.fetch;
  env?: Record<string, string>;
}) => {
  const TestCommandRunner = Layer.succeed(CommandRunner, {
    run: (argv: string[], options?: any) =>
      Effect.tryPromise({
        try: () => (deps.runner ?? noopDeps.runner)(argv, options),
        catch: (e) =>
          new CommandError({
            argv,
            code: 1,
            stderr: e instanceof Error ? e.message : String(e),
          }),
      }),
    runLive: (argv: string[], options?: any) =>
      Effect.tryPromise({
        try: () => (deps.liveRunner ?? noopDeps.liveRunner)(argv, options as any),
        catch: (e) =>
          new CommandError({
            argv,
            code: 1,
            stderr: e instanceof Error ? e.message : String(e),
          }),
      }),
    runWithHeartbeat: (argv: string[], _label: string, options?: any) =>
      Effect.tryPromise({
        try: async () => {
          const code = await (deps.liveRunner ?? noopDeps.liveRunner)(argv, options as any);
          if (code !== 0 && !options?.allowFailure) {
            throw new Error(`${argv.join(" ")} failed (${code})`);
          }
        },
        catch: (e) =>
          new CommandError({
            argv,
            code: 1,
            stderr: e instanceof Error ? e.message : String(e),
          }),
      }),
  });

  // Build the full layer with TestCommandRunner replacing the real one.
  // All services that depend on CommandRunner will use the test one.
  const TestContainerRunner = ContainerRunner.Live.pipe(Layer.provide(TestCommandRunner));
  const TestContainerProbe = ContainerProbe.Live.pipe(Layer.provide(TestCommandRunner));
  const TestGitHubClient = GitHubClient.Live.pipe(Layer.provide(TestCommandRunner));
  const TestEnvWriter = EnvWriter.Live.pipe(Layer.provide(TestCommandRunner));
  const TestImageBuilder = ImageBuilder.Live.pipe(
    Layer.provide(TestCommandRunner),
    Layer.provide(TestContainerRunner),
  );
  const TestMinioClient = MinioClient.Live.pipe(Layer.provide(TestCommandRunner));
  const TestRpcClient = RpcClient.Live.pipe(Layer.provide(TestCommandRunner));

  return Layer.mergeAll(
    TestCommandRunner,
    TestContainerRunner,
    TestContainerProbe,
    TestGitHubClient,
    TestEnvWriter,
    TestImageBuilder,
    TestMinioClient,
    TestRpcClient,
    StateManager.Live,
    BunContext.layer,
  );
};

export const noopLayer = depsToLayer(noopDeps);

import type { RunOptions, RunResult, Runner } from "./utils";
import type { LocalOverride, State, VersionBundle } from "./types";
import { PORTS } from "./layout";

export const STUB_VERSION_ENV: Record<string, string> = {
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
  target: "latest-release",
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
  target: "latest-release",
  lockPath: "/tmp/test.json",
  versions: stubBundle({ env: overrides?.envOverrides }),
  overrides: overrides?.overrides ?? [],
  topology: {
    count: overrides?.count ?? 1,
    threshold: overrides?.threshold ?? overrides?.count ?? 1,
    instances: {},
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

export const captureConsole = (method: "log" | "error") => {
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
};

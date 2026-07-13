import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  clients: [] as Array<{
    baseUrl: string;
    apiKey?: string;
    isReady: ReturnType<typeof vi.fn>;
    close: ReturnType<typeof vi.fn>;
  }>,
  readiness: new Map<string, boolean>(),
  closeFailures: new Map<string, Error>(),
}));

vi.mock("../src/relayer/client", () => ({
  RelayerClient: class {
    readonly baseUrl: string;
    readonly apiKey?: string;
    readonly isReady: ReturnType<typeof vi.fn>;
    readonly close: ReturnType<typeof vi.fn>;

    constructor(options: { baseUrl: string; apiKey?: string }) {
      this.baseUrl = options.baseUrl;
      this.apiKey = options.apiKey;
      this.isReady = vi.fn(async () => mocks.readiness.get(this.baseUrl) ?? true);
      this.close = vi.fn(async () => {
        const failure = mocks.closeFailures.get(this.baseUrl);
        if (failure) throw failure;
      });
      mocks.clients.push(this);
    }
  },
}));

vi.mock("../src/shared/logger", () => ({
  logger: { success: vi.fn(), warn: vi.fn() },
}));

import { assertRelayerReadiness } from "../src/runner/readiness";

const env = {
  network: "testnet" as const,
  contractChainId: 11_155_111,
  relayerUrl: "https://legacy.example",
  relayerBUrl: "https://v2.example",
  dataDir: ".load-test",
};

const originalApiKey = process.env.ZAMA_FHEVM_API_KEY;
const originalApiKeyB = process.env.ZAMA_FHEVM_API_KEY_B;

beforeEach(() => {
  mocks.clients.length = 0;
  mocks.readiness.clear();
  mocks.closeFailures.clear();
  delete process.env.ZAMA_FHEVM_API_KEY;
  delete process.env.ZAMA_FHEVM_API_KEY_B;
});

afterEach(() => {
  if (originalApiKey === undefined) delete process.env.ZAMA_FHEVM_API_KEY;
  else process.env.ZAMA_FHEVM_API_KEY = originalApiKey;
  if (originalApiKeyB === undefined) delete process.env.ZAMA_FHEVM_API_KEY_B;
  else process.env.ZAMA_FHEVM_API_KEY_B = originalApiKeyB;
});

describe("assertRelayerReadiness", () => {
  it("uses ZAMA_FHEVM_API_KEY_B for the candidate, falling back to the shared key", async () => {
    process.env.ZAMA_FHEVM_API_KEY = "shared-key";
    await assertRelayerReadiness({ env });
    expect(mocks.clients.map((client) => client.apiKey)).toEqual(["shared-key", "shared-key"]);

    mocks.clients.length = 0;
    process.env.ZAMA_FHEVM_API_KEY_B = "candidate-key";
    await assertRelayerReadiness({ env });
    expect(mocks.clients.map((client) => client.apiKey)).toEqual(["shared-key", "candidate-key"]);
  });

  it("checks and closes independently owned A/B clients", async () => {
    await assertRelayerReadiness({ env });

    expect(mocks.clients.map((client) => client.baseUrl)).toEqual([
      "https://legacy.example",
      "https://v2.example",
    ]);
    expect(mocks.clients.every((client) => client.isReady.mock.calls.length === 1)).toBe(true);
    expect(mocks.clients.every((client) => client.close.mock.calls.length === 1)).toBe(true);
  });

  it("closes both clients when candidate readiness fails", async () => {
    mocks.readiness.set("https://v2.example", false);
    await expect(assertRelayerReadiness({ env })).rejects.toThrow(/Candidate relayer/);
    expect(mocks.clients.every((client) => client.close.mock.calls.length === 1)).toBe(true);
  });

  it("preserves readiness failure and every cleanup failure", async () => {
    const primaryCloseFailure = new Error("primary close failed");
    const candidateCloseFailure = new Error("candidate close failed");
    mocks.readiness.set("https://legacy.example", false);
    mocks.closeFailures.set("https://legacy.example", primaryCloseFailure);
    mocks.closeFailures.set("https://v2.example", candidateCloseFailure);

    let thrown: unknown;
    try {
      await assertRelayerReadiness({ env });
    } catch (error) {
      thrown = error;
    }
    expect(thrown).toBeInstanceOf(AggregateError);
    expect((thrown as AggregateError).cause).toBeInstanceOf(Error);
    expect(((thrown as AggregateError).cause as Error).message).toBe("Relayer at https://legacy.example failed the readiness check (GET /health/readiness). Older relayers expose health elsewhere (e.g. /liveness, /healthz); pass --no-readiness-check to proceed.");
    expect((thrown as AggregateError).errors).toEqual([
      expect.any(Error), primaryCloseFailure, candidateCloseFailure,
    ]);
  });

  it("preserves candidate failure when one client close also fails", async () => {
    const closeFailure = new Error("candidate close failed");
    mocks.readiness.set("https://v2.example", false);
    mocks.closeFailures.set("https://v2.example", closeFailure);

    let thrown: unknown;
    try {
      await assertRelayerReadiness({ env });
    } catch (error) {
      thrown = error;
    }
    expect(thrown).toBeInstanceOf(AggregateError);
    expect(((thrown as AggregateError).cause as Error).message).toContain("Candidate relayer");
    expect((thrown as AggregateError).errors).toEqual([expect.any(Error), closeFailure]);
  });

  it("does not allocate clients when readiness is explicitly skipped", async () => {
    await assertRelayerReadiness({ env, skipReadiness: true });
    expect(mocks.clients).toEqual([]);
  });
});

import { mkdtemp, readFile, rm, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterEach, describe, expect, it, vi } from "vitest";

import type { LoadTestEnv } from "../src/env";
import {
  formatPoolPlan,
  inspectPoolRequirements,
  preparePoolRequirements,
  type PoolPlanningOperations,
} from "../src/pool/planning";
import type { PoolDeficit } from "../src/pool/requirements";
import { createBuiltinScenario } from "../src/scenario/builtin";

const directories: string[] = [];

afterEach(async () => {
  await Promise.all(directories.splice(0).map((directory) =>
    rm(directory, { recursive: true, force: true }),
  ));
});

const temporaryDirectory = async (): Promise<string> => {
  const directory = await mkdtemp(join(tmpdir(), "load-test-pool-planning-"));
  directories.push(directory);
  return directory;
};

const env: LoadTestEnv = {
  network: "testnet",
  relayerUrl: "https://alice:secret@relayer.example/v2?api_key=secret",
  relayerApiPrefix: "/legacy",
  relayerBUrl: "https://candidate.example",
  relayerBApiPrefix: "/v2",
  contractChainId: 11155111,
  dataDir: "/not-recorded/load-test",
};

const scenario = createBuiltinScenario("open-steady", {
  rps: 2,
  durationSec: 5,
});

const fixedNow = (): Date => new Date("2026-07-13T12:00:00.000Z");

const inputProofDeficit = (deficit: number): PoolDeficit => ({
  pool: "input-proof",
  flow: "input-proof",
  current: 8,
  available: 8,
  workload: { mode: "finite", requestBudget: 10 },
  deficit,
  detail: "single-use payloads; one per request",
});

const delegatedDeficit = (
  deficit: number,
  refreshRequired: boolean,
): PoolDeficit => ({
  pool: "delegated-user-decrypt-handles",
  flow: "delegated-user-decrypt",
  current: deficit === 0 ? 4 : 3,
  available: deficit === 0 ? 4 : 3,
  workload: { mode: "finite", requestBudget: 10 },
  targetItems: 4,
  deficit,
  refreshRequired,
  requiredValidUntil: "1783950000",
  detail: refreshRequired ? "ACL refresh required" : "handles are reusable",
});

const inertOperations = (
  inspect: PoolPlanningOperations["inspect"],
): Partial<PoolPlanningOperations> => ({
  inspect,
  now: fixedNow,
  generateInputProof: vi.fn(async () => undefined) as never,
  createHandles: vi.fn(async () => undefined) as never,
  refreshDelegationAcl: vi.fn(async () => undefined),
});

describe("pool planning evidence", () => {
  it("inspects absent live pool storage without creating it", async () => {
    const root = await temporaryDirectory();
    const liveEnv = { ...env, dataDir: join(root, "data") };
    const plan = await inspectPoolRequirements({
      env: liveEnv,
      scenarios: [scenario],
      artifactDir: join(root, "evidence"),
      operations: { now: fixedNow },
    });

    expect(plan.items).toEqual([
      expect.objectContaining({
        pool: "input-proof",
        observation: { currentItems: 0, availableItems: 0 },
        decision: expect.objectContaining({ deficitItems: 10, ready: false }),
      }),
    ]);
    await expect(stat(join(root, "data", "pools"))).rejects.toThrow();
  });

  it("records public-decrypt availability as per-k combination evidence", async () => {
    const root = await temporaryDirectory();
    const publicScenario = createBuiltinScenario("open-steady", {
      flow: "public-decrypt",
      rps: 2,
      durationSec: 5,
    });
    const plan = await inspectPoolRequirements({
      env: { ...env, dataDir: join(root, "data") },
      scenarios: [publicScenario],
      artifactDir: join(root, "evidence"),
      operations: { now: fixedNow },
    });

    const observation = plan.items[0]?.observation;
    expect(observation).not.toHaveProperty("availableItems");
    expect(observation?.combinationCapacity).toEqual([{
      handlesPerRequest: 1,
      totalCombinations: "0",
      consumedCombinations: "0",
      availableCombinations: "0",
      neededRequests: 10,
    }]);
    expect(await readFile(join(root, "evidence", "pool-plan.json"), "utf8"))
      .not.toContain(Number.MAX_SAFE_INTEGER.toString());
  });

  it("writes a normalized, secret-free, versioned read-only plan", async () => {
    const artifactDir = await temporaryDirectory();
    const inspect = vi.fn(async () => [inputProofDeficit(2)]);
    const operations = inertOperations(inspect);

    const first = await inspectPoolRequirements({
      env,
      scenarios: [scenario],
      pauseSec: 30,
      artifactDir,
      operations,
    });
    const second = await inspectPoolRequirements({
      env,
      scenarios: [scenario],
      pauseSec: 30,
      operations,
    });

    expect(first.version).toBe(2);
    expect(first.kind).toBe("load-test-pool-plan");
    expect(first.scenarioDigest).toBe(second.scenarioDigest);
    expect(first.ready).toBe(false);
    expect(first.plannedActions).toEqual([{
      kind: "generate-input-proof",
      pool: "input-proof",
      flow: "input-proof",
      items: 2,
    }]);
    expect(first.environment.relayer).toBe("https://relayer.example/v2");
    expect(first.environment.relayerApiPrefix).toBe("/legacy");
    expect(first.environment.relayerBApiPrefix).toBe("/v2");
    expect(first.environment).not.toHaveProperty("rpcUrl");
    expect(first.environment).not.toHaveProperty("dataDir");
    expect(operations.generateInputProof).not.toHaveBeenCalled();
    expect(operations.createHandles).not.toHaveBeenCalled();

    const json = await readFile(join(artifactDir, "pool-plan.json"), "utf8");
    const markdown = await readFile(join(artifactDir, "pool-plan.md"), "utf8");
    expect(json).not.toContain("alice");
    expect(json).not.toContain("secret");
    expect(JSON.parse(json)).toEqual(first);
    expect(markdown).toContain("Scenario digest");
    expect(markdown).toContain("action required");
    expect(formatPoolPlan(first).join("\n")).toContain("finite budget 10 request(s)");
    expect(formatPoolPlan(first).join("\n")).toContain("[LOCAL CPU]");
    expect(formatPoolPlan(first).join("\n")).toContain("0 funded transactions");
    const onChain = formatPoolPlan({
      ...first,
      plannedActions: [
        {
          kind: "create-handles",
          pool: "delegated-user-decrypt-handles",
          flow: "delegated-user-decrypt",
          items: 3,
        },
        {
          kind: "refresh-delegation-acl",
          pool: "delegated-user-decrypt-handles",
          flow: "delegated-user-decrypt",
          requiredValidUntil: "1783950000",
        },
      ],
    }).join("\n");
    expect(onChain).toContain("[ON-CHAIN]");
    expect(onChain).toContain("3 funded setter transaction(s), one per handle");
    expect(onChain).toContain("up to one per existing owner lane");
  });

  it("prepares explicit actions and re-inspects live state before declaring readiness", async () => {
    const artifactDir = await temporaryDirectory();
    let inspections = 0;
    const inspect = vi.fn(async () => {
      inspections += 1;
      return inspections === 1
        ? [inputProofDeficit(2), delegatedDeficit(1, true)]
        : [inputProofDeficit(0), delegatedDeficit(0, false)];
    });
    const generateInputProof = vi.fn(async () => undefined) as never;
    const createHandles = vi.fn(async () => undefined) as never;
    const refreshDelegationAcl = vi.fn(async () => undefined);

    const result = await preparePoolRequirements({
      env,
      scenarios: [scenario],
      pauseSec: 30,
      artifactDir,
      lanes: 3,
      operations: {
        inspect,
        now: fixedNow,
        generateInputProof,
        createHandles,
        refreshDelegationAcl,
      },
    });

    expect(inspect).toHaveBeenCalledTimes(2);
    expect(generateInputProof).toHaveBeenCalledWith(env, expect.objectContaining({ count: 2 }));
    expect(createHandles).toHaveBeenCalledWith(env, expect.objectContaining({
      flow: "delegated-user-decrypt",
      count: 1,
      lanes: 3,
    }));
    expect(refreshDelegationAcl).toHaveBeenCalledWith(env, expect.objectContaining({
      requiredValidUntil: expect.any(BigInt),
    }));
    expect(result.plan.ready).toBe(true);
    expect(result.preparation.status).toBe("completed");
    expect(result.preparation.initialReady).toBe(false);
    expect(result.preparation.finalReady).toBe(true);
    expect(result.preparation.finalInspection.status).toBe("completed");
    expect(result.preparation.actions.map((action) => action.status)).toEqual([
      "completed",
      "completed",
      "completed",
    ]);

    const artifact = JSON.parse(
      await readFile(join(artifactDir, "preparation.json"), "utf8"),
    ) as { status: string; finalReady: boolean; actions: unknown[] };
    expect(artifact.status).toBe("completed");
    expect(artifact.finalReady).toBe(true);
    expect(artifact.actions).toHaveLength(3);
    expect(await readFile(join(artifactDir, "preparation.md"), "utf8"))
      .toContain("Final readiness:** ready");
  });

  it("persists redacted failure evidence and still performs final inspection", async () => {
    const artifactDir = await temporaryDirectory();
    const inspect = vi.fn(async () => [inputProofDeficit(2)]);
    const generateInputProof = vi.fn(async (_env, options) => {
      options.onProgress?.(1, 2);
      throw new Error("provider failed api_key=super-secret-value");
    }) as never;

    await expect(preparePoolRequirements({
      env,
      scenarios: [scenario],
      artifactDir,
      operations: {
        ...inertOperations(inspect),
        generateInputProof,
      },
    })).rejects.toThrow("provider failed");

    expect(inspect).toHaveBeenCalledTimes(2);
    const json = await readFile(join(artifactDir, "preparation.json"), "utf8");
    const artifact = JSON.parse(json) as {
      status: string;
      finalReady: boolean;
      actions: Array<{ status: string; actualItems?: number }>;
    };
    expect(artifact.status).toBe("failed");
    expect(artifact.finalReady).toBe(false);
    expect(artifact.actions[0]?.status).toBe("failed");
    expect(artifact.actions[0]?.actualItems).toBe(1);
    expect(json).not.toContain("super-secret-value");
    expect(json).toContain("[REDACTED]");
  });

  it("persists interruption evidence without executing a stored plan", async () => {
    const artifactDir = await temporaryDirectory();
    const controller = new AbortController();
    controller.abort();
    const inspect = vi.fn(async () => [inputProofDeficit(2)]);
    const operations = inertOperations(inspect);

    await expect(preparePoolRequirements({
      env,
      scenarios: [scenario],
      artifactDir,
      signal: controller.signal,
      operations,
    })).rejects.toThrow();

    expect(operations.generateInputProof).not.toHaveBeenCalled();
    expect(inspect).toHaveBeenCalledTimes(2);
    const artifact = JSON.parse(
      await readFile(join(artifactDir, "preparation.json"), "utf8"),
    ) as { status: string; actions: Array<{ status: string }> };
    expect(artifact.status).toBe("interrupted");
    expect(artifact.actions).toEqual([]);
  });

  it("runs the authoritative mutation gate after re-planning and skips it for no-op", async () => {
    const artifactDir = await temporaryDirectory();
    const events: string[] = [];
    const inspect = vi.fn(async () => [inputProofDeficit(2)]);
    const generateInputProof = vi.fn(async () => { events.push("mutate"); }) as never;
    const beforeActions = vi.fn(async () => { events.push("gate"); });

    await expect(preparePoolRequirements({
      env,
      scenarios: [scenario],
      artifactDir,
      beforeActions,
      operations: {
        ...inertOperations(inspect),
        generateInputProof,
      },
    })).rejects.toThrow(/not ready/);

    expect(events).toEqual(["gate", "mutate"]);
    expect(beforeActions).toHaveBeenCalledWith(expect.objectContaining({ ready: false }));

    const noOpGate = vi.fn(async () => undefined);
    const readyInspect = vi.fn(async () => [inputProofDeficit(0)]);
    await preparePoolRequirements({
      env,
      scenarios: [scenario],
      artifactDir: await temporaryDirectory(),
      beforeActions: noOpGate,
      operations: inertOperations(readyInspect),
    });
    expect(noOpGate).not.toHaveBeenCalled();
  });

  it("renders duration-bound reusable work as a target, never a fake request", async () => {
    const reusable = {
      ...delegatedDeficit(0, false),
      workload: { mode: "duration-bound" as const },
      targetItems: 4,
    };
    const plan = await inspectPoolRequirements({
      env,
      scenarios: [scenario],
      operations: inertOperations(vi.fn(async () => [reusable])),
    });
    const output = formatPoolPlan(plan).join("\n");
    expect(output).toContain("duration-bound workload");
    expect(output).toContain("reusable pool target 4 handle(s)");
    expect(output).not.toContain("requires 1 request");
    expect(JSON.stringify(plan)).not.toContain('"requests":1');
  });

  it("persists gate failure evidence and performs no mutation", async () => {
    const artifactDir = await temporaryDirectory();
    const inspect = vi.fn(async () => [inputProofDeficit(2)]);
    const operations = inertOperations(inspect);

    await expect(preparePoolRequirements({
      env,
      scenarios: [scenario],
      artifactDir,
      beforeActions: async () => { throw new Error("relayer unavailable"); },
      operations,
    })).rejects.toThrow("relayer unavailable");

    expect(operations.generateInputProof).not.toHaveBeenCalled();
    expect(JSON.parse(await readFile(join(artifactDir, "preparation.json"), "utf8")))
      .toMatchObject({ status: "failed", error: "relayer unavailable", actions: [] });
  });

  it("retains confirmed partial work when interrupted during generation", async () => {
    const artifactDir = await temporaryDirectory();
    const controller = new AbortController();
    const inspect = vi.fn(async () => [inputProofDeficit(2)]);
    const generateInputProof = vi.fn(async (_env, options) => {
      options.onProgress?.(1, 2);
      controller.abort();
    }) as never;

    await expect(preparePoolRequirements({
      env,
      scenarios: [scenario],
      artifactDir,
      signal: controller.signal,
      operations: {
        ...inertOperations(inspect),
        generateInputProof,
      },
    })).rejects.toThrow();

    const artifact = JSON.parse(
      await readFile(join(artifactDir, "preparation.json"), "utf8"),
    ) as { status: string; actions: Array<{ status: string; actualItems?: number }> };
    expect(artifact.status).toBe("interrupted");
    expect(artifact.actions[0]).toEqual(expect.objectContaining({
      status: "interrupted",
      actualItems: 1,
    }));
  });

  it("marks failed final inspection unavailable instead of reusing stale observations", async () => {
    const artifactDir = await temporaryDirectory();
    let inspections = 0;
    const inspect = vi.fn(async () => {
      inspections += 1;
      if (inspections === 1) return [inputProofDeficit(2)];
      throw new Error("live pool metadata unavailable");
    });

    await expect(preparePoolRequirements({
      env,
      scenarios: [scenario],
      artifactDir,
      operations: inertOperations(inspect),
    })).rejects.toThrow("live pool metadata unavailable");

    const artifact = JSON.parse(
      await readFile(join(artifactDir, "preparation.json"), "utf8"),
    ) as Record<string, unknown> & {
      finalInspection: { status: string; error?: string };
    };
    expect(artifact.finalInspection).toEqual({
      status: "failed",
      error: "live pool metadata unavailable",
    });
    expect(artifact).not.toHaveProperty("finalItems");
    expect(artifact).not.toHaveProperty("finalReady");
  });
});

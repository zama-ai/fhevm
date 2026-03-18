import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, describe, expect, test } from "bun:test";

import { Effect, Layer } from "effect";

import { resolvedComposeEnv, rewriteCoprocessorDependsOn } from "./codegen";
import { resolveEnvMap } from "./services/EnvWriter";
import {
  COMPONENTS,
  REPO_ROOT,
  STATE_DIR,
  TEST_GREP,
  composePath,
  envPath,
  gatewayAddressesSolidityPath,
  relayerConfigPath,
  resolveServiceOverrides,
  versionsEnvPath,
} from "./layout";
import { STEP_NAMES } from "./types";
import { main } from "./cli";
import { probeBootstrap, resolveUpgradePlan } from "./pipeline";
import { COMPAT_MATRIX, compatPolicyForState, requiresMultichainAclAddress, resolveWorkflowCompatEnv, validateBundleCompatibility } from "./compat";
import { predictedCrsId, predictedKeyId } from "./utils";
import { applyVersionEnvOverrides, resolveTarget } from "./resolve";
import { expandBuildOverrides } from "./options";
import { applyInstanceAdjustments } from "./render-compose";
import { CommandRunner } from "./services/CommandRunner";
import { MinioError } from "./errors";
import { GitHubClient } from "./services/GitHubClient";
import { ContainerRunner } from "./services/ContainerRunner";
import { MinioClient } from "./services/MinioClient";
import {
  captureConsole,
  depsToLayer,
  fakeRunner,
  noopDeps,
  noopLayer,
  portCheckResponses,
  stubBundle,
  stubState,
} from "./test-helpers";

const STATE_FILE = path.join(STATE_DIR, "state.json");

const tempDirs: string[] = [];

afterEach(async () => {
  process.exitCode = 0;
  while (tempDirs.length) {
    await fs.rm(tempDirs.pop()!, { recursive: true, force: true });
  }
});

const fixtureDir = async () => {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), "fhevm-cli-"));
  tempDirs.push(dir);
  return dir;
};

const maybeRead = (file: string) => fs.readFile(file, "utf8").catch(() => undefined);

const readyDiscovery = () => ({
  gateway: {
    GATEWAY_CONFIG_ADDRESS: "0x1",
    KMS_GENERATION_ADDRESS: "0x2",
    DECRYPTION_ADDRESS: "0x3",
    INPUT_VERIFICATION_ADDRESS: "0x4",
    CIPHERTEXT_COMMITS_ADDRESS: "0x5",
    MULTICHAIN_ACL_ADDRESS: "0x6",
  },
  host: {
    ACL_CONTRACT_ADDRESS: "0x7",
    FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0x8",
    KMS_VERIFIER_CONTRACT_ADDRESS: "0x9",
    INPUT_VERIFIER_CONTRACT_ADDRESS: "0xa",
    PAUSER_SET_CONTRACT_ADDRESS: "0xb",
  },
  kmsSigner: "0xc",
  fheKeyId: predictedKeyId(),
  crsKeyId: predictedCrsId(),
  actualFheKeyId: predictedKeyId(),
  actualCrsKeyId: predictedCrsId(),
  endpoints: {
    gatewayHttp: "http://gateway-node:8546",
    gatewayWs: "ws://gateway-node:8546",
    hostHttp: "http://host-node:8545",
    hostWs: "ws://host-node:8545",
    minioInternal: "http://minio:9000",
    minioExternal: "http://minio:9000",
  },
});

describe("resolveTarget", () => {
  test("testnet bundle resolves from gitops-style files", async () => {
    const TestGH = Layer.succeed(GitHubClient, {
      latestStableRelease: () => Effect.succeed("v0.11.0"),
      mainCommits: () => Effect.succeed([]),
      packageTags: () => Effect.succeed(new Set<string>()),
      gitopsFile: (file: string) =>
        Effect.succeed(
          file.includes("gw-sc-deploy-1-init") ? "image:\n  name: ghcr.io/zama-ai/fhevm/gateway-contracts\n  tag: v0.10.0\n"
          : file.includes("eth-sc-deploy") ? "image:\n  name: ghcr.io/zama-ai/fhevm/host-contracts\n  tag: v0.10.0\n"
          : file.includes("coproc-infra-db-mig") ? "image:\n  name: ghcr.io/zama-ai/fhevm/coprocessor/db-migration\n  tag: v0.10.9\n"
          : file.includes("eth-coproc-listener") ? "image:\n  name: ghcr.io/zama-ai/fhevm/coprocessor/host-listener\n  tag: v0.10.10\n"
          : file.includes("gw-coprocessor") ? "gw:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/gw-listener\n    tag: v0.10.10\ntx:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/tx-sender\n    tag: v0.10.10\n"
          : file.includes("coproc-workers") ? "tfheWorker:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker\n    tag: v0.10.10\nzkProofWorker:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/zkproof-worker\n    tag: v0.10.10\nsnsWorker:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/sns-worker\n    tag: v0.10.10\n"
          : file.includes("kms-connector") ? "a:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/db-migration\n    tag: v0.10.8\nb:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/gw-listener\n    tag: v0.10.8\nc:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/kms-worker\n    tag: v0.10.8\nd:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/tx-sender\n    tag: v0.10.8\n"
          : file.includes("kms-core") ? "kmsCore:\n  image:\n    name: ghcr.io/zama-ai/kms/core-service-enclave\n    tag: v0.13.3\n"
          : file.includes("relayer/relayer") ? "image:\n  repository: ghcr.io/zama-ai/console/relayer\n  tag: v0.8.11\njob:\n  image:\n    repository: ghcr.io/zama-ai/console/relayer-migrate\n    tag: v0.8.11\n"
          : "image: ghcr.io/zama-ai/fhevm/test-suite/e2e:v0.10.0\n"
        ),
    });
    const bundle = await Effect.runPromise(resolveTarget("testnet").pipe(Effect.provide(TestGH)));
    expect(bundle.env.CONNECTOR_TX_SENDER_VERSION).toBe("v0.10.8");
    expect(bundle.env.RELAYER_VERSION).toBe("v0.8.11");
    expect(bundle.env.TEST_SUITE_VERSION).toBe("v0.10.0");
  });

  test("version env overrides apply on top of the resolved bundle", async () => {
    const bundle = applyVersionEnvOverrides(
      stubBundle({ lockName: "latest-supported.json", sources: ["profile=latest-supported"] }),
      { GATEWAY_VERSION: "custom-gateway", RELAYER_VERSION: "custom-relayer" },
    );
    expect(bundle.env.GATEWAY_VERSION).toBe("custom-gateway");
    expect(bundle.env.RELAYER_VERSION).toBe("custom-relayer");
    expect(bundle.env.CORE_VERSION).toBe("v0.13.0");
    expect(bundle.sources.at(-1)).toBe("env=GATEWAY_VERSION,RELAYER_VERSION");
  });
});

describe("runtime invariants", () => {
  test("expandBuildOverrides leaves coprocessor to scenarios when a scenario file is present", () => {
    expect(expandBuildOverrides().some((item) => item.group === "coprocessor")).toBe(true);
    expect(expandBuildOverrides("./scenarios/two-of-two.yaml").some((item) => item.group === "coprocessor")).toBe(false);
  });

  test("resolvedComposeEnv preserves version keys", () => {
    const env = resolvedComposeEnv({
      versions: {
        target: "latest-supported",
        lockName: "latest-supported.json",
        sources: [],
        env: {
          GATEWAY_VERSION: "v0.11.0",
          CORE_VERSION: "v0.13.0",
        },
      },
    });
    expect(env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(env.CORE_VERSION).toBe("v0.13.0");
  });

  test("compat policy keeps legacy coprocessor API key flags for versions before v0.12.0", () => {
    const makeState = (version: string) =>
      stubState({ envOverrides: {
        COPROCESSOR_DB_MIGRATION_VERSION: version,
        COPROCESSOR_HOST_LISTENER_VERSION: version,
        COPROCESSOR_GW_LISTENER_VERSION: version,
        COPROCESSOR_TX_SENDER_VERSION: version,
        COPROCESSOR_TFHE_WORKER_VERSION: version,
        COPROCESSOR_ZKPROOF_WORKER_VERSION: version,
        COPROCESSOR_SNS_WORKER_VERSION: version,
      }});

    expect(compatPolicyForState(makeState("v0.11.0")).coprocessorArgs["host-listener"]).toEqual([
      ["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }],
    ] as const);
    expect(compatPolicyForState(makeState("v0.11.0")).coprocessorArgs["sns-worker"]).toEqual([
      ["--tenant-api-key", { env: "TENANT_API_KEY" }],
    ] as const);
    expect(compatPolicyForState(makeState("v0.11.0")).coprocessorArgs["transaction-sender"]).toEqual([
      ["--multichain-acl-address", { env: "MULTICHAIN_ACL_ADDRESS" }],
      ["--delegation-fallback-polling", { value: "30" }],
      ["--delegation-max-retry", { value: "100000" }],
      ["--retry-immediately-on-nonce-error", { value: "2" }],
      ["--host-chain-url", { env: "RPC_WS_URL" }],
    ] as const);

    // v0.12.x: all legacy flags removed
    expect(compatPolicyForState(makeState("v0.12.0")).coprocessorArgs["host-listener"]).toBeUndefined();
    expect(compatPolicyForState(makeState("v0.12.0")).coprocessorArgs["sns-worker"]).toBeUndefined();
    expect(compatPolicyForState(makeState("v0.12.0")).coprocessorArgs["transaction-sender"]).toBeUndefined();

    // SHA targets keep only the destructive gw-listener drop until we can compare exact feature cutovers.
    expect(compatPolicyForState(makeState("58aebb0")).coprocessorArgs["host-listener"]).toBeUndefined();
    expect(compatPolicyForState(makeState("58aebb0")).coprocessorArgs["transaction-sender"]).toBeUndefined();
    expect(compatPolicyForState(makeState("58aebb0")).coprocessorDropFlags["gw-listener"]).toEqual([
      "--ciphertext-commits-address",
      "--gateway-config-address",
    ]);
  });

  test("coprocessor depends_on rewrite only renames cloned services", () => {
    expect(
      rewriteCoprocessorDependsOn(
        {
          "coprocessor-db-migration": { condition: "service_completed_successfully" },
          "coprocessor-and-kms-db": { condition: "service_healthy" },
        },
        "coprocessor1-",
        new Set(["coprocessor-db-migration", "coprocessor-host-listener"]),
      ),
    ).toEqual({
      "coprocessor1-db-migration": { condition: "service_completed_successfully" },
      "coprocessor-and-kms-db": { condition: "service_healthy" },
    });
  });

  test("resolveEnvMap fails on unresolved circular references", () => {
    expect(() => resolveEnvMap({ A: "${B}", B: "${A}" })).toThrow("Unresolved env interpolation");
  });

  test("resolveUpgradePlan rejects inactive overrides and expands multicopro services", () => {
    const inactive = {
      overrides: [{ group: "test-suite" as const }],
      scenario: {
        version: 1 as const,
        kind: "coprocessor-consensus" as const,
        origin: "default" as const,
        topology: { count: 2, threshold: 2 },
        instances: [
          { index: 0, source: { mode: "inherit" as const }, env: {}, args: {} },
          { index: 1, source: { mode: "inherit" as const }, env: {}, args: {} },
        ],
      },
    };
    expect(() => resolveUpgradePlan(inactive, "coprocessor")).toThrow(
      "upgrade requires an active local coprocessor instance",
    );

    const plan = resolveUpgradePlan(
      {
        scenario: {
          version: 1,
          kind: "coprocessor-consensus",
          origin: "override-shorthand",
          topology: { count: 2, threshold: 2 },
          instances: [
            {
              index: 0,
              source: { mode: "local" },
              env: {},
              args: {},
            },
            {
              index: 1,
              source: { mode: "local" },
              env: {},
              args: {},
            },
          ],
        },
        overrides: [{ group: "coprocessor" }],
      },
      "coprocessor",
    );
    expect(plan.component).toBe("coprocessor");
    expect(plan.step).toBe("coprocessor");
    expect(plan.services).not.toContain("coprocessor-db-migration");
    expect(plan.services).not.toContain("coprocessor1-db-migration");
    expect(plan.services).toContain("coprocessor-gw-listener");
    expect(plan.services).toContain("coprocessor1-gw-listener");
    expect(plan.services).toHaveLength(14);

    const filteredPlan = resolveUpgradePlan(
      {
        scenario: {
          version: 1,
          kind: "coprocessor-consensus",
          origin: "override-shorthand",
          topology: { count: 2, threshold: 2 },
          instances: [
            {
              index: 0,
              source: { mode: "local" },
              env: {},
              args: {},
              localServices: [
                "coprocessor-host-listener",
                "coprocessor-host-listener-poller",
              ],
            },
            {
              index: 1,
              source: { mode: "local" },
              env: {},
              args: {},
              localServices: [
                "coprocessor-host-listener",
                "coprocessor-host-listener-poller",
              ],
            },
          ],
        },
        overrides: [{ group: "coprocessor", services: ["coprocessor-host-listener", "coprocessor-host-listener-poller"] }],
      },
      "coprocessor",
    );
    expect(filteredPlan.services).toEqual([
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor1-host-listener",
      "coprocessor1-host-listener-poller",
    ]);

    const connectorPlan = resolveUpgradePlan(
      {
        overrides: [{ group: "kms-connector" }],
        scenario: {
          version: 1 as const,
          kind: "coprocessor-consensus" as const,
          origin: "default" as const,
          topology: { count: 1, threshold: 1 },
          instances: [{ index: 0, source: { mode: "inherit" as const }, env: {}, args: {} }],
        },
      },
      "kms-connector",
    );
    expect(connectorPlan.services).toEqual([
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ]);
  });

  test("full modern workspace protocol overrides disable multichain acl discovery requirements", () => {
    expect(
      requiresMultichainAclAddress(
        stubState({
          overrides: [
            { group: "coprocessor" },
            { group: "gateway-contracts" },
            { group: "host-contracts" },
          ],
        }),
      ),
    ).toBe(false);
  });

  test("workspace-all semantics also apply when coprocessor local services come from a scenario", () => {
    expect(
      requiresMultichainAclAddress(
        stubState({
          overrides: [
            { group: "gateway-contracts" },
            { group: "host-contracts" },
          ],
          count: 2,
          threshold: 2,
        }),
      ),
    ).toBe(true);
    expect(
      requiresMultichainAclAddress({
        ...stubState({
          overrides: [
            { group: "gateway-contracts" },
            { group: "host-contracts" },
            { group: "test-suite" },
          ],
          count: 2,
          threshold: 2,
        }),
        scenario: {
          version: 1,
          kind: "coprocessor-consensus",
          origin: "file",
          topology: { count: 2, threshold: 2 },
          instances: [
            { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
            {
              index: 1,
              source: { mode: "local" },
              env: {},
              args: {},
              localServices: ["coprocessor-host-listener"],
            },
          ],
        },
      }),
    ).toBe(false);
  });

  test("compat keys tx-sender flags off tx-sender version", () => {
    expect(
      compatPolicyForState(
        stubState({
          envOverrides: {
            COPROCESSOR_HOST_LISTENER_VERSION: "v0.12.0",
            COPROCESSOR_TX_SENDER_VERSION: "v0.11.0",
          },
        }),
      ).coprocessorArgs["transaction-sender"],
    ).toBeDefined();
  });

  test("gw-listener healthcheck is only disabled by compat policy", () => {
    const baseService = {
      container_name: "coprocessor-gw-listener",
      healthcheck: { test: ["CMD", "curl"] },
      command: ["gw_listener"],
    };
    const modern = applyInstanceAdjustments(
      "coprocessor-gw-listener",
      baseService,
      "/tmp/coprocessor.env",
      {},
    );
    expect(modern.healthcheck).toEqual({ test: ["CMD", "curl"] });

    const legacy = applyInstanceAdjustments(
      "coprocessor-gw-listener",
      baseService,
      "/tmp/coprocessor.env",
      {},
      { env: {}, args: {} },
      {},
      {},
      { "gw-listener": true },
    );
    expect(legacy.healthcheck).toEqual({ disable: true });
  });

  test("probeBootstrap treats ethCallId failures as retryable", async () => {
    const state = stubState({
      discovery: {
        gateway: { KMS_GENERATION_ADDRESS: "0x1234" },
        host: {},
        kmsSigner: "",
        fheKeyId: "1".padStart(64, "0"),
        crsKeyId: "2".padStart(64, "0"),
        endpoints: {
          gatewayHttp: "http://gateway-node:8546",
          gatewayWs: "",
          hostHttp: "",
          hostWs: "",
          minioInternal: "http://minio:9000",
          minioExternal: "http://minio:9000",
        },
      },
    });
    // MinioClient.probeBootstrap returns null = "not ready yet" (RPC calls failed)
    const TestMinioClient = Layer.succeed(MinioClient, {
      discoverSigner: () => Effect.fail(new MinioError({ message: "not used" })),
      ensureMaterial: () => Effect.fail(new MinioError({ message: "not ready" })),
      probeBootstrap: () => Effect.succeed(null),
    });
    const result = await Effect.runPromise(
      probeBootstrap(state).pipe(
        Effect.catchAll(() => Effect.succeed(false)),
        Effect.provide(TestMinioClient),
      ),
    );
    expect(result).toBe(false);
    expect(state.discovery?.actualFheKeyId).toBeUndefined();
  });

  test("probeBootstrap rethrows ensureMaterialUrl timeout as permanent failure", async () => {
    const state = stubState({
      discovery: {
        gateway: { KMS_GENERATION_ADDRESS: "0x1234" },
        host: {},
        kmsSigner: "",
        fheKeyId: predictedKeyId(),
        crsKeyId: predictedCrsId(),
        endpoints: {
          gatewayHttp: "http://localhost:8546",
          gatewayWs: "",
          hostHttp: "http://localhost:8545",
          hostWs: "",
          minioInternal: "http://minio:9000",
          minioExternal: "http://localhost:9000",
        },
      },
    });
    // probeBootstrap returns key IDs (RPC calls succeed), but ensureMaterial fails (HEAD 404)
    const TestMinioClient = Layer.succeed(MinioClient, {
      discoverSigner: () => Effect.fail(new MinioError({ message: "not used" })),
      ensureMaterial: () => Effect.fail(new MinioError({ message: "Material not ready" })),
      probeBootstrap: () => Effect.succeed({ actualFheKeyId: predictedKeyId(), actualCrsKeyId: predictedCrsId() }),
    });
    await expect(
      Effect.runPromise(
        probeBootstrap(state).pipe(Effect.provide(TestMinioClient)),
      ),
    ).rejects.toThrow("Material not ready");
  }, 45_000);

  test("composeDown returns false on non-zero exit", async () => {
    const file = composePath("database");
    await fs.mkdir(path.dirname(file), { recursive: true });
    await fs.writeFile(file, "services: {}\n");
    try {
      const TestCmd = Layer.succeed(CommandRunner, {
        run: () => Effect.succeed({ stdout: "", stderr: "", code: 0 }),
        runLive: () => Effect.succeed(1), // non-zero exit
      });
      const TestRunner = ContainerRunner.Live.pipe(Layer.provide(TestCmd));
      const result = await Effect.runPromise(
        Effect.gen(function* () {
          const runner = yield* ContainerRunner;
          return yield* runner.composeDown("database");
        }).pipe(Effect.provide(TestRunner)),
      );
      expect(result).toBe(false);
    } finally {
      await fs.rm(file, { force: true });
    }
  });

  test("resolveServiceOverrides expands shared-image runtime siblings", () => {
    expect(resolveServiceOverrides("coprocessor", ["host-listener"])).toEqual([
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
    ]);
  });

  test("predicted bootstrap ids are deterministic", () => {
    expect(predictedKeyId()).toBe("0400000000000000000000000000000000000000000000000000000000000001");
    expect(predictedCrsId()).toBe("0500000000000000000000000000000000000000000000000000000000000001");
  });

  test("up rejects unknown step before doing work", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(["bun", "src/cli.ts", "up", "--from-step", "nope"], noopLayer);
    expect(await maybeRead(STATE_FILE)).toBe(before);
  });

  test("up rejects --from-step without --resume outside dry-run", async () => {
    const dir = await fixtureDir();
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(
      ["bun", "src/cli.ts", "up", "--from-step", "relayer"],
      noopLayer,
    );
    expect(process.exitCode).toBe(1);
    process.exitCode = 0;
    expect(await maybeRead(STATE_FILE)).toBe(before);
    void dir;
  });

  test("validation errors do not emit a stale resume hint even when state exists", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.writeFile(
      STATE_FILE,
      JSON.stringify(
        stubState({
          completedSteps: ["resolve", "generate"],
        }),
      ),
    );
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--from-step", "relayer"],
        noopLayer,
      );
    } finally {
      restore();
    }
    expect(logs.some((line) => line.includes("--from-step requires --resume or --dry-run"))).toBe(true);
    expect(logs.some((line) => line.includes("Hint: run with --resume"))).toBe(false);
    if (before) {
      await fs.writeFile(STATE_FILE, before);
    } else {
      await fs.rm(STATE_FILE, { force: true });
    }
  });

  test("up rejects per-service overrides for non-runtime groups before doing work", async () => {
    const dir = await fixtureDir();
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(
      ["bun", "src/cli.ts", "up", "--override", "gateway-contracts:sc-deploy"],
      noopLayer,
    );
    expect(process.exitCode).toBe(1);
    process.exitCode = 0;
    expect(await maybeRead(STATE_FILE)).toBe(before);
    void dir;
  });

  test("up --dry-run rejects latest-supported partial overrides when local migrations diverge", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-supported.json");
    await fs.writeFile(lockFile, JSON.stringify(stubBundle()));
    process.chdir(REPO_ROOT);
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        [
          "bun",
          "src/cli.ts",
          "up",
          "--target",
          "latest-supported",
          "--lock-file",
          lockFile,
          "--override",
          "coprocessor:host-listener",
          "--dry-run",
        ],
        depsToLayer({
          runner: fakeRunner({
            "git rev-parse -q --verify v0.11.0^{commit}": "",
            "git ls-files --others --exclude-standard -- coprocessor/fhevm-engine/db-migration/migrations": "",
            "git diff --quiet --exit-code v0.11.0 -- coprocessor/fhevm-engine/db-migration/migrations": {
              stdout: "",
              stderr: "",
              code: 1,
            },
          }),
        }),
      );
    } finally {
      restore();
    }
    expect(logs.some((l) => l.includes("coprocessor: local DB migrations diverge from v0.11.0"))).toBe(true);
  });

  test("up --dry-run rejects latest-main partial overrides when local migrations diverge", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-main.json");
    await fs.writeFile(
      lockFile,
      JSON.stringify({
        ...stubBundle({ lockName: "latest-main.json", env: { COPROCESSOR_DB_MIGRATION_VERSION: "803f104" }, sources: ["test"] }),
        target: "latest-main",
      }),
    );
    process.chdir(REPO_ROOT);
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        [
          "bun",
          "src/cli.ts",
          "up",
          "--target",
          "latest-main",
          "--lock-file",
          lockFile,
          "--override",
          "coprocessor:host-listener",
          "--dry-run",
        ],
        depsToLayer({
          runner: fakeRunner({
            "git rev-parse -q --verify 803f104^{commit}": "",
            "git ls-files --others --exclude-standard -- coprocessor/fhevm-engine/db-migration/migrations": "",
            "git diff --quiet --exit-code 803f104 -- coprocessor/fhevm-engine/db-migration/migrations": {
              stdout: "",
              stderr: "",
              code: 1,
            },
          }),
        }),
      );
    } finally {
      restore();
    }
    expect(logs.some((l) => l.includes("coprocessor: local DB migrations diverge from 803f104"))).toBe(true);
  });

  test("up --dry-run rejects kms-connector partial overrides when local migrations diverge", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-supported.json");
    await fs.writeFile(lockFile, JSON.stringify(stubBundle()));
    process.chdir(REPO_ROOT);
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        [
          "bun",
          "src/cli.ts",
          "up",
          "--target",
          "latest-supported",
          "--lock-file",
          lockFile,
          "--override",
          "kms-connector:gw-listener",
          "--dry-run",
        ],
        depsToLayer({
          runner: fakeRunner({
            "git rev-parse -q --verify v0.11.0^{commit}": "",
            "git ls-files --others --exclude-standard -- kms-connector/connector-db/migrations": "",
            "git diff --quiet --exit-code v0.11.0 -- kms-connector/connector-db/migrations": {
              stdout: "",
              stderr: "",
              code: 1,
            },
          }),
        }),
      );
    } finally {
      restore();
    }
    expect(logs.some((l) => l.includes("kms-connector: local DB migrations diverge from v0.11.0"))).toBe(true);
  });

  test("up --dry-run rejects latest-supported partial overrides with untracked local migrations", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-supported.json");
    await fs.writeFile(lockFile, JSON.stringify(stubBundle()));
    process.chdir(REPO_ROOT);
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        [
          "bun",
          "src/cli.ts",
          "up",
          "--target",
          "latest-supported",
          "--lock-file",
          lockFile,
          "--override",
          "coprocessor:host-listener",
          "--dry-run",
        ],
        depsToLayer({
          runner: fakeRunner({
            "git rev-parse -q --verify v0.11.0^{commit}": "",
            "git ls-files --others --exclude-standard -- coprocessor/fhevm-engine/db-migration/migrations":
              "coprocessor/fhevm-engine/db-migration/migrations/20260310000000_new.sql\n",
          }),
        }),
      );
    } finally {
      restore();
    }
    expect(logs.some((l) => l.includes("coprocessor: local DB migrations diverge from v0.11.0"))).toBe(true);
  });

  test("up --dry-run allows divergent partial overrides with --allow-schema-mismatch", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-supported.json");
    await fs.writeFile(lockFile, JSON.stringify(stubBundle()));
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(
      [
        "bun",
        "src/cli.ts",
        "up",
        "--target",
        "latest-supported",
        "--lock-file",
        lockFile,
        "--override",
        "coprocessor:host-listener",
        "--allow-schema-mismatch",
        "--dry-run",
      ],
      depsToLayer({
        runner: fakeRunner({
          "which bun": "",
          "which docker": "",
          "which cast": "",
          "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": "",
          ...portCheckResponses,
        }),
      }),
    );
    expect(process.exitCode).toBe(0);
    expect(await maybeRead(STATE_FILE)).toBe(before);
  });

  test("up --dry-run resolves without creating runtime state", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    const runner = fakeRunner({
      "gh api repos/zama-ai/fhevm/releases?per_page=100&page=1": JSON.stringify([
        { tag_name: "v0.11.0", draft: false, prerelease: false },
      ]),
      "which bun": "",
      "which docker": "",
      "which gh": "",
      "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": "",
      ...portCheckResponses,
    });
    await main(
      ["bun", "src/cli.ts", "up", "--target", "latest-supported", "--dry-run", "--from-step", "relayer"],
      depsToLayer({ runner }),
    );
    expect(await maybeRead(STATE_FILE)).toBe(before);
  });

  test("deploy --dry-run aliases up without creating runtime state", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    const runner = fakeRunner({
      "gh api repos/zama-ai/fhevm/releases?per_page=100&page=1": JSON.stringify([
        { tag_name: "v0.11.0", draft: false, prerelease: false },
      ]),
      "which bun": "",
      "which docker": "",
      "which gh": "",
      "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": "",
      ...portCheckResponses,
    });
    await main(
      ["bun", "src/cli.ts", "deploy", "--target", "latest-supported", "--dry-run"],
      depsToLayer({ runner }),
    );
    expect(await maybeRead(STATE_FILE)).toBe(before);
  });

  test("up --dry-run can use a lock file without GitHub resolution", async () => {
    const dir = await fixtureDir();
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    const lockFile = path.join(dir, "ci-bundle.json");
    await fs.writeFile(lockFile, JSON.stringify(stubBundle({ lockName: "ci-workflow.json" })));
    const runner = fakeRunner({
      "which bun": "",
      "which docker": "",
      "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": "",
      ...portCheckResponses,
    });
    await main(
      ["bun", "src/cli.ts", "up", "--target", "latest-supported", "--lock-file", lockFile, "--dry-run"],
      depsToLayer({ runner }),
    );
    expect(await maybeRead(STATE_FILE)).toBe(before);
  });

  test("resume from relayer restores generated runtime artifacts from state", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(
      STATE_FILE,
      JSON.stringify(
        stubState({
          discovery: readyDiscovery(),
          completedSteps: [
            "preflight",
            "resolve",
            "generate",
            "base",
            "kms-signer",
            "gateway-deploy",
            "host-deploy",
            "discover",
            "regenerate",
            "validate",
            "coprocessor",
            "kms-connector",
            "bootstrap",
          ],
        }),
      ),
    );
    await fs.mkdir(path.dirname(versionsEnvPath), { recursive: true });
    await fs.writeFile(versionsEnvPath, "GATEWAY_VERSION=v0.11.0\n");
    await fs.mkdir(path.join(STATE_DIR, "compose"), { recursive: true });
    await Promise.all(
      COMPONENTS.map((component) => fs.writeFile(composePath(component), "services:\n")),
    );
    const runner = fakeRunner({
      "docker inspect fhevm-relayer-db": JSON.stringify([
        { State: { Status: "running", ExitCode: 0, Health: { Status: "healthy" } }, NetworkSettings: { Networks: { default: { IPAddress: "127.0.0.1" } } } },
      ]),
      "docker inspect fhevm-relayer": JSON.stringify([
        { State: { Status: "running", ExitCode: 0 }, NetworkSettings: { Networks: { default: { IPAddress: "127.0.0.1" } } } },
      ]),
      "docker inspect fhevm-test-suite-e2e-debug": JSON.stringify([
        { State: { Status: "running", ExitCode: 0 }, NetworkSettings: { Networks: { default: { IPAddress: "127.0.0.1" } } } },
      ]),
      "docker logs fhevm-relayer": "All servers are ready and responding",
    });
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-supported", "--resume", "--from-step", "relayer"],
        depsToLayer({ runner, liveRunner: async () => 0 }),
      );
      expect(await maybeRead(envPath("gateway-sc"))).toBeDefined();
      expect(await maybeRead(relayerConfigPath)).toContain("gateway:");
      expect(await maybeRead(gatewayAddressesSolidityPath)).toContain("gatewayConfigAddress");
    } finally {
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
  });

  test("resume rejects new overrides and topology flags", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(STATE_FILE, JSON.stringify(stubState({ completedSteps: ["base"] })));
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        [
          "bun",
          "src/cli.ts",
          "up",
          "--target",
          "latest-supported",
          "--resume",
          "--override",
          "coprocessor",
        ],
        noopLayer,
      );
    } finally {
      restore();
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
    expect(logs.some((l) => l.includes("--resume uses the persisted stack configuration"))).toBe(true);
  });

  test("resume rejects --reset", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(STATE_FILE, JSON.stringify(stubState({ completedSteps: ["base"] })));
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-supported", "--resume", "--reset"],
        noopLayer,
      );
    } finally {
      restore();
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
    expect(logs.some((l) => l.includes("--reset"))).toBe(true);
  });

  test("resume logs when there is nothing left to do", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(STATE_FILE, JSON.stringify(stubState({ completedSteps: [...STEP_NAMES] })));
    const { logs, restore } = captureConsole("log");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-supported", "--resume"],
        depsToLayer({
          runner: fakeRunner({
            "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Names}}": "gateway-node\n",
          }),
        }),
      );
    } finally {
      restore();
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
    expect(logs.some((l) => l.includes("[resume] nothing to do"))).toBe(true);
  });

  test("resume rejects stale persisted state when no containers are running", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(STATE_FILE, JSON.stringify(stubState({ completedSteps: [...STEP_NAMES] })));
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-supported", "--resume"],
        depsToLayer({
          runner: fakeRunner({
            "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Names}}": "",
          }),
        }),
      );
    } finally {
      restore();
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
    expect(logs.some((l) => l.includes("Persisted state exists but no fhevm containers are running"))).toBe(true);
  });

  test("resume dry-run previews the persisted stack", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(
      STATE_FILE,
      JSON.stringify({
        ...stubState({ count: 2, threshold: 2, completedSteps: ["preflight", "resolve", "generate", "base"] }),
        requiresGitHub: false,
      }),
    );
    const { logs, restore } = captureConsole("log");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-supported", "--resume", "--dry-run"],
        depsToLayer({
          runner: fakeRunner({
            "which bun": "",
            "which docker": "",
            "which cast": "",
            "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": "",
            ...portCheckResponses,
          }),
        }),
      );
    } finally {
      restore();
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
    expect(logs.some((line) => line.includes("[plan] topology=n2/t2"))).toBe(true);
    expect(logs.some((line) => line.includes("resume preview uses persisted state only"))).toBe(true);
  });

  test("down restores generated runtime artifacts from state before teardown", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(
      STATE_FILE,
      JSON.stringify(stubState({ discovery: readyDiscovery(), completedSteps: ["bootstrap"] })),
    );
    try {
      await main(["bun", "src/cli.ts", "down"], depsToLayer({ liveRunner: async () => 0 }));
      expect(await maybeRead(composePath("coprocessor"))).toContain("services:");
      expect(await maybeRead(path.join(STATE_DIR, "env", "versions.env"))).toContain("GATEWAY_VERSION=");
      expect(await maybeRead(STATE_FILE)).toBeUndefined();
    } finally {
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
  });

  test("clean keeps runtime state when teardown fails", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    const state = stubState({ discovery: readyDiscovery(), completedSteps: ["bootstrap"] });
    await fs.writeFile(STATE_FILE, JSON.stringify(state));
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "clean"], depsToLayer({ liveRunner: async () => 1 }));
    } finally {
      restore();
    }
    expect(await maybeRead(STATE_FILE)).toBeDefined();
    expect(logs.some((l) => l.includes("Failed to stop components"))).toBe(true);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    if (before !== undefined) {
      await fs.mkdir(STATE_DIR, { recursive: true });
      await fs.writeFile(STATE_FILE, before);
    }
  });

  test("up --dry-run reports a helpful message when gh is missing", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-main", "--dry-run"],
        depsToLayer({
          runner: async (argv) => {
            const key = argv.join(" ");
            if (key.startsWith("gh api ")) {
              throw new Error("spawn gh ENOENT");
            }
            return { stdout: "", stderr: "", code: 0 };
          },
        }),
      );
    } finally { restore(); }
    expect(
      logs.some((l) =>
        l.includes("GitHub CLI `gh` is required"),
      ),
    ).toBe(true);
  });

  test("up --dry-run reports authentication guidance for gh api failures", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-main", "--dry-run"],
        depsToLayer({
          runner: async (argv, options) => {
            const key = argv.join(" ");
            if (key === "which bun" || key === "which docker" || key === "which gh") {
              return { stdout: "", stderr: "", code: 0 };
            }
            if (key === "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}") {
              return { stdout: "", stderr: "", code: 0 };
            }
            if (key.startsWith("lsof -nP -iTCP:")) {
              return { stdout: "", stderr: "", code: 1 };
            }
            if (key.startsWith("gh api ")) {
              throw new Error(`${key} failed (1)\nHTTP 401: authentication required`);
            }
            return noopDeps.runner(argv, options);
          },
        }),
      );
    } finally { restore(); }
    expect(
      logs.some((l) =>
        l.includes("GitHub API not authenticated"),
      ),
    ).toBe(true);
  });

  test("up --dry-run reports rate limiting guidance for gh api failures", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-main", "--dry-run"],
        depsToLayer({
          runner: async (argv, options) => {
            const key = argv.join(" ");
            if (key === "which bun" || key === "which docker" || key === "which gh") {
              return { stdout: "", stderr: "", code: 0 };
            }
            if (key === "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}") {
              return { stdout: "", stderr: "", code: 0 };
            }
            if (key.startsWith("lsof -nP -iTCP:")) {
              return { stdout: "", stderr: "", code: 1 };
            }
            if (key.startsWith("gh api ")) {
              throw new Error(`${key} failed (1)\nAPI rate limit exceeded`);
            }
            return noopDeps.runner(argv, options);
          },
        }),
      );
    } finally { restore(); }
    expect(
      logs.some((l) =>
        l.includes("GitHub API rate limit hit"),
      ),
    ).toBe(true);
  });
});

describe("CLI argument validation", () => {
  test("rejects unsupported target", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "bogus"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Unsupported target"))).toBe(true);
  });

  test("rejects removed --coprocessors flag", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--coprocessors", "2"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Received unknown argument: '--coprocessors'"))).toBe(true);
  });

  test("rejects removed --threshold flag", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--threshold", "2"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Received unknown argument: '--threshold'"))).toBe(true);
  });

  test("rejects --scenario with coprocessor overrides", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        [
          "bun",
          "src/cli.ts",
          "up",
          "--scenario",
          "test.yml",
          "--override",
          "coprocessor",
        ],
        noopLayer,
      );
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--scenario cannot be combined with --override coprocessor"))).toBe(true);
  });

  test("rejects --build with --override", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--build", "--override", "gateway-contracts"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--build cannot be combined with --override"))).toBe(true);
  });

  test("rejects --target sha without --sha", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "sha", "--dry-run"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--target sha requires --sha"))).toBe(true);
  });

  test("rejects --sha without --target sha", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "latest-supported", "--sha", "1234abc", "--dry-run"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--sha requires --target sha"))).toBe(true);
  });

  test("rejects --sha with --lock-file", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main([
        "bun",
        "src/cli.ts",
        "up",
        "--target",
        "sha",
        "--sha",
        "1234abc",
        "--lock-file",
        "/tmp/fake.json",
        "--dry-run",
      ], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--sha cannot be used with --lock-file"))).toBe(true);
  });

  test("rejects invalid sha format", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "sha", "--sha", "notasha", "--dry-run"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Invalid sha notasha; expected 7 or 40 hex characters"))).toBe(true);
  });

  test("rejects unknown per-service override suffix", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--override", "coprocessor:local"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes('Unknown service "local" in group "coprocessor"'))).toBe(true);
  });

  test("rejects unsupported override group", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--override", "nonexistent"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Unsupported override"))).toBe(true);
  });

  test("rejects unknown command", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "bogus"], noopLayer);
    } finally { restore(); }
    expect(process.exitCode).toBe(1);
    expect(logs.some((l) => l.includes("Invalid subcommand"))).toBe(true);
  });

  test("doctor shows removal message", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "doctor"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("doctor") && l.includes("removed"))).toBe(true);
  });
});

describe("command error paths", () => {
  test("test includes hcu-block-cap profile", () => {
    expect(TEST_GREP["hcu-block-cap"]).toBe("block cap scenarios");
  });

  test("pause rejects missing scope", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "pause"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Missing argument <scope>"))).toBe(true);
  });

  test("unpause rejects missing scope", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "unpause"], noopLayer);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Missing argument <scope>"))).toBe(true);
  });

  test("test requires completed bootstrap", async () => {
    const stateDir = path.join(REPO_ROOT, ".fhevm");
    const stateFile = path.join(stateDir, "state.json");
    const hadState = await maybeRead(stateFile);
    const { logs, restore } = captureConsole("error");
    try {
      await fs.rm(stateDir, { recursive: true, force: true });
      await main(["bun", "src/cli.ts", "test", "input-proof"], noopLayer);
    } finally {
      restore();
      if (hadState === undefined) {
        await fs.rm(stateDir, { recursive: true, force: true });
      } else {
        await fs.mkdir(stateDir, { recursive: true });
        await fs.writeFile(stateFile, hadState);
      }
    }
    expect(logs.some((l) => l.includes("bootstrap") || l.includes("fhevm-cli up"))).toBe(true);
  });

  test("test rejects unknown profile", async () => {
    const stateDir = path.join(REPO_ROOT, ".fhevm");
    const stateFile = path.join(stateDir, "state.json");
    const hadState = await maybeRead(stateFile);
    try {
      await fs.mkdir(stateDir, { recursive: true });
      const state = stubState({
        discovery: {
          gateway: {}, host: {}, kmsSigner: "", fheKeyId: "", crsKeyId: "",
          actualFheKeyId: "abc",
          endpoints: { gatewayHttp: "", gatewayWs: "", hostHttp: "", hostWs: "", minioInternal: "", minioExternal: "" },
        },
        completedSteps: ["bootstrap"],
      });
      await fs.writeFile(stateFile, JSON.stringify(state));
      const { logs, restore } = captureConsole("error");
      try {
        await main(["bun", "src/cli.ts", "test", "nonexistent-profile"], noopLayer);
      } finally { restore(); }
      expect(logs.some((l) => l.includes("Unknown test profile"))).toBe(true);
    } finally {
      if (hadState) {
        await fs.writeFile(stateFile, hadState);
      } else {
        await fs.rm(stateFile, { force: true });
      }
    }
  });

  test("help prints usage without error", async () => {
    const { logs, restore } = captureConsole("log");
    try {
      await main(["bun", "src/cli.ts", "--help"], noopLayer);
    } finally { restore(); }
    expect(process.exitCode).toBe(0);
    expect(logs.some((l) => l.includes("fhevm-cli"))).toBe(true);
    expect(logs.some((l) => l.includes("doctor"))).toBe(false);
  });

  test("subcommand help includes domain-specific descriptions", async () => {
    const { logs, restore } = captureConsole("log");
    try {
      await main(["bun", "src/cli.ts", "up", "--help"], noopLayer);
    } finally { restore(); }
    expect(logs.some((line) => line.includes("Path to a coprocessor consensus scenario file"))).toBe(true);
    expect(logs.some((line) => line.includes("Boot the fhevm stack from a target, lock file, or persisted state"))).toBe(true);
  });

  test("no command prints usage", async () => {
    const { logs, restore } = captureConsole("log");
    try {
      await main(["bun", "src/cli.ts"], noopLayer);
    } finally { restore(); }
    expect(process.exitCode).toBe(0);
    expect(logs.some((l) => l.includes("fhevm-cli"))).toBe(true);
  });

  test("down runs without error", async () => {
    const stateFile = STATE_FILE;
    const hadState = await maybeRead(stateFile);
    try {
      // Remove ambient state so down takes the no-state path
      await fs.rm(stateFile, { force: true });
      const { logs, restore } = captureConsole("log");
      const { logs: errLogs, restore: restoreErr } = captureConsole("error");
      try {
        await main(["bun", "src/cli.ts", "down"], noopLayer);
      } finally {
        restore();
        restoreErr();
      }
      expect(errLogs.length).toBe(0);
      expect(logs.some((l) => l.includes("nothing to stop") || l.includes("[down]"))).toBe(true);
    } finally {
      if (hadState) {
        await fs.writeFile(stateFile, hadState);
      } else {
        await fs.rm(stateFile, { force: true });
      }
    }
  });

  test("status with no state shows containers", async () => {
    const runner = fakeRunner({
      "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Names}}\t{{.Status}}": "",
    });
    const { logs, restore } = captureConsole("log");
    try {
      await main(["bun", "src/cli.ts", "status"], depsToLayer({ runner }));
    } finally { restore(); }
    expect(logs.some((l) => l.includes("No fhevm containers"))).toBe(true);
  });

  test("status warns when persisted state is stale", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(STATE_FILE, JSON.stringify(stubState({ completedSteps: ["base"] })));
    const runner = fakeRunner({
      "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Names}}\t{{.Status}}": "",
    });
    const { logs, restore } = captureConsole("log");
    try {
      await main(["bun", "src/cli.ts", "status"], depsToLayer({ runner }));
    } finally {
      restore();
      await fs.rm(STATE_DIR, { recursive: true, force: true });
      if (before !== undefined) {
        await fs.mkdir(STATE_DIR, { recursive: true });
        await fs.writeFile(STATE_FILE, before);
      }
    }
    expect(logs.some((l) => l.includes("persisted state exists but no fhevm containers are running"))).toBe(true);
  });
});

describe("compat policy edge cases", () => {
  const stateWith = (coprocessorVersion: string, connectorVersion: string) =>
    stubState({
      envOverrides: {
        COPROCESSOR_DB_MIGRATION_VERSION: coprocessorVersion,
        COPROCESSOR_HOST_LISTENER_VERSION: coprocessorVersion,
        COPROCESSOR_GW_LISTENER_VERSION: coprocessorVersion,
        COPROCESSOR_TX_SENDER_VERSION: coprocessorVersion,
        COPROCESSOR_TFHE_WORKER_VERSION: coprocessorVersion,
        COPROCESSOR_ZKPROOF_WORKER_VERSION: coprocessorVersion,
        COPROCESSOR_SNS_WORKER_VERSION: coprocessorVersion,
        CONNECTOR_DB_MIGRATION_VERSION: connectorVersion,
        CONNECTOR_GW_LISTENER_VERSION: connectorVersion,
        CONNECTOR_KMS_WORKER_VERSION: connectorVersion,
        CONNECTOR_TX_SENDER_VERSION: connectorVersion,
      },
    });

  test("legacy connector chain ID mapping for versions before v0.11.0", () => {
    const policy = compatPolicyForState(stateWith("v0.12.0", "v0.10.5"));
    expect(policy.connectorEnv).toEqual({ KMS_CONNECTOR_CHAIN_ID: "KMS_CONNECTOR_GATEWAY_CHAIN_ID" });
  });

  test("no connector compat for v0.11.0+", () => {
    const policy = compatPolicyForState(stateWith("v0.12.0", "v0.11.0"));
    expect(policy.connectorEnv).toEqual({});
  });

  test("both compat policies active for old versions", () => {
    const policy = compatPolicyForState(stateWith("v0.10.0", "v0.10.0"));
    expect(policy.coprocessorArgs["host-listener"]).toBeDefined();
    expect(policy.connectorEnv.KMS_CONNECTOR_CHAIN_ID).toBe("KMS_CONNECTOR_GATEWAY_CHAIN_ID");
  });

  test("host-listener-poller gets legacy api key too", () => {
    const policy = compatPolicyForState(stateWith("v0.11.0", "v0.11.0"));
    expect(policy.coprocessorArgs["host-listener-poller"]).toEqual([
      ["--coprocessor-api-key", { env: "COPROCESSOR_API_KEY" }],
    ] as const);
  });
});

describe("version resolution edge cases", () => {
  test("env overrides with empty values are ignored", () => {
    const bundle = applyVersionEnvOverrides(
      stubBundle(),
      { GATEWAY_VERSION: "", HOST_VERSION: undefined as unknown as string },
    );
    expect(bundle.env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(bundle.env.HOST_VERSION).toBe("v0.11.0");
    expect(bundle.sources.length).toBe(1);
  });

  test("no overrides returns original bundle identity", () => {
    const original = stubBundle();
    const result = applyVersionEnvOverrides(original, {});
    expect(result).toBe(original);
  });
});

describe("validateBundleCompatibility", () => {
  const stateWithVersions = (relayer: string, testSuite: string) =>
    stubState({ envOverrides: { RELAYER_VERSION: relayer, TEST_SUITE_VERSION: testSuite } });

  test("detects relayer v1 vs test-suite v2 mismatch", () => {
    const issues = validateBundleCompatibility(stateWithVersions("v0.9.0", "v0.11.0"));
    expect(issues).toHaveLength(1);
    expect(issues[0].code).toBe("relayer-v1-vs-test-suite-v2");
  });

  test("modern relayer is OK", () => {
    expect(validateBundleCompatibility(stateWithVersions("v0.10.0", "v0.11.0"))).toEqual([]);
  });

  test("legacy test-suite is OK", () => {
    expect(validateBundleCompatibility(stateWithVersions("v0.9.0", "v0.10.0"))).toEqual([]);
  });

  test("both modern is OK", () => {
    expect(validateBundleCompatibility(stateWithVersions("v0.10.0", "v0.12.0"))).toEqual([]);
  });

  test("SHA relayer treated as modern", () => {
    expect(validateBundleCompatibility(stateWithVersions("abc1234", "v0.11.0"))).toEqual([]);
  });

  test("SHA test-suite treated as modern triggers mismatch", () => {
    const issues = validateBundleCompatibility(stateWithVersions("v0.9.0", "abc1234"));
    expect(issues).toHaveLength(1);
    expect(issues[0].code).toBe("relayer-v1-vs-test-suite-v2");
  });

  test("empty versions treated as modern", () => {
    expect(validateBundleCompatibility(stateWithVersions("", ""))).toEqual([]);
  });

  test("boundary v0.10.0 relayer is OK", () => {
    expect(validateBundleCompatibility(stateWithVersions("v0.10.0", "v0.11.0"))).toEqual([]);
  });
});

describe("COMPAT_MATRIX", () => {
  test("externalDefaults has expected keys", () => {
    expect(COMPAT_MATRIX.externalDefaults).toHaveProperty("RELAYER_VERSION");
    expect(COMPAT_MATRIX.externalDefaults).toHaveProperty("RELAYER_MIGRATE_VERSION");
  });

  test("anchors has valid SIMPLE_ACL_MIN_SHA", () => {
    expect(COMPAT_MATRIX.anchors).toHaveProperty("SIMPLE_ACL_MIN_SHA");
    expect(COMPAT_MATRIX.anchors.SIMPLE_ACL_MIN_SHA).toMatch(/^[0-9a-f]{40}$/);
  });

  test("compat-defaults output shape matches expected structure", () => {
    const output = {
      externalDefaults: COMPAT_MATRIX.externalDefaults,
      anchors: COMPAT_MATRIX.anchors,
    };
    expect(output.externalDefaults.RELAYER_VERSION).toBe("sha-29b0750");
    expect(output.externalDefaults.RELAYER_MIGRATE_VERSION).toBe("sha-29b0750");
    expect(output.anchors.SIMPLE_ACL_MIN_SHA).toBe("803f1048727eabf6d8b3df618203e3c7dda77890");
  });
});

describe("resolveWorkflowCompatEnv", () => {
  test("promotes the stack to modern when any selected ref crosses the cutover", async () => {
    const resolved = await Effect.runPromise(
      resolveWorkflowCompatEnv({
        versions: ["v0.11.0", "sha-29b0750"],
        forceModernRelayer: false,
        isModernRef: (ref) => Effect.succeed(ref === "sha-29b0750"),
      }),
    );
    expect(resolved).toEqual({
      STACK_ERA: "modern",
      RELAYER_VERSION: "sha-29b0750",
      RELAYER_MIGRATE_VERSION: "sha-29b0750",
    });
  });

  test("preserves explicit relayer pins while still backfilling migrate when needed", async () => {
    const resolved = await Effect.runPromise(
      resolveWorkflowCompatEnv({
        versions: [],
        forceModernRelayer: true,
        relayerVersion: "sha-29b0750",
        isModernRef: () => Effect.succeed(false),
      }),
    );
    expect(resolved).toEqual({
      STACK_ERA: "modern",
      RELAYER_MIGRATE_VERSION: "sha-29b0750",
    });
  });
});

describe("compat-resolve-env command", () => {
  test("emits workflow env assignments from git ancestry", async () => {
    const { logs, restore } = captureConsole("log");
    try {
      await main(
        ["bun", "src/cli.ts", "compat-resolve-env", "803f104", "v0.11.0"],
        depsToLayer({
          runner: fakeRunner({
            "git rev-parse -q --verify 803f104^{commit}": "",
            [`git merge-base --is-ancestor ${COMPAT_MATRIX.anchors.SIMPLE_ACL_MIN_SHA} 803f104`]: "",
            "git rev-parse -q --verify v0.11.0^{commit}": { stdout: "", stderr: "", code: 1 },
          }),
        }),
      );
    } finally {
      restore();
    }
    expect(logs).toContain("STACK_ERA=modern\nRELAYER_VERSION=sha-29b0750\nRELAYER_MIGRATE_VERSION=sha-29b0750");
  });
});

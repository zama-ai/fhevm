import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, describe, expect, test } from "bun:test";

import {
  composeDown,
  resolveEnvMap,
  resolvedComposeEnv,
  rewriteCoprocessorDependsOn,
} from "./artifacts";
import { REPO_ROOT, STATE_DIR, TEST_GREP, composePath, resolveServiceOverrides } from "./layout";
import { STEP_NAMES } from "./types";
import { main, overrideWarnings, probeBootstrap, resolveUpgradePlan } from "./runtime";
import { compatPolicyForState, requiresMultichainAclAddress } from "./compat";
import { predictedCrsId, predictedKeyId } from "./utils";
import { applyVersionEnvOverrides, createGitHubClient, resolveTarget } from "./versions";
import {
  captureConsole,
  fakeRunner,
  noopDeps,
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

const LATEST_MAIN_PACKAGES = [
  "fhevm%2Fgateway-contracts",
  "fhevm%2Fhost-contracts",
  "fhevm%2Fcoprocessor%2Fdb-migration",
  "fhevm%2Fcoprocessor%2Fhost-listener",
  "fhevm%2Fcoprocessor%2Fgw-listener",
  "fhevm%2Fcoprocessor%2Ftx-sender",
  "fhevm%2Fcoprocessor%2Ftfhe-worker",
  "fhevm%2Fcoprocessor%2Fzkproof-worker",
  "fhevm%2Fcoprocessor%2Fsns-worker",
  "fhevm%2Fkms-connector%2Fdb-migration",
  "fhevm%2Fkms-connector%2Fgw-listener",
  "fhevm%2Fkms-connector%2Fkms-worker",
  "fhevm%2Fkms-connector%2Ftx-sender",
  "fhevm%2Ftest-suite%2Fe2e",
] as const;

const latestMainPackageResponses = (tag: string) =>
  Object.fromEntries(
    LATEST_MAIN_PACKAGES.map((pkg) => [
      `gh api /orgs/zama-ai/packages/container/${pkg}/versions?per_page=100&page=1`,
      JSON.stringify([{ metadata: { container: { tags: [tag] } } }]),
    ]),
  );

describe("resolveTarget", () => {
  test("latest-main walks back to the first complete sha bundle after the simple-acl floor", async () => {
    const gh = createGitHubClient(
      fakeRunner({
        "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=1":
          JSON.stringify([
            { sha: "1111111000000000000000000000000000000000" },
            { sha: "803f1048727eabf6d8b3df618203e3c7dda77890" },
            { sha: "2222222000000000000000000000000000000000" },
          ]),
        ...latestMainPackageResponses("1111111"),
      }),
    );
    const bundle = await resolveTarget("latest-main", gh);
    expect(bundle.lockName).toBe("latest-main-1111111.json");
    expect(bundle.env.GATEWAY_VERSION).toBe("1111111");
    expect(bundle.env.CORE_VERSION).toBe("v0.13.0");
    expect(bundle.env.RELAYER_VERSION).toBe("sha-29b0750");
    expect(bundle.env.RELAYER_MIGRATE_VERSION).toBe("sha-29b0750");
  });

  test("latest-main rejects complete bundles older than the simple-acl floor", async () => {
    const gh = createGitHubClient(
      fakeRunner({
        "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=1":
          JSON.stringify([
            { sha: "1111111000000000000000000000000000000000" },
            { sha: "803f1048727eabf6d8b3df618203e3c7dda77890" },
            { sha: "2222222000000000000000000000000000000000" },
          ]),
        ...latestMainPackageResponses("2222222"),
      }),
    );
    await expect(resolveTarget("latest-main", gh)).rejects.toThrow(
      "Could not find a supported modern latest-main image set",
    );
  });

  test("sha resolves an explicit complete repo-owned image set", async () => {
    const gh = createGitHubClient(
      fakeRunner({
        "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=1":
          JSON.stringify([
            { sha: "1234abc999999999999999999999999999999999" },
            { sha: "803f1048727eabf6d8b3df618203e3c7dda77890" },
          ]),
        ...latestMainPackageResponses("1234abc"),
      }),
    );
    const bundle = await resolveTarget("sha", gh, { sha: "1234abc999999999999999999999999999999999" });
    expect(bundle.lockName).toBe("sha-1234abc.json");
    expect(bundle.env.GATEWAY_VERSION).toBe("1234abc");
    expect(bundle.env.CORE_VERSION).toBe("v0.13.0");
    expect(bundle.env.RELAYER_VERSION).toBe("sha-29b0750");
    expect(bundle.env.RELAYER_MIGRATE_VERSION).toBe("sha-29b0750");
    expect(bundle.sources).toContain("requested-sha=1234abc999999999999999999999999999999999");
  });

  test("sha rejects commits older than the simple-acl floor", async () => {
    const gh = createGitHubClient(
      fakeRunner({
        "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=1":
          JSON.stringify([
            { sha: "803f1048727eabf6d8b3df618203e3c7dda77890" },
            { sha: "1234abc999999999999999999999999999999999" },
          ]),
        ...latestMainPackageResponses("1234abc"),
      }),
    );
    await expect(resolveTarget("sha", gh, { sha: "1234abc" })).rejects.toThrow(
      "sha target 1234abc predates the simple-ACL cutover and is unsupported",
    );
  });

  test("sha rejects non-main commits even if images exist", async () => {
    const gh = createGitHubClient(
      fakeRunner({
        "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=1":
          JSON.stringify([{ sha: "803f1048727eabf6d8b3df618203e3c7dda77890" }]),
        ...latestMainPackageResponses("1234abc"),
      }),
    );
    await expect(resolveTarget("sha", gh, { sha: "1234abc" })).rejects.toThrow(
      "sha target 1234abc is unsupported; only main commits at or after 803f104 are supported",
    );
  });

  test("sha rejects missing repo-owned images", async () => {
    const responses = latestMainPackageResponses("1234abc");
    responses["gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Fsns-worker/versions?per_page=100&page=1"] =
      JSON.stringify([{ metadata: { container: { tags: ["other"] } } }]);
    const gh = createGitHubClient(fakeRunner(responses));
    await expect(resolveTarget("sha", gh, { sha: "1234abc" })).rejects.toThrow(
      "Could not find a complete sha image set for 1234abc; missing: fhevm/coprocessor/sns-worker",
    );
  });

  test("testnet bundle resolves from gitops-style files", async () => {
    const gh = {
      latestStableRelease: async () => "v0.11.0",
      mainCommits: async () => [],
      packageTags: async () => new Set<string>(),
      gitopsFile: async (file: string) => {
        if (file.includes("gw-sc-deploy-1-init")) return "image:\n  name: ghcr.io/zama-ai/fhevm/gateway-contracts\n  tag: v0.10.0\n";
        if (file.includes("eth-sc-deploy")) return "image:\n  name: ghcr.io/zama-ai/fhevm/host-contracts\n  tag: v0.10.0\n";
        if (file.includes("coproc-infra-db-mig")) return "image:\n  name: ghcr.io/zama-ai/fhevm/coprocessor/db-migration\n  tag: v0.10.9\n";
        if (file.includes("eth-coproc-listener")) return "image:\n  name: ghcr.io/zama-ai/fhevm/coprocessor/host-listener\n  tag: v0.10.10\n";
        if (file.includes("gw-coprocessor")) {
          return "gw:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/gw-listener\n    tag: v0.10.10\ntx:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/tx-sender\n    tag: v0.10.10\n";
        }
        if (file.includes("coproc-workers")) {
          return "tfheWorker:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker\n    tag: v0.10.10\nzkProofWorker:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/zkproof-worker\n    tag: v0.10.10\nsnsWorker:\n  image:\n    name: ghcr.io/zama-ai/fhevm/coprocessor/sns-worker\n    tag: v0.10.10\n";
        }
        if (file.includes("kms-connector")) {
          return "a:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/db-migration\n    tag: v0.10.8\nb:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/gw-listener\n    tag: v0.10.8\nc:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/kms-worker\n    tag: v0.10.8\nd:\n  image:\n    name: ghcr.io/zama-ai/fhevm/kms-connector/tx-sender\n    tag: v0.10.8\n";
        }
        if (file.includes("kms-core")) return "kmsCore:\n  image:\n    name: ghcr.io/zama-ai/kms/core-service-enclave\n    tag: v0.13.3\n";
        if (file.includes("relayer/relayer")) return "image:\n  repository: ghcr.io/zama-ai/console/relayer\n  tag: v0.8.11\njob:\n  image:\n    repository: ghcr.io/zama-ai/console/relayer-migrate\n    tag: v0.8.11\n";
        return "image: ghcr.io/zama-ai/fhevm/test-suite/e2e:v0.10.0\n";
      },
    };
    const bundle = await resolveTarget("testnet", gh);
    expect(bundle.env.CONNECTOR_TX_SENDER_VERSION).toBe("v0.10.8");
    expect(bundle.env.RELAYER_VERSION).toBe("v0.8.11");
    expect(bundle.env.TEST_SUITE_VERSION).toBe("v0.10.0");
  });

  test("version env overrides apply on top of the resolved bundle", async () => {
    const bundle = applyVersionEnvOverrides(
      stubBundle({ lockName: "latest-release-v0.11.0.json", sources: ["preset=latest-release", "repo-owned=v0.11.0"] }),
      { GATEWAY_VERSION: "custom-gateway", RELAYER_VERSION: "custom-relayer" },
    );
    expect(bundle.env.GATEWAY_VERSION).toBe("custom-gateway");
    expect(bundle.env.RELAYER_VERSION).toBe("custom-relayer");
    expect(bundle.env.CORE_VERSION).toBe("v0.13.0");
    expect(bundle.sources.at(-1)).toBe("env=GATEWAY_VERSION,RELAYER_VERSION");
  });
});

describe("runtime invariants", () => {
  test("resolvedComposeEnv preserves version keys", () => {
    const env = resolvedComposeEnv({
      versions: {
        target: "latest-release",
        lockName: "latest-release-v0.11.0.json",
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

    // latest-main SHAs stay modern-only once resolution enforces the floor
    expect(compatPolicyForState(makeState("58aebb0")).coprocessorArgs["host-listener"]).toBeUndefined();
    expect(compatPolicyForState(makeState("58aebb0")).coprocessorArgs["transaction-sender"]).toBeUndefined();
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
      topology: { count: 2, threshold: 2, instances: {} },
    };
    expect(() => resolveUpgradePlan(inactive, "coprocessor")).toThrow(
      "upgrade requires an active local override for coprocessor",
    );

    const plan = resolveUpgradePlan(
      {
        overrides: [{ group: "coprocessor" }],
        topology: { count: 2, threshold: 2, instances: {} },
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
        overrides: [{ group: "coprocessor", services: ["coprocessor-host-listener", "coprocessor-host-listener-poller"] }],
        topology: { count: 2, threshold: 2, instances: {} },
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
        topology: { count: 1, threshold: 1, instances: {} },
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
    const result = await probeBootstrap(
      state,
      {
        ...noopDeps,
        fetch: (async (_url: string | URL, init?: RequestInit) => {
          if (init?.method === "POST") {
            throw new Error("RPC not ready");
          }
          throw new Error("minio not ready");
        }) as typeof fetch,
      },
      0,
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
    let fetchCount = 0;
    const deps = {
      ...noopDeps,
      fetch: (async (_url: string | URL, init?: RequestInit) => {
        if (init?.method === "POST") {
          const body = JSON.parse(String(init.body)) as { params: { data: string }[] };
          const selector = body.params[0].data;
          if (selector === "0xd52f10eb") {
            return new Response(JSON.stringify({ jsonrpc: "2.0", id: 1, result: `0x${predictedKeyId()}` }));
          }
          if (selector === "0xbaff211e") {
            return new Response(JSON.stringify({ jsonrpc: "2.0", id: 1, result: `0x${predictedCrsId()}` }));
          }
        }
        if (init?.method === "HEAD") {
          fetchCount++;
          return new Response("", { status: 404 });
        }
        return new Response("{}");
      }) as typeof fetch,
    };
    await expect(probeBootstrap(state, deps, 0)).rejects.toThrow("Material endpoint not ready");
    expect(fetchCount).toBeGreaterThanOrEqual(30);
  }, 45_000);

  test("composeDown warns on non-zero exit", async () => {
    const file = composePath("database");
    await fs.mkdir(path.dirname(file), { recursive: true });
    await fs.writeFile(file, "services: {}\n");
    const { logs, restore } = captureConsole("warn");
    try {
      await composeDown("database", {
        liveRunner: async () => 1,
      });
    } finally {
      restore();
      await fs.rm(file, { force: true });
    }
    expect(logs.some((l) => l.includes("compose down failed for database (1)"))).toBe(true);
  });

  test("resolveServiceOverrides expands shared-image runtime siblings", () => {
    expect(resolveServiceOverrides("coprocessor", ["host-listener"])).toEqual([
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
    ]);
  });

  test("overrideWarnings flag shared-db per-service runtime overrides only", () => {
    expect(
      overrideWarnings([
        { group: "coprocessor", services: ["coprocessor-host-listener"] },
        { group: "test-suite", services: ["test-suite-e2e-debug"] },
      ]),
    ).toEqual([
      "coprocessor: per-service override with a shared database. If your changes include DB migrations, non-overridden services may fail. Use --override coprocessor (full group) in that case.",
    ]);
  });

  test("predicted bootstrap ids are deterministic", () => {
    expect(predictedKeyId()).toBe("0400000000000000000000000000000000000000000000000000000000000001");
    expect(predictedCrsId()).toBe("0500000000000000000000000000000000000000000000000000000000000001");
  });

  test("up rejects unknown step before doing work", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(["bun", "src/cli.ts", "up", "--from-step", "nope"], noopDeps);
    expect(await maybeRead(STATE_FILE)).toBe(before);
  });

  test("up rejects --from-step without --resume outside dry-run", async () => {
    const dir = await fixtureDir();
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(
      ["bun", "src/cli.ts", "up", "--from-step", "relayer"],
      {
        runner: async () => ({ stdout: "", stderr: "", code: 0 }),
        liveRunner: async () => 0,
        now: () => "2026-03-06T00:00:00.000Z",
        fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
      },
    );
    expect(process.exitCode).toBe(1);
    process.exitCode = 0;
    expect(await maybeRead(STATE_FILE)).toBe(before);
    void dir;
  });

  test("up rejects per-service overrides for non-runtime groups before doing work", async () => {
    const dir = await fixtureDir();
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(
      ["bun", "src/cli.ts", "up", "--override", "gateway-contracts:sc-deploy"],
      {
        runner: async () => ({ stdout: "", stderr: "", code: 0 }),
        liveRunner: async () => 0,
        now: () => "2026-03-06T00:00:00.000Z",
        fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
      },
    );
    expect(process.exitCode).toBe(1);
    process.exitCode = 0;
    expect(await maybeRead(STATE_FILE)).toBe(before);
    void dir;
  });

  test("up --dry-run rejects latest-release partial overrides when local migrations diverge", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-release.json");
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
          "latest-release",
          "--lock-file",
          lockFile,
          "--override",
          "coprocessor:host-listener",
          "--dry-run",
        ],
        {
          runner: fakeRunner({
            "git rev-parse -q --verify v0.11.0^{commit}": "",
            "git ls-files --others --exclude-standard -- coprocessor/fhevm-engine/db-migration/migrations": "",
            "git diff --quiet --exit-code v0.11.0 -- coprocessor/fhevm-engine/db-migration/migrations": {
              stdout: "",
              stderr: "",
              code: 1,
            },
          }),
          liveRunner: async () => 0,
          now: () => "2026-03-06T00:00:00.000Z",
          fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
          env: {},
        },
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
        {
          runner: fakeRunner({
            "git rev-parse -q --verify 803f104^{commit}": "",
            "git ls-files --others --exclude-standard -- coprocessor/fhevm-engine/db-migration/migrations": "",
            "git diff --quiet --exit-code 803f104 -- coprocessor/fhevm-engine/db-migration/migrations": {
              stdout: "",
              stderr: "",
              code: 1,
            },
          }),
          liveRunner: async () => 0,
          now: () => "2026-03-06T00:00:00.000Z",
          fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
          env: {},
        },
      );
    } finally {
      restore();
    }
    expect(logs.some((l) => l.includes("coprocessor: local DB migrations diverge from 803f104"))).toBe(true);
  });

  test("up --dry-run rejects kms-connector partial overrides when local migrations diverge", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-release.json");
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
          "latest-release",
          "--lock-file",
          lockFile,
          "--override",
          "kms-connector:gw-listener",
          "--dry-run",
        ],
        {
          runner: fakeRunner({
            "git rev-parse -q --verify v0.11.0^{commit}": "",
            "git ls-files --others --exclude-standard -- kms-connector/connector-db/migrations": "",
            "git diff --quiet --exit-code v0.11.0 -- kms-connector/connector-db/migrations": {
              stdout: "",
              stderr: "",
              code: 1,
            },
          }),
          liveRunner: async () => 0,
          now: () => "2026-03-06T00:00:00.000Z",
          fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
          env: {},
        },
      );
    } finally {
      restore();
    }
    expect(logs.some((l) => l.includes("kms-connector: local DB migrations diverge from v0.11.0"))).toBe(true);
  });

  test("up --dry-run rejects latest-release partial overrides with untracked local migrations", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-release.json");
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
          "latest-release",
          "--lock-file",
          lockFile,
          "--override",
          "coprocessor:host-listener",
          "--dry-run",
        ],
        {
          runner: fakeRunner({
            "git rev-parse -q --verify v0.11.0^{commit}": "",
            "git ls-files --others --exclude-standard -- coprocessor/fhevm-engine/db-migration/migrations":
              "coprocessor/fhevm-engine/db-migration/migrations/20260310000000_new.sql\n",
          }),
          liveRunner: async () => 0,
          now: () => "2026-03-06T00:00:00.000Z",
          fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
          env: {},
        },
      );
    } finally {
      restore();
    }
    expect(logs.some((l) => l.includes("coprocessor: local DB migrations diverge from v0.11.0"))).toBe(true);
  });

  test("up --dry-run allows divergent partial overrides with --allow-schema-mismatch", async () => {
    const dir = await fixtureDir();
    const lockFile = path.join(dir, "latest-release.json");
    await fs.writeFile(lockFile, JSON.stringify(stubBundle()));
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(
      [
        "bun",
        "src/cli.ts",
        "up",
        "--target",
        "latest-release",
        "--lock-file",
        lockFile,
        "--override",
        "coprocessor:host-listener",
        "--allow-schema-mismatch",
        "--dry-run",
      ],
      {
        runner: fakeRunner({
          "which bun": "",
          "which docker": "",
          "which cast": "",
          "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": "",
          ...portCheckResponses,
        }),
        liveRunner: async () => 0,
        now: () => "2026-03-06T00:00:00.000Z",
        fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
        env: {},
      },
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
      ["bun", "src/cli.ts", "up", "--target", "latest-release", "--dry-run", "--from-step", "relayer"],
      { ...noopDeps, runner },
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
      ["bun", "src/cli.ts", "deploy", "--target", "latest-release", "--dry-run"],
      { ...noopDeps, runner },
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
      ["bun", "src/cli.ts", "up", "--target", "latest-release", "--lock-file", lockFile, "--dry-run"],
      { ...noopDeps, runner },
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
        ["bun", "src/cli.ts", "up", "--target", "latest-release", "--resume", "--from-step", "relayer"],
        { ...noopDeps, runner, liveRunner: async () => 0 },
      );
      expect(await maybeRead(composePath("relayer"))).toContain("services:");
      expect(await maybeRead(path.join(STATE_DIR, "env", "versions.env"))).toContain("GATEWAY_VERSION=");
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
          "latest-release",
          "--resume",
          "--override",
          "coprocessor",
          "--coprocessors",
          "2",
        ],
        noopDeps,
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

  test("resume logs when there is nothing left to do", async () => {
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await fs.rm(STATE_DIR, { recursive: true, force: true });
    await fs.mkdir(STATE_DIR, { recursive: true });
    await fs.writeFile(STATE_FILE, JSON.stringify(stubState({ completedSteps: [...STEP_NAMES] })));
    const { logs, restore } = captureConsole("log");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-release", "--resume"],
        noopDeps,
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
      await main(["bun", "src/cli.ts", "down"], { ...noopDeps, liveRunner: async () => 0 });
      expect(await maybeRead(composePath("database"))).toContain("services:");
      expect(await maybeRead(path.join(STATE_DIR, "env", "versions.env"))).toContain("GATEWAY_VERSION=");
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
      await main(["bun", "src/cli.ts", "clean"], { ...noopDeps, liveRunner: async () => 1 });
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
        ["bun", "src/cli.ts", "up", "--target", "latest-release", "--dry-run"],
        {
          ...noopDeps,
          runner: async (argv) => {
            const key = argv.join(" ");
            if (key === "gh api repos/zama-ai/fhevm/releases?per_page=100&page=1") {
              throw new Error("spawn gh ENOENT");
            }
            return { stdout: "", stderr: "", code: 0 };
          },
        },
      );
    } finally { restore(); }
    expect(
      logs.some((l) =>
        l.includes("GitHub CLI `gh` is required for target resolution"),
      ),
    ).toBe(true);
  });

  test("up --dry-run reports authentication guidance for gh api failures", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-release", "--dry-run"],
        {
          ...noopDeps,
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
            if (key === "gh api repos/zama-ai/fhevm/releases?per_page=100&page=1") {
              throw new Error("gh api repos/zama-ai/fhevm/releases?per_page=100&page=1 failed (1)\nHTTP 401: authentication required");
            }
            return noopDeps.runner(argv, options);
          },
        },
      );
    } finally { restore(); }
    expect(
      logs.some((l) =>
        l.includes("GitHub API access is not authenticated"),
      ),
    ).toBe(true);
  });

  test("up --dry-run reports rate limiting guidance for gh api failures", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(
        ["bun", "src/cli.ts", "up", "--target", "latest-release", "--dry-run"],
        {
          ...noopDeps,
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
            if (key === "gh api repos/zama-ai/fhevm/releases?per_page=100&page=1") {
              throw new Error("gh api repos/zama-ai/fhevm/releases?per_page=100&page=1 failed (1)\nAPI rate limit exceeded");
            }
            return noopDeps.runner(argv, options);
          },
        },
      );
    } finally { restore(); }
    expect(
      logs.some((l) =>
        l.includes("GitHub API rate limit hit while resolving versions"),
      ),
    ).toBe(true);
  });
});

describe("CLI argument validation", () => {
  test("rejects unsupported target", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "bogus"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Unsupported target"))).toBe(true);
  });

  test("rejects coprocessors outside 1-5 range", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--coprocessors", "0"], noopDeps);
      await main(["bun", "src/cli.ts", "up", "--coprocessors", "6"], noopDeps);
    } finally { restore(); }
    expect(logs.filter((l) => l.includes("--coprocessors must be between 1 and 5")).length).toBe(2);
  });

  test("rejects threshold > coprocessors", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--coprocessors", "2", "--threshold", "3"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--threshold must be between 1 and --coprocessors"))).toBe(true);
  });

  test("rejects --target sha without --sha", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "sha", "--dry-run"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--target sha requires --sha"))).toBe(true);
  });

  test("rejects --sha without --target sha", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "latest-release", "--sha", "1234abc", "--dry-run"], noopDeps);
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
      ], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--sha cannot be used with --lock-file"))).toBe(true);
  });

  test("rejects invalid sha format", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--target", "sha", "--sha", "notasha", "--dry-run"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Invalid sha notasha; expected 7 or 40 hex characters"))).toBe(true);
  });

  test("rejects unknown per-service override suffix", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--override", "coprocessor:local"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes('Unknown service "local" in group "coprocessor"'))).toBe(true);
  });

  test("rejects unsupported override group", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--override", "nonexistent"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Unsupported override"))).toBe(true);
  });

  test("rejects unknown command", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "bogus"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Unknown command bogus"))).toBe(true);
  });

  test("doctor shows removal message", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "doctor"], noopDeps);
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
      await main(["bun", "src/cli.ts", "pause"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("pause expects `host` or `gateway`"))).toBe(true);
  });

  test("unpause rejects missing scope", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "unpause"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("unpause expects `host` or `gateway`"))).toBe(true);
  });

  test("test requires completed bootstrap", async () => {
    const stateDir = path.join(REPO_ROOT, ".fhevm");
    const stateFile = path.join(stateDir, "state.json");
    const hadState = await maybeRead(stateFile);
    const { logs, restore } = captureConsole("error");
    try {
      await fs.rm(stateDir, { recursive: true, force: true });
      await main(["bun", "src/cli.ts", "test", "input-proof"], noopDeps);
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
        await main(["bun", "src/cli.ts", "test", "nonexistent-profile"], noopDeps);
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
      await main(["bun", "src/cli.ts", "help"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Usage: fhevm-cli"))).toBe(true);
  });

  test("no command prints usage", async () => {
    const { logs, restore } = captureConsole("log");
    try {
      await main(["bun", "src/cli.ts"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("Usage: fhevm-cli"))).toBe(true);
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
        await main(["bun", "src/cli.ts", "down"], noopDeps);
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
      await main(["bun", "src/cli.ts", "status"], { ...noopDeps, runner });
    } finally { restore(); }
    expect(logs.some((l) => l.includes("No fhevm containers"))).toBe(true);
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

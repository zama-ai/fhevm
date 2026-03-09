import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, describe, expect, test } from "bun:test";

import { REPO_ROOT, STATE_DIR } from "./layout";
import { main } from "./runtime";
import { compatPolicyForState } from "./compat";
import type { RunOptions, RunResult, Runner } from "./utils";
import { predictedCrsId, predictedKeyId } from "./utils";
import { applyVersionEnvOverrides, createGitHubClient, resolveTarget } from "./versions";
import type { State } from "./types";

const STATE_FILE = path.join(STATE_DIR, "state.json");

const tempDirs: string[] = [];

afterEach(async () => {
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

const fakeRunner = (responses: Record<string, string | RunResult>): Runner => async (argv: string[], _options?: RunOptions) => {
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

describe("resolveTarget", () => {
  test("latest-main walks back to the first complete sha bundle", async () => {
    const gh = createGitHubClient(
      fakeRunner({
        "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=1":
          JSON.stringify([{ sha: "1111111000000000000000000000000000000000" }, { sha: "2222222000000000000000000000000000000000" }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fgateway-contracts/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fhost-contracts/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Fdb-migration/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Fhost-listener/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Fgw-listener/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Ftx-sender/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Ftfhe-worker/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Fzkproof-worker/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Fsns-worker/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fkms-connector%2Fdb-migration/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fkms-connector%2Fgw-listener/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fkms-connector%2Fkms-worker/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Fkms-connector%2Ftx-sender/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
        "gh api /orgs/zama-ai/packages/container/fhevm%2Ftest-suite%2Fe2e/versions?per_page=100&page=1":
          JSON.stringify([{ metadata: { container: { tags: ["2222222"] } } }]),
      }),
    );
    const bundle = await resolveTarget("latest-main", gh);
    expect(bundle.lockName).toBe("latest-main-2222222.json");
    expect(bundle.env.GATEWAY_VERSION).toBe("2222222");
    expect(bundle.env.CORE_VERSION).toBe("v0.13.0");
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
      {
        target: "latest-release",
        lockName: "latest-release-v0.11.0.json",
        sources: ["preset=latest-release", "repo-owned=v0.11.0"],
        env: {
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
        },
      },
      {
        GATEWAY_VERSION: "custom-gateway",
        RELAYER_VERSION: "custom-relayer",
      },
    );
    expect(bundle.env.GATEWAY_VERSION).toBe("custom-gateway");
    expect(bundle.env.RELAYER_VERSION).toBe("custom-relayer");
    expect(bundle.env.CORE_VERSION).toBe("v0.13.0");
    expect(bundle.sources.at(-1)).toBe("env=GATEWAY_VERSION,RELAYER_VERSION");
  });
});

describe("runtime invariants", () => {
  test("compat policy keeps legacy coprocessor API key flags for versions before v0.12.0", () => {
    const state = (version: string) =>
      ({
        target: "latest-release",
        lockPath: "",
        versions: {
          target: "latest-release",
          lockName: "",
          sources: [],
          env: {
            GATEWAY_VERSION: "v0.11.0",
            HOST_VERSION: "v0.11.0",
            COPROCESSOR_DB_MIGRATION_VERSION: version,
            COPROCESSOR_HOST_LISTENER_VERSION: version,
            COPROCESSOR_GW_LISTENER_VERSION: version,
            COPROCESSOR_TX_SENDER_VERSION: version,
            COPROCESSOR_TFHE_WORKER_VERSION: version,
            COPROCESSOR_ZKPROOF_WORKER_VERSION: version,
            COPROCESSOR_SNS_WORKER_VERSION: version,
            CONNECTOR_DB_MIGRATION_VERSION: "v0.11.0",
            CONNECTOR_GW_LISTENER_VERSION: "v0.11.0",
            CONNECTOR_KMS_WORKER_VERSION: "v0.11.0",
            CONNECTOR_TX_SENDER_VERSION: "v0.11.0",
            CORE_VERSION: "v0.13.0",
            RELAYER_VERSION: "v0.9.0",
            RELAYER_MIGRATE_VERSION: "v0.9.0",
            TEST_SUITE_VERSION: "v0.11.0",
          },
        },
        overrides: [],
        topology: { count: 1, threshold: 1, instances: {} },
        completedSteps: [],
        updatedAt: "2026-03-09T00:00:00.000Z",
      }) satisfies State;

    // v0.11.x: all coprocessor services still need legacy API key flags
    expect(compatPolicyForState(state("v0.11.0")).coprocessorArgs["host-listener"]).toEqual([
      ["--coprocessor-api-key", "COPROCESSOR_API_KEY"],
    ]);
    expect(compatPolicyForState(state("v0.11.0")).coprocessorArgs["sns-worker"]).toEqual([
      ["--tenant-api-key", "TENANT_API_KEY"],
    ]);

    // v0.12.x: all legacy flags removed
    expect(compatPolicyForState(state("v0.12.0")).coprocessorArgs["host-listener"]).toBeUndefined();
    expect(compatPolicyForState(state("v0.12.0")).coprocessorArgs["sns-worker"]).toBeUndefined();

    // SHA versions: no legacy flags (assumed latest)
    expect(compatPolicyForState(state("58aebb0")).coprocessorArgs["host-listener"]).toBeUndefined();
  });

  test("predicted bootstrap ids are deterministic", () => {
    expect(predictedKeyId()).toBe("0400000000000000000000000000000000000000000000000000000000000001");
    expect(predictedCrsId()).toBe("0500000000000000000000000000000000000000000000000000000000000001");
  });

  test("up rejects unknown step before doing work", async () => {
    const dir = await fixtureDir();
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    await main(
      ["bun", "src/cli.ts", "up", "--from-step", "nope"],
      {
        runner: async () => ({ stdout: "", stderr: "", code: 0 }),
        liveRunner: async () => 0,
        now: () => "2026-03-06T00:00:00.000Z",
        fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
      },
    );
    process.exitCode = 0;
    expect(await maybeRead(STATE_FILE)).toBe(before);
    void dir;
  });

  test("up --dry-run resolves without creating runtime state", async () => {
    const dir = await fixtureDir();
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
      "lsof -nP -iTCP:3000 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:3001 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:5432 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:5433 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:8545 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:8546 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:9000 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:9001 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
    });
    await main(
      ["bun", "src/cli.ts", "up", "--target", "latest-release", "--dry-run"],
      {
        runner,
        liveRunner: async () => 0,
        now: () => "2026-03-06T00:00:00.000Z",
        fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
      },
    );
    process.exitCode = 0;
    expect(await maybeRead(STATE_FILE)).toBe(before);
    void dir;
  });

  test("up --dry-run can use a lock file without GitHub resolution", async () => {
    const dir = await fixtureDir();
    process.chdir(REPO_ROOT);
    const before = await maybeRead(STATE_FILE);
    const lockFile = path.join(dir, "ci-bundle.json");
    await fs.writeFile(
      lockFile,
      JSON.stringify({
        target: "latest-release",
        lockName: "ci-workflow.json",
        sources: ["test"],
        env: {
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
        },
      }),
    );
    const runner = fakeRunner({
      "which bun": "",
      "which docker": "",
      "docker ps --filter label=com.docker.compose.project=fhevm --format {{.Ports}}": "",
      "lsof -nP -iTCP:3000 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:3001 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:5432 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:5433 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:8545 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:8546 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:9000 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
      "lsof -nP -iTCP:9001 -sTCP:LISTEN": { stdout: "", stderr: "", code: 1 },
    });
    await main(
      ["bun", "src/cli.ts", "up", "--target", "latest-release", "--lock-file", lockFile, "--dry-run"],
      {
        runner,
        liveRunner: async () => 0,
        now: () => "2026-03-06T00:00:00.000Z",
        fetch: ((async () => new Response("{}")) as unknown) as typeof fetch,
      },
    );
    process.exitCode = 0;
    expect(await maybeRead(STATE_FILE)).toBe(before);
    void dir;
  });
});

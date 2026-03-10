import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, describe, expect, test } from "bun:test";

import { REPO_ROOT, STATE_DIR } from "./layout";
import { main } from "./runtime";
import { compatPolicyForState } from "./compat";
import { predictedCrsId, predictedKeyId } from "./utils";
import { applyVersionEnvOverrides, createGitHubClient, resolveTarget } from "./versions";
import type { State } from "./types";
import {
  STUB_VERSION_ENV,
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
      ["--coprocessor-api-key", "COPROCESSOR_API_KEY"],
    ]);
    expect(compatPolicyForState(makeState("v0.11.0")).coprocessorArgs["sns-worker"]).toEqual([
      ["--tenant-api-key", "TENANT_API_KEY"],
    ]);
    expect(compatPolicyForState(makeState("v0.12.0")).coprocessorArgs["host-listener"]).toBeUndefined();
    expect(compatPolicyForState(makeState("v0.12.0")).coprocessorArgs["sns-worker"]).toBeUndefined();
    expect(compatPolicyForState(makeState("58aebb0")).coprocessorArgs["host-listener"]).toBeUndefined();
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
      ["bun", "src/cli.ts", "up", "--target", "latest-release", "--dry-run"],
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

  test("rejects --profile without --override", async () => {
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "up", "--profile", "debug"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("--profile requires at least one --override"))).toBe(true);
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
    const { logs, restore } = captureConsole("error");
    try {
      await main(["bun", "src/cli.ts", "test", "input-proof"], noopDeps);
    } finally { restore(); }
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
    const { logs, restore } = captureConsole("log");
    try {
      await main(["bun", "src/cli.ts", "down"], noopDeps);
    } finally { restore(); }
    expect(logs.some((l) => l.includes("nothing to stop") || l.includes("[down]"))).toBe(true);
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
      ["--coprocessor-api-key", "COPROCESSOR_API_KEY"],
    ]);
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

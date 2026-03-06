import { afterEach, describe, expect, test } from "bun:test";
import { mkdir, readFile, rm, stat } from "fs/promises";

import { __internal as discoveryInternal } from "./discovery";
import { runBootPipeline } from "./boot-pipeline";
import {
  createInitialState,
  loadState,
  markPipelineFailed,
  markStepCompleted,
  markStepFailed,
  markStepRunning,
  saveState,
} from "./state";
import { BOOT_STEPS } from "./steps";
import { __internal as servicesInternal } from "../docker/services";
import { __internal as diagnosticsInternal } from "../ci/diagnostics";
import { __internal as keyCacheInternal } from "../keys/cache";

function makeTempDir(): string {
  return `.fhevm/test-boot/${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

async function withTempCwd<T>(fn: (dir: string) => Promise<T>): Promise<T> {
  const originalCwd = process.cwd();
  const dir = makeTempDir();
  await mkdir(dir, { recursive: true });
  process.chdir(dir);

  try {
    return await fn(dir);
  } finally {
    process.chdir(originalCwd);
    await rm(dir, { recursive: true, force: true });
  }
}

const VERSION_ENV_KEYS = [
  "COPROCESSOR_VERSION",
  "KMS_CONNECTOR_VERSION",
  "FHEVM_CONTRACTS_VERSION",
  "KMS_CORE_VERSION",
  "FHEVM_RELAYER_VERSION",
  "FHEVM_TEST_SUITE_VERSION",
];

async function createRelayerTemplate(): Promise<void> {
  const templateDir = "test-suite/fhevm/config/relayer";
  await mkdir(templateDir, { recursive: true });
  await Bun.write(
    `${templateDir}/local.yaml`,
    [
      "gateway:",
      '  blockchain_rpc:',
      '    http_url: "http://gateway-node:8546"',
      '    read_http_url: "http://gateway-node:8546"',
      "    chain_id: 54321",
      "  listener_pool:",
      '    reconnect_config:',
      "      max_attempts: 20",
      "    listeners:",
      '      - type: subscription',
      '        url: "ws://gateway-node:8546"',
      "  tx_engine:",
      "    private_key: 0xaaaa000000000000000000000000000000000000000000000000000000000001",
      "  contracts:",
      '    decryption_address: "0x0000"',
      '    input_verification_address: "0x0000"',
      "keyurl:",
      "  fhe_public_key:",
      '    data_id: "fhe-public-key-data-id"',
      '    url: "http://0.0.0.0:3001/publicKey.bin"',
      "  crs:",
      '    data_id: "crs-data-id"',
      '    url: "http://0.0.0.0:3001/crs2048.bin"',
      "storage:",
      '  sql_database_url: "postgresql://postgres:postgres@relayer-db:5432/relayer_db"',
    ].join("\n"),
  );
}

function setupBootMocks(options: { failServices?: string[] } = {}) {
  // Prevent resolveAllVersions from hitting GitHub API
  for (const key of VERSION_ENV_KEYS) {
    process.env[key] = "v0.0.0-test";
  }

  const upCalls: string[][] = [];
  const stopCalls: string[][] = [];
  const composeUpCalls: Array<{ services: string[]; files: string[] }> = [];

  servicesInternal.setDockerOpsForTests({
    composeUp: async (composeOptions) => {
      const services = [...(composeOptions.services ?? [])];
      upCalls.push(services);
      composeUpCalls.push({ services, files: [...composeOptions.files] });
    },
    composeStop: async (services) => {
      stopCalls.push([...services]);
    },
    waitForAllReady: async (services) => {
      const failing = options.failServices?.find((name) => services.some((service) => service.name === name));
      if (failing) {
        throw new Error(`simulated readiness failure for ${failing}`);
      }

      return services.map((service) => ({
        service: service.name,
        ready: true,
        elapsedMs: 5,
      }));
    },
    composeDown: async () => {},
    composePs: async () => [],
    composeStart: async () => {},
    listProjectContainers: async () => [],
  });

  const keyHandle = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
  const fheKeyId = "aabbccdd00112233445566778899aabbccddeeff00112233445566778899aabb";
  const crsKeyId = "ccddee0011223344556677889900aabbccddeeff00112233445566778899aabb";
  discoveryInternal.setDiscoveryOpsForTests({
    discoverMinioIp: async () => "172.18.0.2",
    getContainerLogs: async () => `signing key handle: ${keyHandle}`,
    fetch: (async (input: RequestInfo | URL) => {
      const url = typeof input === "string" ? input : input instanceof URL ? input.href : input.url;
      if (url.includes(`/kms-public/PUB/VerfAddress/${keyHandle}`)) {
        return new Response("0x1234567890abcdef1234567890abcdef12345678");
      }
      if (url.includes("prefix=PUB%2FServerKey%2F")) {
        const xml = `<ListBucketResult><Contents><Key>PUB/ServerKey/${fheKeyId}</Key></Contents></ListBucketResult>`;
        return new Response(xml, { headers: { "Content-Type": "application/xml" } });
      }
      if (url.includes("prefix=PUB%2FCRS%2F")) {
        const xml = `<ListBucketResult><Contents><Key>PUB/CRS/${crsKeyId}</Key></Contents></ListBucketResult>`;
        return new Response(xml, { headers: { "Content-Type": "application/xml" } });
      }
      return new Response("missing", { status: 404 });
    }) as typeof fetch,
    readFileFromContainer: async (_container: string, path: string) => {
      if (path.endsWith(".env.gateway")) {
        return [
          "GATEWAY_CONFIG_ADDRESS=0xaaaa000000000000000000000000000000000001",
          "KMS_GENERATION_ADDRESS=0xaaaa000000000000000000000000000000000002",
          "INPUT_VERIFICATION_ADDRESS=0xaaaa000000000000000000000000000000000003",
          "DECRYPTION_ADDRESS=0xaaaa000000000000000000000000000000000004",
          "MULTICHAIN_ACL_ADDRESS=0xaaaa000000000000000000000000000000000005",
          "CIPHERTEXT_COMMITS_ADDRESS=0xaaaa000000000000000000000000000000000006",
        ].join("\n");
      }
      if (path.endsWith(".env.host")) {
        return [
          "ACL_CONTRACT_ADDRESS=0xbbbb000000000000000000000000000000000001",
          "FHEVM_EXECUTOR_CONTRACT_ADDRESS=0xbbbb000000000000000000000000000000000002",
        ].join("\n");
      }
      return undefined;
    },
  });

  keyCacheInternal.setOpsForTests({
    downloadBucket: async () => 0,
    uploadBucket: async () => 0,
    exportVolume: async () => {},
    importVolume: async () => {},
  });

  return { upCalls, stopCalls, composeUpCalls };
}

async function exists(path: string): Promise<boolean> {
  try {
    await stat(path);
    return true;
  } catch {
    return false;
  }
}

afterEach(() => {
  delete process.env.CI;
  delete process.env.GITHUB_ACTIONS;
  for (const key of VERSION_ENV_KEYS) {
    delete process.env[key];
  }
  servicesInternal.resetDockerOpsForTests();
  discoveryInternal.resetDiscoveryOpsForTests();
  diagnosticsInternal.resetOpsForTests();
  keyCacheInternal.resetOpsForTests();
});

describe("boot pipeline", () => {
  test("runs full 13-step pipeline and persists completed state", async () => {
    await withTempCwd(async () => {
      const { upCalls } = setupBootMocks();
      await createRelayerTemplate();

      await runBootPipeline({});

      const state = await loadState(".fhevm/state.json");
      expect(state).not.toBeNull();
      expect(state?.status).toBe("completed");
      expect(state?.lastStep).toBe(13);
      expect(state?.runtime.minioIp).toBe("172.18.0.2");
      expect(state?.runtime.kmsSigner).toBe("0x1234567890abcdef1234567890abcdef12345678");
      expect(state?.runtime.fheKeyId).toBe("aabbccdd00112233445566778899aabbccddeeff00112233445566778899aabb");
      expect(state?.runtime.crsKeyId).toBe("ccddee0011223344556677889900aabbccddeeff00112233445566778899aabb");
      expect(state?.runtime.contractAddresses?.acl).toBe("0xbbbb000000000000000000000000000000000001");

      // 14 calls: mocked-payment splits into 2 (deploy + set-relayer),
      // host-contracts splits into 3 (deploy, add-pausers, gateway phase 2)
      expect(upCalls.length).toBe(14);
      expect(upCalls.some((services) => services.includes("host-node"))).toBe(true);
      expect(upCalls.some((services) => services.includes("gateway-node"))).toBe(true);
    });
  });

  test("restores key cache and skips keygen services when snapshots exist", async () => {
    await withTempCwd(async () => {
      const { upCalls } = setupBootMocks();
      await createRelayerTemplate();
      const restoreCalls: string[] = [];
      keyCacheInternal.setOpsForTests({
        downloadBucket: async () => 0,
        exportVolume: async () => {},
        importVolume: async () => {
          restoreCalls.push("importVolume");
        },
        uploadBucket: async () => {
          restoreCalls.push("uploadBucket");
          return 0;
        },
      });

      await mkdir(".fhevm/keys/minio-snapshot/PUB", { recursive: true });
      await mkdir(".fhevm/keys/volume-snapshot", { recursive: true });
      await Bun.write(".fhevm/keys/minio-snapshot/PUB/key", "cached");
      await Bun.write(".fhevm/keys/volume-snapshot/key", "cached");

      await runBootPipeline({});

      expect(restoreCalls).toEqual(["importVolume", "uploadBucket"]);
      const allServices = upCalls.flat();
      expect(allServices.includes("gateway-sc-trigger-keygen")).toBe(false);
      expect(allServices.includes("gateway-sc-trigger-crsgen")).toBe(false);
      expect(allServices.includes("gateway-sc-deploy")).toBe(true);
    });
  });

  test("resumes from failed step and skips already completed steps", async () => {
    await withTempCwd(async () => {
      const { upCalls } = setupBootMocks();
      await createRelayerTemplate();

      const state = createInitialState(BOOT_STEPS.map((step) => ({ number: step.number, name: step.name })));
      for (let i = 1; i <= 3; i += 1) {
        markStepRunning(state, i);
        markStepCompleted(state, i, 10);
      }
      state.runtime.minioIp = "172.18.0.2";
      state.runtime.kmsSigner = "0x1234567890abcdef1234567890abcdef12345678";
      markStepRunning(state, 4);
      markStepFailed(state, 4, "postgres failed");
      markPipelineFailed(state, 4, "postgres failed");
      await saveState(".fhevm/state.json", state);

      await runBootPipeline({ resume: true });

      const allServices = upCalls.flat();
      expect(allServices.includes("minio")).toBe(false);
      expect(allServices.includes("kms-core")).toBe(false);
      expect(allServices.includes("db")).toBe(true);

      const finalState = await loadState(".fhevm/state.json");
      expect(finalState?.status).toBe("completed");
      expect(finalState?.lastStep).toBe(13);
    });
  });

  test("runs multi-coprocessor step with per-instance compose files and env files", async () => {
    await withTempCwd(async () => {
      const { composeUpCalls } = setupBootMocks();
      await createRelayerTemplate();

      await runBootPipeline({ numCoprocessors: 3, threshold: 2 });

      const coprocessorBatches = composeUpCalls.filter((call) =>
        call.services.some((service) =>
          /^coprocessor(?:-\d+)?-(db-migration|host-listener|host-listener-poller|gw-listener|tfhe-worker|zkproof-worker|sns-worker|transaction-sender)$/.test(
            service,
          ),
        ),
      );
      expect(coprocessorBatches).toHaveLength(3);
      expect(coprocessorBatches.some((call) => call.services.includes("coprocessor-db-migration"))).toBe(true);
      expect(coprocessorBatches.some((call) => call.services.includes("coprocessor-2-db-migration"))).toBe(true);
      expect(coprocessorBatches.some((call) => call.services.includes("coprocessor-3-db-migration"))).toBe(true);
      expect(coprocessorBatches.some((call) => call.files.some((file) => file.endsWith("coprocessor-2.yml")))).toBe(true);
      expect(coprocessorBatches.some((call) => call.files.some((file) => file.endsWith("coprocessor-3.yml")))).toBe(true);

      expect(await exists(".fhevm/compose/coprocessor-2.yml")).toBe(true);
      expect(await exists(".fhevm/compose/coprocessor-3.yml")).toBe(true);
      expect(await exists("test-suite/fhevm/env/staging/coprocessor-2.env")).toBe(true);
      expect(await exists("test-suite/fhevm/env/staging/coprocessor-3.env")).toBe(true);

      const compose = await readFile(".fhevm/compose/coprocessor-2.yml", "utf8");
      expect(compose).toContain("coprocessor-2-tfhe-worker");
    });
  });

  test("supports --from by tearing down target step onward in reverse order", async () => {
    await withTempCwd(async () => {
      const { upCalls, stopCalls } = setupBootMocks();
      await createRelayerTemplate();

      const state = createInitialState(BOOT_STEPS.map((step) => ({ number: step.number, name: step.name })));
      for (const step of BOOT_STEPS) {
        markStepRunning(state, step.number);
        markStepCompleted(state, step.number, 10);
      }
      state.runtime.minioIp = "172.18.0.2";
      state.runtime.kmsSigner = "0x1234567890abcdef1234567890abcdef12345678";
      await saveState(".fhevm/state.json", state);

      await runBootPipeline({ from: "7" });

      // Step order: 7=gateway-mocked-payment, 8=gateway-contracts, 9=host-contracts,
      // 10=kms-connector, 11=coprocessor, 12=relayer, 13=test-suite
      const teardownCalls = stopCalls.slice(0, 7);
      expect(teardownCalls[0]).toEqual(["test-suite-e2e-debug"]);
      expect(teardownCalls[1]).toEqual(["relayer-db", "relayer-db-migration", "relayer"]);
      expect(teardownCalls[2]).toEqual([
        "coprocessor-db-migration",
        "coprocessor-host-listener",
        "coprocessor-host-listener-poller",
        "coprocessor-gw-listener",
        "coprocessor-tfhe-worker",
        "coprocessor-zkproof-worker",
        "coprocessor-sns-worker",
        "coprocessor-transaction-sender",
      ]);
      expect(teardownCalls[3]).toEqual([
        "kms-connector-db-migration",
        "kms-connector-gw-listener",
        "kms-connector-kms-worker",
        "kms-connector-tx-sender",
      ]);
      expect(teardownCalls[4]).toEqual(["host-sc-deploy", "host-sc-add-pausers"]);
      expect(teardownCalls[5]).toEqual([
        "gateway-sc-deploy",
        "gateway-sc-add-network",
        "gateway-sc-add-pausers",
        "gateway-sc-trigger-keygen",
        "gateway-sc-trigger-crsgen",
      ]);
      expect(teardownCalls[6]).toEqual(["gateway-deploy-mocked-zama-oft", "gateway-set-relayer-mocked-payment"]);

      const allStarted = upCalls.flat();
      expect(allStarted.includes("host-node")).toBe(false);
      expect(allStarted.includes("gateway-node")).toBe(false);
      expect(allStarted.includes("coprocessor-db-migration")).toBe(true);

      const finalState = await loadState(".fhevm/state.json");
      expect(finalState?.status).toBe("completed");
    });
  });

  test("fails a parallel group when one step fails and the other succeeds", async () => {
    await withTempCwd(async () => {
      const { upCalls } = setupBootMocks({ failServices: ["host-node"] });

      await expect(runBootPipeline({})).rejects.toMatchObject({
        step: "Step 5: Host Node (Anvil)",
      });

      const state = await loadState(".fhevm/state.json");
      expect(state?.status).toBe("failed");
      expect(state?.failedStep).toBe(5);
      expect(state?.steps.find((step) => step.number === 5)?.status).toBe("failed");
      expect(state?.steps.find((step) => step.number === 6)?.status).toBe("completed");
      expect(state?.steps.find((step) => step.number === 7)?.status).toBe("pending");

      const allServices = upCalls.flat();
      expect(allServices.includes("host-node")).toBe(true);
      expect(allServices.includes("gateway-node")).toBe(true);
      expect(allServices.includes("coprocessor-db-migration")).toBe(false);
    });
  });

  test("captures diagnostics on failure in CI mode", async () => {
    await withTempCwd(async () => {
      const { upCalls } = setupBootMocks({ failServices: ["host-node"] });
      process.env.CI = "true";

      let diagnosticsListed = false;
      diagnosticsInternal.setOpsForTests({
        listProjectContainers: async () => {
          diagnosticsListed = true;
          return [];
        },
      });

      await expect(runBootPipeline({})).rejects.toMatchObject({
        step: "Step 5: Host Node (Anvil)",
      });

      expect(diagnosticsListed).toBe(true);
      expect(upCalls.flat().includes("coprocessor-db-migration")).toBe(false);
    });
  });
});

import path from "node:path";
import { describe, expect, test } from "bun:test";

import { renderEnvMaps } from "./generate/env";
import { COMPONENTS, TEMPLATE_ENV_DIR } from "./layout";
import { presetBundle } from "./resolve/target";
import { stackSpecForState } from "./stack-spec/stack-spec";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";
import { readEnvFile } from "./utils/fs";

const deriveWallet = async (_mnemonic: string, index: number) => ({
  address: `0x${String(index + 1).padStart(40, "1")}`,
  privateKey: `0x${String(index + 1).padStart(64, "2")}`,
});

describe("env", () => {
  test("renders non-empty host-chain metadata defaults for multi-chain gateway registration", async () => {
    const templateEnvs = Object.fromEntries(
      await Promise.all(
        COMPONENTS.map(async (component) => [
          component,
          await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
        ]),
      ),
    ) as Record<string, Record<string, string>>;
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/latest-main.json",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides: [],
      scenario: testDefaultScenario({
        hostChains: [
          { key: "host", chainId: "12345", rpcPort: 8545 },
          { key: "chain-b", chainId: "67890", rpcPort: 8547 },
        ],
      }),
      completedSteps: [],
      updatedAt: "2026-03-30T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: undefined }, stackSpecForState(state), templateEnvs, deriveWallet);

    expect(rendered.componentEnvs["gateway-sc"].HOST_CHAIN_NAME_0).toBe("Host chain 0");
    expect(rendered.componentEnvs["gateway-sc"].HOST_CHAIN_WEBSITE_0).toBe("https://host-chain-0.com");
    expect(rendered.componentEnvs["gateway-sc"].HOST_CHAIN_NAME_1).toBe("Host chain 1");
    expect(rendered.componentEnvs["gateway-sc"].HOST_CHAIN_WEBSITE_1).toBe("https://host-chain-1.com");
  });

  test("projects custom primary host settings into runtime envs", async () => {
    const templateEnvs = Object.fromEntries(
      await Promise.all(
        COMPONENTS.map(async (component) => [
          component,
          await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
        ]),
      ),
    ) as Record<string, Record<string, string>>;
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/latest-main.json",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides: [],
      scenario: testDefaultScenario({
        hostChains: [{ key: "host", chainId: "543210", rpcPort: 9650 }],
      }),
      completedSteps: [],
      updatedAt: "2026-03-31T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: undefined }, stackSpecForState(state), templateEnvs, deriveWallet);

    expect(rendered.componentEnvs["host-node"].HOST_NODE_PORT).toBe("9650");
    expect(rendered.componentEnvs["host-node"].HOST_NODE_CHAIN_ID).toBe("543210");
    expect(rendered.componentEnvs["host-sc"].RPC_URL).toBe("http://host-node:9650");
    expect(rendered.componentEnvs["coprocessor"].RPC_HTTP_URL).toBe("http://host-node:9650");
    expect(rendered.componentEnvs["kms-connector"].KMS_CONNECTOR_ETHEREUM_URL).toBe("http://host-node:9650");
    expect(rendered.componentEnvs["test-suite"].RPC_URL).toBe("http://host-node:9650");
    expect(rendered.componentEnvs["test-suite"].CHAIN_ID_HOST).toBe("543210");
  });

  test("treats the first explicit chain key as the default runtime chain", async () => {
    const templateEnvs = Object.fromEntries(
      await Promise.all(
        COMPONENTS.map(async (component) => [
          component,
          await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
        ]),
      ),
    ) as Record<string, Record<string, string>>;
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/latest-main.json",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides: [],
      scenario: testDefaultScenario({
        hostChains: [
          { key: "chain-a", chainId: "543210", rpcPort: 9650 },
          { key: "chain-b", chainId: "67890", rpcPort: 9750 },
        ],
      }),
      completedSteps: [],
      updatedAt: "2026-04-01T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: undefined }, stackSpecForState(state), templateEnvs, deriveWallet);

    expect(rendered.componentEnvs["host-node"].HOST_NODE_PORT).toBe("9650");
    expect(rendered.componentEnvs["host-node"].HOST_NODE_CHAIN_ID).toBe("543210");
    expect(rendered.componentEnvs["host-sc"].RPC_URL).toBe("http://host-node:9650");
    expect(rendered.componentEnvs["host-sc"].HOST_ADDRESS_DIR).toBe("chain-a");
    expect(rendered.componentEnvs["test-suite"].CHAIN_ID_HOST).toBe("543210");
    expect(rendered.instanceEnvs["host-node-chain-b"]?.HOST_NODE_PORT).toBe("9750");
    expect(rendered.instanceEnvs["host-sc-chain-b"]?.CHAIN_ID).toBe("67890");
    expect(rendered.instanceEnvs["host-sc-chain-b"]?.HOST_ADDRESS_DIR).toBe("chain-b");
  });

  test("renders final env values without self-interpolation placeholders", async () => {
    const templateEnvs = Object.fromEntries(
      await Promise.all(
        COMPONENTS.map(async (component) => [
          component,
          await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
        ]),
      ),
    ) as Record<string, Record<string, string>>;
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/latest-main.json",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides: [],
      scenario: testDefaultScenario(),
      completedSteps: [],
      updatedAt: "2026-04-01T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: undefined }, stackSpecForState(state), templateEnvs, deriveWallet);
    for (const env of [...Object.values(rendered.componentEnvs), ...Object.values(rendered.instanceEnvs)]) {
      for (const value of Object.values(env)) {
        expect(value.includes("${")).toBe(false);
      }
    }
  });

  test("projects relayer image repositories into versions env", async () => {
    const templateEnvs = Object.fromEntries(
      await Promise.all(
        COMPONENTS.map(async (component) => [
          component,
          await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
        ]),
      ),
    ) as Record<string, Record<string, string>>;
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/latest-main.json",
      requiresGitHub: true,
      versions: {
        ...presetBundle("latest-main", "abcdef0", "latest-main.json"),
        env: {
          ...presetBundle("latest-main", "abcdef0", "latest-main.json").env,
          RELAYER_VERSION: "b799892",
          RELAYER_MIGRATE_VERSION: "65cf86e",
        },
      },
      overrides: [],
      scenario: testDefaultScenario(),
      completedSteps: [],
      updatedAt: "2026-04-01T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: undefined }, stackSpecForState(state), templateEnvs, deriveWallet);
    expect(rendered.versionsEnv.RELAYER_IMAGE_REPOSITORY).toBe("ghcr.io/zama-ai/fhevm/relayer");
    expect(rendered.versionsEnv.RELAYER_MIGRATE_IMAGE_REPOSITORY).toBe("ghcr.io/zama-ai/fhevm/relayer-migrate");
  });

  test("exports optional tfhe-worker thread overrides into coprocessor env", async () => {
    const templateEnvs = Object.fromEntries(
      await Promise.all(
        COMPONENTS.map(async (component) => [
          component,
          await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
        ]),
      ),
    ) as Record<string, Record<string, string>>;
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/latest-main.json",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides: [],
      coprocessorTfheWorkerThreads: 64,
      coprocessorTfheWorkerTokioThreads: 16,
      coprocessorTfheWorkerPollingIntervalMs: 500,
      coprocessorTfheWorkerWorkItemsBatchSize: 64,
      scenario: testDefaultScenario(),
      completedSteps: [],
      updatedAt: "2026-04-01T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps(state, stackSpecForState(state), templateEnvs, deriveWallet);
    expect(rendered.componentEnvs["coprocessor"].COPROCESSOR_TFHE_WORKER_FHE_THREADS).toBe("64");
    expect(rendered.componentEnvs["coprocessor"].COPROCESSOR_TFHE_WORKER_TOKIO_THREADS).toBe("16");
    expect(rendered.componentEnvs["coprocessor"].COPROCESSOR_TFHE_WORKER_POLLING_INTERVAL_MS).toBe("500");
    expect(rendered.componentEnvs["coprocessor"].COPROCESSOR_TFHE_WORKER_WORK_ITEMS_BATCH_SIZE).toBe("64");
  });
});

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

  test("projects gateway kms node settings into host contract envs", async () => {
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
      updatedAt: "2026-04-28T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: undefined }, stackSpecForState(state), templateEnvs, deriveWallet);

    expect(rendered.componentEnvs["host-sc"].KMS_TX_SENDER_ADDRESS_0).toBe(
      rendered.componentEnvs["gateway-sc"].KMS_TX_SENDER_ADDRESS_0,
    );
    expect(rendered.componentEnvs["host-sc"].KMS_NODE_STORAGE_URL_0).toBe(
      rendered.componentEnvs["gateway-sc"].KMS_NODE_STORAGE_URL_0,
    );
    expect(rendered.componentEnvs["host-sc"].USER_DECRYPTION_THRESHOLD).toBe(
      rendered.componentEnvs["gateway-sc"].USER_DECRYPTION_THRESHOLD,
    );
    expect(rendered.componentEnvs["host-sc"].KMS_GEN_THRESHOLD).toBe(
      rendered.componentEnvs["gateway-sc"].KMS_GENERATION_THRESHOLD,
    );
    expect(rendered.componentEnvs["host-sc"].MPC_THRESHOLD).toBe(
      rendered.componentEnvs["gateway-sc"].MPC_THRESHOLD,
    );
  });

  test("sources kms-generation addresses from host contracts for modern bundles", async () => {
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
      discovery: {
        gateway: {
          GATEWAY_CONFIG_ADDRESS: "0x0000000000000000000000000000000000000001",
          INPUT_VERIFICATION_ADDRESS: "0x0000000000000000000000000000000000000002",
          CIPHERTEXT_COMMITS_ADDRESS: "0x0000000000000000000000000000000000000003",
          DECRYPTION_ADDRESS: "0x0000000000000000000000000000000000000004",
        },
        hosts: {
          host: {
            ACL_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000010",
            FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000011",
            INPUT_VERIFIER_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000012",
            KMS_VERIFIER_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000013",
            PAUSER_SET_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000014",
            KMS_GENERATION_CONTRACT_ADDRESS: "0x0000000000000000000000000000000000000015",
          },
        },
        kmsSigner: "",
        fheKeyId: "f".repeat(64),
        crsKeyId: "c".repeat(64),
        endpoints: {
          gateway: { http: "http://gateway-node:8546", ws: "ws://gateway-node:8546" },
          hosts: { host: { http: "http://host-node:8545", ws: "ws://host-node:8545" } },
          minioInternal: "http://minio:9000",
          minioExternal: "http://localhost:9000",
        },
      },
      completedSteps: [],
      updatedAt: "2026-04-27T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: state.discovery }, stackSpecForState(state), templateEnvs, deriveWallet);

    expect(rendered.componentEnvs["coprocessor"].KMS_GENERATION_ADDRESS).toBe(
      "0x0000000000000000000000000000000000000015",
    );
    expect(rendered.componentEnvs["kms-connector"].KMS_CONNECTOR_KMS_GENERATION_CONTRACT__ADDRESS).toBe(
      "0x0000000000000000000000000000000000000015",
    );
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
});

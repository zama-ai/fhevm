import { describe, expect, test } from "bun:test";

import { assertContractTaskStackRunning } from "./flow/contracts";
import { validateDiscovery } from "./flow/discovery";
import {
  displayedBundle,
  multiChainCoprocessorUpgradeTargets,
  preflightPorts,
  resumeRepairStep,
  runtimeArtifactPaths,
  shouldShowResumeHint,
} from "./flow/up-flow";
import { envPath, hostChainAddressesPath, kmsCoreConfigPath } from "./layout";
import { type Discovery, OVERRIDE_GROUPS, type State } from "./types";

const completeState = (): State => ({
  target: "latest-main",
  lockPath: "/tmp/latest-main.json",
  versions: {
    target: "latest-main",
    lockName: "latest-main.json",
    sources: ["test"],
    env: {
      GATEWAY_VERSION: "02f6cc0",
      HOST_VERSION: "02f6cc0",
      COPROCESSOR_DB_MIGRATION_VERSION: "02f6cc0",
      COPROCESSOR_HOST_LISTENER_VERSION: "02f6cc0",
      COPROCESSOR_GW_LISTENER_VERSION: "02f6cc0",
      COPROCESSOR_TX_SENDER_VERSION: "02f6cc0",
      COPROCESSOR_TFHE_WORKER_VERSION: "02f6cc0",
      COPROCESSOR_ZKPROOF_WORKER_VERSION: "02f6cc0",
      COPROCESSOR_SNS_WORKER_VERSION: "02f6cc0",
      CONNECTOR_DB_MIGRATION_VERSION: "02f6cc0",
      CONNECTOR_GW_LISTENER_VERSION: "02f6cc0",
      CONNECTOR_KMS_WORKER_VERSION: "02f6cc0",
      CONNECTOR_TX_SENDER_VERSION: "02f6cc0",
      CORE_VERSION: "v0.13.20-0",
      RELAYER_VERSION: "v0.11.0-rc.2",
      RELAYER_MIGRATE_VERSION: "v0.11.0-rc.1",
      TEST_SUITE_VERSION: "02f6cc0",
    },
  },
  overrides: [],
  scenario: {
    version: 1,
    kind: "coprocessor-consensus",
    origin: "default",
    hostChains: [{ key: "host", chainId: "12345", rpcPort: 8545 }],
    topology: { count: 1, threshold: 1 },
    instances: [{ index: 0, source: { mode: "inherit" }, env: {}, args: {} }],
  },
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
    "relayer",
    "test-suite",
  ],
  updatedAt: "2026-03-31T00:00:00.000Z",
});

const validDiscovery = (hostKeys: string[]): Discovery => ({
  gateway: {
    GATEWAY_CONFIG_ADDRESS: "0x1",
    KMS_GENERATION_ADDRESS: "0x2",
    DECRYPTION_ADDRESS: "0x3",
    INPUT_VERIFICATION_ADDRESS: "0x4",
    CIPHERTEXT_COMMITS_ADDRESS: "0x5",
    MULTICHAIN_ACL_ADDRESS: "0x6",
  },
  hosts: Object.fromEntries(
    hostKeys.map((key, index) => [
      key,
      {
        ACL_CONTRACT_ADDRESS: "0x10",
        FHEVM_EXECUTOR_CONTRACT_ADDRESS: "0x11",
        KMS_VERIFIER_CONTRACT_ADDRESS: "0x12",
        INPUT_VERIFIER_CONTRACT_ADDRESS: "0x13",
        PAUSER_SET_CONTRACT_ADDRESS: "0x14",
        PROTOCOL_CONFIG_CONTRACT_ADDRESS: "0x15",
        ...(index === 0 ? { KMS_GENERATION_CONTRACT_ADDRESS: "0x16" } : {}),
      },
    ]),
  ),
  kmsSigner: "0x7",
  fheKeyId: "a".repeat(64),
  crsKeyId: "b".repeat(64),
  endpoints: {
    gateway: { http: "http://gateway-node:8546", ws: "ws://gateway-node:8546" },
    hosts: Object.fromEntries(hostKeys.map((key) => [key, { http: `http://${key}:8545`, ws: `ws://${key}:8545` }])),
    minioInternal: "http://minio:9000",
    minioExternal: "http://localhost:9000",
  },
});

describe("validateDiscovery", () => {
  test("requires KMSGeneration on the canonical host", () => {
    const state = completeState();
    state.discovery = validDiscovery(["host"]);
    delete state.discovery.hosts.host.KMS_GENERATION_CONTRACT_ADDRESS;

    expect(() => validateDiscovery(state)).toThrow(
      'Missing host discovery value KMS_GENERATION_CONTRACT_ADDRESS for chain "host"',
    );
  });

  test("rejects KMSGeneration on non-canonical hosts", () => {
    const state = completeState();
    state.scenario.hostChains = [
      { key: "chain-a", chainId: "12345", rpcPort: 8545 },
      { key: "chain-b", chainId: "67890", rpcPort: 8547 },
    ];
    state.discovery = validDiscovery(["chain-a", "chain-b"]);
    state.discovery.hosts["chain-b"].KMS_GENERATION_CONTRACT_ADDRESS = "0x17";

    expect(() => validateDiscovery(state)).toThrow(
      'Host discovery for non-canonical chain "chain-b" contains KMS_GENERATION_CONTRACT_ADDRESS',
    );
  });

  test("accepts KMSGeneration only on the canonical host", () => {
    const state = completeState();
    state.scenario.hostChains = [
      { key: "chain-a", chainId: "12345", rpcPort: 8545 },
      { key: "chain-b", chainId: "67890", rpcPort: 8547 },
    ];
    state.discovery = validDiscovery(["chain-a", "chain-b"]);

    expect(() => validateDiscovery(state)).not.toThrow();
  });
});

describe("resumeRepairStep", () => {
  test("repairs from relayer when a partially running stack is missing fhevm-relayer", () => {
    const running = [
      "fhevm-minio",
      "coprocessor-and-kms-db",
      "kms-core",
      "host-node",
      "gateway-node",
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor-gw-listener",
      "coprocessor-tfhe-worker",
      "coprocessor-zkproof-worker",
      "coprocessor-sns-worker",
      "coprocessor-transaction-sender",
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
      "fhevm-relayer-db",
      "fhevm-test-suite-e2e-debug",
    ];
    expect(resumeRepairStep(completeState(), running)).toBe("relayer");
  });

  test("returns nothing when every steady-state service is present", () => {
    const running = [
      "fhevm-minio",
      "coprocessor-and-kms-db",
      "kms-core",
      "host-node",
      "gateway-node",
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor-gw-listener",
      "coprocessor-tfhe-worker",
      "coprocessor-zkproof-worker",
      "coprocessor-sns-worker",
      "coprocessor-transaction-sender",
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
      "fhevm-relayer-db",
      "fhevm-relayer",
      "fhevm-test-suite-e2e-debug",
    ];
    expect(resumeRepairStep(completeState(), running)).toBeUndefined();
  });

  test("repairs multi-instance stacks when a secondary coprocessor service is missing", () => {
    const state = completeState();
    state.scenario.topology = { count: 2, threshold: 2 };
    state.scenario.instances = [
      { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
      { index: 1, source: { mode: "inherit" }, env: {}, args: {} },
    ];
    const running = [
      "fhevm-minio",
      "coprocessor-and-kms-db",
      "kms-core",
      "host-node",
      "gateway-node",
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor-gw-listener",
      "coprocessor-tfhe-worker",
      "coprocessor-zkproof-worker",
      "coprocessor-sns-worker",
      "coprocessor-transaction-sender",
      "coprocessor1-host-listener",
      "coprocessor1-host-listener-poller",
      "coprocessor1-gw-listener",
      "coprocessor1-tfhe-worker",
      "coprocessor1-zkproof-worker",
      "coprocessor1-sns-worker",
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
      "fhevm-relayer-db",
      "fhevm-relayer",
      "fhevm-test-suite-e2e-debug",
    ];
    expect(resumeRepairStep(state, running)).toBe("coprocessor");
  });

  test("repairs partially completed stacks from base when base services are missing", () => {
    const state = completeState();
    state.completedSteps = ["preflight", "resolve", "generate", "base", "kms-signer"];
    const running = ["fhevm-minio", "coprocessor-and-kms-db", "kms-core"];
    expect(resumeRepairStep(state, running)).toBe("base");
  });

  test("does not expect non-listener extra-chain coprocessor services on healthy multi-chain stacks", () => {
    const state = completeState();
    state.scenario.hostChains = [
      { key: "host", chainId: "12345", rpcPort: 8545 },
      { key: "chain-b", chainId: "67890", rpcPort: 8547 },
    ];
    const running = [
      "fhevm-minio",
      "coprocessor-and-kms-db",
      "kms-core",
      "host-node",
      "host-node-chain-b",
      "gateway-node",
      "coprocessor-host-listener",
      "coprocessor-host-listener-poller",
      "coprocessor-gw-listener",
      "coprocessor-tfhe-worker",
      "coprocessor-zkproof-worker",
      "coprocessor-sns-worker",
      "coprocessor-transaction-sender",
      "coprocessor-host-listener-chain-b",
      "coprocessor-host-listener-poller-chain-b",
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
      "fhevm-relayer-db",
      "fhevm-relayer",
      "fhevm-test-suite-e2e-debug",
    ];
    expect(resumeRepairStep(state, running)).toBeUndefined();
  });

  test("repairs from coprocessor when a required runtime container is unhealthy", () => {
    const live = new Map([
      ["fhevm-minio", { status: "running" }],
      ["coprocessor-and-kms-db", { status: "running", health: "healthy" }],
      ["kms-core", { status: "running" }],
      ["host-node", { status: "running" }],
      ["gateway-node", { status: "running" }],
      ["coprocessor-host-listener", { status: "running" }],
      ["coprocessor-host-listener-poller", { status: "running" }],
      ["coprocessor-gw-listener", { status: "running" }],
      ["coprocessor-tfhe-worker", { status: "running", health: "unhealthy" }],
      ["coprocessor-zkproof-worker", { status: "running" }],
      ["coprocessor-sns-worker", { status: "running" }],
      ["coprocessor-transaction-sender", { status: "running" }],
      ["kms-connector-gw-listener", { status: "running" }],
      ["kms-connector-kms-worker", { status: "running" }],
      ["kms-connector-tx-sender", { status: "running" }],
      ["fhevm-relayer-db", { status: "running", health: "healthy" }],
      ["fhevm-relayer", { status: "running" }],
      ["fhevm-test-suite-e2e-debug", { status: "running" }],
    ]);
    expect(resumeRepairStep(completeState(), live)).toBe("coprocessor");
  });
});

describe("runtime helpers", () => {
  test("contract tasks reject missing persisted state", () => {
    expect(() => assertContractTaskStackRunning(false, 0)).toThrow("Stack is not running; run `fhevm-cli up` first");
  });

  test("contract tasks reject stopped stacks with persisted state", () => {
    expect(() => assertContractTaskStackRunning(true, 0)).toThrow(
      "Stack is stopped; run `fhevm-cli up --resume` first",
    );
  });

  test("preflight includes custom host rpc ports", () => {
    const state = completeState();
    state.scenario.hostChains = [
      { key: "host", chainId: "12345", rpcPort: 9555 },
      { key: "chain-b", chainId: "67890", rpcPort: 9666 },
    ];
    expect(preflightPorts(state)).toContain(9555);
    expect(preflightPorts(state)).toContain(9666);
  });

  test("runtime artifacts include extra-chain host env files", () => {
    const state = completeState();
    state.scenario.hostChains = [
      { key: "host", chainId: "12345", rpcPort: 8545 },
      { key: "chain-b", chainId: "67890", rpcPort: 8547 },
    ];
    const paths = runtimeArtifactPaths(state);
    expect(paths).toContain(envPath("host-node-chain-b"));
    expect(paths).toContain(envPath("host-sc-chain-b"));
  });

  test("runtime artifacts include the generated kms-core config", () => {
    expect(runtimeArtifactPaths(completeState())).toContain(kmsCoreConfigPath);
  });

  test("runtime artifacts use the first explicit chain key for default host addresses", () => {
    const state = completeState();
    state.scenario.hostChains = [
      { key: "chain-a", chainId: "12345", rpcPort: 8545 },
      { key: "chain-b", chainId: "67890", rpcPort: 8547 },
    ];
    state.discovery = {
      gateway: {} as NonNullable<State["discovery"]>["gateway"],
      hosts: {
        "chain-a": {} as NonNullable<State["discovery"]>["hosts"][string],
        "chain-b": {} as NonNullable<State["discovery"]>["hosts"][string],
      },
      kmsSigner: "0x1",
      fheKeyId: "a".repeat(64),
      crsKeyId: "b".repeat(64),
      endpoints: {
        gateway: { http: "http://gateway-node:8546", ws: "ws://gateway-node:8546" },
        hosts: {
          "chain-a": { http: "http://host-node:8545", ws: "ws://host-node:8545" },
          "chain-b": { http: "http://host-node-chain-b:8547", ws: "ws://host-node-chain-b:8547" },
        },
        minioInternal: "http://minio:9000",
        minioExternal: "http://localhost:9000",
      },
    };
    const paths = runtimeArtifactPaths(state);
    expect(paths).toContain(hostChainAddressesPath("chain-a"));
    expect(paths).not.toContain(hostChainAddressesPath("host"));
  });

  test("coprocessor upgrade includes extra-chain listener services", () => {
    const state = completeState();
    state.scenario.hostChains = [
      { key: "host", chainId: "12345", rpcPort: 8545 },
      { key: "chain-b", chainId: "67890", rpcPort: 8547 },
    ];
    state.scenario.topology = { count: 2, threshold: 2 };
    state.scenario.instances = [
      { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
      { index: 1, source: { mode: "inherit" }, env: {}, args: {} },
    ];
    expect(
      multiChainCoprocessorUpgradeTargets(state, ["coprocessor-host-listener", "coprocessor1-host-listener-poller"]),
    ).toEqual([
      {
        compose: "coprocessor-chain-b",
        chainKey: "chain-b",
        services: [
          "coprocessor-host-listener-chain-b",
          "coprocessor1-host-listener-chain-b",
          "coprocessor-host-listener-poller-chain-b",
          "coprocessor1-host-listener-poller-chain-b",
        ],
      },
    ]);
  });

  test("resume hint is suppressed for equals-form fresh-stack flags", () => {
    expect(shouldShowResumeHint(["up", "--target=sha", "--sha=badbad"])).toBe(false);
  });

  test("displayedBundle prints local build for repo-owned versions under full build", () => {
    const state = completeState();
    state.overrides = OVERRIDE_GROUPS.map((group) => ({ group }));
    const bundle = displayedBundle(state.versions, state.overrides);
    expect(bundle.env.GATEWAY_VERSION).toBe("LOCAL BUILD");
    expect(bundle.env.RELAYER_VERSION).toBe("LOCAL BUILD");
    expect(bundle.env.TEST_SUITE_VERSION).toBe("LOCAL BUILD");
    expect(bundle.env.CORE_VERSION).toBe("v0.13.20-0");
  });
});

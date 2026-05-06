import { describe, expect, test } from "bun:test";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import YAML from "yaml";

import { generateComposeOverrides, loadMergedComposeDoc } from "./generate/compose";
import { TEMPLATE_COMPOSE_DIR, composePath, envPath } from "./layout";
import { presetBundle } from "./resolve/target";
import { parseCoprocessorScenario, resolveScenarioFile } from "./scenario/resolve";
import { stackSpecForState } from "./stack-spec/stack-spec";
import { testDefaultScenario } from "./test-fixtures";
import { withTempStateDir } from "./test-state";
import type { State } from "./types";
import { composeEnv } from "./utils/process";

const scenario = resolveScenarioFile(
  path.join("/tmp", "two-of-two-local.yaml"),
  parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
instances:
  - index: 1
    source:
      mode: local
    localServices:
      - host-listener
`),
);

const state: State = {
  target: "latest-main",
  lockPath: "/tmp/latest-main.json",
  requiresGitHub: true,
  versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
  overrides: [],
  scenario,
  completedSteps: [],
  updatedAt: "2026-03-19T00:00:00.000Z",
};

const inheritedScenarioState: State = {
  ...state,
  overrides: [{ group: "coprocessor" }],
  scenario: resolveScenarioFile(
    path.join("/tmp", "two-of-two-inherit.yaml"),
    parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
`),
  ),
};

const multiChainHostContractsState: State = {
  ...state,
  overrides: [{ group: "host-contracts" }],
  scenario: testDefaultScenario({
    hostChains: [
      { key: "host", chainId: "12345", rpcPort: 8545 },
      { key: "chain-b", chainId: "67890", rpcPort: 8547 },
    ],
  }),
};

const relayerOverrideState: State = {
  ...state,
  overrides: [{ group: "relayer" }],
};

const gatewayContractsOverrideState: State = {
  ...state,
  overrides: [{ group: "gateway-contracts" }],
  scenario: testDefaultScenario(),
};

const envAndArgsScenarioState: State = {
  ...state,
  scenario: resolveScenarioFile(
    path.join("/tmp", "env-and-args.yaml"),
    parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
instances:
  - index: 1
    source:
      mode: local
    env:
      EXTRA_FLAG: enabled
    args:
      "*":
        - --error-sleep-max-secs=30
      host-listener:
        - --initial-block-time=2
`),
  ),
};

describe("render-compose", () => {
  test("keeps pinned base services image-only until a local override is requested", async () => {
    await withTempStateDir(async () => {
      const coprocessor = await loadMergedComposeDoc("coprocessor");
      const connector = await loadMergedComposeDoc("kms-connector");
      const hostSc = await loadMergedComposeDoc("host-sc");
      const gatewaySc = await loadMergedComposeDoc("gateway-sc");
      const gatewayMockedPayment = await loadMergedComposeDoc("gateway-mocked-payment");
      const relayer = await loadMergedComposeDoc("relayer");
      const testSuite = await loadMergedComposeDoc("test-suite");
      expect(coprocessor.services["coprocessor-host-listener"]?.build).toBeUndefined();
      expect(connector.services["kms-connector-gw-listener"]?.build).toBeUndefined();
      expect(hostSc.services["host-sc-deploy"]?.build).toBeUndefined();
      expect(gatewaySc.services["gateway-sc-deploy"]?.build).toBeUndefined();
      expect(gatewayMockedPayment.services["gateway-deploy-mocked-zama-oft"]?.build).toBeUndefined();
      expect(relayer.services.relayer?.build).toBeUndefined();
      expect(testSuite.services["test-suite-e2e-debug"]?.build).toBeUndefined();
    });
  });

  test("exports the active state dir to compose env", async () => {
    await withTempStateDir(async (stateDir) => {
      expect((await composeEnv("coprocessor")).FHEVM_STATE_DIR).toBe(stateDir);
    });
  });

  test("renders multi-instance coprocessor overrides with local poller siblings", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(state, stackSpecForState(state));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { image?: string; command?: string[] }>;
      };
      expect(Object.keys(doc.services)).toContain("coprocessor1-host-listener");
      expect(Object.keys(doc.services)).toContain("coprocessor1-host-listener-poller");
      expect(doc.services["coprocessor1-host-listener"]?.image).toContain(":fhevm-local-i1");
      expect(doc.services["coprocessor1-host-listener-poller"]?.image).toContain(":fhevm-local-i1");
      expect(String((doc.services["coprocessor-db-migration"]?.command as string[] | undefined)?.[0] ?? "")).toContain(
        "/initialize_db.sh && ( [ ! -x /insert_test_host_chain.sh ] || /insert_test_host_chain.sh )",
      );
    });
  });

  test("renders inherited two-of-two instances with local build tags when coprocessor build is active", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(inheritedScenarioState, stackSpecForState(inheritedScenarioState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown }>;
      };
      expect(doc.services["coprocessor-host-listener"]?.image).toContain(":fhevm-local-i0");
      expect(doc.services["coprocessor1-host-listener"]?.image).toContain(":fhevm-local-i1");
      expect(doc.services["coprocessor-host-listener"]?.build).toBeTruthy();
      expect(doc.services["coprocessor1-host-listener"]?.build).toBeTruthy();
    });
  });

  test("keeps local host-contract builds on extra host chains", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("host-sc")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor-chain-b.0"), "\n");
      await writeFile(envPath("host-sc"), "\n");
      await writeFile(envPath("host-sc-chain-b"), "\n");
      await generateComposeOverrides(multiChainHostContractsState, stackSpecForState(multiChainHostContractsState));
      const doc = YAML.parse(await readFile(composePath("host-sc-chain-b"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown }>;
      };
      expect(doc.services["host-sc-chain-b-deploy"]?.image).toContain(":fhevm-local");
      expect(doc.services["host-sc-chain-b-deploy"]?.build).toBeTruthy();
      expect(doc.services["host-sc-chain-b-add-pausers"]?.image).toContain(":fhevm-local");
      expect(doc.services["host-sc-chain-b-add-pausers"]?.build).toBeTruthy();
      expect(doc.services["host-sc-chain-b-trigger-keygen"]).toBeUndefined();
      expect(doc.services["host-sc-chain-b-trigger-crsgen"]).toBeUndefined();
    });
  });

  test("keeps legacy gateway trigger services in local gateway overrides", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await generateComposeOverrides(gatewayContractsOverrideState, stackSpecForState(gatewayContractsOverrideState));
      const doc = YAML.parse(await readFile(composePath("gateway-sc"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown }>;
      };
      expect(doc.services["gateway-sc-trigger-keygen"]?.image).toContain(":fhevm-local");
      expect(doc.services["gateway-sc-trigger-keygen"]?.build).toBeTruthy();
      expect(doc.services["gateway-sc-trigger-crsgen"]?.image).toContain(":fhevm-local");
      expect(doc.services["gateway-sc-trigger-crsgen"]?.build).toBeTruthy();
    });
  });

  test("retags relayer services for local builds when the relayer group is overridden", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(relayerOverrideState, stackSpecForState(relayerOverrideState));
      const doc = YAML.parse(await readFile(composePath("relayer"), "utf8")) as {
        services: Record<string, { image?: string; build?: { context?: string; dockerfile?: string } }>;
      };
      expect(doc.services["relayer-db-migration"]?.image).toContain(":fhevm-local");
      expect(doc.services["relayer-db-migration"]?.build?.dockerfile).toContain(
        "relayer/docker/relayer-migrate/Dockerfile",
      );
      expect(doc.services["relayer"]?.image).toContain(":fhevm-local");
      expect(doc.services["relayer"]?.build?.dockerfile).toContain("relayer/docker/relayer/Dockerfile");
    });
  });

  test("uses the first explicit chain key for default host-contract address mounts", async () => {
    const nonHostDefaultState: State = {
      ...state,
      scenario: testDefaultScenario({
        hostChains: [
          { key: "chain-a", chainId: "12345", rpcPort: 9545 },
          { key: "chain-b", chainId: "67890", rpcPort: 10545 },
        ],
      }),
    };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("host-sc")), { recursive: true });
      await writeFile(envPath("host-sc"), "HOST_ADDRESS_DIR=chain-a\n");
      await writeFile(envPath("host-sc-chain-b"), "HOST_ADDRESS_DIR=chain-b\n");
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor-chain-b.0"), "\n");
      await generateComposeOverrides(nonHostDefaultState, stackSpecForState(nonHostDefaultState));
      const env = await composeEnv("host-sc");
      const hostAddressDir = (env as Record<string, string>).HOST_ADDRESS_DIR ?? "host";
      const template = YAML.parse(
        await readFile(path.join(TEMPLATE_COMPOSE_DIR, "host-sc-docker-compose.yml"), "utf8"),
      ) as { services: Record<string, { volumes?: string[] }> };
      const defaultMount = String(template.services["host-sc-deploy"]?.volumes?.[0] ?? "").replace(
        /\$\{HOST_ADDRESS_DIR:-host\}/g,
        hostAddressDir,
      );
      const extra = YAML.parse(await readFile(composePath("host-sc-chain-b"), "utf8")) as {
        services: Record<string, { volumes?: string[] }>;
      };
      expect(defaultMount).toContain("/addresses/chain-a:/app/addresses");
      expect(extra.services["host-sc-chain-b-deploy"]?.volumes?.[0]).toContain("/addresses/chain-b:/app/addresses");
    });
  });

  test("host-sc deploy service reads KMSGeneration args from env", async () => {
    const template = YAML.parse(
      await readFile(path.join(TEMPLATE_COMPOSE_DIR, "host-sc-docker-compose.yml"), "utf8"),
    ) as { services: Record<string, { command?: string[] }> };

    const cmd = (template.services["host-sc-deploy"]?.command ?? []).join(" ");
    expect(cmd).toContain("task:deployAllHostContracts");
    expect(cmd).toContain("$${HOST_SC_DEPLOY_KMS_GENERATION_ARGS}");
  });

  test("merges instance env into list-form service environments without dropping KEY_ID", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "FHE_KEY_ID=deadbeef\n");
      await generateComposeOverrides(envAndArgsScenarioState, stackSpecForState(envAndArgsScenarioState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { environment?: Record<string, string> }>;
      };
      expect(doc.services["coprocessor1-db-migration"]?.environment).toMatchObject({
        KEY_ID: "deadbeef",
        EXTRA_FLAG: "enabled",
      });
    });
  });

  test("composes wildcard and service-specific scenario args", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(envAndArgsScenarioState, stackSpecForState(envAndArgsScenarioState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { command?: string[] }>;
      };
      expect(doc.services["coprocessor1-host-listener"]?.command).toEqual(
        expect.arrayContaining(["--error-sleep-max-secs=30", "--initial-block-time=2"]),
      );
    });
  });
});

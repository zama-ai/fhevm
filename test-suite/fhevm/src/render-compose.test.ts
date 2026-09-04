import { describe, expect, test } from "bun:test";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import YAML from "yaml";

import { generateComposeOverrides, loadMergedComposeDoc, serviceNameList } from "./generate/compose";
import { TEMPLATE_COMPOSE_DIR, composePath, envPath } from "./layout";
import { presetBundle } from "./resolve/target";
import {
  loadCoprocessorScenario,
  parseBlueGreenScenario,
  parseCoprocessorScenario,
  resolveBlueGreenScenario,
  resolveScenarioFile,
} from "./scenario/resolve";
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

const listenerCoreOverrideState: State = {
  ...state,
  overrides: [{ group: "listener-core" }],
  scenario: testDefaultScenario(),
};

const gatewayContractsOverrideState: State = {
  ...state,
  overrides: [{ group: "gateway-contracts" }],
  scenario: testDefaultScenario(),
};

const testSuiteOverrideState: State = {
  ...state,
  overrides: [{ group: "test-suite" }],
  scenario: testDefaultScenario(),
};

const kmsConnectorOverrideState: State = {
  ...state,
  overrides: [{ group: "kms-connector" }],
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
      const listenerCore = await loadMergedComposeDoc("listener-core");
      const testSuite = await loadMergedComposeDoc("test-suite");
      expect(coprocessor.services["coprocessor-host-listener"]?.build).toBeUndefined();
      expect(connector.services["kms-connector-gw-listener"]?.build).toBeUndefined();
      expect(hostSc.services["host-sc-deploy"]?.build).toBeUndefined();
      expect(gatewaySc.services["gateway-sc-deploy"]?.build).toBeUndefined();
      expect(gatewayMockedPayment.services["gateway-deploy-mocked-zama-oft"]?.build).toBeUndefined();
      expect(relayer.services.relayer?.build).toBeUndefined();
      expect(listenerCore.services["listener-publisher-for-anvil"]?.build).toBeUndefined();
      expect(testSuite.services["test-suite-e2e-debug"]?.build).toBeUndefined();
    });
  });

  test("exports the active state dir to compose env", async () => {
    await withTempStateDir(async (stateDir) => {
      expect((await composeEnv("coprocessor")).FHEVM_STATE_DIR).toBe(stateDir);
    });
  });

  test("persists kms-core private vault across container recreates", async () => {
    const doc = await loadMergedComposeDoc("core");
    const volumes = doc.services["kms-core"]?.volumes as string[] | undefined;
    expect(doc.services["kms-core"]?.user).toBe("root");
    expect(volumes).toContain("fhevm_kms_core_keys:/app/kms/core/service/keys");
  });

  test("keeps localhost MinIO URLs reachable from the e2e container", async () => {
    const doc = await loadMergedComposeDoc("test-suite");
    expect(doc.services["test-suite-e2e-debug"]?.network_mode).toBe("container:fhevm-minio");
  });

  test("renders listener-core local override for the publisher only", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await generateComposeOverrides(listenerCoreOverrideState, stackSpecForState(listenerCoreOverrideState));
      const doc = YAML.parse(await readFile(composePath("listener-core"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown }>;
      };
      expect(doc.services["listener-publisher-for-anvil"]?.image).toContain(":fhevm-local");
      expect(doc.services["listener-publisher-for-anvil"]?.build).toBeTruthy();
      expect(doc.services["listener-redis"]).toBeUndefined();
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
        "/initialize_db.sh",
      );
    });
  });

  test("gives every coprocessor runtime service a bounded on-failure restart policy", async () => {
    await withTempStateDir(async () => {
      const doc = await loadMergedComposeDoc("coprocessor");
      const runtime = [
        "coprocessor-host-listener",
        "coprocessor-host-listener-poller",
        "coprocessor-host-listener-consumer",
        "coprocessor-gw-listener",
        "coprocessor-tfhe-worker",
        "coprocessor-zkproof-worker",
        "coprocessor-sns-worker",
        "coprocessor-transaction-sender",
        "coprocessor-consensus-detector",
        "coprocessor-upgrade-controller",
      ];
      // These implement exit-for-restart: fatal error, non-zero exit, and they
      // expect to be brought back. Without a policy the stack degraded
      // permanently on any fatal path (L-5).
      for (const service of runtime) {
        expect(doc.services[service]?.restart, `${service} must recover from a fatal exit`).toBe("on-failure:10");
      }
      // `on-failure`, never `unless-stopped`: a clean exit 0 is a service that
      // meant to finish, and restarting it would loop. Bounded, so a genuinely
      // broken deploy stops instead of hiding in a crash loop.
      for (const service of runtime) {
        expect(String(doc.services[service]?.restart)).toStartWith("on-failure");
      }
      // One-shots must stay one-shot: readiness waits for db-migration to
      // reach `complete`, and a failing migration should surface at once
      // rather than retry.
      expect(doc.services["coprocessor-db-migration"]?.restart).toBeUndefined();
    });
  });

  test("exposes tfhe-worker metrics on the base service and on every clone", async () => {
    await withTempStateDir(async () => {
      // The run-validity gates read
      // `coprocessor_worker_deferred_transactions_current` from each operator's
      // worker and refuse to guess when they cannot. That only works if the
      // flag survives clone generation, which rewrites the command.
      const base = await loadMergedComposeDoc("coprocessor");
      const baseCommand = (base.services["coprocessor-tfhe-worker"]?.command ?? []) as string[];
      expect(baseCommand.some((arg) => String(arg).startsWith("--metrics-addr="))).toBe(true);

      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(state, stackSpecForState(state));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { command?: string[] }>;
      };
      const clone = (doc.services["coprocessor1-tfhe-worker"]?.command ?? []) as string[];
      expect(
        clone.some((arg) => String(arg).startsWith("--metrics-addr=")),
        "operator 1's worker must expose metrics or its validity gate cannot be evaluated",
      ).toBe(true);
    });
  });

  test("carries the restart policy onto per-operator clones", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(state, stackSpecForState(state));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { restart?: string }>;
      };
      // Operator 1 is the one the failure matrix injects faults into, so it is
      // the operator whose recovery matters most.
      expect(doc.services["coprocessor1-host-listener"]?.restart).toBe("on-failure:10");
      expect(doc.services["coprocessor1-host-listener-poller"]?.restart).toBe("on-failure:10");
    });
  });

  test("does not request host-listener consumer services for legacy coprocessor bundles", () => {
    const legacyState: State = {
      ...state,
      versions: {
        ...state.versions,
        env: {
          ...state.versions.env,
          COPROCESSOR_HOST_LISTENER_VERSION: "v0.12.2",
        },
      },
    };

    const services = serviceNameList(legacyState, "coprocessor");
    expect(services).not.toContain("coprocessor-host-listener-consumer");
    expect(services).not.toContain("coprocessor1-host-listener-consumer");
  });

  test("does not request consensus-detector or upgrade-controller services for legacy coprocessor bundles", () => {
    const legacyState: State = {
      ...state,
      versions: {
        ...state.versions,
        env: {
          ...state.versions.env,
          COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "v0.12.2",
          COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "v0.12.2",
        },
      },
    };

    const services = serviceNameList(legacyState, "coprocessor");
    expect(services).not.toContain("coprocessor-consensus-detector");
    expect(services).not.toContain("coprocessor-upgrade-controller");
    expect(services).not.toContain("coprocessor1-consensus-detector");
    expect(services).not.toContain("coprocessor1-upgrade-controller");

    const modernServices = serviceNameList(state, "coprocessor");
    expect(modernServices).toContain("coprocessor-consensus-detector");
    expect(modernServices).toContain("coprocessor-upgrade-controller");
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
      const args = (doc.services["coprocessor-host-listener"]?.build as { args?: Record<string, string> })?.args;
      expect(args?.COPROCESSOR_RUNTIME_BASE_IMAGE).toBeUndefined();
      expect(args?.COPROCESSOR_DB_MIGRATION_RUNTIME_BASE_IMAGE).toBeUndefined();
    });
  });

  test("routes every locally-built coprocessor target through the explicit public E2E runtime bases", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      const publicRuntimeState: State = { ...inheritedScenarioState, e2ePublicRuntime: true };
      await generateComposeOverrides(publicRuntimeState, stackSpecForState(publicRuntimeState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { build?: { args?: Record<string, string> } }>;
      };

      for (const [name, service] of Object.entries(doc.services)) {
        expect(service.build?.args?.COPROCESSOR_RUNTIME_BASE_IMAGE, name).toBe("e2e-public-runtime");
        expect(service.build?.args?.COPROCESSOR_DB_MIGRATION_RUNTIME_BASE_IMAGE, name).toBe(
          "e2e-public-db-migration-runtime",
        );
      }
    });
  });

  test("routes only local KMS connector runtime services through the public E2E runtime base", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      const publicRuntimeState: State = { ...kmsConnectorOverrideState, e2ePublicRuntime: true };
      await generateComposeOverrides(publicRuntimeState, stackSpecForState(publicRuntimeState));
      const doc = YAML.parse(await readFile(composePath("kms-connector"), "utf8")) as {
        services: Record<string, { build?: { args?: Record<string, string> } }>;
      };

      for (const name of ["kms-connector-gw-listener", "kms-connector-kms-worker", "kms-connector-tx-sender"]) {
        expect(doc.services[name]?.build?.args?.KMS_CONNECTOR_RUNTIME_BASE_IMAGE, name).toBe("e2e-public-runtime");
        expect(doc.services[name]?.build?.args?.BUILD_ID, name).toMatch(/^(?:[0-9a-f]{7,}|unknown)$/);
      }
      expect(doc.services["kms-connector-db-migration"]?.build?.args?.KMS_CONNECTOR_RUNTIME_BASE_IMAGE).toBeUndefined();
      expect(doc.services["kms-connector-db-migration"]?.build?.args?.BUILD_ID).toBeUndefined();
    });
  });

  test("keeps the certified KMS connector runtime base when the public E2E flag is absent", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(kmsConnectorOverrideState, stackSpecForState(kmsConnectorOverrideState));
      const doc = YAML.parse(await readFile(composePath("kms-connector"), "utf8")) as {
        services: Record<string, { build?: { args?: Record<string, string> } }>;
      };

      for (const name of ["kms-connector-gw-listener", "kms-connector-kms-worker", "kms-connector-tx-sender"]) {
        expect(doc.services[name]?.build?.args?.KMS_CONNECTOR_RUNTIME_BASE_IMAGE, name).toBeUndefined();
        expect(doc.services[name]?.build?.args?.BUILD_ID, name).toMatch(/^(?:[0-9a-f]{7,}|unknown)$/);
      }
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
        services: Record<string, { image?: string; build?: unknown; command?: string[] }>;
      };
      expect(doc.services["gateway-sc-trigger-keygen"]?.image).toContain(":fhevm-local");
      expect(doc.services["gateway-sc-trigger-keygen"]?.build).toBeTruthy();
      expect(doc.services["gateway-sc-trigger-keygen"]?.command?.[0]).toContain("${KEYGEN_PARAMS_TYPE:-0}");
      expect(doc.services["gateway-sc-trigger-crsgen"]?.image).toContain(":fhevm-local");
      expect(doc.services["gateway-sc-trigger-crsgen"]?.build).toBeTruthy();
      expect(doc.services["gateway-sc-trigger-crsgen"]?.command?.[0]).toContain("${KEYGEN_PARAMS_TYPE:-0}");
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

  test("does not duplicate the test-suite Docker socket group in a local build override", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await generateComposeOverrides(testSuiteOverrideState, stackSpecForState(testSuiteOverrideState));
      const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown; group_add?: unknown }>;
      };
      const testSuite = doc.services["test-suite-e2e-debug"];
      expect(testSuite?.image).toContain(":fhevm-local");
      expect(testSuite?.build).toBeTruthy();
      expect(testSuite?.group_add).toBeUndefined();
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
    expect(cmd).toContain("$${HOST_SC_DEPLOY_PROTOCOL_CONFIG_ARGS}");
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

  // Guards the shipped scenario file itself, not a copy of it inline: the
  // point of the heterogeneous-scheduling topology is that the three operators
  // come up scheduling DIFFERENTLY, and a silent regression to a uniform fleet
  // would leave the byte-consensus gate passing while asserting nothing.
  test("heterogeneous-scheduling scenario renders three distinct tfhe-worker configurations", async () => {
    const heterogeneous = resolveScenarioFile(
      path.join("/tmp", "three-of-three-heterogeneous-scheduling.yaml"),
      await loadCoprocessorScenario("three-of-three-heterogeneous-scheduling"),
    );
    const heterogeneousState: State = { ...state, scenario: heterogeneous };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      for (const name of ["coprocessor", "coprocessor.1", "coprocessor.2"]) {
        await writeFile(envPath(name), "\n");
      }
      await generateComposeOverrides(heterogeneousState, stackSpecForState(heterogeneousState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { command?: string[]; environment?: Record<string, string> }>;
      };

      const workers = ["coprocessor-tfhe-worker", "coprocessor1-tfhe-worker", "coprocessor2-tfhe-worker"];
      const windows = workers.map(
        (name) =>
          doc.services[name]?.command?.find((argument) => argument.startsWith("--work-items-batch-size=")) ?? "missing",
      );
      const chains = workers.map(
        (name) =>
          doc.services[name]?.command?.find((argument) => argument.startsWith("--dependence-chains-per-batch=")) ??
          "missing",
      );

      expect(windows).toEqual([
        "--work-items-batch-size=100",
        "--work-items-batch-size=1",
        "--work-items-batch-size=200",
      ]);
      expect(chains).toEqual([
        "--dependence-chains-per-batch=20",
        "--dependence-chains-per-batch=1",
        "--dependence-chains-per-batch=64",
      ]);

      // The compose template already carries `--work-items-batch-size=10`, so
      // an override that appended instead of replacing would leave the worker
      // with the flag twice. Assert each appears exactly once.
      for (const name of workers) {
        const command = doc.services[name]?.command ?? [];
        expect(command.filter((argument) => argument.startsWith("--work-items-batch-size=")).length).toBe(1);
      }

      // The two DCID booleans have no command-line route that can turn them
      // off, so they must arrive as environment entries on the right operators
      // and nowhere else.
      expect(doc.services["coprocessor1-tfhe-worker"]?.environment?.FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION).toBe("false");
      expect(doc.services["coprocessor2-tfhe-worker"]?.environment?.FHEVM_DCID_BATCH_EXECUTION).toBe("false");
      expect(doc.services["coprocessor-tfhe-worker"]?.environment?.FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION).toBeUndefined();
      expect(doc.services["coprocessor-tfhe-worker"]?.environment?.FHEVM_DCID_BATCH_EXECUTION).toBeUndefined();

      // No operator may invert its own pair. The adaptive window gives each
      // acquired chain ceil(window / acquired-chains) transactions and turns
      // itself off once more chains are acquired than the window admits, so an
      // operator whose window is below its chain count schedules
      // non-adaptively -- while this scenario presents it as a scheduling
      // variant of the adaptive baseline. Asserted as a property rather than
      // left to the literals above, because the literals are exactly what went
      // wrong: operator 0 shipped as 10/20 for a while, under a comment calling
      // it the adaptive default.
      windows.forEach((window, index) => {
        const windowValue = Number.parseInt(window.split("=")[1] ?? "", 10);
        const chainValue = Number.parseInt(chains[index].split("=")[1] ?? "", 10);
        expect(Number.isInteger(windowValue) && Number.isInteger(chainValue)).toBe(true);
        expect(
          windowValue >= chainValue,
          `operator ${index} has window ${windowValue} below its ${chainValue} chains, which disables ` +
            "the adaptive window under load",
        ).toBe(true);
      });

      // The whole point: no two operators schedule alike.
      const classes = workers.map((name) => JSON.stringify(doc.services[name]?.command));
      expect(new Set(classes).size).toBe(workers.length);
    });
  });

  // The dual-Anvil scenario was re-derived from the wave2 original, which was
  // written against binaries that no longer exist in this shape. Two flags it
  // used were removed with the RFC 011 settlement machinery, and passing an
  // unknown flag makes the listener exit at startup -- every operator would
  // crash-loop and the fork suite would fail for a reason unrelated to forks.
  test("fork scenario routes one instance to the fork and passes no retired listener flags", async () => {
    const forkScenario = resolveScenarioFile(
      path.join("/tmp", "three-of-three-fork.yaml"),
      await loadCoprocessorScenario("three-of-three-fork"),
    );
    const forkState: State = { ...state, scenario: forkScenario };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      for (const name of ["coprocessor", "coprocessor.1", "coprocessor.2"]) {
        await writeFile(envPath(name), "\n");
      }
      await generateComposeOverrides(forkState, stackSpecForState(forkState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { command?: string[]; environment?: Record<string, string> }>;
      };

      // Exactly one instance follows the fork.
      const routed = Object.entries(doc.services).filter(([, service]) =>
        Object.values(service.environment ?? {}).some((value) => String(value).includes("fork-anvil")),
      );
      expect(routed.length).toBeGreaterThan(0);
      for (const [name] of routed) expect(name.startsWith("coprocessor2-")).toBe(true);
      expect(doc.services["coprocessor2-host-listener"]?.environment?.RPC_HTTP_URL).toBe("http://fork-anvil:8546");
      expect(doc.services["coprocessor-host-listener"]?.environment?.RPC_HTTP_URL).toBeUndefined();

      // Retired flags must not come back. `--settlement-finality-lag` no
      // longer exists on either binary, and the poller never accepted
      // `--reorg-maximum-duration-in-blocks`.
      for (const [name, service] of Object.entries(doc.services)) {
        const command = (service.command ?? []).map(String);
        expect(
          command.some((argument) => argument.startsWith("--settlement-finality-lag")),
          `${name} passes a flag no current binary accepts`,
        ).toBe(false);
        if (name.endsWith("host-listener-poller")) {
          expect(
            command.some((argument) => argument.startsWith("--reorg-maximum-duration-in-blocks")),
            `${name} passes a flag the poller does not accept`,
          ).toBe(false);
        }
      }
    });
  });

  test("blue-green scenario emits coprocessor-gcs-* services from local HEAD build", async () => {
    const blueGreenScenario = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-test.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
gcs:
  source: { mode: local }
  stackVersion: "0.15.0"
`),
    );
    const bgState: State = { ...state, scenario: blueGreenScenario };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await generateComposeOverrides(bgState, stackSpecForState(bgState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<
          string,
          { container_name?: string; build?: { args?: Record<string, string> } }
        >;
      };
      // BCS side keeps the base `coprocessor-*` names.
      expect(doc.services["coprocessor-host-listener"]?.container_name).toBe("coprocessor-host-listener");
      // GCS side is layered on with `coprocessor-gcs-*` prefix.
      expect(doc.services["coprocessor-gcs-host-listener"]?.container_name).toBe(
        "coprocessor-gcs-host-listener",
      );
      expect(doc.services["coprocessor-gcs-upgrade-controller"]?.container_name).toBe(
        "coprocessor-gcs-upgrade-controller",
      );
      // GCS builds from local HEAD compiled at the newer version (build arg enables the override feature).
      expect(doc.services["coprocessor-gcs-host-listener"]?.build).toBeDefined();
      expect(doc.services["coprocessor-gcs-host-listener"]?.build?.args?.BUILD_STACK_VERSION).toBe("0.15.0");
      // GCS reuses BCS's db-migration — no `coprocessor-gcs-db-migration`.
      expect(doc.services["coprocessor-gcs-db-migration"]).toBeUndefined();
    });
  });

  test("multi-operator blue-green emits BCS + GCS fleets per operator with correct prefixes", async () => {
    const multiOpBlueGreen = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-2op.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
topology:
  count: 2
  threshold: 2
gcs:
  source: { mode: local }
  stackVersion: "0.15.0"
`),
    );
    const bgState: State = { ...state, scenario: multiOpBlueGreen };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(bgState, stackSpecForState(bgState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { container_name?: string }>;
      };
      // Operator 0: BCS as `coprocessor-*`, GCS as `coprocessor-gcs-*`.
      expect(doc.services["coprocessor-host-listener"]?.container_name).toBe("coprocessor-host-listener");
      expect(doc.services["coprocessor-gcs-host-listener"]?.container_name).toBe(
        "coprocessor-gcs-host-listener",
      );
      // Operator 1: BCS as `coprocessor1-*`, GCS as `coprocessor1-gcs-*`.
      expect(doc.services["coprocessor1-host-listener"]?.container_name).toBe(
        "coprocessor1-host-listener",
      );
      expect(doc.services["coprocessor1-gcs-host-listener"]?.container_name).toBe(
        "coprocessor1-gcs-host-listener",
      );
      expect(doc.services["coprocessor1-gcs-upgrade-controller"]?.container_name).toBe(
        "coprocessor1-gcs-upgrade-controller",
      );
      // Each operator has its own db-migration (BCS); GCS reuses it.
      expect(doc.services["coprocessor-db-migration"]).toBeDefined();
      expect(doc.services["coprocessor1-db-migration"]).toBeDefined();
      expect(doc.services["coprocessor-gcs-db-migration"]).toBeUndefined();
      expect(doc.services["coprocessor1-gcs-db-migration"]).toBeUndefined();
    });
  });

  test("blue-green with bcs.source.mode=registry pins BCS to previous-release images except db-migration", async () => {
    const realUpgradeScenario = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-real-test.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
bcs:
  source:
    mode: registry
    tag: v0.13.0
gcs:
  source: { mode: local }
  stackVersion: "0.15.0"
`),
    );
    const bgState: State = { ...state, scenario: realUpgradeScenario };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await generateComposeOverrides(bgState, stackSpecForState(bgState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<
          string,
          {
            container_name?: string;
            image?: string;
            build?: { args?: Record<string, string> } | undefined;
            environment?: Record<string, string>;
          }
        >;
      };
      // BCS runtime services are pinned to the registry tag.
      expect(doc.services["coprocessor-host-listener"]?.image).toContain(":v0.13.0");
      expect(doc.services["coprocessor-tfhe-worker"]?.image).toContain(":v0.13.0");
      expect(doc.services["coprocessor-sns-worker"]?.image).toContain(":v0.13.0");
      expect(doc.services["coprocessor-zkproof-worker"]?.image).toContain(":v0.13.0");
      expect(doc.services["coprocessor-gw-listener"]?.image).toContain(":v0.13.0");
      expect(doc.services["coprocessor-transaction-sender"]?.image).toContain(":v0.13.0");
      // Registry-mode services should NOT carry a build spec.
      expect(doc.services["coprocessor-tfhe-worker"]?.build).toBeUndefined();
      // db-migration is force-local so GCS gets the v0.14 schema.
      expect(doc.services["coprocessor-db-migration"]?.build).toBeDefined();
      expect(doc.services["coprocessor-db-migration"]?.image).not.toContain(":v0.13.0");
      expect(doc.services["coprocessor-gcs-tfhe-worker"]?.build).toBeDefined();
      expect(
        doc.services["coprocessor-host-listener"]?.environment
          ?.CANONICAL_PROTOCOL_CONFIG_CHAIN_ID,
      ).toBeUndefined();
      expect(
        doc.services["coprocessor-host-listener-poller"]?.environment
          ?.CANONICAL_PROTOCOL_CONFIG_CHAIN_ID,
      ).toBeUndefined();
      expect(
        doc.services["coprocessor-gcs-host-listener"]?.environment
          ?.CANONICAL_PROTOCOL_CONFIG_CHAIN_ID,
      ).toBeUndefined();
      expect(doc.services["coprocessor-gcs-upgrade-controller"]?.container_name).toBe(
        "coprocessor-gcs-upgrade-controller",
      );
    });
  });

  test("blue-green shims the BCS fleet from its pinned tag, not the resolved bundle", async () => {
    const pinnedBcsScenario = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-pinned-bcs.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
bcs:
  source:
    mode: registry
    tag: v0.14.0-7
gcs:
  source: { mode: local }
  stackVersion: "0.15.0"
`),
    );
    const bgState: State = { ...state, scenario: pinnedBcsScenario };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "BUCKET_NAME=coproc-0\n");
      await generateComposeOverrides(bgState, stackSpecForState(bgState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { command?: string[] }>;
      };
      // BCS runs the v0.14 image, which predates the unified --bucket-name flag,
      // even though the resolved bundle points at HEAD.
      const bcsCommand = doc.services["coprocessor-sns-worker"]?.command ?? [];
      expect(bcsCommand).toContain("--bucket-name-ct128=coproc-0");
      expect(bcsCommand).toContain("--bucket-name-ct64=coproc-0");
      expect(bcsCommand).not.toContain("--bucket-name=coproc-0");
      // GCS builds from the working tree, so it keeps the modern flag.
      const gcsCommand = doc.services["coprocessor-gcs-sns-worker"]?.command ?? [];
      expect(gcsCommand).toContain("--bucket-name=coproc-0");
      expect(gcsCommand.filter((arg) => arg.startsWith("--bucket-name-"))).toEqual([]);
    });
  });

  test("a GPU-tagged worker image gets the driver and devices; a CPU-tagged one does not", async () => {
    // The published GPU images are tagged <revision>-cuda<version>-sm<arch>.
    // Pinning one worker to a GPU tag and leaving another on the CPU tag proves
    // both halves in one render: the GPU service must be given the nvidia
    // runtime's env and a device reservation, and the CPU service must be left
    // alone -- a CPU run that demands a GPU cannot start on a host without one.
    //
    // Why this is needed at all: measured on a host whose docker default runtime
    // is already `nvidia`, a container with NVIDIA_VISIBLE_DEVICES unset sees
    // zero /dev/nvidia* devices. A GPU image would run, serve, and compute
    // nothing on the GPU -- the silent failure this wiring exists to prevent.
    const gpuState: State = {
      ...state,
      versions: {
        ...state.versions,
        env: {
          ...state.versions.env,
          COPROCESSOR_TFHE_WORKER_VERSION: "921b69113-cuda12.8-sm90",
          COPROCESSOR_SNS_WORKER_VERSION: "921b69113",
        },
      },
    };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(gpuState, stackSpecForState(gpuState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<
          string,
          {
            image?: string;
            environment?: Record<string, string>;
            deploy?: { resources?: { reservations?: { devices?: unknown[] } } };
          }
        >;
      };

      const gpu = doc.services["coprocessor-tfhe-worker"];
      // The override deliberately keeps the ${...} placeholder; the GPU wiring is
      // keyed on the resolved version, which is what compose will substitute.
      expect(gpu?.image).toContain("${COPROCESSOR_TFHE_WORKER_VERSION}");
      expect(gpu?.environment?.NVIDIA_VISIBLE_DEVICES).toBe("all");
      expect(gpu?.environment?.NVIDIA_DRIVER_CAPABILITIES).toBe("compute,utility");
      expect(gpu?.deploy?.resources?.reservations?.devices).toHaveLength(1);

      const cpu = doc.services["coprocessor-sns-worker"];
      expect(cpu?.image).toContain("${COPROCESSOR_SNS_WORKER_VERSION}");
      expect(cpu?.environment?.NVIDIA_VISIBLE_DEVICES).toBeUndefined();
      expect(cpu?.deploy).toBeUndefined();
    });
  });

});

import { describe, expect, test } from "bun:test";
import { statSync } from "node:fs";
import { mkdir, mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import YAML from "yaml";

import {
  blueGreenServiceNames,
  dockerSocketRuntime,
  generateComposeOverrides,
  loadMergedComposeDoc,
  serviceNameList,
} from "./generate/compose";
import { COMPOSE_OUT_DIR, TEMPLATE_COMPOSE_DIR, composePath, envPath } from "./layout";
import { presetBundle } from "./resolve/target";
import {
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

// Single-instance, no local builds: the test-suite override then contains only whatever
// the generator adds on its own, which is the host Docker socket wiring.
const socketWiringState: State = {
  ...state,
  overrides: [],
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

/** Runs a test body with `DOCKER_HOST` pinned to a known value, restoring it after. */
const withDockerHost = async <T>(value: string | undefined, run: () => Promise<T>) => {
  const previous = process.env.DOCKER_HOST;
  if (value === undefined) {
    delete process.env.DOCKER_HOST;
  } else {
    process.env.DOCKER_HOST = value;
  }
  try {
    return await run();
  } finally {
    if (previous === undefined) {
      delete process.env.DOCKER_HOST;
    } else {
      process.env.DOCKER_HOST = previous;
    }
  }
};

/** Runs a test body against a real, listening unix socket in a temporary directory. */
const withUnixSocket = async <T>(run: (socketPath: string) => Promise<T>) => {
  const dir = await mkdtemp(path.join(tmpdir(), "fhevm-docker-sock-"));
  const socketPath = path.join(dir, "docker.sock");
  const server = Bun.listen({ unix: socketPath, socket: { data() {} } });
  try {
    return await run(socketPath);
  } finally {
    server.stop(true);
    await rm(dir, { recursive: true, force: true });
  }
};

/** Collects every service in every generated compose override, keyed by file and name. */
const generatedServices = async () => {
  const entries = await readdir(COMPOSE_OUT_DIR);
  const services: Array<[string, Record<string, unknown>]> = [];
  for (const entry of entries.filter((name) => name.endsWith(".yml"))) {
    const doc = YAML.parse(await readFile(path.join(COMPOSE_OUT_DIR, entry), "utf8")) as {
      services?: Record<string, Record<string, unknown>>;
    };
    for (const [name, service] of Object.entries(doc.services ?? {})) {
      services.push([`${entry}:${name}`, service]);
    }
  }
  return services;
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

  test("renders a test-suite local build override without daemon access on a socketless host", async () => {
    await withDockerHost("unix:///nonexistent/docker.sock", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await generateComposeOverrides(testSuiteOverrideState, stackSpecForState(testSuiteOverrideState));
        const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
          services: Record<string, { image?: string; build?: unknown; volumes?: unknown; group_add?: unknown }>;
        };
        const testSuite = doc.services["test-suite-e2e-debug"];
        expect(testSuite?.image).toContain(":fhevm-local");
        expect(testSuite?.build).toBeTruthy();
        expect(testSuite?.volumes).toBeUndefined();
        expect(testSuite?.group_add).toBeUndefined();
      });
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

  test("blue-green can run Green from a published image tag", async () => {
    const blueGreenScenario = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-registry-gcs.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
hostChains:
  - key: host
    chainId: "12345"
    rpcPort: 8545
  - key: chain-b
    chainId: "67890"
    rpcPort: 8547
bcs:
  source: { mode: registry, tag: v0.14.0-10 }
gcs:
  source: { mode: registry, tag: target-sha }
  stackVersion: "0.15.0"
`),
    );
    const bgState: State = { ...state, scenario: blueGreenScenario };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor-chain-b.0"), "\n");
      await generateComposeOverrides(bgState, stackSpecForState(bgState));
      const primary = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown }>;
      };
      const secondary = YAML.parse(await readFile(composePath("coprocessor-chain-b"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown }>;
      };

      expect(primary.services["coprocessor-db-migration"]?.image).toEndWith(":target-sha");
      expect(primary.services["coprocessor-db-migration"]?.build).toBeUndefined();
      expect(primary.services["coprocessor-gcs-host-listener"]?.image).toEndWith(":target-sha");
      expect(primary.services["coprocessor-gcs-host-listener"]?.build).toBeUndefined();
      expect(secondary.services["coprocessor-gcs-host-listener-chain-b"]?.image).toEndWith(":target-sha");
      expect(secondary.services["coprocessor-gcs-host-listener-chain-b"]?.build).toBeUndefined();
    });
  });

  test("deferred Green is omitted from startup until explicitly requested", () => {
    const deferredScenario = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-deferred-test.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
gcs:
  source: { mode: local }
  stackVersion: "0.15.0"
  deferredStart: true
`),
    );
    const deferredState: State = { ...state, scenario: deferredScenario };

    const initialServices = blueGreenServiceNames(deferredState, { includeMigration: true });
    expect(initialServices.some((service) => service.includes("-gcs-"))).toBe(false);

    const explicitGreenServices = blueGreenServiceNames(deferredState, {
      includeMigration: false,
      includeDeferredGreen: true,
    });
    expect(explicitGreenServices).toContain("coprocessor-gcs-host-listener");
    expect(explicitGreenServices).toContain("coprocessor-gcs-upgrade-controller");
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

  test("blue-green keeps the release command shape for a SHA-tagged BCS hotfix", async () => {
    const hotfixScenario = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-hotfix-bcs.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
bcs:
  source:
    mode: registry
    tag: v0.14.0-10
gcs:
  source: { mode: local }
  stackVersion: "0.15.0"
`),
    );
    hotfixScenario.bcs.source = { mode: "registry", tag: "04fb072", compatTag: "v0.14.0-10" };
    const bgState: State = { ...state, scenario: hotfixScenario };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "BUCKET_NAME=coproc-0\n");
      await generateComposeOverrides(bgState, stackSpecForState(bgState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { command?: string[]; image?: string }>;
      };
      const bcs = doc.services["coprocessor-sns-worker"] ?? {};
      expect(bcs.image).toEndWith(":04fb072");
      expect(bcs.command).toContain("--bucket-name-ct128=coproc-0");
      expect(bcs.command).toContain("--bucket-name-ct64=coproc-0");
      expect(bcs.command).not.toContain("--bucket-name=coproc-0");
    });
  });

  test("blue-green uses v0.15 commands when a v0.14 BCS is upgraded to a v0.15 SHA", async () => {
    const upgradedScenario = resolveBlueGreenScenario(
      path.join("/tmp", "blue-green-upgraded-bcs.yaml"),
      parseBlueGreenScenario(`
version: 1
kind: blue-green
bcs:
  source:
    mode: registry
    tag: v0.14.0-10
gcs:
  source: { mode: local }
  stackVersion: "0.15.1"
`),
    );
    upgradedScenario.bcs.source = { mode: "registry", tag: "15abcde", compatTag: "v0.15.0" };
    const bgState: State = { ...state, scenario: upgradedScenario };
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "BUCKET_NAME=coproc-0\n");
      await generateComposeOverrides(bgState, stackSpecForState(bgState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { command?: string[]; image?: string }>;
      };
      const blue = doc.services["coprocessor-sns-worker"] ?? {};
      expect(blue.image).toEndWith(":15abcde");
      expect(blue.command).toContain("--bucket-name=coproc-0");
      expect(blue.command?.filter((arg) => arg.startsWith("--bucket-name-"))).toEqual([]);
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

  type RenderedWorker = {
    image?: string;
    build?: unknown;
    environment?: Record<string, string>;
    deploy?: { resources?: { reservations?: { devices?: unknown[] } } };
  };
  const renderWorkers = async (rendered: State) => {
    await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
    await writeFile(envPath("coprocessor"), "\n");
    await writeFile(envPath("coprocessor.1"), "\n");
    await generateComposeOverrides(rendered, stackSpecForState(rendered));
    const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
      services: Record<string, RenderedWorker>;
    };
    return doc.services;
  };
  const expectGpuWiring = (service: RenderedWorker | undefined) => {
    expect(service?.environment?.NVIDIA_VISIBLE_DEVICES).toBe("all");
    expect(service?.environment?.NVIDIA_DRIVER_CAPABILITIES).toBe("compute,utility");
    expect(service?.deploy?.resources?.reservations?.devices).toHaveLength(1);
  };
  const expectNoGpuWiring = (service: RenderedWorker | undefined) => {
    expect(service?.environment?.NVIDIA_VISIBLE_DEVICES).toBeUndefined();
    expect(service?.environment?.NVIDIA_DRIVER_CAPABILITIES).toBeUndefined();
    expect(service?.deploy).toBeUndefined();
  };
  const withWorkerTag = (base: State, tag: string): State => ({
    ...base,
    versions: {
      ...base.versions,
      env: { ...base.versions.env, COPROCESSOR_TFHE_WORKER_VERSION: tag, COPROCESSOR_SNS_WORKER_VERSION: "921b69113" },
    },
  });
  const pinnedWorkerScenario = (tag: string) =>
    resolveScenarioFile(
      path.join("/tmp", "two-of-two-pinned.yaml"),
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
instances:
  - index: 1
    source:
      mode: registry
      tag: ${tag}
      compatTag: v0.15.0
`),
    );

  test("a registry pin decides the GPU wiring, not the bundle's tag", async () => {
    // The pin replaces the bundle image after the instance adjustments ran, so
    // wiring keyed on the bundle would follow the wrong image. Both directions
    // matter: a CPU pin over a GPU bundle carrying the nvidia env and a device
    // reservation cannot start on a host without a GPU; a GPU pin over a CPU
    // bundle without them silently computes on the CPU.
    await withTempStateDir(async () => {
      const cpuPinOverGpuBundle = withWorkerTag(
        { ...state, scenario: pinnedWorkerScenario("921b69113") },
        "921b69113-cuda12.8-sm90",
      );
      let services = await renderWorkers(cpuPinOverGpuBundle);
      expectGpuWiring(services["coprocessor-tfhe-worker"]);
      expect(services["coprocessor1-tfhe-worker"]?.image).toMatch(/:921b69113$/);
      expectNoGpuWiring(services["coprocessor1-tfhe-worker"]);

      const gpuPinOverCpuBundle = withWorkerTag(
        { ...state, scenario: pinnedWorkerScenario("921b69113-cuda12.8-sm90") },
        "921b69113",
      );
      services = await renderWorkers(gpuPinOverCpuBundle);
      expectNoGpuWiring(services["coprocessor-tfhe-worker"]);
      expect(services["coprocessor1-tfhe-worker"]?.image).toMatch(/:921b69113-cuda12\.8-sm90$/);
      expectGpuWiring(services["coprocessor1-tfhe-worker"]);
    });
  });

  test("a worker pinned to specific cards requests exactly those devices", async () => {
    // Docker resolves `count: all` against every GPU on the host and overwrites
    // NVIDIA_VISIBLE_DEVICES with the result, so a pin of `0` next to `count:
    // all` still hands the container every card, and two workers pinned to
    // different cards compete for one card's memory. The request has to follow
    // the pin: `device_ids` alone, no `count`.
    const pinnedScenario = resolveScenarioFile(
      path.join("/tmp", "two-of-two-pinned-cards.yaml"),
      parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
instances:
  - index: 0
    env:
      NVIDIA_VISIBLE_DEVICES: "0"
  - index: 1
    env:
      NVIDIA_VISIBLE_DEVICES: "GPU-3f2a1b, 1"
`),
    );
    await withTempStateDir(async () => {
      const services = await renderWorkers(withWorkerTag({ ...state, scenario: pinnedScenario }, "921b69113-cuda12.8-sm90"));
      const expectDevices = (name: string, pin: string, ids: string[]) => {
        const worker = services[name];
        expect(worker?.environment?.NVIDIA_VISIBLE_DEVICES).toBe(pin);
        expect(worker?.environment?.NVIDIA_DRIVER_CAPABILITIES).toBe("compute,utility");
        expect(worker?.deploy?.resources?.reservations?.devices).toEqual([
          { driver: "nvidia", device_ids: ids, capabilities: ["gpu"] },
        ]);
      };
      expectDevices("coprocessor-tfhe-worker", "0", ["0"]);
      expectDevices("coprocessor1-tfhe-worker", "GPU-3f2a1b, 1", ["GPU-3f2a1b", "1"]);
      // Instance env reaches every service on the node, so the CPU sibling
      // carries the pin as plain env; what it must not get is a device request.
      for (const name of ["coprocessor-sns-worker", "coprocessor1-sns-worker"]) {
        expect(services[name]?.environment?.NVIDIA_DRIVER_CAPABILITIES).toBeUndefined();
        expect(services[name]?.deploy).toBeUndefined();
      }
    });
  });

  test("a locally built worker gets no GPU wiring even under a GPU bundle", async () => {
    // A local build replaces the bundle image with the `fhevm-local-*` tag and
    // compiles Dockerfile.workspace, which is a CPU image: demanding a GPU for
    // it would keep a CPU run from starting on a host without one.
    await withTempStateDir(async () => {
      const services = await renderWorkers(withWorkerTag(inheritedScenarioState, "921b69113-cuda12.8-sm90"));
      for (const name of ["coprocessor-tfhe-worker", "coprocessor1-tfhe-worker"]) {
        expect(services[name]?.image).toMatch(/:fhevm-local-i[01]$/);
        expect(services[name]?.build).toBeDefined();
        expectNoGpuWiring(services[name]);
      }
    });
  });
});

describe("test-suite docker socket runtime", () => {
  test("dockerSocketRuntime resolves a unix DOCKER_HOST socket with its group id", async () => {
    await withUnixSocket(async (socketPath) => {
      const runtime = dockerSocketRuntime({ DOCKER_HOST: `unix://${socketPath}` });
      expect(runtime?.path).toBe(socketPath);
      expect(runtime?.gid).toBe(statSync(socketPath).gid);
    });
  });

  test("dockerSocketRuntime returns undefined for a missing socket", () => {
    expect(dockerSocketRuntime({ DOCKER_HOST: "unix:///nonexistent/docker.sock" })).toBeUndefined();
  });

  test("dockerSocketRuntime treats a remote DOCKER_HOST as socketless", async () => {
    await withUnixSocket(async (socketPath) => {
      // A tcp:// daemon is never local, even while a unix socket exists elsewhere.
      expect(dockerSocketRuntime({ DOCKER_HOST: "tcp://127.0.0.1:2375" })).toBeUndefined();
      expect(dockerSocketRuntime({ DOCKER_HOST: `unix://${socketPath}` })).toBeDefined();
    });
  });

  test("dockerSocketRuntime falls back to the default socket path without DOCKER_HOST", () => {
    // The fallback path is a host fact, so only the path it probes is asserted here.
    const runtime = dockerSocketRuntime({});
    expect(runtime === undefined || runtime.path === "/var/run/docker.sock").toBe(true);
  });

  test("grants the e2e runner the host docker socket when one exists", async () => {
    await withUnixSocket(async (socketPath) => {
      await withDockerHost(`unix://${socketPath}`, async () => {
        await withTempStateDir(async () => {
          await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
          await writeFile(envPath("coprocessor"), "\n");
          await generateComposeOverrides(socketWiringState, stackSpecForState(socketWiringState));
          const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
            services: Record<string, { volumes?: string[]; group_add?: string[] }>;
          };
          const runner = doc.services["test-suite-e2e-debug"];
          expect(runner?.volumes).toEqual([`${socketPath}:/var/run/docker.sock`]);
          expect(runner?.group_add).toEqual([String(statSync(socketPath).gid)]);
          const others = (await generatedServices()).filter(([key]) => key !== "test-suite.yml:test-suite-e2e-debug");
          expect(others.length).toBeGreaterThan(0);
          for (const [key, service] of others) {
            expect(YAML.stringify({ [key]: service })).not.toContain("/var/run/docker.sock");
            expect(service.group_add).toBeUndefined();
          }
        });
      });
    });
  });

  test("keeps the local build override alongside the socket wiring", async () => {
    await withUnixSocket(async (socketPath) => {
      await withDockerHost(`unix://${socketPath}`, async () => {
        await withTempStateDir(async () => {
          await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
          await writeFile(envPath("coprocessor"), "\n");
          await generateComposeOverrides(testSuiteOverrideState, stackSpecForState(testSuiteOverrideState));
          const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
            services: Record<string, { image?: string; build?: unknown; volumes?: string[]; group_add?: string[] }>;
          };
          const runner = doc.services["test-suite-e2e-debug"];
          expect(runner?.image).toContain(":fhevm-local");
          expect(runner?.build).toBeTruthy();
          expect(runner?.volumes).toEqual([`${socketPath}:/var/run/docker.sock`]);
          expect(runner?.group_add).toEqual([String(statSync(socketPath).gid)]);
        });
      });
    });
  });

  test("renders an empty test-suite override on a host without a docker socket", async () => {
    await withDockerHost("unix:///nonexistent/docker.sock", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await generateComposeOverrides(socketWiringState, stackSpecForState(socketWiringState));
        const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
          services: Record<string, { volumes?: unknown; group_add?: unknown }>;
        };
        expect(doc.services).toEqual({});
        expect(doc.services["test-suite-e2e-debug"]?.volumes).toBeUndefined();
        expect(doc.services["test-suite-e2e-debug"]?.group_add).toBeUndefined();
      });
    });
  });

  test("leaves the runner socketless when DOCKER_HOST points at a remote daemon", async () => {
    await withDockerHost("tcp://127.0.0.1:2375", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await generateComposeOverrides(socketWiringState, stackSpecForState(socketWiringState));
        const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
          services: Record<string, unknown>;
        };
        expect(doc.services).toEqual({});
      });
    });
  });

  test("the tracked test-suite compose file carries no docker socket wiring", async () => {
    const raw = await readFile(path.join(TEMPLATE_COMPOSE_DIR, "test-suite-docker-compose.yml"), "utf8");
    expect(raw).not.toContain("docker.sock");
    expect(raw).not.toContain("group_add");
    expect(raw).not.toContain("DOCKER_GID");
  });
});

describe("test-suite docker socket runtime", () => {
  test("dockerSocketRuntime resolves a unix DOCKER_HOST socket with its group id", async () => {
    await withUnixSocket(async (socketPath) => {
      const runtime = dockerSocketRuntime({ DOCKER_HOST: `unix://${socketPath}` });
      expect(runtime?.path).toBe(socketPath);
      expect(runtime?.gid).toBe(statSync(socketPath).gid);
    });
  });

  test("dockerSocketRuntime returns undefined for a missing socket", () => {
    expect(dockerSocketRuntime({ DOCKER_HOST: "unix:///nonexistent/docker.sock" })).toBeUndefined();
  });

  test("dockerSocketRuntime treats a remote DOCKER_HOST as socketless", async () => {
    await withUnixSocket(async (socketPath) => {
      // A tcp:// daemon is never local, even while a unix socket exists elsewhere.
      expect(dockerSocketRuntime({ DOCKER_HOST: "tcp://127.0.0.1:2375" })).toBeUndefined();
      expect(dockerSocketRuntime({ DOCKER_HOST: `unix://${socketPath}` })).toBeDefined();
    });
  });

  test("dockerSocketRuntime falls back to the default socket path without DOCKER_HOST", () => {
    // The fallback path is a host fact, so only the path it probes is asserted here.
    const runtime = dockerSocketRuntime({});
    expect(runtime === undefined || runtime.path === "/var/run/docker.sock").toBe(true);
  });

  test("grants the e2e runner the host docker socket when one exists", async () => {
    await withUnixSocket(async (socketPath) => {
      await withDockerHost(`unix://${socketPath}`, async () => {
        await withTempStateDir(async () => {
          await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
          await writeFile(envPath("coprocessor"), "\n");
          await generateComposeOverrides(socketWiringState, stackSpecForState(socketWiringState));
          const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
            services: Record<string, { volumes?: string[]; group_add?: string[] }>;
          };
          const runner = doc.services["test-suite-e2e-debug"];
          expect(runner?.volumes).toEqual([`${socketPath}:/var/run/docker.sock`]);
          expect(runner?.group_add).toEqual([String(statSync(socketPath).gid)]);
          const others = (await generatedServices()).filter(([key]) => key !== "test-suite.yml:test-suite-e2e-debug");
          expect(others.length).toBeGreaterThan(0);
          for (const [key, service] of others) {
            expect(YAML.stringify({ [key]: service })).not.toContain("/var/run/docker.sock");
            expect(service.group_add).toBeUndefined();
          }
        });
      });
    });
  });

  test("keeps the local build override alongside the socket wiring", async () => {
    await withUnixSocket(async (socketPath) => {
      await withDockerHost(`unix://${socketPath}`, async () => {
        await withTempStateDir(async () => {
          await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
          await writeFile(envPath("coprocessor"), "\n");
          await generateComposeOverrides(testSuiteOverrideState, stackSpecForState(testSuiteOverrideState));
          const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
            services: Record<string, { image?: string; build?: unknown; volumes?: string[]; group_add?: string[] }>;
          };
          const runner = doc.services["test-suite-e2e-debug"];
          expect(runner?.image).toContain(":fhevm-local");
          expect(runner?.build).toBeTruthy();
          expect(runner?.volumes).toEqual([`${socketPath}:/var/run/docker.sock`]);
          expect(runner?.group_add).toEqual([String(statSync(socketPath).gid)]);
        });
      });
    });
  });

  test("renders an empty test-suite override on a host without a docker socket", async () => {
    await withDockerHost("unix:///nonexistent/docker.sock", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await generateComposeOverrides(socketWiringState, stackSpecForState(socketWiringState));
        const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
          services: Record<string, { volumes?: unknown; group_add?: unknown }>;
        };
        expect(doc.services).toEqual({});
        expect(doc.services["test-suite-e2e-debug"]?.volumes).toBeUndefined();
        expect(doc.services["test-suite-e2e-debug"]?.group_add).toBeUndefined();
      });
    });
  });

  test("leaves the runner socketless when DOCKER_HOST points at a remote daemon", async () => {
    await withDockerHost("tcp://127.0.0.1:2375", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await generateComposeOverrides(socketWiringState, stackSpecForState(socketWiringState));
        const doc = YAML.parse(await readFile(composePath("test-suite"), "utf8")) as {
          services: Record<string, unknown>;
        };
        expect(doc.services).toEqual({});
      });
    });
  });

  test("the tracked test-suite compose file carries no docker socket wiring", async () => {
    const raw = await readFile(path.join(TEMPLATE_COMPOSE_DIR, "test-suite-docker-compose.yml"), "utf8");
    expect(raw).not.toContain("docker.sock");
    expect(raw).not.toContain("group_add");
    expect(raw).not.toContain("DOCKER_GID");
  });
});

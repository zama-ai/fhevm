import { describe, expect, test } from "bun:test";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import YAML from "yaml";

import { generateComposeOverrides, loadMergedComposeDoc, serviceNameList } from "./generate/compose";
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

const solanaProofServiceOverrideState: State = {
  ...state,
  overrides: [{ group: "solana-proof-service" }],
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

  test("persists kms-core private vault and supplies both keygen CLI formats", async () => {
    const doc = await loadMergedComposeDoc("core");
    const volumes = doc.services["kms-core"]?.volumes as string[] | undefined;
    const entrypoint = JSON.stringify(doc.services["kms-core"]?.entrypoint);
    expect(doc.services["kms-core"]?.user).toBe("root");
    expect(volumes).toContain("fhevm_kms_core_keys:/app/kms/core/service/keys");
    expect(volumes?.some((mount) => mount.endsWith("config/kms-gen-keys.toml"))).toBe(true);
    expect(entrypoint).toContain("--public-storage");
    expect(entrypoint).toContain("--config-file config/kms-gen-keys.toml");
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

      // Relayer override must not piggyback the standalone proof image.
      await expect(readFile(composePath("solana-proof-service"), "utf8")).rejects.toMatchObject({
        code: "ENOENT",
      });
    });
  });

  test("retags solana-proof-service for local builds when its override group is set", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      await generateComposeOverrides(
        solanaProofServiceOverrideState,
        stackSpecForState(solanaProofServiceOverrideState),
      );
      const proofDoc = YAML.parse(await readFile(composePath("solana-proof-service"), "utf8")) as {
        services: Record<string, { image?: string; build?: { context?: string; dockerfile?: string } }>;
      };
      expect(proofDoc.services["solana-proof-service"]?.image).toBe(
        "${SOLANA_PROOF_SERVICE_IMAGE_REPOSITORY:-solana-proof-service}:fhevm-local",
      );
      expect(proofDoc.services["solana-proof-service"]?.build?.dockerfile).toContain(
        "solana-proof-service/Dockerfile",
      );
      expect(proofDoc.services["solana-proof-db"]).toBeUndefined();
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

  test("builds coprocessor and kms-connector images from the branch-local workspace Dockerfiles", async () => {
    await withTempStateDir(async () => {
      await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
      await writeFile(envPath("coprocessor"), "\n");
      await writeFile(envPath("coprocessor.1"), "\n");
      const kmsState: State = { ...state, overrides: [{ group: "coprocessor" }, { group: "kms-connector" }] };
      await generateComposeOverrides(kmsState, stackSpecForState(kmsState));
      type BuildDoc = { services: Record<string, { build?: { dockerfile?: string; target?: string } }> };
      const coprocessor = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as BuildDoc;
      const connector = YAML.parse(await readFile(composePath("kms-connector"), "utf8")) as BuildDoc;
      const buildFor = (doc: BuildDoc, service: string) => doc.services[service]?.build ?? {};
      // Every coprocessor image builds from the shared workspace Dockerfile (one cargo pass for
      // all binaries) with a per-image runtime target.
      const coprocessorTargets: Record<string, string> = {
        "coprocessor-db-migration": "db-migration",
        "coprocessor-host-listener": "host-listener",
        "coprocessor-host-listener-poller": "host-listener",
        "coprocessor-host-listener-consumer": "host-listener",
        "coprocessor-gw-listener": "gw-listener",
        "coprocessor-tfhe-worker": "tfhe-worker",
        "coprocessor-zkproof-worker": "zkproof-worker",
        "coprocessor-sns-worker": "sns-worker",
        "coprocessor-transaction-sender": "transaction-sender",
      };
      for (const [service, target] of Object.entries(coprocessorTargets)) {
        const build = buildFor(coprocessor, service);
        expect(build.dockerfile).toContain("coprocessor/fhevm-engine/Dockerfile.workspace");
        expect(build.target).toBe(target);
      }
      // The kms-connector worker images build from their shared workspace Dockerfile; connector-db
      // was never workspace-built (sqlx-cli install, not a workspace member) and keeps its own
      // per-image Dockerfile.
      const connectorTargets: Record<string, string> = {
        "kms-connector-gw-listener": "gw-listener",
        "kms-connector-kms-worker": "kms-worker",
        "kms-connector-tx-sender": "tx-sender",
      };
      for (const [service, target] of Object.entries(connectorTargets)) {
        const build = buildFor(connector, service);
        expect(build.dockerfile).toContain("kms-connector/Dockerfile.workspace");
        expect(build.target).toBe(target);
      }
      const connectorDb = buildFor(connector, "kms-connector-db-migration");
      expect(connectorDb.dockerfile).toContain("kms-connector/connector-db/Dockerfile");
      expect(connectorDb.target).toBe("prod");
    });
  });

  describe("registry build-cache wiring", () => {
    const BUILDCACHE_ENV = { FHEVM_BUILDCACHE_TAG: "buildcache-test" } as const;

    const buildCacheState: State = { ...state, overrides: [{ group: "coprocessor" }, { group: "kms-connector" }] };

    // Renders the coprocessor + kms-connector overrides with the given env applied for the
    // duration of the generation (same harness shape as the sccache tests below).
    const renderOverrides = async (env: Record<string, string>) => {
      const saved = new Map<string, string | undefined>();
      for (const key of Object.keys({ ...BUILDCACHE_ENV })) {
        saved.set(key, process.env[key]);
        delete process.env[key];
      }
      for (const [key, value] of Object.entries(env)) {
        process.env[key] = value;
      }
      try {
        await generateComposeOverrides(buildCacheState, stackSpecForState(buildCacheState));
        return {
          coprocessor: await readFile(composePath("coprocessor"), "utf8"),
          connector: await readFile(composePath("kms-connector"), "utf8"),
        };
      } finally {
        for (const [key, value] of saved) {
          if (value === undefined) {
            delete process.env[key];
          } else {
            process.env[key] = value;
          }
        }
      }
    };

    test("emits compose with no cache_from when FHEVM_BUILDCACHE_TAG is unset", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await writeFile(envPath("coprocessor.1"), "\n");
        const rendered = await renderOverrides({});
        for (const raw of [rendered.coprocessor, rendered.connector]) {
          expect(raw).not.toContain("cache_from");
          expect(raw).not.toContain("buildcache");
        }
      });
    });

    test("adds a per-image registry cache_from only for per-image Dockerfile builds", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await writeFile(envPath("coprocessor.1"), "\n");
        const withoutEnv = await renderOverrides({});
        const withEnv = await renderOverrides({ ...BUILDCACHE_ENV });

        type BuildDoc = { services: Record<string, { build?: { cache_from?: string[] } }> };
        const connector = YAML.parse(withEnv.connector) as BuildDoc;

        // The publish workflow exports per-image build caches; those cannot hit against the
        // branch-local workspace Dockerfiles, so workspace-built services get no cache_from and
        // a set FHEVM_BUILDCACHE_TAG stays harmless.
        expect(withEnv.coprocessor).not.toContain("cache_from");
        expect(connector.services["kms-connector-kms-worker"]?.build?.cache_from).toBeUndefined();

        // connector-db still builds from its per-image Dockerfile, so its cache ref is the
        // service's own image repository at the cache tag, matching what
        // solana-images-publish.yml exports.
        expect(connector.services["kms-connector-db-migration"]?.build?.cache_from).toEqual([
          "ghcr.io/zama-ai/fhevm/kms-connector/db-migration:buildcache-test",
        ]);

        // Structural proof of graceful degradation: strip the cache_from additions from the
        // enabled documents and they are identical to the disabled ones.
        for (const [enabledRaw, disabledRaw] of [
          [withEnv.coprocessor, withoutEnv.coprocessor],
          [withEnv.connector, withoutEnv.connector],
        ] as const) {
          const stripped = YAML.parse(enabledRaw) as BuildDoc;
          for (const service of Object.values(stripped.services)) {
            if (service.build) {
              delete service.build.cache_from;
            }
          }
          expect(stripped).toEqual(YAML.parse(disabledRaw) as BuildDoc);
        }
      });
    });
  });

  describe("sccache build wiring", () => {
    const SCCACHE_ENV = {
      SCCACHE_BUCKET: "gh-actions-cache-eu-west-3",
      SCCACHE_REGION: "eu-west-3",
      SCCACHE_S3_PREFIX: "sccache/fhevm-coprocessor",
      AWS_ACCESS_KEY_S3_USER: "test-access-key",
      AWS_SECRET_KEY_S3_USER: "test-secret-key",
    } as const;

    // Renders the coprocessor override with the given sccache env applied for the duration of the
    // generation. Both renders below run inside the SAME temp state dir so the only differences are
    // the sccache additions (env_file paths embed the temp dir and would otherwise differ).
    const renderCoprocessor = async (env: Record<string, string>) => {
      const saved = new Map<string, string | undefined>();
      for (const key of Object.keys({ ...SCCACHE_ENV })) {
        saved.set(key, process.env[key]);
        delete process.env[key];
      }
      for (const [key, value] of Object.entries(env)) {
        process.env[key] = value;
      }
      try {
        await generateComposeOverrides(inheritedScenarioState, stackSpecForState(inheritedScenarioState));
        return await readFile(composePath("coprocessor"), "utf8");
      } finally {
        for (const [key, value] of saved) {
          if (value === undefined) {
            delete process.env[key];
          } else {
            process.env[key] = value;
          }
        }
      }
    };

    test("emits compose with no sccache tokens when SCCACHE_BUCKET is unset", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await writeFile(envPath("coprocessor.1"), "\n");
        const raw = await renderCoprocessor({});
        expect(raw).not.toContain("SCCACHE");
        expect(raw).not.toContain("sccache_aws");
        expect(raw).not.toMatch(/^secrets:/m);
      });
    });

    test("adds only the sccache build args + secrets when SCCACHE_BUCKET is set", async () => {
      await withTempStateDir(async () => {
        await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
        await writeFile(envPath("coprocessor"), "\n");
        await writeFile(envPath("coprocessor.1"), "\n");
        const withoutEnv = await renderCoprocessor({});
        const withEnv = await renderCoprocessor({ ...SCCACHE_ENV });

        type BuildDoc = {
          secrets?: Record<string, unknown>;
          services: Record<string, { build?: { args?: Record<string, string>; secrets?: string[] } }>;
        };
        const enabled = YAML.parse(withEnv) as BuildDoc;

        // Top-level secrets map BuildKit ids to the CI credential env vars.
        expect(enabled.secrets).toEqual({
          sccache_aws_access_key_id: { environment: "AWS_ACCESS_KEY_S3_USER" },
          sccache_aws_secret_access_key: { environment: "AWS_SECRET_KEY_S3_USER" },
        });
        const hostListener = enabled.services["coprocessor-host-listener"]?.build;
        expect(hostListener?.args).toMatchObject({
          SCCACHE_BUCKET: "gh-actions-cache-eu-west-3",
          SCCACHE_REGION: "eu-west-3",
          SCCACHE_S3_PREFIX: "sccache/fhevm-coprocessor",
        });
        expect(hostListener?.secrets).toEqual(["sccache_aws_access_key_id", "sccache_aws_secret_access_key"]);

        // Structural proof of graceful degradation: strip the sccache additions from the enabled
        // document and it is identical to the disabled one.
        const stripped = YAML.parse(withEnv) as BuildDoc;
        delete stripped.secrets;
        for (const service of Object.values(stripped.services)) {
          if (service.build) {
            for (const key of ["SCCACHE_BUCKET", "SCCACHE_REGION", "SCCACHE_S3_PREFIX"]) {
              delete service.build.args?.[key];
            }
            if (service.build.args && Object.keys(service.build.args).length === 0) {
              delete service.build.args;
            }
            delete service.build.secrets;
          }
        }
        expect(stripped).toEqual(YAML.parse(withoutEnv) as BuildDoc);
      });
    });
  });
});

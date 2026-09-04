import { describe, expect, test } from "bun:test";

import { DEFAULT_EXTRA_HOST_RPC_PORT } from "./layout";
import {
  adoptE2ePublicKmsConnectorOverride,
  adoptRunningE2eKmsConnectorOverride,
  assertVersionLockChanges,
  changedVersionKeys,
  isVerifiedE2ePublicKmsConnectorSchemaSuperset,
  kmsConnectorRuntimeReplacementServices,
  previewStateFromBundle,
  removeRuntimeUpgradeOverrides,
  resumePendingE2eKmsConnectorRuntimeAdoption,
  resolveUpgradePlan,
} from "./flow/up-flow";
import { SchemaGuardError } from "./errors";
import { presetBundle } from "./resolve/target";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";
const defaultScenario: State["scenario"] = testDefaultScenario();

const kmsAdoptionState = (): State => ({
  target: "latest-main",
  lockPath: "/tmp/option-c-e2e.lock.json",
  requiresGitHub: false,
  versions: presetBundle("latest-main", "abcdef0", "option-c-e2e.lock.json"),
  overrides: [{ group: "coprocessor" }, { group: "test-suite" }],
  scenario: testDefaultScenario(),
  completedSteps: ["base", "coprocessor", "kms-connector", "bootstrap", "test-suite"],
  updatedAt: "2026-08-07T00:00:00.000Z",
});

const approvedKmsConnectorSupersetSql = `-- The GPU key-generation path stores compressed keysets in the connector
-- response tables. Keep the database enum in sync with KeyType::CompressedKeySet.
ALTER TYPE key_type ADD VALUE IF NOT EXISTS 'CompressedKeySet';
`;
const approvedKmsConnectorSupersetChecksum =
  "e3c67db173c72841bb23f81937e1b0a1f49dc4a1ac0dd8711e77e2ced145c05909cd783f89642acefa4967578ed8611d";
const approvedKmsConnectorSupersetPath = "kms-connector/connector-db/migrations/20260804000000_add_compressed_key_set.sql";

const approvedKmsConnectorSupersetOperations = () => ({
  async run(argv: string[]) {
    if (argv[1] === "rev-parse") {
      return {
        code: 0,
        stdout: argv.at(-1) === "72af12a^{commit}" ? "72af12a184c2304008a06e7ecdf97d2da2e0f532\n" : "different\n",
        stderr: "",
      };
    }
    if (argv[1] === "ls-files") {
      return { code: 0, stdout: "", stderr: "" };
    }
    if (argv[1] === "diff") {
      return { code: 1, stdout: `D\t${approvedKmsConnectorSupersetPath}\n`, stderr: "" };
    }
    if (argv[1] === "show") {
      return { code: 0, stdout: approvedKmsConnectorSupersetSql, stderr: "" };
    }
    throw new Error(`unexpected command: ${argv.join(" ")}`);
  },
  async postgresExec() {
    return { code: 0, stdout: `${approvedKmsConnectorSupersetChecksum}|true\n`, stderr: "" };
  },
});

describe("stack", () => {
  test("dry-run preview state uses the resolved lock target", () => {
    const bundle = {
      ...presetBundle("latest-main", "abcdef0", "devnet.json"),
      target: "devnet" as const,
      lockName: "devnet.json",
    };
    const state = previewStateFromBundle({ overrides: [], lockFile: "/tmp/devnet-lock.json" }, bundle, defaultScenario);
    expect(state.target).toBe("devnet");
    expect(state.requiresGitHub).toBe(false);
  });

  test("rejects multi-chain scenarios on network targets during preview", () => {
    const bundle = {
      ...presetBundle("latest-main", "abcdef0", "testnet.json"),
      target: "testnet" as const,
      lockName: "testnet.json",
    };
    const multiChainScenario: State["scenario"] = {
      ...defaultScenario,
      hostChains: [
        defaultScenario.hostChains[0],
        { key: "chain-b", chainId: "67890", rpcPort: DEFAULT_EXTRA_HOST_RPC_PORT },
      ],
    };
    expect(() => previewStateFromBundle({ overrides: [], lockFile: "/tmp/testnet-lock.json" }, bundle, multiChainScenario)).toThrow(
      "--target testnet does not currently support multi-chain scenarios",
    );
  });

  test("rejects multi-chain scenarios on legacy coprocessor bundles during preview", () => {
    const bundle = {
      ...presetBundle("latest-main", "abcdef0", "latest-supported.json"),
      target: "latest-supported" as const,
      env: {
        ...presetBundle("latest-main", "abcdef0", "latest-supported.json").env,
        HOST_VERSION: "v0.11.0",
        GATEWAY_VERSION: "v0.11.0",
        COPROCESSOR_HOST_LISTENER_VERSION: "v0.11.0",
        COPROCESSOR_HOST_LISTENER_POLLER_VERSION: "v0.11.0",
      },
    };
    const multiChainScenario: State["scenario"] = {
      ...defaultScenario,
      hostChains: [
        defaultScenario.hostChains[0],
        { key: "chain-b", chainId: "67890", rpcPort: DEFAULT_EXTRA_HOST_RPC_PORT },
      ],
    };
    expect(() => previewStateFromBundle({ overrides: [], lockFile: "/tmp/latest-supported-lock.json" }, bundle, multiChainScenario)).toThrow(
      "Multi-chain scenarios require coprocessor runtime >= v0.12.0",
    );
  });

  test("upgrade plan restarts runtime services for a full kms-connector override", () => {
    const plan = resolveUpgradePlan({ overrides: [{ group: "kms-connector" }], scenario: defaultScenario }, "kms-connector");
    expect(plan.migrationServices).toEqual(["kms-connector-db-migration"]);
    expect(plan.runtimeServices).toEqual([
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ]);
  });

  test("upgrade plan supports schema-coupled partial runtime overrides when runtime services exist", () => {
    const plan = resolveUpgradePlan(
      {
        overrides: [{ group: "kms-connector", services: ["kms-connector-gw-listener"] }],
        scenario: defaultScenario,
      },
      "kms-connector",
    );
    expect(plan.migrationServices).toEqual([]);
    expect(plan.runtimeServices).toEqual(["kms-connector-gw-listener"]);
  });

  test("upgrade plan supports relayer version locks without a local override", () => {
    const plan = resolveUpgradePlan({ overrides: [], scenario: defaultScenario }, "relayer", { lockFile: true });
    expect(plan.versionKeys).toEqual(["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"]);
    expect(plan.migrationServices).toEqual(["relayer-db-migration"]);
    expect(plan.runtimeServices).toEqual(["relayer"]);
  });

  test("upgrade plan links kms-core and connector for version-lock upgrades", () => {
    const plan = resolveUpgradePlan({ overrides: [], scenario: defaultScenario }, "kms", { lockFile: true });
    expect(plan.versionKeys).toEqual([
      "CORE_VERSION",
      "CONNECTOR_DB_MIGRATION_VERSION",
      "CONNECTOR_GW_LISTENER_VERSION",
      "CONNECTOR_KMS_WORKER_VERSION",
      "CONNECTOR_TX_SENDER_VERSION",
    ]);
    expect(plan.migrationServices).toEqual(["kms-connector-db-migration"]);
    expect(plan.runtimeServices).toEqual([
      "kms-core",
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
    ]);
  });

  test("upgrade plan supports listener-core version-lock upgrades", () => {
    const plan = resolveUpgradePlan({ overrides: [], scenario: defaultScenario }, "listener-core", { lockFile: true });
    expect(plan.versionKeys).toEqual(["LISTENER_CORE_VERSION"]);
    expect(plan.migrationServices).toEqual([]);
    expect(plan.runtimeServices).toEqual(["listener-redis", "listener-publisher-for-anvil"]);
  });

  test("upgrade plan supports listener-core local overrides", () => {
    const plan = resolveUpgradePlan({ overrides: [{ group: "listener-core" }], scenario: defaultScenario }, "listener-core");
    expect(plan.versionKeys).toEqual(["LISTENER_CORE_VERSION"]);
    expect(plan.migrationServices).toEqual([]);
    expect(plan.runtimeServices).toEqual(["listener-publisher-for-anvil"]);
  });

  test("runtime upgrade locks drop only overrides owned by the upgraded group", () => {
    expect(
      removeRuntimeUpgradeOverrides(
        [{ group: "relayer" }, { group: "test-suite" }],
        "relayer",
      ),
    ).toEqual([{ group: "test-suite" }]);
    expect(
      removeRuntimeUpgradeOverrides(
        [{ group: "kms-connector" }, { group: "listener-core" }, { group: "test-suite" }],
        "kms",
      ),
    ).toEqual([{ group: "listener-core" }, { group: "test-suite" }]);
    expect(
      removeRuntimeUpgradeOverrides(
        [{ group: "listener-core" }, { group: "test-suite" }],
        "listener-core",
      ),
    ).toEqual([{ group: "test-suite" }]);
  });

  test("version lock checks reject unrelated version keys", () => {
    const base = presetBundle("latest-main", "abcdef0", "base.json");
    const next = {
      ...base,
      env: {
        ...base.env,
        RELAYER_VERSION: "next-relayer",
        HOST_VERSION: "next-host",
      },
    };
    const changed = changedVersionKeys(base, next);
    expect(changed).toEqual(["HOST_VERSION", "RELAYER_VERSION"]);
    expect(() => assertVersionLockChanges("relayer", ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"], changed)).toThrow(
      "HOST_VERSION",
    );
  });

  test("upgrade treats inherited multi-instance coprocessor build overrides as an active local runtime path", () => {
    const plan = resolveUpgradePlan(
      {
        overrides: [{ group: "coprocessor" }],
        scenario: {
          ...defaultScenario,
          topology: { count: 2, threshold: 2 },
          instances: [
            { index: 0, source: { mode: "inherit" }, env: {}, args: {} },
            { index: 1, source: { mode: "inherit" }, env: {}, args: {} },
          ],
        },
      },
      "coprocessor",
    );
    expect(plan.runtimeServices).toContain("coprocessor-host-listener");
    expect(plan.runtimeServices).toContain("coprocessor1-host-listener");
  });

  test("coprocessor release upgrade restarts inherited registry services", () => {
    const plan = resolveUpgradePlan(
      {
        overrides: [{ group: "test-suite" }],
        scenario: {
          ...defaultScenario,
          topology: { count: 2, threshold: 2 },
          instances: [],
        },
      },
      "coprocessor",
      { lockFile: true },
    );
    expect(plan.migrationServices).toContain("coprocessor-db-migration");
    expect(plan.runtimeServices).toContain("coprocessor-host-listener");
    expect(plan.runtimeServices).toContain("coprocessor1-host-listener");
  });

  test("adopts only KMS connector runtime services for an already-running public E2E recovery", () => {
    const state = kmsAdoptionState();
    const next = adoptE2ePublicKmsConnectorOverride(state);

    expect(next).not.toBe(state);
    expect(next.e2ePublicRuntime).toBe(true);
    expect(next.e2eKmsConnectorRuntimeAdoptionPending).toBe(true);
    expect(next.overrides).toEqual([
      { group: "coprocessor" },
      { group: "test-suite" },
      {
        group: "kms-connector",
        services: ["kms-connector-gw-listener", "kms-connector-kms-worker", "kms-connector-tx-sender"],
      },
    ]);
    // The recovery intentionally does not perturb the persisted deployment
    // identity or completed pipeline state: its only work is rebuilding the
    // connector runtime containers against existing material.
    expect(next.versions).toBe(state.versions);
    expect(next.scenario).toBe(state.scenario);
    expect(next.completedSteps).toBe(state.completedSteps);
    expect(state.e2ePublicRuntime).toBeUndefined();
    expect(state.overrides).toEqual([{ group: "coprocessor" }, { group: "test-suite" }]);
  });

  test("recognizes only the verified additive KMS connector database superset", async () => {
    const state = {
      ...kmsAdoptionState(),
      versions: {
        ...kmsAdoptionState().versions,
        env: { ...kmsAdoptionState().versions.env, CONNECTOR_DB_MIGRATION_VERSION: "72af12a" },
      },
    };
    expect(await isVerifiedE2ePublicKmsConnectorSchemaSuperset(state, approvedKmsConnectorSupersetOperations())).toBe(true);

    const additionalDiff = approvedKmsConnectorSupersetOperations();
    const run = additionalDiff.run;
    additionalDiff.run = async (argv: string[]) =>
      argv[1] === "diff"
        ? { code: 1, stdout: `D\t${approvedKmsConnectorSupersetPath}\nA\tkms-connector/connector-db/migrations/unapproved.sql\n`, stderr: "" }
        : run(argv);
    expect(await isVerifiedE2ePublicKmsConnectorSchemaSuperset(state, additionalDiff)).toBe(false);

    const wrongRef = {
      ...state,
      versions: { ...state.versions, env: { ...state.versions.env, CONNECTOR_DB_MIGRATION_VERSION: "different" } },
    };
    expect(await isVerifiedE2ePublicKmsConnectorSchemaSuperset(wrongRef, approvedKmsConnectorSupersetOperations())).toBe(false);

    const threshold = {
      ...state,
      scenario: testDefaultScenario({
        kms: { mode: "threshold", parties: 3, threshold: 2, committeeSize: 3, fheParams: "Test" },
      }),
    };
    expect(await isVerifiedE2ePublicKmsConnectorSchemaSuperset(threshold, approvedKmsConnectorSupersetOperations())).toBe(false);
  });

  test("KMS connector E2E adoption fails closed without the expected local E2E stack", () => {
    const noCoprocessor = kmsAdoptionState();
    noCoprocessor.overrides = [{ group: "test-suite" }];
    expect(() => adoptE2ePublicKmsConnectorOverride(noCoprocessor)).toThrow("active local coprocessor override");

    const noTestSuite = kmsAdoptionState();
    noTestSuite.overrides = [{ group: "coprocessor" }];
    expect(() => adoptE2ePublicKmsConnectorOverride(noTestSuite)).toThrow("active local test-suite override");

    const incomplete = kmsAdoptionState();
    incomplete.completedSteps = ["base", "coprocessor"];
    expect(() => adoptE2ePublicKmsConnectorOverride(incomplete)).toThrow("completed the kms-connector step");

    const alreadyLocal = kmsAdoptionState();
    alreadyLocal.overrides.push({ group: "kms-connector" });
    expect(() => adoptE2ePublicKmsConnectorOverride(alreadyLocal)).toThrow("already has a local override");
  });

  test("KMS connector E2E adoption builds and recreates only runtime services without dependencies or migrations", async () => {
    const state = kmsAdoptionState();
    const calls: string[] = [];
    let persisted: State | undefined;

    await adoptRunningE2eKmsConnectorOverride(state, {
      async assertSchemaCompatibility(_versions, overrides, _scenario, allowMismatch) {
        calls.push(`schema:${allowMismatch}:${overrides.at(-1)?.services?.join(",")}`);
      },
      async saveState(next) {
        persisted = next;
        calls.push(`save:${next.e2eKmsConnectorRuntimeAdoptionPending}`);
      },
      async generateRuntime(next) {
        calls.push(`generate:${next.e2ePublicRuntime}`);
      },
      async maybeBuild(component, next, options = {}) {
        calls.push(`build:${component}:${options.force}:${next.overrides.at(-1)?.services?.join(",")}`);
      },
      async composeUp(component, services = [], options = {}) {
        calls.push(`up:${component}:${services.join(",")}:${options.noDeps}:${options.forceRecreate}`);
      },
      async waitForKmsConnector(next) {
        calls.push(`ready:${next.e2ePublicRuntime}`);
      },
      async postBootHealthGate(containers) {
        calls.push(`health:${containers.join(",")}`);
      },
    });

    expect(calls).toEqual([
      "schema:false:kms-connector-gw-listener,kms-connector-kms-worker,kms-connector-tx-sender",
      "save:true",
      "generate:true",
      "build:kms-connector:true:kms-connector-gw-listener,kms-connector-kms-worker,kms-connector-tx-sender",
      "up:kms-connector:kms-connector-gw-listener,kms-connector-kms-worker,kms-connector-tx-sender:true:true",
      "ready:true",
      "health:kms-connector-gw-listener,kms-connector-kms-worker,kms-connector-tx-sender",
      "save:undefined",
    ]);
    expect(persisted?.overrides.at(-1)).toEqual({
      group: "kms-connector",
      services: ["kms-connector-gw-listener", "kms-connector-kms-worker", "kms-connector-tx-sender"],
    });
    expect(persisted?.e2eKmsConnectorRuntimeAdoptionPending).toBeUndefined();
  });

  test("KMS connector E2E adoption permits only the verified additive DB-superset after the ordinary guard rejects it", async () => {
    const calls: string[] = [];
    await adoptRunningE2eKmsConnectorOverride(kmsAdoptionState(), {
      async assertSchemaCompatibility() {
        throw new SchemaGuardError("kms-connector", "expected migration mismatch");
      },
      async isVerifiedE2ePublicKmsConnectorSchemaSuperset() {
        calls.push("verified-superset");
        return true;
      },
      async saveState(next) {
        calls.push(`save:${next.e2eKmsConnectorRuntimeAdoptionPending}`);
      },
      async generateRuntime() {
        calls.push("generate");
      },
      async maybeBuild() {
        calls.push("build");
      },
      async composeUp(_component, _services, options = {}) {
        calls.push(`up:${options.noDeps}:${options.forceRecreate}`);
      },
      async waitForKmsConnector() {
        calls.push("ready");
      },
      async postBootHealthGate() {
        calls.push("health");
      },
    });
    expect(calls).toEqual([
      "verified-superset",
      "save:true",
      "generate",
      "build",
      "up:true:true",
      "ready",
      "health",
      "save:undefined",
    ]);
  });

  test("KMS connector E2E adoption keeps all other schema mismatches closed", async () => {
    const calls: string[] = [];
    await expect(
      adoptRunningE2eKmsConnectorOverride(kmsAdoptionState(), {
        async assertSchemaCompatibility() {
          throw new SchemaGuardError("kms-connector", "unexpected migration mismatch");
        },
        async isVerifiedE2ePublicKmsConnectorSchemaSuperset() {
          calls.push("verified-superset");
          return false;
        },
        async saveState() {
          calls.push("save");
        },
        async generateRuntime() {},
        async maybeBuild() {},
        async composeUp() {},
        async waitForKmsConnector() {},
        async postBootHealthGate() {},
      }),
    ).rejects.toThrow("unexpected migration mismatch");
    expect(calls).toEqual(["verified-superset"]);
  });

  test("KMS connector E2E adoption rejects bundles without a local migration guard", async () => {
    const state = kmsAdoptionState();
    state.versions = { ...state.versions, target: "devnet" };
    await expect(
      adoptRunningE2eKmsConnectorOverride(state, {
        async assertSchemaCompatibility() {
          throw new Error("must not bypass the missing schema guard");
        },
        async saveState() {},
        async generateRuntime() {},
        async maybeBuild() {},
        async composeUp() {},
        async waitForKmsConnector() {},
        async postBootHealthGate() {},
      }),
    ).rejects.toThrow("requires a schema-guarded bundle");
  });

  test("retries a failed KMS connector E2E adoption through the same runtime-only path", async () => {
    const state = kmsAdoptionState();
    let persisted: State | undefined;
    await expect(
      adoptRunningE2eKmsConnectorOverride(state, {
        async assertSchemaCompatibility() {},
        async saveState(next) {
          persisted = next;
        },
        async generateRuntime() {},
        async maybeBuild() {
          throw new Error("simulated local connector build failure");
        },
        async composeUp() {},
        async waitForKmsConnector() {},
        async postBootHealthGate() {},
      }),
    ).rejects.toThrow("simulated local connector build failure");

    expect(persisted?.e2eKmsConnectorRuntimeAdoptionPending).toBe(true);
    expect(persisted?.overrides.at(-1)).toEqual({
      group: "kms-connector",
      services: ["kms-connector-gw-listener", "kms-connector-kms-worker", "kms-connector-tx-sender"],
    });

    const retryCalls: string[] = [];
    await adoptRunningE2eKmsConnectorOverride(persisted!, {
      async assertSchemaCompatibility() {
        retryCalls.push("schema");
      },
      async saveState(next) {
        retryCalls.push(`save:${next.e2eKmsConnectorRuntimeAdoptionPending}`);
      },
      async generateRuntime() {
        retryCalls.push("generate");
      },
      async maybeBuild(component, next, options = {}) {
        retryCalls.push(`build:${component}:${options.force}:${next.overrides.at(-1)?.services?.join(",")}`);
      },
      async composeUp(component, services = [], options = {}) {
        retryCalls.push(`up:${component}:${services.join(",")}:${options.noDeps}:${options.forceRecreate}`);
      },
      async waitForKmsConnector() {
        retryCalls.push("ready");
      },
      async postBootHealthGate() {
        retryCalls.push("health");
      },
    });

    expect(retryCalls).toEqual([
      "schema",
      "save:true",
      "generate",
      "build:kms-connector:true:kms-connector-gw-listener,kms-connector-kms-worker,kms-connector-tx-sender",
      "up:kms-connector:kms-connector-gw-listener,kms-connector-kms-worker,kms-connector-tx-sender:true:true",
      "ready",
      "health",
      "save:undefined",
    ]);
    expect(persisted?.e2eKmsConnectorRuntimeAdoptionPending).toBeUndefined();
  });

  test("resume selects a pending KMS adoption before generic pipeline repair and rejects --from-step", async () => {
    const pending = adoptE2ePublicKmsConnectorOverride(kmsAdoptionState());
    const resumed: State[] = [];

    await expect(
      resumePendingE2eKmsConnectorRuntimeAdoption(pending, "kms-connector", async () => {
        throw new Error("must not run adoption with --from-step");
      }),
    ).rejects.toThrow("must resume its exact runtime-only replacement");
    expect(resumed).toEqual([]);

    expect(
      await resumePendingE2eKmsConnectorRuntimeAdoption(pending, undefined, async (state) => {
        resumed.push(state);
      }),
    ).toBe(true);
    expect(resumed).toEqual([pending]);
  });

  test("KMS connector E2E adoption recreates every threshold party runtime but no migration service", () => {
    const state = kmsAdoptionState();
    state.scenario = testDefaultScenario({
      kms: { mode: "threshold", parties: 3, threshold: 1, committeeSize: 3, fheParams: "Test" },
    });

    expect(kmsConnectorRuntimeReplacementServices(state)).toEqual([
      "kms-connector-gw-listener",
      "kms-connector-kms-worker",
      "kms-connector-tx-sender",
      "kms-connector-2-gw-listener",
      "kms-connector-2-kms-worker",
      "kms-connector-2-tx-sender",
      "kms-connector-3-gw-listener",
      "kms-connector-3-kms-worker",
      "kms-connector-3-tx-sender",
    ]);
  });
});

/**
 * pipeline.ts — The 15-step boot sequence using Effect services.
 *
 * Each step from the original runStep switch is ported as an Effect.gen
 * that composes the relevant services (ContainerRunner, ContainerProbe,
 * MinioClient, RpcClient, StateManager, EnvWriter, ImageBuilder, CommandRunner).
 */
import { Effect, Schedule } from "effect";

import { validateBundleCompatibility, requiresMultichainAclAddress } from "./compat";
import { regen, serviceNameList, resolvedComposeEnv } from "./codegen";
import { describeBundle } from "./resolve";
import {
  BootstrapTimeout,
  IncompatibleVersions,
  PreflightError,
  SchemaGuardError,
} from "./errors";
import {
  COMPONENT_BY_STEP,
  COMPONENTS,
  GROUP_BUILD_COMPONENTS,
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  PORTS,
  PROJECT,
  REPO_ROOT,
  SCHEMA_COUPLED_GROUPS,
  composePath,
  dockerArgs,
  envPath,
  gatewayAddressesPath,
  hostAddressesPath,
  versionsEnvPath,
} from "./layout";
import { CommandRunner } from "./services/CommandRunner";
import { ContainerProbe } from "./services/ContainerProbe";
import { ContainerRunner } from "./services/ContainerRunner";
import { ImageBuilder } from "./services/ImageBuilder";
import { MinioClient } from "./services/MinioClient";
import { RpcClient } from "./services/RpcClient";
import { StateManager } from "./services/StateManager";
import type {
  LocalOverride,
  OverrideGroup,
  State,
  StepName,
  VersionBundle,
} from "./types";
import { STEP_NAMES } from "./types";
import {
  exists,
  hostReachableMaterialUrl,
  predictedCrsId,
  predictedKeyId,
  readEnvFile,
  toServiceName,
  withHexPrefix,
} from "./utils";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

export const SCHEMA_GUARDS = {
  coprocessor: {
    versionKey: "COPROCESSOR_DB_MIGRATION_VERSION",
    repoPath: "coprocessor/fhevm-engine/db-migration/migrations",
  },
  "kms-connector": {
    versionKey: "CONNECTOR_DB_MIGRATION_VERSION",
    repoPath: "kms-connector/connector-db/migrations",
  },
} as const satisfies Partial<
  Record<OverrideGroup, { versionKey: string; repoPath: string }>
>;

export const SCHEMA_GUARD_TARGETS = new Set<VersionBundle["target"]>([
  "latest-release",
  "latest-main",
  "sha",
]);

export const UPGRADEABLE_GROUPS = [
  "coprocessor",
  "kms-connector",
  "test-suite",
] as const;
export type UpgradeGroup = (typeof UPGRADEABLE_GROUPS)[number];

export const POST_BOOT_HEALTH_GATE_DELAY_MS = 5_000;

export const KMS_CONNECTOR_HEALTH_CONTAINERS = [
  "kms-connector-gw-listener",
  "kms-connector-kms-worker",
  "kms-connector-tx-sender",
];

const NETWORK_TARGETS: ReadonlySet<string> = new Set([
  "devnet",
  "testnet",
  "mainnet",
]);

// ---------------------------------------------------------------------------
// Pure helpers
// ---------------------------------------------------------------------------

export const stateStepIndex = (step: StepName) => STEP_NAMES.indexOf(step);

export const coprocessorHealthContainers = (
  state: Pick<State, "topology">,
): string[] => {
  const suffixes = GROUP_SERVICE_SUFFIXES["coprocessor"].filter(
    (s) => !s.includes("migration"),
  );
  const names: string[] = [];
  for (let index = 0; index < state.topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

export const coprocessorServicesForOverrides = (
  state: Pick<State, "topology">,
  services?: string[],
) => {
  if (!services?.length) {
    return serviceNameList(state, "coprocessor");
  }
  const suffixes = [
    ...new Set(services.map((service) => service.replace(/^coprocessor-/, ""))),
  ];
  const names: string[] = [];
  for (let index = 0; index < state.topology.count; index += 1) {
    for (const suffix of suffixes) {
      names.push(toServiceName(suffix, index));
    }
  }
  return names;
};

export const validateDiscovery = (
  state: Pick<State, "target" | "versions" | "discovery" | "overrides">,
): Effect.Effect<void, PreflightError> =>
  Effect.gen(function* () {
    const discovery = state.discovery;
    if (!discovery) {
      return yield* Effect.fail(
        new PreflightError({ message: "Missing discovery state" }),
      );
    }
    const requiredGateway = [
      "GATEWAY_CONFIG_ADDRESS",
      "KMS_GENERATION_ADDRESS",
      "DECRYPTION_ADDRESS",
      "INPUT_VERIFICATION_ADDRESS",
      "CIPHERTEXT_COMMITS_ADDRESS",
      ...(requiresMultichainAclAddress(state)
        ? ["MULTICHAIN_ACL_ADDRESS"]
        : []),
    ];
    const requiredHost = [
      "ACL_CONTRACT_ADDRESS",
      "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
      "KMS_VERIFIER_CONTRACT_ADDRESS",
      "INPUT_VERIFIER_CONTRACT_ADDRESS",
      "PAUSER_SET_CONTRACT_ADDRESS",
    ];
    for (const key of requiredGateway) {
      if (!discovery.gateway[key]) {
        return yield* Effect.fail(
          new PreflightError({ message: `Missing gateway discovery value ${key}` }),
        );
      }
    }
    for (const key of requiredHost) {
      if (!discovery.host[key]) {
        return yield* Effect.fail(
          new PreflightError({ message: `Missing host discovery value ${key}` }),
        );
      }
    }
    if (!discovery.kmsSigner) {
      return yield* Effect.fail(
        new PreflightError({ message: "Missing KMS signer discovery" }),
      );
    }
    if (!discovery.fheKeyId || !discovery.crsKeyId) {
      return yield* Effect.fail(
        new PreflightError({ message: "Missing predicted key ids" }),
      );
    }
  });

export const describeOverride = (item: { group: string; services?: string[] }) =>
  `${item.group}${item.services?.length ? `[${item.services.join(",")}]` : ""}`;

export const overrideWarnings = (
  overrides: LocalOverride[],
  target?: string,
) => {
  const warnings = overrides.flatMap((item) =>
    item.services?.length && SCHEMA_COUPLED_GROUPS.includes(item.group)
      ? [
          `${item.group}: per-service override with a shared database. ` +
            `If your changes include DB migrations, non-overridden services may fail. ` +
            `Use --override ${item.group} (full group) in that case.`,
        ]
      : [],
  );
  if (target && NETWORK_TARGETS.has(target) && overrides.length) {
    warnings.push(
      `Overriding on network target '${target}': ensure your local code is compatible ` +
        `with ${target}'s DB schema, contract interfaces, and service versions.`,
    );
  }
  return warnings;
};

export const printBundle = (bundle: VersionBundle) =>
  Effect.gen(function* () {
    yield* Effect.log(`[resolve] ${bundle.lockName}`);
    yield* Effect.log(describeBundle(bundle));
  });

export const printPlan = (
  state: Pick<State, "target" | "overrides" | "topology">,
  fromStep?: StepName,
) =>
  Effect.gen(function* () {
    yield* Effect.log(`[plan] target=${state.target}`);
    if (state.overrides.length) {
      yield* Effect.log(
        `[plan] overrides=${state.overrides.map(describeOverride).join(", ")}`,
      );
      for (const warning of overrideWarnings(state.overrides, state.target)) {
        yield* Effect.log(`[warn] ${warning}`);
      }
    }
    yield* Effect.log(
      `[plan] topology=n${state.topology.count}/t${state.topology.threshold}`,
    );
    yield* Effect.log(
      `[plan] steps=${STEP_NAMES.slice(stateStepIndex(fromStep ?? STEP_NAMES[0])).join(" -> ")}`,
    );
  });

export const shellEscape = (value: string) =>
  `'${value.replaceAll("'", `'\\''`)}'`;

// ---------------------------------------------------------------------------
// Partial schema overrides helper
// ---------------------------------------------------------------------------

const partialSchemaOverrides = (overrides: LocalOverride[]) =>
  overrides.filter(
    (item): item is LocalOverride & { services: string[] } =>
      !!item.services?.length && SCHEMA_COUPLED_GROUPS.includes(item.group),
  );

// ---------------------------------------------------------------------------
// Effect helpers that use services
// ---------------------------------------------------------------------------

export const minioIp = Effect.gen(function* () {
  const cmd = yield* CommandRunner;
  const result = yield* cmd.run(["docker", "inspect", "fhevm-minio"], {
    allowFailure: true,
  });
  if (result.code !== 0) {
    return yield* Effect.fail(
      new PreflightError({ message: "Could not determine MinIO IP" }),
    );
  }
  const inspected = JSON.parse(result.stdout) as Array<{
    NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
  }>;
  const ip = inspected[0]
    ? Object.values(inspected[0].NetworkSettings.Networks)[0]?.IPAddress
    : "";
  if (!ip) {
    return yield* Effect.fail(
      new PreflightError({ message: "Could not determine MinIO IP" }),
    );
  }
  return ip;
});

export const defaultEndpoints = Effect.gen(function* () {
  const ip = yield* minioIp;
  return {
    gatewayHttp: "http://gateway-node:8546",
    gatewayWs: "ws://gateway-node:8546",
    hostHttp: "http://host-node:8545",
    hostWs: "ws://host-node:8545",
    minioInternal: "http://minio:9000",
    minioExternal: `http://${ip}:9000`,
  };
});

export const discoverContracts = Effect.gen(function* () {
  const gwExists = yield* Effect.promise(() => exists(gatewayAddressesPath));
  const hostExists = yield* Effect.promise(() => exists(hostAddressesPath));
  if (!gwExists || !hostExists) {
    return yield* Effect.fail(
      new PreflightError({ message: "Missing generated address files under .fhevm/addresses" }),
    );
  }
  return {
    gateway: yield* Effect.promise(() => readEnvFile(gatewayAddressesPath)),
    host: yield* Effect.promise(() => readEnvFile(hostAddressesPath)),
  };
});

export const preflight = (
  state: State,
  strictPorts = true,
  needsGitHub = true,
) =>
  Effect.gen(function* () {
    const cmd = yield* CommandRunner;
    const requiredCommands = [
      "bun",
      "docker",
      ...(needsGitHub ? ["gh"] : []),
      ...(state.topology.count > 1 ? ["cast"] : []),
    ];
    const whichResults = yield* Effect.all(
      requiredCommands.map((command) =>
        cmd.run(["which", command], { allowFailure: true }).pipe(
          Effect.map((r) => ({ command, found: r.code === 0 })),
          Effect.catchAll(() => Effect.succeed({ command, found: false })),
        ),
      ),
      { concurrency: "unbounded" },
    );
    for (const { command, found } of whichResults) {
      if (!found) {
        return yield* Effect.fail(
          new PreflightError({
            message: `Required command "${command}" not found`,
          }),
        );
      }
    }
    const projectPorts = yield* cmd
      .run(
        [
          "docker",
          "ps",
          "--filter",
          `label=com.docker.compose.project=${PROJECT}`,
          "--format",
          "{{.Ports}}",
        ],
        { allowFailure: true },
      )
      .pipe(
        Effect.catchAll(() =>
          Effect.succeed({ stdout: "", stderr: "", code: 1 }),
        ),
      );
    const portResults = yield* Effect.all(
      PORTS.map((port) =>
        cmd
          .run(["lsof", "-nP", `-iTCP:${port}`, "-sTCP:LISTEN"], {
            allowFailure: true,
          })
          .pipe(
            Effect.map((r) => ({ port, busy: r })),
            Effect.catchAll(() =>
              Effect.succeed({ port, busy: { stdout: "", stderr: "", code: 1 } }),
            ),
          ),
      ),
      { concurrency: "unbounded" },
    );
    for (const { port, busy } of portResults) {
      if (
        busy.code === 0 &&
        busy.stdout.trim() &&
        !projectPorts.stdout.includes(`:${port}->`)
      ) {
        const message = `Port ${port} is already in use\n${busy.stdout}`;
        if (strictPorts) {
          return yield* Effect.fail(new PreflightError({ message }));
        }
        yield* Effect.log(`[preflight] warning: ${message}`);
      }
    }
  });

export const assertSchemaCompatibility = (
  bundle: VersionBundle,
  overrides: LocalOverride[],
  allowSchemaMismatch: boolean,
) =>
  Effect.gen(function* () {
    if (allowSchemaMismatch || !SCHEMA_GUARD_TARGETS.has(bundle.target)) {
      return;
    }
    const cmd = yield* CommandRunner;
    for (const item of partialSchemaOverrides(overrides)) {
      const guard =
        SCHEMA_GUARDS[item.group as keyof typeof SCHEMA_GUARDS];
      if (!guard) {
        continue;
      }
      const ref = bundle.env[guard.versionKey];
      if (!ref) {
        continue;
      }
      const verified = yield* cmd.run(
        ["git", "rev-parse", "-q", "--verify", `${ref}^{commit}`],
        { cwd: REPO_ROOT, allowFailure: true },
      );
      if (verified.code !== 0) {
        return yield* Effect.fail(
          new SchemaGuardError({
            group: item.group,
            message:
              `Cannot compare local ${item.group} migrations against ${ref}; local git ref is missing. ` +
              `Run \`git fetch --tags\` or pass --allow-schema-mismatch.`,
          }),
        );
      }
      const untracked = yield* cmd.run(
        [
          "git",
          "ls-files",
          "--others",
          "--exclude-standard",
          "--",
          guard.repoPath,
        ],
        { cwd: REPO_ROOT, allowFailure: true },
      );
      if (untracked.code !== 0) {
        return yield* Effect.fail(
          new SchemaGuardError({
            group: item.group,
            message: `Failed to inspect local ${item.group} migrations`,
          }),
        );
      }
      if (untracked.stdout.trim()) {
        return yield* Effect.fail(
          new SchemaGuardError({
            group: item.group,
            message:
              `${item.group}: local DB migrations diverge from ${ref}. ` +
              `Use --override ${item.group} or pass --allow-schema-mismatch if you know this service remains compatible.`,
          }),
        );
      }
      const diff = yield* cmd.run(
        ["git", "diff", "--quiet", "--exit-code", ref, "--", guard.repoPath],
        { cwd: REPO_ROOT, allowFailure: true },
      );
      if (diff.code === 0) {
        continue;
      }
      if (diff.code === 1) {
        return yield* Effect.fail(
          new SchemaGuardError({
            group: item.group,
            message:
              `${item.group}: local DB migrations diverge from ${ref}. ` +
              `Use --override ${item.group} or pass --allow-schema-mismatch if you know this service remains compatible.`,
          }),
        );
      }
      return yield* Effect.fail(
        new SchemaGuardError({
          group: item.group,
          message: `Failed to compare local ${item.group} migrations against ${ref}`,
        }),
      );
    }
  });

export const ensureRuntimeArtifacts = (state: State, reason: string) =>
  Effect.gen(function* () {
    const versionsExists = yield* Effect.promise(() =>
      exists(versionsEnvPath),
    );
    const composeExist = yield* Effect.promise(() =>
      Promise.all(COMPONENTS.map((component) => exists(composePath(component)))),
    );
    const missing = !versionsExists || !composeExist.every(Boolean);
    if (!missing) {
      return;
    }
    yield* Effect.log(`[regen] restoring runtime artifacts for ${reason}`);
    yield* regen(state);
  });

export const resetAfterStep = (step: StepName) =>
  Effect.gen(function* () {
    const containerRunner = yield* ContainerRunner;
    const start = stateStepIndex(step);
    const failed: string[] = [];
    for (let index = STEP_NAMES.length - 1; index >= start; index -= 1) {
      for (const component of COMPONENT_BY_STEP[STEP_NAMES[index]]) {
        const ok = yield* containerRunner.composeDown(component);
        if (!ok) {
          failed.push(component);
        }
      }
    }
    if (failed.length) {
      return yield* Effect.fail(
        new PreflightError({
          message: `Failed to stop components while resetting from ${step}: ${failed.join(", ")}`,
        }),
      );
    }
  });

export const resolveUpgradePlan = (
  state: Pick<State, "overrides" | "topology">,
  groupValue: string | undefined,
) => {
  if (
    !groupValue ||
    !UPGRADEABLE_GROUPS.includes(groupValue as UpgradeGroup)
  ) {
    throw new Error(
      `upgrade expects one of ${UPGRADEABLE_GROUPS.join(", ")}`,
    );
  }
  const group = groupValue as UpgradeGroup;
  if (!state.overrides.some((item) => item.group === group)) {
    throw new Error(
      `upgrade requires an active local override for ${group}`,
    );
  }
  const [component] = GROUP_BUILD_COMPONENTS[group];
  if (!component) {
    throw new Error(`No runtime component registered for ${group}`);
  }
  const groupOverrides = state.overrides.filter(
    (item) => item.group === group,
  );
  const selectedServices = groupOverrides.flatMap(
    (item) => item.services ?? [],
  );
  const restartableServices = (services: string[]) =>
    services.filter((service) => !service.endsWith("-db-migration"));
  const plannedServices =
    group === "coprocessor"
      ? coprocessorServicesForOverrides(state, selectedServices)
      : selectedServices.length
        ? [...new Set(selectedServices)]
        : GROUP_BUILD_SERVICES[group];
  const services = restartableServices(plannedServices);
  if (!services.length) {
    throw new Error(
      `upgrade requires restartable runtime services for ${group}`,
    );
  }
  return {
    component,
    group,
    services,
    step: group === "coprocessor" ? "coprocessor" : group,
  } as const;
};

// ---------------------------------------------------------------------------
// Waiting helpers (use ContainerProbe + MinioClient + RpcClient)
// ---------------------------------------------------------------------------

export const waitForCoprocessor = (state: State) =>
  Effect.gen(function* () {
    const probe = yield* ContainerProbe;
    for (let index = 0; index < state.topology.count; index += 1) {
      yield* probe.waitForComplete(toServiceName("db-migration", index));
      yield* probe.waitForRunning(toServiceName("host-listener", index));
      yield* probe.waitForRunning(toServiceName("gw-listener", index));
      yield* probe.waitForRunning(toServiceName("tfhe-worker", index));
      yield* probe.waitForRunning(toServiceName("zkproof-worker", index));
      yield* probe.waitForRunning(toServiceName("sns-worker", index));
      yield* probe.waitForRunning(
        toServiceName("transaction-sender", index),
      );
    }
  });

export const waitForKmsConnector = Effect.gen(function* () {
  const probe = yield* ContainerProbe;
  yield* probe.waitForComplete("kms-connector-db-migration");
  yield* probe.waitForRunning("kms-connector-gw-listener");
  yield* probe.waitForRunning("kms-connector-kms-worker");
  yield* probe.waitForRunning("kms-connector-tx-sender");
});

const waitForTestSuite = Effect.gen(function* () {
  const probe = yield* ContainerProbe;
  yield* probe.waitForRunning("fhevm-test-suite-e2e-debug");
});

// ---------------------------------------------------------------------------
// probeBootstrap — port of the original probeBootstrap
// ---------------------------------------------------------------------------

export const probeBootstrap = (state: State) =>
  Effect.gen(function* () {
    const minio = yield* MinioClient;
    const discovery = state.discovery!;
    const kp = discovery.minioKeyPrefix ?? "PUB";
    const result = yield* minio.probeBootstrap(
      discovery.endpoints.gatewayHttp,
      discovery.gateway.KMS_GENERATION_ADDRESS,
      discovery.endpoints.minioExternal,
      kp,
    );
    if (!result) {
      return false;
    }
    // Ensure material is available (parallel)
    yield* Effect.all(
      [
        minio.ensureMaterial(
          hostReachableMaterialUrl(
            `${discovery.endpoints.minioExternal}/kms-public/${kp}/PublicKey/${result.actualFheKeyId}`,
          ),
        ),
        minio.ensureMaterial(
          hostReachableMaterialUrl(
            `${discovery.endpoints.minioExternal}/kms-public/${kp}/CRS/${result.actualCrsKeyId}`,
          ),
        ),
      ],
      { concurrency: 2 },
    );
    if (
      discovery.fheKeyId !== result.actualFheKeyId ||
      discovery.crsKeyId !== result.actualCrsKeyId
    ) {
      return yield* Effect.fail(
        new PreflightError({
          message: `Predicted bootstrap ids drifted: expected ${discovery.fheKeyId}/${discovery.crsKeyId}, got ${result.actualFheKeyId}/${result.actualCrsKeyId}`,
        }),
      );
    }
    state.discovery!.actualFheKeyId = result.actualFheKeyId;
    state.discovery!.actualCrsKeyId = result.actualCrsKeyId;
    return true;
  });

// waitForBootstrap: retry probeBootstrap up to `attempts` times
const waitForBootstrap = (state: State, attempts = 120) =>
  probeBootstrap(state).pipe(
    Effect.filterOrFail(
      (ok) => ok,
      () => "not-ready" as const,
    ),
    Effect.retry({
      while: (e): e is "not-ready" => e === "not-ready",
      schedule: Schedule.spaced("2 seconds").pipe(
        Schedule.compose(Schedule.recurs(attempts - 1)),
      ),
    }),
    Effect.tapError(() => Effect.log("[wait] bootstrap materials")),
    Effect.catchAll((error) =>
      typeof error === "string"
        ? Effect.fail(new BootstrapTimeout({ elapsed: attempts * 2 }))
        : Effect.fail(error),
    ),
    Effect.asVoid,
  );

// maybeBuild + composeUp shorthand
const stepComposeUp = (
  component: string,
  state: State,
  services?: string[],
  options?: { noDeps?: boolean },
) =>
  Effect.gen(function* () {
    const stateManager = yield* StateManager;
    const imageBuilder = yield* ImageBuilder;
    const containerRunner = yield* ContainerRunner;
    yield* imageBuilder.maybeBuild(component, state, (s) => stateManager.save(s));
    yield* containerRunner.composeUp(component, services, options);
  });

// ---------------------------------------------------------------------------
// runStep — dispatch a single pipeline step
// ---------------------------------------------------------------------------

export const runStep = (state: State, step: StepName) =>
  Effect.gen(function* () {
    const probe = yield* ContainerProbe;
    const stateManager = yield* StateManager;

    yield* Effect.log(`[step] ${step}`);

    switch (step) {
      case "preflight":
        yield* preflight(state);
        break;

      case "resolve":
        yield* printBundle(state.versions);
        break;

      case "generate":
        yield* regen(state);
        break;

      case "base":
        yield* stepComposeUp("minio", state);
        yield* probe.waitForHealthy("fhevm-minio");
        yield* probe.waitForComplete("fhevm-minio-setup");
        yield* stepComposeUp("core", state);
        yield* probe.waitForLog("kms-core", /KMS Server service socket address/);
        yield* stepComposeUp("database", state);
        yield* probe.waitForHealthy("coprocessor-and-kms-db");
        yield* stepComposeUp("host-node", state);
        yield* probe.waitForRpc("http://localhost:8545");
        yield* stepComposeUp("gateway-node", state);
        yield* probe.waitForRpc("http://localhost:8546");
        state.discovery = {
          gateway: {},
          host: {},
          kmsSigner: "",
          fheKeyId: predictedKeyId(),
          crsKeyId: predictedCrsId(),
          endpoints: yield* defaultEndpoints,
        };
        yield* regen(state);
        break;

      case "kms-signer": {
        state.discovery ??= {
          gateway: {},
          host: {},
          kmsSigner: "",
          fheKeyId: predictedKeyId(),
          crsKeyId: predictedCrsId(),
          endpoints: yield* defaultEndpoints,
        };
        const minio = yield* MinioClient;
        const signer = yield* minio.discoverSigner();
        state.discovery.kmsSigner = signer.address;
        state.discovery.minioKeyPrefix = signer.minioKeyPrefix;
        yield* regen(state);
        break;
      }

      case "gateway-deploy":
        yield* stepComposeUp("gateway-mocked-payment", state, [
          "gateway-deploy-mocked-zama-oft",
        ]);
        yield* probe.waitForComplete("gateway-deploy-mocked-zama-oft");
        yield* stepComposeUp("gateway-sc", state, ["gateway-sc-deploy"]);
        yield* probe.waitForComplete(
          "gateway-sc-deploy",
          "Contract deployment done!",
        );
        state.discovery = {
          gateway: yield* Effect.promise(() =>
            readEnvFile(gatewayAddressesPath),
          ),
          host: state.discovery?.host ?? {},
          kmsSigner: state.discovery?.kmsSigner ?? "",
          fheKeyId: state.discovery?.fheKeyId ?? predictedKeyId(),
          crsKeyId: state.discovery?.crsKeyId ?? predictedCrsId(),
          actualFheKeyId: state.discovery?.actualFheKeyId,
          actualCrsKeyId: state.discovery?.actualCrsKeyId,
          minioKeyPrefix: state.discovery?.minioKeyPrefix,
          endpoints:
            state.discovery?.endpoints ?? (yield* defaultEndpoints),
        };
        yield* regen(state);
        yield* stepComposeUp(
          "gateway-mocked-payment",
          state,
          ["gateway-set-relayer-mocked-payment"],
          { noDeps: true },
        );
        yield* probe.waitForComplete(
          "gateway-set-relayer-mocked-payment",
        );
        break;

      case "host-deploy":
        yield* stepComposeUp("host-sc", state, ["host-sc-deploy"]);
        yield* probe.waitForComplete(
          "host-sc-deploy",
          "Contract deployment done!",
        );
        break;

      case "discover": {
        const contracts = yield* discoverContracts;
        state.discovery = {
          gateway: contracts.gateway,
          host: contracts.host,
          kmsSigner: state.discovery?.kmsSigner ?? "",
          fheKeyId: state.discovery?.fheKeyId ?? predictedKeyId(),
          crsKeyId: state.discovery?.crsKeyId ?? predictedCrsId(),
          actualFheKeyId: state.discovery?.actualFheKeyId,
          actualCrsKeyId: state.discovery?.actualCrsKeyId,
          minioKeyPrefix: state.discovery?.minioKeyPrefix,
          endpoints:
            state.discovery?.endpoints ?? (yield* defaultEndpoints),
        };
        break;
      }

      case "regenerate":
        yield* regen(state);
        break;

      case "validate": {
        yield* validateDiscovery(state);
        const incompatibilities = validateBundleCompatibility(state);
        if (incompatibilities.length) {
          return yield* Effect.fail(
            new IncompatibleVersions({
              issues: incompatibilities.map((i) => i.message),
            }),
          );
        }
        break;
      }

      case "coprocessor":
        yield* stepComposeUp(
          "coprocessor",
          state,
          serviceNameList(state, "coprocessor"),
        );
        yield* waitForCoprocessor(state);
        yield* probe.postBootHealthGate(
          coprocessorHealthContainers(state),
        );
        break;

      case "kms-connector":
        yield* stepComposeUp("kms-connector", state);
        yield* waitForKmsConnector;
        yield* probe.postBootHealthGate(KMS_CONNECTOR_HEALTH_CONTAINERS);
        break;

      case "bootstrap": {
        yield* stepComposeUp(
          "gateway-sc",
          state,
          ["gateway-sc-add-network"],
          { noDeps: true },
        );
        yield* probe.waitForComplete("gateway-sc-add-network");

        const bootstrapDone = yield* probeBootstrap(state).pipe(
          Effect.catchTag("MinioError", () => Effect.succeed(false)),
        );
        if (bootstrapDone) {
          yield* regen(state);
          break;
        }

        const [hostEnv, gatewayEnv] = yield* Effect.all(
          [
            Effect.promise(() => readEnvFile(envPath("host-sc"))),
            Effect.promise(() => readEnvFile(envPath("gateway-sc"))),
          ],
          { concurrency: 2 },
        );
        const rpc = yield* RpcClient;

        // Check host pauser
        const hostPauserRegistered = yield* rpc.castBool(
          state.discovery!.endpoints.hostHttp,
          withHexPrefix(state.discovery!.host.PAUSER_SET_CONTRACT_ADDRESS),
          "isPauser(address)(bool)",
          withHexPrefix(hostEnv.PAUSER_ADDRESS_0),
        ).pipe(Effect.catchAll(() => Effect.succeed(false)));

        if (!hostPauserRegistered) {
          yield* stepComposeUp(
            "host-sc",
            state,
            ["host-sc-add-pausers"],
            { noDeps: true },
          );
          yield* probe.waitForComplete("host-sc-add-pausers");
        }

        // Check gateway pauser
        const gatewayPauserRegistered = yield* rpc.castBool(
          state.discovery!.endpoints.gatewayHttp,
          withHexPrefix(gatewayEnv.PAUSER_SET_ADDRESS),
          "isPauser(address)(bool)",
          withHexPrefix(gatewayEnv.PAUSER_ADDRESS_0),
        ).pipe(Effect.catchAll(() => Effect.succeed(false)));

        if (!gatewayPauserRegistered) {
          yield* stepComposeUp(
            "gateway-sc",
            state,
            ["gateway-sc-add-pausers"],
            { noDeps: true },
          );
          yield* probe.waitForComplete("gateway-sc-add-pausers");
        }

        yield* stepComposeUp(
          "gateway-sc",
          state,
          ["gateway-sc-trigger-keygen"],
          { noDeps: true },
        );
        yield* probe.waitForComplete("gateway-sc-trigger-keygen");

        yield* stepComposeUp(
          "gateway-sc",
          state,
          ["gateway-sc-trigger-crsgen"],
          { noDeps: true },
        );
        yield* probe.waitForComplete("gateway-sc-trigger-crsgen");

        yield* waitForBootstrap(state);
        yield* regen(state);
        break;
      }

      case "relayer":
        yield* stepComposeUp("relayer", state);
        yield* probe.waitForHealthy("fhevm-relayer-db");
        yield* probe.waitForRunning("fhevm-relayer");
        yield* probe.waitForLog(
          "fhevm-relayer",
          /All servers are ready and responding/,
        );
        break;

      case "test-suite":
        yield* stepComposeUp("test-suite", state);
        yield* waitForTestSuite;
        break;
    }

    yield* stateManager.markStep(state, step);
  });

// ---------------------------------------------------------------------------
// runWithHeartbeat — run a long process with idle-time heartbeat logging
// ---------------------------------------------------------------------------

export const runWithHeartbeat = (
  argv: string[],
  label: string,
) =>
  Effect.gen(function* () {
    yield* Effect.promise(
      () =>
        new Promise<void>((resolve, reject) => {
          const proc = Bun.spawn(argv, {
            stdin: "inherit",
            stdout: "pipe",
            stderr: "pipe",
            env: process.env as Record<string, string>,
          });

          let lastOutput = Date.now();
          let announced = 0;

          const pump = async (
            stream: ReadableStream<Uint8Array> | null,
            writer: NodeJS.WriteStream,
          ) => {
            if (!stream) return;
            const reader = stream.getReader();
            try {
              while (true) {
                const { done, value } = await reader.read();
                if (done) return;
                if (value?.length) {
                  lastOutput = Date.now();
                  writer.write(Buffer.from(value));
                }
              }
            } finally {
              reader.releaseLock();
            }
          };

          const timer = setInterval(() => {
            const silent = Date.now() - lastOutput;
            if (silent >= 15_000 && silent - announced >= 15_000) {
              announced = silent;
              console.log(
                `[wait] ${label} still running (${Math.floor(silent / 1000)}s)`,
              );
            }
          }, 5_000);

          Promise.all([
            proc.exited,
            pump(proc.stdout, process.stdout),
            pump(proc.stderr, process.stderr),
          ])
            .then(([code]) => {
              clearInterval(timer);
              if (code !== 0) {
                reject(new Error(`${argv.join(" ")} failed (${code})`));
              } else {
                resolve();
              }
            })
            .catch((error) => {
              clearInterval(timer);
              proc.kill();
              reject(error);
            });
        }),
    );
  });

/**
 * Runs named e2e test profiles, light-suite orchestration, and topology-specific test flows.
 */
import { compatPolicyForState } from "../compat/compat";
import { DRIFT_CLEANUP_SQL, DRIFT_INSTALL_SQL, driftDatabaseName, parseDriftInstanceIndex, parsePositiveInteger } from "../drift";
import { PreflightError } from "../errors";
import { pause, shellEscape, unpause } from "../flow/up-flow";
import { run, runWithHeartbeat } from "../utils/process";
import { loadState } from "../state/state";
import { topologyForState } from "../stack-spec/stack-spec";
import {
  COPROCESSOR_DB_CONTAINER,
  LIGHT_TEST_PROFILES,
  TEST_GREP,
  TEST_PARALLEL,
  TEST_SUITE_CONTAINER,
} from "../layout";
import type { TestOptions } from "../types";

const DRIFT_WARNING = '"message":"Drift detected: observed multiple digest variants for handle"';
const DRIFT_HANDLE = /"handle":"0x([0-9a-f]+)"/i;
const DB_REVERT_CONTAINERS = [
  "host-listener",
  "host-listener-poller",
  "gw-listener",
  "tfhe-worker",
  "sns-worker",
  "transaction-sender",
  "zkproof-worker",
] as const;
const DEFAULT_DB_REVERT_CHAIN_ID = "12345";
const DEFAULT_DB_REVERT_TESTS = "test add 42 to uint64 input and decrypt";

/** Formats a progress label with elapsed wall-clock time. */
const timedLabel = (label: string, started: number) =>
  `${label} (${Math.round((Date.now() - started) / 1000)}s)`;

const TEST_PROFILE_NAMES = [...Object.keys(TEST_GREP), "ciphertext-drift", "coprocessor-db-state-revert", "light"].sort();

/** Logs pass/fail timing around one test task. */
const runLogged = async <T>(label: string, started: number, task: () => Promise<T>) => {
  try {
    const result = await task();
    console.log(`[pass] ${timedLabel(label, started)}`);
    return result;
  } catch (error) {
    console.log(`[fail] ${timedLabel(label, started)}`);
    throw error;
  }
};

/** Executes psql inside the coprocessor database container. */
const psql = async (
  dbName: string,
  sqlArgs: string[],
  options: {
    postgresContainer: string;
    postgresUser: string;
    postgresPassword: string;
  },
  input?: string,
) => {
  const result = await run(
    [
      "docker",
      "exec",
      ...(input ? ["-i"] : []),
      "-e",
      `PGPASSWORD=${options.postgresPassword}`,
      options.postgresContainer,
      "psql",
      "-U",
      options.postgresUser,
      "-d",
      dbName,
      ...sqlArgs,
    ],
    { input },
  );
  return result.stdout.trim();
};

/** Installs and waits for the SQL trigger that injects ciphertext drift. */
const injectCiphertextDrift = async (options: {
  instanceIndex: number;
  timeoutSeconds: number;
  pollIntervalSeconds: number;
  postgresContainer: string;
  postgresUser: string;
  postgresPassword: string;
}) => {
  const dbName = driftDatabaseName(options.instanceIndex);
  await psql(dbName, [], options, DRIFT_INSTALL_SQL);
  try {
    const attempts = Math.max(0, Math.ceil(options.timeoutSeconds / options.pollIntervalSeconds));
    for (let attempt = 0; attempt <= attempts; attempt += 1) {
      const consumed = await psql(
        dbName,
        ["-t", "-A", "-c", "SELECT consumed::int FROM drift_injection_state WHERE id = TRUE;"],
        options,
      );
      if (consumed === "1") {
        const handleHex = await psql(
          dbName,
          ["-t", "-A", "-c", "SELECT encode(injected_handle, 'hex') FROM drift_injection_state WHERE id = TRUE;"],
          options,
        );
        if (handleHex) {
          return handleHex;
        }
      }
      if (attempt === attempts) {
        throw new PreflightError(`timed out waiting for drift injection trigger to fire in ${dbName}`);
      }
      await Bun.sleep(options.pollIntervalSeconds * 1000);
    }
    throw new PreflightError(`timed out waiting for drift injection trigger to fire in ${dbName}`);
  } finally {
    await psql(dbName, [], options, DRIFT_CLEANUP_SQL).catch(() => undefined);
  }
};

/** Lists running coprocessor gateway-listener containers. */
const coprocessorGwListeners = async () => {
  const result = await run(["docker", "ps", "--format", "{{.Names}}"], { allowFailure: true });
  if (result.code !== 0) {
    throw new PreflightError(result.stderr.trim() || "docker ps failed");
  }
  return result.stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => /^coprocessor(\d+)?-gw-listener$/.test(line));
};

/** Finds the expected drift warning for a specific injected handle. */
const findDriftWarning = (output: string, expectedHandleHex: string) => {
  for (const line of output.split(/\r?\n/)) {
    if (!line.includes(DRIFT_WARNING)) {
      continue;
    }
    const matchedHandle = line.match(DRIFT_HANDLE)?.[1];
    if (matchedHandle?.toLowerCase() === expectedHandleHex.toLowerCase()) {
      return matchedHandle;
    }
  }
  return undefined;
};

/** Waits until a gateway listener emits the expected drift warning. */
const waitForDriftWarning = async (
  handleHex: string,
  options: { since: string; timeoutSeconds: number; pollIntervalSeconds: number },
) => {
  const attempts = Math.max(0, Math.ceil(options.timeoutSeconds / options.pollIntervalSeconds));
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    const containers = await coprocessorGwListeners();
    for (const container of containers) {
      const logs = await run(["docker", "logs", "--since", options.since, container], { allowFailure: true });
      const matched = findDriftWarning(logs.stdout + logs.stderr, handleHex);
      if (matched) {
        return { container, handleHex: matched };
      }
    }
    if (attempt === attempts) {
      throw new PreflightError(`drift warning was not observed after injecting handle ${handleHex}`);
    }
    await Bun.sleep(options.pollIntervalSeconds * 1000);
  }
  throw new PreflightError(`drift warning was not observed after injecting handle ${handleHex}`);
};

/** Builds the `docker exec` argv used to run tests inside the test-suite container. */
export const buildTestContainerArgs = (tail: string[], extraExecArgs: string[] = []) => [
  "docker",
  "exec",
  "-e",
  "npm_config_update_notifier=false",
  "-e",
  "NPM_CONFIG_UPDATE_NOTIFIER=false",
  ...extraExecArgs,
  TEST_SUITE_CONTAINER,
  ...tail,
];

/** Runs a narrow e2e grep inside the test-suite container. */
const runNamedE2e = async (network: string, grep: string, label: string) =>
  runWithHeartbeat(buildTestContainerArgs(["./run-tests.sh", "-n", network, "-g", grep]), label);

/** Builds the coprocessor runtime container names for every configured instance. */
const coprocessorRuntimeContainers = (instanceCount: number) =>
  Array.from({ length: instanceCount }, (_, index) => {
    const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
    return DB_REVERT_CONTAINERS.map((suffix) => `${prefix}${suffix}`);
  }).flat();

/** Stops or starts each named container and ignores already-stopped/missing cases. */
const setContainersRunning = async (containers: string[], action: "start" | "stop") => {
  for (const container of containers) {
    await run(["docker", action, container], { allowFailure: true });
  }
};

/** Reads a one-line postgres query result as trimmed text. */
const scalarQuery = async (
  dbName: string,
  sql: string,
  options: {
    postgresContainer: string;
    postgresUser: string;
    postgresPassword: string;
  },
) => psql(dbName, ["-t", "-A", "-c", sql], options);

type DbRevertSnapshot = {
  computationsDone: number;
  computationsTotal: number;
  allowedHandles: number;
  pbsComputations: number;
  ciphertextDigest: number;
  ciphertexts: number;
  ciphertexts128: number;
  erroredComputations?: number;
};

/** Snapshots the revert-sensitive coprocessor tables for one host chain. */
const dbRevertSnapshot = async (
  chainId: string,
  options: {
    postgresContainer: string;
    postgresUser: string;
    postgresPassword: string;
    postgresDb: string;
  },
): Promise<DbRevertSnapshot> => {
  const query = (sql: string) => scalarQuery(options.postgresDb, sql, options).then((value) => Number(value || "0"));
  return {
    computationsDone: await query(`SELECT COUNT(*) FROM computations WHERE is_completed = true AND host_chain_id = ${chainId}`),
    computationsTotal: await query(`SELECT COUNT(*) FROM computations WHERE host_chain_id = ${chainId}`),
    allowedHandles: await query(`SELECT COUNT(*) FROM allowed_handles WHERE host_chain_id = ${chainId}`),
    pbsComputations: await query(`SELECT COUNT(*) FROM pbs_computations WHERE host_chain_id = ${chainId}`),
    ciphertextDigest: await query(`SELECT COUNT(*) FROM ciphertext_digest WHERE host_chain_id = ${chainId}`),
    ciphertexts: await query(`SELECT COUNT(*) FROM ciphertexts WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = ${chainId})`),
    ciphertexts128: await query(`SELECT COUNT(*) FROM ciphertexts128 WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = ${chainId})`),
    erroredComputations: await query(`SELECT COUNT(*) FROM computations WHERE is_error = true AND host_chain_id = ${chainId}`),
  };
};

/** Formats a compact database revert progress summary. */
const formatDbRevertSnapshot = (snapshot: DbRevertSnapshot) =>
  `comp=${snapshot.computationsDone}/${snapshot.computationsTotal} acl=${snapshot.allowedHandles} pbs=${snapshot.pbsComputations} digest=${snapshot.ciphertextDigest} ct=${snapshot.ciphertexts} ct128=${snapshot.ciphertexts128} err=${snapshot.erroredComputations ?? 0}`;

/** Ensures the revert step actually deleted some chain-scoped coprocessor data. */
const assertRevertDeletedData = (before: DbRevertSnapshot, after: DbRevertSnapshot) => {
  const unchanged = [
    after.computationsTotal >= before.computationsTotal ? "computations" : "",
    before.allowedHandles > 0 && after.allowedHandles >= before.allowedHandles ? "allowed_handles" : "",
    before.pbsComputations > 0 && after.pbsComputations >= before.pbsComputations ? "pbs_computations" : "",
    before.ciphertextDigest > 0 && after.ciphertextDigest >= before.ciphertextDigest ? "ciphertext_digest" : "",
    before.ciphertexts > 0 && after.ciphertexts >= before.ciphertexts ? "ciphertexts" : "",
    before.ciphertexts128 > 0 && after.ciphertexts128 >= before.ciphertexts128 ? "ciphertexts128" : "",
  ].filter(Boolean);
  if (unchanged.length) {
    throw new PreflightError(`db-state-revert did not delete expected data for: ${unchanged.join(", ")}`);
  }
};

/** Waits for the coprocessor to repopulate revert-sensitive tables after a rollback. */
const waitForDbRevertRecovery = async (
  before: DbRevertSnapshot,
  chainId: string,
  options: {
    timeoutSeconds: number;
    pollIntervalSeconds: number;
    postgresContainer: string;
    postgresUser: string;
    postgresPassword: string;
    postgresDb: string;
  },
) => {
  const attempts = Math.max(0, Math.ceil(options.timeoutSeconds / options.pollIntervalSeconds));
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    const snapshot = await dbRevertSnapshot(chainId, options);
    console.log(`[revert] recovery ${formatDbRevertSnapshot(snapshot)}`);
    if ((snapshot.erroredComputations ?? 0) > 0) {
      throw new PreflightError(`db-state-revert found ${snapshot.erroredComputations} errored computations after restart`);
    }
    if (
      snapshot.computationsDone >= before.computationsDone &&
      snapshot.computationsTotal >= before.computationsTotal &&
      snapshot.allowedHandles >= before.allowedHandles &&
      snapshot.pbsComputations >= before.pbsComputations &&
      snapshot.ciphertextDigest >= before.ciphertextDigest &&
      snapshot.ciphertexts >= before.ciphertexts &&
      snapshot.ciphertexts128 >= before.ciphertexts128
    ) {
      return;
    }
    if (attempt === attempts) {
      throw new PreflightError(`db-state-revert timed out waiting for coprocessor recovery: ${formatDbRevertSnapshot(snapshot)}`);
    }
    await Bun.sleep(options.pollIntervalSeconds * 1000);
  }
};

/** Runs the coprocessor DB state revert e2e flow against the active stack. */
const runDbStateRevert = async (
  state: Awaited<ReturnType<typeof loadState>>,
  options: TestOptions,
) => {
  if (!state) {
    throw new PreflightError("Stack has not completed bootstrap; run `fhevm-cli up` first");
  }
  const started = Date.now();
  const chainId = process.env.CHAIN_ID ?? DEFAULT_DB_REVERT_CHAIN_ID;
  if (!/^\d+$/.test(chainId)) {
    throw new PreflightError(`Invalid CHAIN_ID ${chainId}; expected a positive integer`);
  }
  const postgres = {
    postgresContainer: process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
    postgresUser: process.env.POSTGRES_USER ?? "postgres",
    postgresPassword: process.env.POSTGRES_PASSWORD ?? "postgres",
    postgresDb: process.env.POSTGRES_DB ?? "coprocessor",
  };
  const testsToRun = process.env.TESTS_TO_RUN ?? DEFAULT_DB_REVERT_TESTS;
  const timeoutSeconds = parsePositiveInteger(process.env.REVERT_POLL_TIMEOUT_SECONDS ?? "300", "REVERT_POLL_TIMEOUT_SECONDS");
  const pollIntervalSeconds = parsePositiveInteger(process.env.REVERT_POLL_INTERVAL_SECONDS ?? "2", "REVERT_POLL_INTERVAL_SECONDS");
  const containers = coprocessorRuntimeContainers(topologyForState(state).count);
  const migrationVersion = state.versions.env.COPROCESSOR_DB_MIGRATION_VERSION;
  if (!migrationVersion) {
    throw new PreflightError("db-state-revert requires COPROCESSOR_DB_MIGRATION_VERSION in the active stack state");
  }
  const revertImage = `ghcr.io/zama-ai/fhevm/coprocessor/db-migration:${migrationVersion}`;
  console.log("[test] coprocessor-db-state-revert");

  return runLogged("coprocessor-db-state-revert", started, async () => {
    await runNamedE2e(options.network, testsToRun, "test coprocessor-db-state-revert seed");

    const before = await dbRevertSnapshot(chainId, postgres);
    console.log(`[revert] before ${formatDbRevertSnapshot(before)}`);
    if (before.computationsDone === 0) {
      throw new PreflightError("db-state-revert found no completed computations; nothing to revert");
    }

    const maxBlock = Number(
      await scalarQuery(postgres.postgresDb, `SELECT COALESCE(MAX(block_number), 0) FROM transactions WHERE chain_id = ${chainId}`, postgres),
    );
    const revertTo = Math.floor(maxBlock / 2);
    if (revertTo <= 0) {
      throw new PreflightError(`db-state-revert requires a positive midpoint block; got max block ${maxBlock}`);
    }

    let stopped = false;
    try {
      await setContainersRunning(containers, "stop");
      stopped = true;

      const network = (
        await run([
          "docker",
          "inspect",
          postgres.postgresContainer,
          "--format",
          "{{range $k, $v := .NetworkSettings.Networks}}{{$k}}{{end}}",
        ])
      ).stdout.trim();
      if (!network) {
        throw new PreflightError(`db-state-revert could not resolve the docker network for ${postgres.postgresContainer}`);
      }

      await runWithHeartbeat(
        [
          "docker",
          "run",
          "--rm",
          "--network",
          network,
          "-e",
          `DATABASE_URL=postgres://${postgres.postgresUser}:${postgres.postgresPassword}@${postgres.postgresContainer}:5432/${postgres.postgresDb}`,
          "-e",
          `CHAIN_ID=${chainId}`,
          "-e",
          `TO_BLOCK_NUMBER=${revertTo}`,
          revertImage,
          "/revert_coprocessor_db_state.sh",
        ],
        "db-state-revert",
      );

      const after = await dbRevertSnapshot(chainId, postgres);
      console.log(`[revert] after ${formatDbRevertSnapshot(after)}`);
      assertRevertDeletedData(before, after);
    } finally {
      if (stopped) {
        await setContainersRunning(containers, "start");
      }
    }

    await waitForDbRevertRecovery(before, chainId, {
      ...postgres,
      timeoutSeconds,
      pollIntervalSeconds,
    });

    await runNamedE2e(options.network, testsToRun, "test coprocessor-db-state-revert verify");
  });
};

/** Runs a named test profile, custom grep, or the light-suite orchestration. */
export const test = async (testName: string | undefined, options: TestOptions) => {
  const state = await loadState();
  if (!state?.discovery?.actualFheKeyId) {
    throw new PreflightError("Stack has not completed bootstrap; run `fhevm-cli up` first");
  }

  const ciphertextDriftRequirement = () => {
    const topology = topologyForState(state);
    if (topology.count < 2) {
      return "ciphertext-drift requires a multi-coprocessor topology; rerun `fhevm-cli up --scenario two-of-two` first";
    }
    const faultyInstanceIndex = parseDriftInstanceIndex(process.env.FAULTY_INSTANCE_INDEX ?? "1");
    if (faultyInstanceIndex >= topology.count) {
      return `ciphertext-drift targets coprocessor instance ${faultyInstanceIndex}, but the current topology only has ${topology.count} instance${topology.count === 1 ? "" : "s"}`;
    }
    const compat = compatPolicyForState(state);
    if ((compat.coprocessorDropFlags["gw-listener"] ?? []).includes("--ciphertext-commits-address")) {
      return "ciphertext-drift requires a gw-listener build with drift addresses enabled; use latest-main or a newer supported bundle";
    }
    return undefined;
  };

  const runProfile = async (name: string) => {
    if (name === "coprocessor-db-state-revert") {
      return runDbStateRevert(state, options);
    }
    if (name === "ciphertext-drift") {
      console.log("[test] ciphertext-drift");
      const started = Date.now();
      const precondition = ciphertextDriftRequirement();
      if (precondition) {
        throw new PreflightError(precondition);
      }
      return runLogged("ciphertext-drift", started, async () => {
        const logSince = new Date().toISOString();
        const faultyInstanceIndex = parseDriftInstanceIndex(process.env.FAULTY_INSTANCE_INDEX ?? "1");
        const driftInjectTimeoutSeconds = parsePositiveInteger(process.env.DRIFT_INJECT_TIMEOUT_SECONDS ?? "180", "DRIFT_INJECT_TIMEOUT_SECONDS");
        const driftInjectPollIntervalSeconds = parsePositiveInteger(process.env.DRIFT_INJECT_POLL_INTERVAL_SECONDS ?? "2", "DRIFT_INJECT_POLL_INTERVAL_SECONDS");
        const driftAlertTimeoutSeconds = parsePositiveInteger(process.env.DRIFT_ALERT_TIMEOUT_SECONDS ?? "180", "DRIFT_ALERT_TIMEOUT_SECONDS");
        const driftAlertPollIntervalSeconds = parsePositiveInteger(process.env.DRIFT_ALERT_POLL_INTERVAL_SECONDS ?? "2", "DRIFT_ALERT_POLL_INTERVAL_SECONDS");
        const grepPattern = process.env.GREP_PATTERN ?? "test user input uint64 \\(non-trivial\\)";
        const injector = injectCiphertextDrift({
          instanceIndex: faultyInstanceIndex,
          timeoutSeconds: driftInjectTimeoutSeconds,
          pollIntervalSeconds: driftInjectPollIntervalSeconds,
          postgresContainer: process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
          postgresUser: process.env.POSTGRES_USER ?? "postgres",
          postgresPassword: process.env.POSTGRES_PASSWORD ?? "postgres",
        });
        await runWithHeartbeat(
          buildTestContainerArgs(["./run-tests.sh", "-n", "staging", "-g", grepPattern], ["-e", "GATEWAY_RPC_URL="]),
          "test ciphertext-drift",
        );
        const injectedHandleHex = await injector;
        const warning = await waitForDriftWarning(injectedHandleHex, {
          since: logSince,
          timeoutSeconds: driftAlertTimeoutSeconds,
          pollIntervalSeconds: driftAlertPollIntervalSeconds,
        });
        console.log(`[drift] detected in ${warning.container} for injected handle 0x${injectedHandleHex}`);
      });
    }

    const filter = TEST_GREP[name];
    if (!filter) {
      throw new PreflightError(`Unknown test profile ${name}. Valid: ${TEST_PROFILE_NAMES.join(", ")}`);
    }
    const shouldParallel = options.parallel ?? TEST_PARALLEL[name];
    console.log(`[test] ${name} (${options.network})`);
    const started = Date.now();
    const command = [
      "./run-tests.sh",
      options.verbose ? "-v" : "",
      shouldParallel ? "--parallel" : "",
      "-n",
      shellEscape(options.network),
      "-g",
      shellEscape(filter),
    ]
      .filter(Boolean)
      .join(" ");
    return runLogged(name, started, () =>
      runWithHeartbeat(buildTestContainerArgs(["sh", "-lc", command]), `test ${name}`),
    );
  };

  if (testName === "light") {
    if (options.grep) {
      throw new PreflightError("`fhevm-cli test light` does not accept `--grep`; run a named profile instead");
    }
    if (options.parallel === true) {
      throw new PreflightError("`fhevm-cli test light` does not accept `--parallel`; suite members choose their own mode");
    }
    console.log(`[test] light (${options.network})`);
    const started = Date.now();
    await runLogged("light", started, async () => {
      await pause("host");
      try {
        await runProfile("paused-host-contracts");
      } finally {
        await unpause("host").catch(() => undefined);
      }

      await pause("gateway");
      try {
        await runProfile("paused-gateway-contracts");
      } finally {
        await unpause("gateway").catch(() => undefined);
      }

      const driftPrecondition = ciphertextDriftRequirement();
      const profiles = driftPrecondition
        ? LIGHT_TEST_PROFILES.filter((profile) => profile !== "ciphertext-drift")
        : LIGHT_TEST_PROFILES;
      if (driftPrecondition) {
        console.log(`[skip] ciphertext-drift: ${driftPrecondition}`);
      }
      for (const profile of profiles.slice(2)) {
        await runProfile(profile);
      }

      await runWithHeartbeat(["docker", "stop", "coprocessor-host-listener"], "stop host listener");
      try {
        await runProfile("erc20");
      } finally {
        await runWithHeartbeat(["docker", "start", "coprocessor-host-listener"], "start host listener", { allowFailure: true });
      }
    });
    return;
  }

  if (options.grep) {
    console.log(`[test] custom (${options.network})`);
    const started = Date.now();
    const command = [
      "./run-tests.sh",
      options.verbose ? "-v" : "",
      options.parallel ? "--parallel" : "",
      "-n",
      shellEscape(options.network),
      "-g",
      shellEscape(options.grep),
    ]
      .filter(Boolean)
      .join(" ");
    await runLogged("custom", started, () =>
      runWithHeartbeat(buildTestContainerArgs(["sh", "-lc", command]), "test custom"),
    );
    return;
  }

  await runProfile(testName ?? "input-proof");
};

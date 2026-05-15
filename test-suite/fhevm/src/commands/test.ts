/**
 * Runs named e2e test profiles, standard/heavy CI suites, and topology-specific test flows.
 */
import { compatPolicyForState, supportsCoprocessorDbStateRevert } from "../compat/compat";
import { DRIFT_CLEANUP_SQL, DRIFT_INSTALL_SQL, driftDatabaseName, parseDriftInstanceIndex, parsePositiveInteger } from "../drift";
import { PreflightError, formatCliError } from "../errors";
import { dockerInspect } from "../flow/readiness";
import { pause, shellEscape, unpause } from "../flow/up-flow";
import { hostReachableRpcUrl } from "../utils/fs";
import { run, runWithHeartbeat } from "../utils/process";
import { loadState } from "../state/state";
import { topologyForState } from "../stack-spec/stack-spec";
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_CHAIN_ID,
  DEFAULT_POSTGRES_DB,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
  HEAVY_TEST_PROFILES,
  LIGHT_TEST_PROFILES,
  POSTGRES_HOST,
  STANDARD_TEST_PROFILES,
  TEST_GREP,
  TEST_PARALLEL,
  TEST_SUITE_CONTAINER,
} from "../layout";
import type { State, TestOptions } from "../types";

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
const DB_REVERT_RECOVERY_PROFILE = "input-proof-compute-decrypt";
const KEY_BOOTSTRAP_LOG = /Fetched keyset/;
const KEY_BOOTSTRAP_PROFILES = new Set(["input-proof", "input-proof-compute-decrypt"]);
// Intentional ciphertext drift must never run on shared/live networks.
const CIPHERTEXT_DRIFT_FORBIDDEN_NETWORKS = new Set(["sepolia", "mainnet", "zwsDev"]);

/** Formats a progress label with elapsed wall-clock time. */
const timedLabel = (label: string, started: number) =>
  `${label} (${Math.round((Date.now() - started) / 1000)}s)`;

const TEST_PROFILE_NAMES = [...Object.keys(TEST_GREP), "ciphertext-drift", "ciphertext-drift-auto-recovery", "coprocessor-db-state-revert", "heavy", "light", "standard"].sort();
const ZERO_TESTS_RE = /\b0 passing\b/;
const PAUSE_PROFILE_SCOPE: Record<string, string> = {
  "paused-host-contracts": "host",
  "paused-gateway-contracts": "gateway",
};
const TEST_PROFILE_DESCRIPTIONS: Partial<Record<(typeof TEST_PROFILE_NAMES)[number], string>> = {
  light: "Run the lightweight smoke suite.",
  standard: "Run the default CI suite for the active topology.",
  heavy: "Run the long operators suite.",
  "paused-host-contracts": "Run pause-mode checks with host contracts paused.",
  "paused-gateway-contracts": "Run pause-mode checks with gateway contracts paused.",
  "input-proof": "Run basic user input proof coverage.",
  "input-proof-compute-decrypt": "Run compute-and-decrypt input proof coverage.",
  "user-decryption": "Run user decryption coverage.",
  "delegated-user-decryption": "Run delegated user decryption coverage.",
  "public-decryption": "Run async public decryption coverage.",
  "public-decrypt-http-ebool": "Run HTTP public decrypt coverage for ebool payloads.",
  "public-decrypt-http-mixed": "Run mixed HTTP public decrypt coverage.",
  random: "Run random generation coverage.",
  "random-subset": "Run a narrower random generation subset.",
  operators: "Run manual operator workflows.",
  "hcu-block-cap": "Run HCU block cap scenarios.",
  erc20: "Run ERC20 transfer coverage.",
  "negative-acl": "Run negative ACL scenarios.",
  "multi-chain-isolation": "Run multi-chain state isolation coverage.",
  "ciphertext-drift": "Run ciphertext drift detection checks (requires 2+ coprocessors).",
  "ciphertext-drift-auto-recovery":
    "Run ciphertext drift auto-recovery checks — services self-recover (requires 2+ coprocessors).",
  "coprocessor-db-state-revert": "Run coprocessor DB state revert checks.",
};

/** Validates whether a named profile supports an extra grep narrowing expression. */
export const validateNamedProfileGrep = (testName: string | undefined, grep: string | undefined) => {
  if (testName && grep && !(testName in TEST_GREP)) {
    throw new PreflightError(`\`fhevm-cli test ${testName}\` does not accept \`--grep\`; use either a named profile or a custom grep`);
  }
};

/** Combines a named profile grep with an extra narrowing grep expression. */
export const narrowedProfileGrep = (filter: string, grep?: string) =>
  grep ? `(?=.*(?:${filter}))(?=.*(?:${grep}))` : filter;

/** Prints the supported named test profiles with short descriptions. */
export const listTestProfiles = () => {
  for (const name of TEST_PROFILE_NAMES) {
    const description = TEST_PROFILE_DESCRIPTIONS[name] ?? "Run this named test profile.";
    const topologyTags = [
      name === "ciphertext-drift" ? "2+ coprocessors" : undefined,
      name === "multi-chain-isolation" ? "multi-chain" : undefined,
    ].filter(Boolean);
    const suiteTags = [
      LIGHT_TEST_PROFILES.includes(name as (typeof LIGHT_TEST_PROFILES)[number]) ? "light" : undefined,
      STANDARD_TEST_PROFILES.includes(name as (typeof STANDARD_TEST_PROFILES)[number]) ? "standard" : undefined,
      HEAVY_TEST_PROFILES.includes(name as (typeof HEAVY_TEST_PROFILES)[number]) ? "heavy" : undefined,
      ...topologyTags,
    ].filter(Boolean);
    console.log(`${name}${suiteTags.length ? ` - ${suiteTags.join(", ")}` : ""}`);
    console.log(`  ${description}`);
  }
};

const ciphertextDriftNetworkRequirement = (network: string) =>
  CIPHERTEXT_DRIFT_FORBIDDEN_NETWORKS.has(network)
    ? `ciphertext-drift is not allowed on ${network}; run it only on local disposable environments`
    : undefined;

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
    await psql(dbName, [], options, DRIFT_CLEANUP_SQL).catch((error) => {
      console.log(`[warn] drift cleanup failed: ${formatCliError(error) ?? "unknown error"}`);
    });
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

/** Topic hash for AddCiphertextMaterial(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address coprocessorTxSender). */
const ADD_CIPHERTEXT_MATERIAL_TOPIC = "0x7249a80e5b91709d2170511b960e8a92e1d5849d200f320524dfffd8b50308f7";

/** Queries on-chain AddCiphertextMaterial events and asserts the two submissions have divergent digests. */
const assertOnChainDivergence = async (
  gatewayRpcUrl: string,
  contractAddress: string,
  handleHex: string,
) => {
  const paddedHandle = `0x${handleHex.toLowerCase().padStart(64, "0")}`;
  const response = await fetch(gatewayRpcUrl, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: 1,
      method: "eth_getLogs",
      params: [{ fromBlock: "0x0", toBlock: "latest", address: contractAddress, topics: [ADD_CIPHERTEXT_MATERIAL_TOPIC, paddedHandle] }],
    }),
  });
  if (!response.ok) {
    throw new PreflightError(`eth_getLogs failed: ${response.status} ${response.statusText}`);
  }
  const payload = (await response.json()) as { result?: { data: string }[] };
  const logs = payload.result ?? [];
  if (logs.length < 2) {
    throw new PreflightError(`expected 2+ AddCiphertextMaterial events for handle 0x${handleHex}, got ${logs.length}`);
  }
  // data layout: keyId (32B) | ciphertextDigest (32B) | snsCiphertextDigest (32B) | coprocessorTxSender (32B)
  // ciphertextDigest starts at byte offset 32 (chars 66..130 in the 0x-prefixed hex)
  const digests = logs.map((log) => log.data.slice(66, 130));
  const unique = new Set(digests);
  if (unique.size < 2) {
    throw new PreflightError(`on-chain AddCiphertextMaterial events show identical digests — drift not visible on chain`);
  }
  console.log(`[drift] on-chain divergence confirmed: ${logs.length} submissions with ${unique.size} distinct digest(s)`);
};

type DriftRevertDbOptions = {
  instanceIndex: number;
  postgresContainer: string;
  postgresUser: string;
  postgresPassword: string;
};

/** Counts computations rows for the host chain (coprocessor DB). */
const countComputations = async (dbOptions: DriftRevertDbOptions, hostChainId: string) => {
  const dbName = driftDatabaseName(dbOptions.instanceIndex);
  const value = await psql(
    dbName,
    ["-t", "-A", "-c", `SELECT COUNT(*) FROM computations WHERE host_chain_id = ${hostChainId};`],
    dbOptions,
  );
  return Number(value);
};

/** Polls `computations` row count until it reaches `target` — signals that
 * the host-listener has caught up with blocks re-processed after a revert. */
const waitForComputationsCatchup = async (
  options: DriftRevertDbOptions & {
    hostChainId: string;
    target: number;
    timeoutSeconds: number;
    pollIntervalSeconds: number;
  },
) => {
  const attempts = Math.max(0, Math.ceil(options.timeoutSeconds / options.pollIntervalSeconds));
  let lastCount = -1;
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    lastCount = await countComputations(options, options.hostChainId);
    if (lastCount >= options.target) {
      return lastCount;
    }
    if (attempt === attempts) {
      throw new PreflightError(
        `timed out waiting for computations to catch up: have ${lastCount}, need >= ${options.target}`,
      );
    }
    await Bun.sleep(options.pollIntervalSeconds * 1000);
  }
  return lastCount;
};

/** Polls drift_revert_signal until the latest row reaches a given status. */
const waitForDriftRevertStatus = async (
  options: DriftRevertDbOptions & {
    targetStatus: "reverting" | "done";
    timeoutSeconds: number;
    pollIntervalSeconds: number;
  },
) => {
  const dbName = driftDatabaseName(options.instanceIndex);
  const attempts = Math.max(0, Math.ceil(options.timeoutSeconds / options.pollIntervalSeconds));
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    const status = await psql(
      dbName,
      ["-t", "-A", "-c", "SELECT status FROM drift_revert_signal ORDER BY id DESC LIMIT 1;"],
      options,
    );
    if (status === options.targetStatus) {
      return;
    }
    // "done" is also acceptable when waiting for "reverting" — we may have
    // missed the transition if our polling was slower than the hold.
    if (options.targetStatus === "reverting" && status === "done") {
      return;
    }
    if (status.startsWith("failed")) {
      throw new PreflightError(`drift_revert_signal transitioned to status=${status}`);
    }
    if (attempt === attempts) {
      throw new PreflightError(
        `timed out waiting for drift_revert_signal to reach status=${options.targetStatus} in ${dbName} (last status: ${status || "<none>"})`,
      );
    }
    await Bun.sleep(options.pollIntervalSeconds * 1000);
  }
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

/** Builds the run-tests.sh argv for the current CLI options. */
const runTestsArgs = (
  options: Pick<TestOptions, "network" | "verbose" | "parallel" | "noHardhatCompile"> & { grep: string },
) => [
  "./run-tests.sh",
  options.verbose ? "-v" : "",
  options.parallel ? "--parallel" : "",
  options.noHardhatCompile ? "--no-hardhat-compile" : "",
  "-n",
  options.network,
  "-g",
  options.grep,
].filter(Boolean);

/** Builds a shell-safe run-tests.sh command string. */
const runTestsCommand = (
  options: Pick<TestOptions, "network" | "verbose" | "parallel" | "noHardhatCompile"> & { grep: string },
) => runTestsArgs(options).map(shellEscape).join(" ");

/** Runs a narrow e2e grep inside the test-suite container. */
const assertMatchedTests = (output: string, label: string) => {
  if (ZERO_TESTS_RE.test(output)) {
    throw new PreflightError(`${label} matched zero tests`);
  }
};

/** Runs a narrow e2e grep inside the test-suite container. */
const runNamedE2e = async (
  options: Pick<TestOptions, "network" | "noHardhatCompile">,
  grep: string,
  label: string,
) => {
  const result = await runWithHeartbeat(
    buildTestContainerArgs(runTestsArgs({ ...options, verbose: false, parallel: false, grep })),
    label,
  );
  assertMatchedTests(result.stdout + result.stderr, label);
};

/** Builds the coprocessor runtime container names for every configured instance. */
const coprocessorRuntimeContainers = (instanceCount: number) =>
  Array.from({ length: instanceCount }, (_, index) => {
    const prefix = index === 0 ? "coprocessor-" : `coprocessor${index}-`;
    return DB_REVERT_CONTAINERS.map((suffix) => `${prefix}${suffix}`);
  }).flat();

/** Builds the sns-worker container names for every configured coprocessor instance. */
const snsWorkerContainers = (instanceCount: number) =>
  Array.from({ length: instanceCount }, (_, index) => (index === 0 ? "coprocessor-sns-worker" : `coprocessor${index}-sns-worker`));

/** Builds the docker logs command used to detect sns-worker key bootstrap. */
export const keyBootstrapLogArgs = (container: string, since?: string) => [
  "docker",
  "logs",
  ...(since ? ["--since", since] : []),
  container,
];

/** Waits until enough sns-worker containers have fetched key material after bootstrap. */
export const waitForKeyBootstrap = async (
  state: NonNullable<Awaited<ReturnType<typeof loadState>>>,
  deps: {
    readLogs?: (container: string) => Promise<{ stdout: string; stderr: string }>;
    sleep?: (ms: number) => Promise<void>;
  } = {},
) => {
  const topology = topologyForState(state);
  const containers = snsWorkerContainers(topology.count);
  const readLogs =
    deps.readLogs ??
    (async (container: string) => {
      const [inspect] = await dockerInspect(container);
      return run(keyBootstrapLogArgs(container, inspect?.State.StartedAt), { allowFailure: true });
    });
  const sleep = deps.sleep ?? ((ms: number) => Bun.sleep(ms));
  for (let attempt = 0; attempt <= 60; attempt += 1) {
    let ready = 0;
    for (const container of containers) {
      const logs = await readLogs(container);
      if (KEY_BOOTSTRAP_LOG.test(logs.stdout) || KEY_BOOTSTRAP_LOG.test(logs.stderr)) {
        ready += 1;
      }
    }
    if (ready >= topology.threshold) {
      console.log(`[wait] key bootstrap ready=${ready}/${containers.length} threshold=${topology.threshold}`);
      return;
    }
    if (attempt === 60) {
      throw new PreflightError(
        `key bootstrap did not reach threshold ${topology.threshold}/${containers.length}; sns-worker key material is still pending`,
      );
    }
    if (attempt % 5 === 0) {
      console.log(`[wait] key bootstrap ready=${ready}/${containers.length} threshold=${topology.threshold}`);
    }
    await sleep(5_000);
  }
};

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

const postgresRuntime = () => ({
  postgresContainer: process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
  postgresUser: process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  postgresPassword: process.env.POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
  postgresDb: process.env.POSTGRES_DB ?? DEFAULT_POSTGRES_DB,
  postgresHost: process.env.POSTGRES_HOST,
});

/** Chooses the revert anchor just before the seed-generated block range. */
export const dbRevertTargetBlock = (seedStartBlock: number) => {
  const revertTo = seedStartBlock - 1;
  if (revertTo <= 0) {
    throw new PreflightError(`db-state-revert requires a positive seed boundary; got first seed block ${seedStartBlock}`);
  }
  return revertTo;
};

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

type DbRevertMetric = Exclude<keyof DbRevertSnapshot, "erroredComputations">;

/** Lists seed-generated tables that should shrink after the revert. */
export const dbRevertDeleteExpectations = (baseline: DbRevertSnapshot, seeded: DbRevertSnapshot) => {
  const metrics: DbRevertMetric[] = [
    "computationsDone",
    "computationsTotal",
    "allowedHandles",
    "pbsComputations",
    "ciphertextDigest",
    "ciphertexts",
    "ciphertexts128",
  ];
  return metrics.filter((metric) => seeded[metric] > baseline[metric]);
};

/** Ensures the revert step actually deleted seed-generated coprocessor data. */
const assertRevertDeletedData = (baseline: DbRevertSnapshot, seeded: DbRevertSnapshot, after: DbRevertSnapshot) => {
  const expected = dbRevertDeleteExpectations(baseline, seeded);
  if (!expected.length) {
    throw new PreflightError("db-state-revert seed did not create any revert-sensitive data");
  }
  const unchanged = [
    expected.includes("computationsDone") && after.computationsDone >= seeded.computationsDone ? "completed computations" : "",
    expected.includes("computationsTotal") && after.computationsTotal >= seeded.computationsTotal ? "computations" : "",
    expected.includes("allowedHandles") && after.allowedHandles >= seeded.allowedHandles ? "allowed_handles" : "",
    expected.includes("pbsComputations") && after.pbsComputations >= seeded.pbsComputations ? "pbs_computations" : "",
    expected.includes("ciphertextDigest") && after.ciphertextDigest >= seeded.ciphertextDigest ? "ciphertext_digest" : "",
    expected.includes("ciphertexts") && after.ciphertexts >= seeded.ciphertexts ? "ciphertexts" : "",
    expected.includes("ciphertexts128") && after.ciphertexts128 >= seeded.ciphertexts128 ? "ciphertexts128" : "",
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

const localDbMigrationImageRef = (state: Pick<State, "overrides" | "builtImages">) =>
  state.overrides.some((override) => override.group === "coprocessor")
    ? state.builtImages?.find((image) => image.group === "coprocessor" && image.ref.includes("/coprocessor/db-migration:"))?.ref
    : undefined;

/** Runs the coprocessor DB state revert e2e flow against the active stack. */
const runDbStateRevert = async (
  state: Awaited<ReturnType<typeof loadState>>,
  options: TestOptions,
) => {
  if (!state) {
    throw new PreflightError("Stack has not completed bootstrap; run `fhevm-cli up` first");
  }
  const started = Date.now();
  const chainId = process.env.CHAIN_ID ?? state.scenario.hostChains[0]?.chainId ?? DEFAULT_CHAIN_ID;
  if (!/^\d+$/.test(chainId)) {
    throw new PreflightError(`Invalid CHAIN_ID ${chainId}; expected a positive integer`);
  }
  const postgres = {
    ...postgresRuntime(),
  };
  const testsToRun = process.env.TESTS_TO_RUN ?? TEST_GREP[DB_REVERT_RECOVERY_PROFILE];
  if (!testsToRun) {
    throw new PreflightError(`Missing grep pattern for ${DB_REVERT_RECOVERY_PROFILE}`);
  }
  const timeoutSeconds = parsePositiveInteger(process.env.REVERT_POLL_TIMEOUT_SECONDS ?? "300", "REVERT_POLL_TIMEOUT_SECONDS");
  const pollIntervalSeconds = parsePositiveInteger(process.env.REVERT_POLL_INTERVAL_SECONDS ?? "2", "REVERT_POLL_INTERVAL_SECONDS");
  const containers = coprocessorRuntimeContainers(topologyForState(state).count);
  const migrationVersion = state.versions.env.COPROCESSOR_DB_MIGRATION_VERSION;
  const revertImage =
    localDbMigrationImageRef(state) ??
    (migrationVersion ? `ghcr.io/zama-ai/fhevm/coprocessor/db-migration:${migrationVersion}` : undefined);
  if (!revertImage) {
    throw new PreflightError("db-state-revert requires either a local coprocessor db-migration image or COPROCESSOR_DB_MIGRATION_VERSION");
  }
  console.log("[test] coprocessor-db-state-revert");

  return runLogged("coprocessor-db-state-revert", started, async () => {
    const maxBlockBeforeSeed = Number(
      await scalarQuery(postgres.postgresDb, `SELECT COALESCE(MAX(block_number), 0) FROM transactions WHERE chain_id = ${chainId}`, postgres),
    );
    const baseline = await dbRevertSnapshot(chainId, postgres);
    console.log(`[revert] baseline ${formatDbRevertSnapshot(baseline)}`);
    await runNamedE2e(options, testsToRun, "test coprocessor-db-state-revert seed");

    let before;
    let stopped = false;
    try {
      await setContainersRunning(containers, "stop");
      stopped = true;

      before = await dbRevertSnapshot(chainId, postgres);
      console.log(`[revert] before ${formatDbRevertSnapshot(before)}`);
      if (before.computationsDone === 0) {
        throw new PreflightError("db-state-revert found no completed computations; nothing to revert");
      }

      const seedStartBlock = Number(
        await scalarQuery(
          postgres.postgresDb,
          `SELECT COALESCE(MIN(block_number), 0) FROM transactions WHERE chain_id = ${chainId} AND block_number > ${maxBlockBeforeSeed}`,
          postgres,
        ),
      );
      const revertTo = dbRevertTargetBlock(seedStartBlock);

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
          `DATABASE_URL=postgres://${postgres.postgresUser}:${postgres.postgresPassword}@${postgres.postgresHost ?? POSTGRES_HOST.replace(/^db:/, `${postgres.postgresContainer}:`)}/${postgres.postgresDb}`,
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
      assertRevertDeletedData(baseline, before, after);
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

    await runNamedE2e(options, testsToRun, "test coprocessor-db-state-revert verify");
  });
};

/** Runs a named test profile, custom grep, or the standard/heavy CI suites. */
export const test = async (testName: string | undefined, options: TestOptions) => {
  if (testName === "list") {
    listTestProfiles();
    return;
  }
  if (testName && !TEST_PROFILE_NAMES.includes(testName)) {
    throw new PreflightError(`Unknown test profile ${testName}. Valid: ${TEST_PROFILE_NAMES.join(", ")}`);
  }
  validateNamedProfileGrep(testName, options.grep);
  const state = await loadState();
  if (!state?.discovery?.actualFheKeyId) {
    throw new PreflightError("Stack has not completed bootstrap; run `fhevm-cli up` first");
  }

  const ciphertextDriftRequirement = () => {
    const networkRequirement = ciphertextDriftNetworkRequirement(options.network);
    if (networkRequirement) {
      return networkRequirement;
    }
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

  // Auto-recovery requires consensus mismatch detection — the gateway must be
  // able to reach consensus while a minority dissents. This means threshold
  // must be strictly less than count (e.g., 2-of-3).
  const ciphertextDriftAutoRecoveryRequirement = () => {
    const base = ciphertextDriftRequirement();
    if (base) {
      return base;
    }
    const topology = topologyForState(state);
    if (topology.threshold >= topology.count) {
      return "ciphertext-drift-auto-recovery requires a topology where threshold < count (e.g. 2-of-3); rerun `fhevm-cli up --scenario two-of-three` first";
    }
    return undefined;
  };

  const ciphertextDriftAutoRecoverySkipReason = () =>
    ciphertextDriftNetworkRequirement(options.network) ??
    (state.scenario.topology.count < 3 ||
      state.scenario.topology.threshold >= state.scenario.topology.count
      ? "topology does not support consensus mismatch detection (needs threshold < count, e.g. 2-of-3)"
      : ciphertextDriftAutoRecoveryRequirement());

  const multiChainIsolationRequirement = () =>
    state.scenario.hostChains.length > 1
      ? undefined
      : "multi-chain-isolation requires a multi-chain topology; rerun `fhevm-cli up --scenario multi-chain` first";

  const multiChainIsolationSkipReason = () =>
    state.scenario.hostChains.length > 1 ? undefined : "topology has fewer than 2 host chains";

  const dbStateRevertSkipReason = () =>
    supportsCoprocessorDbStateRevert(state)
      ? undefined
      : `COPROCESSOR_DB_MIGRATION_VERSION=${state.versions.env.COPROCESSOR_DB_MIGRATION_VERSION || "unknown"} is older than v0.12.0`;

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
        const postgres = postgresRuntime();
        const logSince = new Date().toISOString();
        const faultyInstanceIndex = parseDriftInstanceIndex(process.env.FAULTY_INSTANCE_INDEX ?? "1");
        const driftInjectTimeoutSeconds = parsePositiveInteger(process.env.DRIFT_INJECT_TIMEOUT_SECONDS ?? "180", "DRIFT_INJECT_TIMEOUT_SECONDS");
        const driftInjectPollIntervalSeconds = parsePositiveInteger(process.env.DRIFT_INJECT_POLL_INTERVAL_SECONDS ?? "2", "DRIFT_INJECT_POLL_INTERVAL_SECONDS");
        const driftAlertTimeoutSeconds = parsePositiveInteger(process.env.DRIFT_ALERT_TIMEOUT_SECONDS ?? "360", "DRIFT_ALERT_TIMEOUT_SECONDS");
        const driftAlertPollIntervalSeconds = parsePositiveInteger(process.env.DRIFT_ALERT_POLL_INTERVAL_SECONDS ?? "2", "DRIFT_ALERT_POLL_INTERVAL_SECONDS");
        const grepPattern = process.env.GREP_PATTERN ?? "test add 42 to uint64 input and decrypt";
        const injector = injectCiphertextDrift({
          instanceIndex: faultyInstanceIndex,
          timeoutSeconds: driftInjectTimeoutSeconds,
          pollIntervalSeconds: driftInjectPollIntervalSeconds,
          postgresContainer: postgres.postgresContainer,
          postgresUser: postgres.postgresUser,
          postgresPassword: postgres.postgresPassword,
        });
        const result = await runWithHeartbeat(
          buildTestContainerArgs(
            runTestsArgs({ ...options, parallel: false, grep: grepPattern }),
            ["-e", "GATEWAY_RPC_URL="],
          ),
          "test ciphertext-drift",
        );
        assertMatchedTests(result.stdout + result.stderr, "test ciphertext-drift");
        const injectedHandleHex = await injector;
        const warning = await waitForDriftWarning(injectedHandleHex, {
          since: logSince,
          timeoutSeconds: driftAlertTimeoutSeconds,
          pollIntervalSeconds: driftAlertPollIntervalSeconds,
        });
        console.log(`[drift] detected in ${warning.container} for injected handle 0x${injectedHandleHex}`);
        const ciphertextCommitsAddress = state.discovery!.gateway.CIPHERTEXT_COMMITS_ADDRESS;
        if (ciphertextCommitsAddress) {
          const gatewayRpcUrl = hostReachableRpcUrl(state.discovery!.endpoints.gateway.http);
          await assertOnChainDivergence(gatewayRpcUrl, ciphertextCommitsAddress, injectedHandleHex);
        }
      });
    }
    if (name === "ciphertext-drift-auto-recovery") {
      console.log("[test] ciphertext-drift-auto-recovery");
      const started = Date.now();
      const precondition = ciphertextDriftAutoRecoveryRequirement();
      if (precondition) {
        throw new PreflightError(precondition);
      }
      return runLogged("ciphertext-drift-auto-recovery", started, async () => {
        const postgres = postgresRuntime();
        const logSince = new Date().toISOString();
        const faultyInstanceIndex = parseDriftInstanceIndex(process.env.FAULTY_INSTANCE_INDEX ?? "1");
        const driftInjectTimeoutSeconds = parsePositiveInteger(
          process.env.DRIFT_INJECT_TIMEOUT_SECONDS ?? "180",
          "DRIFT_INJECT_TIMEOUT_SECONDS",
        );
        const driftInjectPollIntervalSeconds = parsePositiveInteger(
          process.env.DRIFT_INJECT_POLL_INTERVAL_SECONDS ?? "2",
          "DRIFT_INJECT_POLL_INTERVAL_SECONDS",
        );
        const driftAlertTimeoutSeconds = parsePositiveInteger(
          process.env.DRIFT_ALERT_TIMEOUT_SECONDS ?? "360",
          "DRIFT_ALERT_TIMEOUT_SECONDS",
        );
        const driftAlertPollIntervalSeconds = parsePositiveInteger(
          process.env.DRIFT_ALERT_POLL_INTERVAL_SECONDS ?? "2",
          "DRIFT_ALERT_POLL_INTERVAL_SECONDS",
        );
        const driftRecoveryTimeoutSeconds = parsePositiveInteger(
          process.env.DRIFT_RECOVERY_TIMEOUT_SECONDS ?? "300",
          "DRIFT_RECOVERY_TIMEOUT_SECONDS",
        );
        const driftRecoveryPollIntervalSeconds = parsePositiveInteger(
          process.env.DRIFT_RECOVERY_POLL_INTERVAL_SECONDS ?? "2",
          "DRIFT_RECOVERY_POLL_INTERVAL_SECONDS",
        );
        // Must target a test that produces a compute output (byte 21 = 0xff).
        // The drift injector only corrupts compute-output handles, because
        // input drift is out of scope for auto-recovery.
        const grepPattern = process.env.GREP_PATTERN ?? "test add 42 to uint64 input and decrypt";

        const injector = injectCiphertextDrift({
          instanceIndex: faultyInstanceIndex,
          timeoutSeconds: driftInjectTimeoutSeconds,
          pollIntervalSeconds: driftInjectPollIntervalSeconds,
          postgresContainer: postgres.postgresContainer,
          postgresUser: postgres.postgresUser,
          postgresPassword: postgres.postgresPassword,
        });

        // Run the e2e tests that trigger the drift. This run may surface
        // errors due to the drift itself — we tolerate that and verify
        // recovery in the next step.
        await runWithHeartbeat(
          buildTestContainerArgs(
            runTestsArgs({ ...options, parallel: false, grep: grepPattern }),
            ["-e", "GATEWAY_RPC_URL="],
          ),
          "test ciphertext-drift-auto-recovery (initial run)",
        ).catch((error) => {
          console.log(
            `[drift-auto-recovery] initial test run failed (expected due to drift): ${formatCliError(error) ?? "unknown error"}`,
          );
        });

        const injectedHandleHex = await injector;
        const warning = await waitForDriftWarning(injectedHandleHex, {
          since: logSince,
          timeoutSeconds: driftAlertTimeoutSeconds,
          pollIntervalSeconds: driftAlertPollIntervalSeconds,
        });
        console.log(
          `[drift-auto-recovery] drift detected in ${warning.container} for handle 0x${injectedHandleHex}`,
        );

        // Cross-check that drift was visible on chain (≥2 distinct digests
        // across the AddCiphertextMaterial submissions). Folded in from the
        // former `ciphertext-drift` profile so this single test covers both
        // detection and recovery.
        const ciphertextCommitsAddress = state.discovery!.gateway.CIPHERTEXT_COMMITS_ADDRESS;
        if (ciphertextCommitsAddress) {
          const gatewayRpcUrl = hostReachableRpcUrl(state.discovery!.endpoints.gateway.http);
          await assertOnChainDivergence(gatewayRpcUrl, ciphertextCommitsAddress, injectedHandleHex);
        }

        const dbOptions = {
          instanceIndex: faultyInstanceIndex,
          postgresContainer: postgres.postgresContainer,
          postgresUser: postgres.postgresUser,
          postgresPassword: postgres.postgresPassword,
        };
        const hostChainId = process.env.CHAIN_ID ?? "12345";
        if (!/^\d+$/.test(hostChainId)) {
          throw new PreflightError(`Invalid CHAIN_ID ${hostChainId}; expected a positive integer`);
        }

        // Snapshot row counts before the revert runs. The gw-listener is
        // still in the grace period (pending status), so this is stable.
        const computationsBefore = await countComputations(dbOptions, hostChainId);
        console.log(`[drift-auto-recovery] computations before revert: ${computationsBefore}`);

        // Wait for the revert to actually run (status transitions to
        // "reverting" and the SQL completes). DRIFT_REVERT_TEST_HOLD_SECS
        // (defaulted in generated env) keeps status=reverting for ~15s so
        // we have a window to query the post-SQL state.
        await waitForDriftRevertStatus({
          ...dbOptions,
          targetStatus: "reverting",
          timeoutSeconds: driftRecoveryTimeoutSeconds,
          pollIntervalSeconds: driftRecoveryPollIntervalSeconds,
        });

        // Snapshot counts while status=reverting. Services are still blocked
        // waiting for status=done, so no new writes are racing.
        const computationsAfterRevert = await countComputations(dbOptions, hostChainId);
        console.log(`[drift-auto-recovery] computations after revert: ${computationsAfterRevert}`);
        if (computationsAfterRevert >= computationsBefore) {
          throw new PreflightError(
            `drift revert did not delete any computations (before=${computationsBefore}, after=${computationsAfterRevert})`,
          );
        }

        // Wait for the revert to finish.
        await waitForDriftRevertStatus({
          ...dbOptions,
          targetStatus: "done",
          timeoutSeconds: driftRecoveryTimeoutSeconds,
          pollIntervalSeconds: driftRecoveryPollIntervalSeconds,
        });

        // Wait for host-listener to re-process the reverted blocks so the
        // follow-up test isn't racing against catchup.
        const caughtUp = await waitForComputationsCatchup({
          ...dbOptions,
          hostChainId,
          target: computationsBefore,
          timeoutSeconds: driftRecoveryTimeoutSeconds,
          pollIntervalSeconds: driftRecoveryPollIntervalSeconds,
        });
        console.log(`[drift-auto-recovery] revert completed; computations caught up to ${caughtUp} (>= ${computationsBefore}); re-running tests to verify recovery`);

        // Re-run the e2e tests to verify the services have fully recovered.
        const followUp = await runWithHeartbeat(
          buildTestContainerArgs(
            runTestsArgs({ ...options, parallel: false, grep: grepPattern }),
            ["-e", "GATEWAY_RPC_URL="],
          ),
          "test ciphertext-drift-auto-recovery (post-recovery)",
        );
        assertMatchedTests(
          followUp.stdout + followUp.stderr,
          "test ciphertext-drift-auto-recovery (post-recovery)",
        );
      });
    }
    if (name === "multi-chain-isolation") {
      const precondition = multiChainIsolationRequirement();
      if (precondition) {
        throw new PreflightError(precondition);
      }
    }

    const filter = TEST_GREP[name];
    if (!filter) {
      throw new PreflightError(`Unknown test profile ${name}. Valid: ${TEST_PROFILE_NAMES.join(", ")}`);
    }

    const runGrep = async () => {
      const shouldParallel = options.parallel ?? TEST_PARALLEL[name];
      const grep = narrowedProfileGrep(filter, options.grep);
      console.log(`[test] ${name} (${options.network})`);
      const started = Date.now();
      const command = runTestsCommand({ ...options, parallel: shouldParallel, grep });
      return runLogged(name, started, async () => {
        if (KEY_BOOTSTRAP_PROFILES.has(name)) {
          await waitForKeyBootstrap(state);
        }
        const result = await runWithHeartbeat(buildTestContainerArgs(["sh", "-lc", command]), `test ${name}`);
        assertMatchedTests(result.stdout + result.stderr, `test ${name}`);
      });
    };

    const pauseScope = PAUSE_PROFILE_SCOPE[name];
    if (pauseScope) {
      await pause(pauseScope);
      try {
        return await runGrep();
      } finally {
        await unpause(pauseScope).catch((error) => {
          console.log(`[warn] unpause ${pauseScope} failed: ${formatCliError(error) ?? "unknown error"}`);
        });
      }
    }

    return runGrep();
  };

  const runStandardSuite = async () => {
    if (options.grep) {
      throw new PreflightError("`fhevm-cli test standard` does not accept `--grep`; run a named profile instead");
    }
    if (options.parallel === true) {
      throw new PreflightError("`fhevm-cli test standard` does not accept `--parallel`; suite members choose their own mode");
    }
    console.log(`[test] standard (${options.network})`);
    const started = Date.now();
    await runLogged("standard", started, async () => {
      for (const profile of STANDARD_TEST_PROFILES) {
        if (profile === "multi-chain-isolation") {
          const skipReason = multiChainIsolationSkipReason();
          if (skipReason) {
            console.log(`[test] skipping multi-chain-isolation: ${skipReason}`);
            continue;
          }
        }
        if (profile === "coprocessor-db-state-revert") {
          const skipReason = dbStateRevertSkipReason();
          if (skipReason) {
            console.log(`[test] skipping coprocessor-db-state-revert: ${skipReason}`);
            continue;
          }
        }
        if (profile === "ciphertext-drift-auto-recovery") {
          const skipReason = ciphertextDriftAutoRecoverySkipReason();
          if (skipReason) {
            console.log(`[test] skipping ciphertext-drift-auto-recovery: ${skipReason}`);
            continue;
          }
        }
        await runProfile(profile);
      }
    });
  };

  if (testName === "standard") {
    await runStandardSuite();
    return;
  }

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
      for (const profile of LIGHT_TEST_PROFILES) {
        await runProfile(profile);
      }
    });
    return;
  }

  if (testName === "heavy") {
    if (options.grep) {
      throw new PreflightError("`fhevm-cli test heavy` does not accept `--grep`; run a named profile instead");
    }
    if (options.parallel === true) {
      throw new PreflightError("`fhevm-cli test heavy` does not accept `--parallel`; suite members choose their own mode");
    }
    console.log(`[test] heavy (${options.network})`);
    const started = Date.now();
    await runLogged("heavy", started, async () => {
      for (const profile of HEAVY_TEST_PROFILES) {
        await runProfile(profile);
      }
    });
    return;
  }

  if (options.grep) {
    console.log(`[test] custom (${options.network})`);
    const started = Date.now();
    const command = runTestsCommand({ ...options, grep: options.grep });
    await runLogged("custom", started, async () => {
      const result = await runWithHeartbeat(buildTestContainerArgs(["sh", "-lc", command]), "test custom");
      assertMatchedTests(result.stdout + result.stderr, "test custom");
    });
    return;
  }

  await runProfile(testName ?? "input-proof");
};

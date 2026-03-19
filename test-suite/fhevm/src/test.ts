import { compatPolicyForState } from "./compat";
import { DRIFT_CLEANUP_SQL, DRIFT_INSTALL_SQL, driftDatabaseName, parseDriftInstanceIndex, parsePositiveInteger } from "./drift";
import { PreflightError } from "./errors";
import { run, runWithHeartbeat } from "./shell";
import { loadState } from "./state";
import { pause, shellEscape, unpause } from "./stack";
import { topologyForState } from "./runtime-plan";
import {
  COPROCESSOR_DB_CONTAINER,
  LIGHT_TEST_PROFILES,
  TEST_GREP,
  TEST_PARALLEL,
  TEST_SUITE_CONTAINER,
} from "./layout";
import type { TestOptions } from "./types";

const DRIFT_WARNING = '"message":"Drift detected: observed multiple digest variants for handle"';
const DRIFT_HANDLE = /"handle":"0x([0-9a-f]+)"/i;

const timedLabel = (label: string, started: number) =>
  `${label} (${Math.round((Date.now() - started) / 1000)}s)`;

const TEST_PROFILE_NAMES = [...Object.keys(TEST_GREP), "ciphertext-drift", "light"].sort();

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

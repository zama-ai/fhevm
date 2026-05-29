/**
 * `single-signer-soak` test profile.
 *
 * Asserts the behavior of coprocessors EXCLUDED by a gateway single-signer cutover
 * (GatewayConfig.updateCoprocessors([zama], 1) + host InputVerifier.defineNewContext).
 * The excluded partner tx-senders must decode the gateway revert (NotCoprocessorTxSender)
 * as a non-retryable config error and STOP submitting, while still computing and staying
 * healthy, and the protocol keeps operating on the single surviving (Zama, index 0) signer.
 *
 * Grounding (verified against coprocessor transaction-sender):
 *   - stop-retry logs: add_ciphertext.rs:108, verify_proof.rs:186
 *   - terminal DB marker set by stop_retrying_add_ciphertext_on_config_error:
 *       ciphertext_digest.txn_limited_retries_count := add_ciphertexts_max_retries
 *       (default i32::MAX = 2147483647)   add_ciphertext.rs:317-336, config.rs:48
 *     verify_proofs.retry_count is set to verify_proof_resp_max_retries and the row is
 *     then DELETED (--verify-proof-remove-after-max-retries), so the verify stop is
 *     asserted via its log line rather than the column.
 *   - consensus health: e2e/test/consensusWatchdog.ts (auto-enabled when GATEWAY_RPC_URL
 *     + CIPHERTEXT_COMMITS_ADDRESS are set on the test container)
 *   - end-to-end decrypt: e2e/scripts/smoke-inputflow.ts
 *
 * The low-level test/DB/container helpers below are shared infrastructure that lives in
 * `./test` (reused by the drift and db-state-revert profiles too); only the
 * single-signer-specific logic lives here.
 */
import { driftDatabaseName, parsePositiveInteger } from "../drift";
import { PreflightError } from "../errors";
import { dockerInspect } from "../flow/readiness";
import { topologyForState } from "../stack-spec/stack-spec";
import type { State, TestOptions } from "../types";
import { run, runWithHeartbeat } from "../utils/process";
import {
  DB_REVERT_CONTAINERS,
  assertMatchedTests,
  buildTestContainerArgs,
  postgresRuntime,
  runLogged,
  runTestsArgs,
  scalarQuery,
} from "./test";

const SINGLE_SIGNER_STOP_RETRY_ADD_CIPHERTEXT =
  "Non-retryable gateway coprocessor config error while adding ciphertext";
const SINGLE_SIGNER_STOP_RETRY_VERIFY_PROOF =
  "Non-retryable gateway coprocessor config error while sending verify_proof transaction";
const ADD_CIPHERTEXT_TERMINAL_RETRY_COUNT = 2147483647; // add_ciphertexts_max_retries default (i32::MAX)
const SINGLE_SIGNER_SOAK_GREP = "test add 42 to uint64 input and decrypt";
// The gateway is reachable from inside the test/coprocessor containers on the
// internal compose network (mirrors the rollout's in-container GATEWAY_RPC_URL).
const GATEWAY_INTERNAL_RPC_URL = "http://gateway-node:8546";
// The cutover mutates the on-chain coprocessor set, so this profile is local-only.
const SINGLE_SIGNER_SOAK_FORBIDDEN_NETWORKS = new Set(["sepolia", "mainnet", "zwsDev"]);

/** Names a coprocessor instance's container by index (index 0 = Zama/primary). */
const coprocessorContainer = (index: number, suffix: string) =>
  `${index === 0 ? "coprocessor-" : `coprocessor${index}-`}${suffix}`;

const singleSignerSoakRequirement = (state: State, network: string) => {
  if (SINGLE_SIGNER_SOAK_FORBIDDEN_NETWORKS.has(network)) {
    return `single-signer-soak mutates the on-chain coprocessor set; run it only on local disposable environments, not ${network}`;
  }
  if (topologyForState(state).count < 2) {
    return "single-signer-soak requires a multi-coprocessor topology so there is an excluded partner to observe; rerun `fhevm-cli up --scenario two-of-three` first";
  }
  return undefined;
};

/** Polls until every unsent ciphertext digest is parked at the terminal retry count. */
const waitForAddCiphertextStopRetry = async (opts: {
  dbName: string;
  postgres: ReturnType<typeof postgresRuntime>;
  timeoutSeconds: number;
  pollIntervalSeconds: number;
  label: string;
}) => {
  const attempts = Math.max(0, Math.ceil(opts.timeoutSeconds / opts.pollIntervalSeconds));
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    const terminal = Number(
      await scalarQuery(
        opts.dbName,
        `SELECT count(*) FROM ciphertext_digest WHERE txn_limited_retries_count = ${ADD_CIPHERTEXT_TERMINAL_RETRY_COUNT}`,
        opts.postgres,
      ),
    );
    const climbing = Number(
      await scalarQuery(
        opts.dbName,
        `SELECT count(*) FROM ciphertext_digest WHERE txn_is_sent = FALSE AND txn_limited_retries_count <> ${ADD_CIPHERTEXT_TERMINAL_RETRY_COUNT}`,
        opts.postgres,
      ),
    );
    if (terminal >= 1 && climbing === 0) {
      return terminal;
    }
    await Bun.sleep(opts.pollIntervalSeconds * 1000);
  }
  throw new PreflightError(
    `excluded coprocessor ${opts.label} (${opts.dbName}) did not park its unsent ciphertext digests at the terminal retry count within ${opts.timeoutSeconds}s`,
  );
};

/** Reads one prometheus gauge from a coprocessor container's metrics endpoint. */
const scrapeCoprocessorMetric = async (container: string, metric: string): Promise<number | undefined> => {
  // Port 9100 (transaction_sender metrics_addr) is not host-published, so scrape
  // from inside the container; tolerate the image lacking wget/curl.
  const result = await run(
    [
      "docker",
      "exec",
      container,
      "sh",
      "-c",
      "wget -qO- localhost:9100/metrics 2>/dev/null || curl -s localhost:9100/metrics 2>/dev/null || true",
    ],
    { allowFailure: true },
  );
  const line = result.stdout.split("\n").find((entry) => entry.startsWith(metric) && !entry.startsWith("#"));
  if (!line) {
    return undefined;
  }
  const value = Number(line.trim().split(/\s+/).pop());
  return Number.isFinite(value) ? value : undefined;
};

/** Advisory: assert unsent work plateaus when the metrics endpoint is reachable. */
const assertUnsentGaugePlateausBestEffort = async (container: string, pollIntervalSeconds: number) => {
  const gauge = "coprocessor_add_ciphertext_material_unsent_gauge";
  const first = await scrapeCoprocessorMetric(container, gauge);
  if (first === undefined) {
    console.log(`[single-signer-soak] ${container}: metrics endpoint unreachable from inside the container; skipping plateau check`);
    return;
  }
  await Bun.sleep(pollIntervalSeconds * 1000);
  const second = await scrapeCoprocessorMetric(container, gauge);
  if (second === undefined) {
    return;
  }
  if (second > first) {
    throw new PreflightError(
      `excluded coprocessor ${container} ${gauge} is still growing (${first} -> ${second}); unsent work is not plateauing`,
    );
  }
  console.log(`[single-signer-soak] ${container} ${gauge} plateaued at ${second}`);
};

/** Reads a container's cumulative restart count (crash-loop signal). */
const readRestartCount = async (container: string): Promise<number> => {
  const result = await run(["docker", "inspect", "-f", "{{.RestartCount}}", container], { allowFailure: true });
  const count = Number(result.stdout.trim());
  return Number.isFinite(count) ? count : 0;
};

/**
 * Asserts an excluded coprocessor instance degraded GRACEFULLY: every runtime
 * container is still running and healthy, and none restarted during the soak.
 * Stopping retries must not mean crashing.
 */
const assertExcludedCoprocessorAlive = async (index: number, restartBaseline: Map<string, number>) => {
  for (const suffix of DB_REVERT_CONTAINERS) {
    const container = coprocessorContainer(index, suffix);
    const [inspect] = await dockerInspect(container);
    if (!inspect) {
      throw new PreflightError(`excluded coprocessor container ${container} is missing; it did not stay up`);
    }
    if (inspect.State.Status !== "running") {
      throw new PreflightError(
        `excluded coprocessor container ${container} is ${inspect.State.Status}, not running; it did not degrade gracefully`,
      );
    }
    const health = inspect.State.Health?.Status;
    if (health && health !== "healthy") {
      throw new PreflightError(`excluded coprocessor container ${container} reports health=${health}`);
    }
    const restarts = await readRestartCount(container);
    const baseline = restartBaseline.get(container) ?? 0;
    if (restarts > baseline) {
      throw new PreflightError(
        `excluded coprocessor container ${container} restarted ${restarts - baseline} time(s) during the soak (crash-looping?); baseline=${baseline} now=${restarts}`,
      );
    }
  }
};

/**
 * Asserts an excluded coprocessor KEEPS COMPUTING — not just "process alive".
 * Stopping on-chain submission must not stop local FHE compute: its compute tables
 * must keep pace with the surviving signer (index 0). This is the substantive half of
 * "stay healthy" — the excluded coprocessor still ingests host events, computes
 * ciphertexts, and produces digests; only the tx-sender submission stops.
 */
const assertExcludedCoprocessorComputing = async (opts: {
  index: number;
  postgres: ReturnType<typeof postgresRuntime>;
  timeoutSeconds: number;
  pollIntervalSeconds: number;
}) => {
  const referenceDb = driftDatabaseName(0); // index 0 = surviving Zama coprocessor
  const excludedDb = driftDatabaseName(opts.index);
  const count = (db: string, table: string) => scalarQuery(db, `SELECT count(*) FROM ${table}`, opts.postgres);
  // Traffic has ended by the time this runs (post stop-retry settle), so the
  // reference counts are stable.
  const referenceComputations = Number(await count(referenceDb, "computations"));
  const referenceDigests = Number(await count(referenceDb, "ciphertext_digest"));
  const attempts = Math.max(0, Math.ceil(opts.timeoutSeconds / opts.pollIntervalSeconds));
  for (let attempt = 0; attempt <= attempts; attempt += 1) {
    const computations = Number(await count(excludedDb, "computations"));
    const digests = Number(await count(excludedDb, "ciphertext_digest"));
    if (computations > 0 && computations >= referenceComputations && digests >= referenceDigests) {
      console.log(
        `[single-signer-soak] coprocessor${opts.index} keeps computing: computations=${computations} digests=${digests} (surviving-signer reference computations=${referenceComputations})`,
      );
      return;
    }
    await Bun.sleep(opts.pollIntervalSeconds * 1000);
  }
  throw new PreflightError(
    `excluded coprocessor${opts.index} (${excludedDb}) is not keeping pace with FHE compute (expected >= ${referenceComputations} computations, like the surviving signer); it may have stopped computing, not just stopped submitting`,
  );
};

/** Runs the single-signer soak: excluded coprocessors stop submitting, keep computing, stay healthy. */
export const runSingleSignerSoak = async (state: State, options: TestOptions) => {
  if (!state.discovery) {
    throw new PreflightError("Stack is not running; run `fhevm-cli up` first");
  }
  const liveState = state;
  const precondition = singleSignerSoakRequirement(liveState, options.network);
  if (precondition) {
    throw new PreflightError(precondition);
  }
  const started = Date.now();
  return runLogged("single-signer-soak", started, async () => {
    const topology = topologyForState(liveState);
    // Index 0 is the surviving Zama coprocessor; 1..n-1 were excluded by the cutover.
    const excludedIndices = Array.from({ length: topology.count - 1 }, (_, index) => index + 1);
    const postgres = postgresRuntime();
    const logSince = new Date().toISOString();
    const ciphertextCommitsAddress = liveState.discovery!.gateway.CIPHERTEXT_COMMITS_ADDRESS;
    const inputVerificationAddress = liveState.discovery!.gateway.INPUT_VERIFICATION_ADDRESS;
    if (!ciphertextCommitsAddress) {
      throw new PreflightError("single-signer-soak needs CIPHERTEXT_COMMITS_ADDRESS discovery; rerun `fhevm-cli up`");
    }
    const grep = process.env.GREP_PATTERN ?? SINGLE_SIGNER_SOAK_GREP;
    const settleTimeoutSeconds = parsePositiveInteger(
      process.env.SINGLE_SIGNER_SOAK_SETTLE_TIMEOUT_SECONDS ?? "240",
      "SINGLE_SIGNER_SOAK_SETTLE_TIMEOUT_SECONDS",
    );
    const settlePollIntervalSeconds = parsePositiveInteger(
      process.env.SINGLE_SIGNER_SOAK_POLL_INTERVAL_SECONDS ?? "5",
      "SINGLE_SIGNER_SOAK_POLL_INTERVAL_SECONDS",
    );

    // Snapshot restart counts up front so we can prove the excluded coprocessors
    // do not crash-loop on the rejected submissions during the soak window.
    const restartBaseline = new Map<string, number>();
    for (const index of excludedIndices) {
      for (const suffix of DB_REVERT_CONTAINERS) {
        const container = coprocessorContainer(index, suffix);
        restartBaseline.set(container, await readRestartCount(container));
      }
    }

    // 1. Drive write-path traffic (input proof -> ciphertext add + proof verify ->
    //    decrypt) with the consensus watchdog enabled. This exercises the surviving
    //    Zama signer end-to-end (the watchdog's afterEach/afterAll assert consensus
    //    is still reached and never diverges or stalls) and generates the work the
    //    excluded coprocessors will try, and fail, to submit.
    const watchdogEnv = [
      "-e",
      `GATEWAY_RPC_URL=${GATEWAY_INTERNAL_RPC_URL}`,
      "-e",
      `CIPHERTEXT_COMMITS_ADDRESS=${ciphertextCommitsAddress}`,
      ...(inputVerificationAddress ? ["-e", `INPUT_VERIFICATION_ADDRESS=${inputVerificationAddress}`] : []),
    ];
    const e2e = await runWithHeartbeat(
      buildTestContainerArgs(runTestsArgs({ ...options, parallel: false, grep }), watchdogEnv),
      "single-signer-soak write-path + consensus watchdog",
    );
    assertMatchedTests(e2e.stdout + e2e.stderr, "single-signer-soak write-path");

    // 2. End-to-end smoke: input -> encrypt -> tx -> user + public decrypt == 49.
    await runWithHeartbeat(
      buildTestContainerArgs(["npx", "hardhat", "run", "scripts/smoke-inputflow.ts", "--network", options.network]),
      "single-signer-soak smoke-inputflow",
    );

    // 3. Every excluded coprocessor must STOP retrying: terminal DB marker + log line.
    for (const index of excludedIndices) {
      const dbName = driftDatabaseName(index);
      const txSender = coprocessorContainer(index, "transaction-sender");

      await waitForAddCiphertextStopRetry({
        dbName,
        postgres,
        timeoutSeconds: settleTimeoutSeconds,
        pollIntervalSeconds: settlePollIntervalSeconds,
        label: txSender,
      });

      const logs = await run(["docker", "logs", "--since", logSince, txSender], { allowFailure: true });
      const logText = `${logs.stdout}${logs.stderr}`;
      if (!logText.includes(SINGLE_SIGNER_STOP_RETRY_ADD_CIPHERTEXT)) {
        throw new PreflightError(
          `excluded coprocessor ${txSender} never logged the non-retryable add-ciphertext stop line; it may still be retrying`,
        );
      }
      if (!logText.includes(SINGLE_SIGNER_STOP_RETRY_VERIFY_PROOF)) {
        // Proof-verify traffic can lag ciphertext-add; the deterministic
        // add-ciphertext signal above stays the gate.
        console.log(`[single-signer-soak] ${txSender}: verify_proof stop line not seen yet (proof traffic may lag)`);
      }

      await assertUnsentGaugePlateausBestEffort(txSender, settlePollIntervalSeconds);

      // The excluded instance must have degraded gracefully — still up, no crash-loop...
      await assertExcludedCoprocessorAlive(index, restartBaseline);
      // ...and must KEEP COMPUTING — only submission stops, not FHE compute.
      await assertExcludedCoprocessorComputing({
        index,
        postgres,
        timeoutSeconds: settleTimeoutSeconds,
        pollIntervalSeconds: settlePollIntervalSeconds,
      });
    }
    console.log(
      `[single-signer-soak] ${excludedIndices.length} excluded coprocessor(s) stopped submitting but kept computing and stayed healthy; Zama signer consensus healthy`,
    );
  });
};

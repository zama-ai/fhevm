/**
 * Run-validity gates: the difference between a result and a number.
 *
 * The coverage register sets this as a harness contract rather than a test:
 * "a run that fails a gate aborts as invalid rather than reporting numbers".
 * The reasoning is the measurement history of this repository. A GPU byte-gate
 * once compared one stream against itself; a fork suite's precondition compared
 * zero against zero; and the bootstrap readiness probe has reported ready on a
 * stack whose keys had not ingested. Every one of those produced numbers that
 * looked like evidence. A gate that fires turns that class of run into a loud
 * failure instead.
 *
 * These gates therefore check the *preconditions of measurement*, not the
 * behaviour under test:
 *
 *   - key and CRS material is actually present, and the operators agree on
 *     which key they hold -- read from the tables the workers use, not from the
 *     bootstrap probe, because that probe has already lied once;
 *   - no transactions are stuck deferred, which is how a wedged scheduling
 *     window looks from outside;
 *   - the chain is still advancing, so a suite cannot pass by measuring a
 *     stalled world.
 *
 * A gate that cannot be evaluated fails. It never skips: an unreachable metrics
 * endpoint or an unqueryable database is exactly the situation in which a green
 * run means least, so silence there would defeat the purpose.
 *
 * The fourth gate the register names -- a journal grep for
 * "Not all locks extended" -- lives in the shell runners instead, because it
 * needs container logs and the e2e container deliberately has no Docker socket.
 */
import { Pool } from 'pg';

/** Thrown when the run cannot produce trustworthy numbers, whatever the tests would say. */
export class InvalidRunError extends Error {
  constructor(message: string) {
    super(`invalid run: ${message}`);
    this.name = 'InvalidRunError';
  }
}

/** `coprocessor_worker_deferred_transactions_current`, as exposed by each tfhe-worker. */
export const DEFERRED_TRANSACTIONS_METRIC = 'coprocessor_worker_deferred_transactions_current';

/** In-network metrics endpoint of one operator's tfhe-worker. */
export function tfheWorkerMetricsUrl(operatorIndex: number, port = 9100): string {
  // A GPU-swapped topology has no tfhe-worker containers at all: the swap stops
  // them and runs the workers on the host, so container DNS resolves nothing and
  // this gate reports every operator unreachable. The runner knows where they
  // actually listen -- `gpu-consensus-workers.sh` binds worker `i` on
  // 19100 + i*10 -- and passes the URLs in, index-ordered and comma-separated.
  // Container DNS remains the default for every compose topology. A list that is
  // set but short falls back to container DNS for the missing indices, which the
  // gate then reports as unreachable rather than passing quietly.
  const explicit = process.env.TFHE_WORKER_METRICS_URLS;
  if (explicit) {
    const urls = explicit
      .split(',')
      .map((entry) => entry.trim())
      .filter((entry) => entry.length > 0);
    if (urls[operatorIndex]) return urls[operatorIndex];
  }
  const host = operatorIndex === 0 ? 'coprocessor-tfhe-worker' : `coprocessor${operatorIndex}-tfhe-worker`;
  return `http://${host}:${port}/metrics`;
}

/**
 * Reads one gauge out of a Prometheus text exposition.
 *
 * Returns undefined when the metric is absent, which is not the same as zero
 * and not the same as a fault. The workers register metrics lazily, so a gauge
 * appears only once something has touched it: a worker that has never deferred
 * a transaction exposes no deferred gauge at all. The caller decides what
 * absence means, and needs [`looksLikeCoprocessorWorker`] to tell "never
 * deferred" from "not the process we think we are reading".
 */
export function parseGauge(exposition: string, metric: string): number | undefined {
  for (const line of exposition.split('\n')) {
    const trimmed = line.trim();
    if (trimmed.length === 0 || trimmed.startsWith('#')) continue;
    // `name value` or `name{labels} value`
    const match = new RegExp(`^${metric}(?:\\{[^}]*\\})?\\s+(-?[0-9.eE+]+)$`).exec(trimmed);
    if (match) {
      const value = Number.parseFloat(match[1]);
      if (Number.isFinite(value)) return value;
    }
  }
  return undefined;
}

/**
 * Is this exposition coming from a coprocessor worker at all?
 *
 * Needed because metrics register lazily. Absence of the deferred gauge is
 * normal on a worker that has never deferred, so the gate cannot treat absence
 * as a fault -- but it must still catch the case where it is reading something
 * else entirely, or a build that no longer has these metrics. Any
 * `coprocessor_`-prefixed series is enough to establish that.
 */
export function looksLikeCoprocessorWorker(exposition: string): boolean {
  return exposition.split('\n').some((line) => line.trimStart().startsWith('coprocessor_'));
}

async function withPool<T>(databaseUrl: string, fn: (pool: Pool) => Promise<T>): Promise<T> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    return await fn(pool);
  } finally {
    await pool.end();
  }
}

export interface KeyMaterialReport {
  operator: number;
  keyRows: number;
  keyIdsWithKeyset: number;
  crsRows: number;
  keyIdGw: string | null;
}

/**
 * Gate 1: every operator holds usable key material, and they hold the *same* key.
 *
 * `compressed_xof_keyset` is the column the SNS worker actually loads, so a row
 * that exists with that column null is not usable material however healthy the
 * bootstrap looked. Operators disagreeing on `key_id_gw` is worse than either
 * having none: every non-scalar operation would then diverge deterministically,
 * and a byte-consensus suite would report that as a consensus defect.
 */
export async function assertKeyMaterial(
  databaseUrls: readonly string[],
  operators: readonly number[] = databaseUrls.map((_, index) => index),
): Promise<KeyMaterialReport[]> {
  const reports: KeyMaterialReport[] = [];
  for (const operator of operators) {
    const databaseUrl = databaseUrls[operator];
    let report: KeyMaterialReport;
    try {
      report = await withPool(databaseUrl, async (pool) => {
        const keys = await pool.query<{ total: string; with_keyset: string; key_id_gw: string | null }>(
          `SELECT COUNT(*)::text AS total,
                  COUNT(compressed_xof_keyset)::text AS with_keyset,
                  MIN(encode(key_id_gw, 'hex')) AS key_id_gw
             FROM keys`,
        );
        const crs = await pool.query<{ total: string }>(
          'SELECT COUNT(*)::text AS total FROM kms_crs_activation_events',
        );
        return {
          operator,
          keyRows: Number.parseInt(keys.rows[0].total, 10),
          keyIdsWithKeyset: Number.parseInt(keys.rows[0].with_keyset, 10),
          crsRows: Number.parseInt(crs.rows[0].total, 10),
          keyIdGw: keys.rows[0].key_id_gw,
        };
      });
    } catch (error) {
      throw new InvalidRunError(
        `operator ${operator}'s key material could not be read from ${databaseUrl}: ` +
          `${error instanceof Error ? error.message : String(error)}`,
      );
    }
    if (report.keyRows === 0) {
      throw new InvalidRunError(`operator ${operator} holds no key rows; nothing it computes is meaningful`);
    }
    if (report.keyIdsWithKeyset === 0) {
      throw new InvalidRunError(
        `operator ${operator} has ${report.keyRows} key row(s) but no compressed_xof_keyset; ` +
          'that is the column the SNS worker loads, so the material is not usable',
      );
    }
    if (report.crsRows === 0) {
      throw new InvalidRunError(`operator ${operator} holds no CRS activation rows`);
    }
    reports.push(report);
  }

  const keyIds = new Set(reports.map((report) => report.keyIdGw ?? 'none'));
  if (keyIds.size > 1) {
    throw new InvalidRunError(
      `operators hold different keys (${[...keyIds].map((id) => id.slice(0, 16)).join(', ')}); ` +
        'every non-scalar operation would diverge deterministically and a byte-consensus result ' +
        'would blame the protocol for a provisioning fault',
    );
  }
  return reports;
}

/**
 * Gate 2: no operator is sitting on deferred transactions.
 *
 * Deferral is a legitimate transient inside a work window, so this polls rather
 * than sampling once -- "pinned at zero" means it reaches zero, not that it was
 * never non-zero. A gauge that never returns to zero is a wedged scheduler, and
 * every number a suite produces afterwards describes a stack that stopped
 * working.
 */
export async function assertNoDeferredTransactions(
  operators: readonly number[],
  timeoutMs = 60_000,
  metricsPort = Number.parseInt(process.env.TFHE_WORKER_METRICS_PORT ?? '9100', 10),
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let last = new Map<number, number>();
  // Operators whose endpoint was unreachable on this pass, so a retry does not
  // read a stale zero from the previous one.
  const unreachable = new Map<number, string>();
  for (;;) {
    last = new Map();
    unreachable.clear();
    for (const operator of operators) {
      const url = tfheWorkerMetricsUrl(operator, metricsPort);
      let exposition: string;
      try {
        const response = await fetch(url);
        if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
        exposition = await response.text();
      } catch (error) {
        // Unreachable is retryable inside the deadline, not instantly fatal. A
        // worker that just exited on a fatal database error and is coming back
        // under `restart: on-failure` serves nothing for a few seconds, and the
        // failure-matrix database cells create exactly that on purpose -- so
        // failing here turned two legitimate cells into "invalid run". After
        // the deadline it is still a failure: a gate that cannot be evaluated
        // must not pass.
        if (Date.now() < deadline) {
          unreachable.set(operator, error instanceof Error ? error.message : String(error));
          await new Promise((resolve) => setTimeout(resolve, 5_000));
          continue;
        }
        throw new InvalidRunError(
          `operator ${operator}'s tfhe-worker metrics stayed unreachable at ${url} for ` +
            `${Math.round(timeoutMs / 1000)}s ` +
            `(${error instanceof Error ? error.message : String(error)}). The gate cannot be ` +
            'evaluated, and a run whose scheduler state is unknown is not a run worth reporting. ' +
            'Give the worker --metrics-addr, or set TFHE_WORKER_METRICS_PORT.',
        );
      }
      if (!looksLikeCoprocessorWorker(exposition)) {
        throw new InvalidRunError(
          `operator ${operator} answers at ${url} but exposes no coprocessor_ metrics; ` +
            'the gate is reading something other than a coprocessor worker',
        );
      }
      // Absent gauge means the worker has never deferred a transaction: the
      // metrics registry is lazy, so an untouched gauge is simply not there.
      // That is the healthy case this gate wants, not a fault.
      last.set(operator, parseGauge(exposition, DEFERRED_TRANSACTIONS_METRIC) ?? 0);
    }
    if (unreachable.size === 0 && last.size === operators.length && [...last.values()].every((v) => v === 0)) {
      return;
    }
    if (Date.now() >= deadline) {
      const stuck = [...last.entries()]
        .filter(([, value]) => value !== 0)
        .map(([operator, value]) => `operator ${operator}: ${value}`)
        .join(', ');
      throw new InvalidRunError(
        `deferred transactions did not drain within ${Math.round(timeoutMs / 1000)}s (${stuck}); ` +
          'the scheduling window is wedged',
      );
    }
    await new Promise((resolve) => setTimeout(resolve, 5_000));
  }
}

/**
 * Gate 3: the chain is still advancing.
 *
 * A stalled host chain makes every downstream assertion vacuous in the quietest
 * possible way -- nothing new is ingested, so nothing disagrees.
 */
export async function assertChainAdvances(rpcUrl: string, timeoutMs = 60_000): Promise<number> {
  const head = async () => {
    const response = await fetch(rpcUrl, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'eth_blockNumber', params: [] }),
    });
    if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
    const payload = (await response.json()) as { result?: string };
    if (!payload.result) throw new Error('no result');
    return Number.parseInt(payload.result, 16);
  };

  let start: number;
  try {
    start = await head();
  } catch (error) {
    throw new InvalidRunError(
      `the host chain at ${rpcUrl} could not be read ` +
        `(${error instanceof Error ? error.message : String(error)})`,
    );
  }

  const deadline = Date.now() + timeoutMs;
  for (;;) {
    await new Promise((resolve) => setTimeout(resolve, 3_000));
    const now = await head().catch(() => start);
    if (now > start) return now;
    if (Date.now() >= deadline) {
      throw new InvalidRunError(
        `the host chain has not advanced past block ${start} in ${Math.round(timeoutMs / 1000)}s; ` +
          'a suite measuring a stalled chain proves nothing',
      );
    }
  }
}

/**
 * `assertKeyMaterial`, retried until a deadline.
 *
 * Key rows are written by `host-listener`'s kms_generation module when it
 * observes the KMSGeneration events, so they appear asynchronously and, on a
 * fork topology, only once the operator's chain actually carries the keygen
 * history. A one-shot assert there reports "holds no key rows" for an operator
 * that is merely not there yet. This waits, and still fails loudly if the
 * material never lands -- the point is to distinguish slow from absent, not to
 * soften the gate.
 */
export async function waitForKeyMaterial(
  databaseUrls: readonly string[],
  operators: readonly number[] = databaseUrls.map((_, index) => index),
  deadlineMs = 6 * 60_000,
): Promise<KeyMaterialReport[]> {
  const deadline = Date.now() + deadlineMs;
  let last: unknown;
  for (;;) {
    try {
      return await assertKeyMaterial(databaseUrls, operators);
    } catch (error) {
      last = error;
      if (Date.now() >= deadline) break;
      await new Promise((resolve) => setTimeout(resolve, 5_000));
    }
  }
  throw new InvalidRunError(
    `key material never landed on operator(s) ${operators.join(',')} within ${Math.round(deadlineMs / 1000)}s: ` +
      `${last instanceof Error ? last.message : String(last)}`,
  );
}

/** RFC-023 `ciphertext128_format`: 11 is compressed on CPU, 21 compressed on GPU. */
export const CIPHERTEXT128_FORMAT_CPU = 11;
export const CIPHERTEXT128_FORMAT_GPU = 21;

/**
 * Asserts every operator's squashed ciphertexts carry the format this run's
 * backend produces, and that they all carry the same one.
 *
 * This is B-1 made diagnostic. When a CPU container raced a CUDA unit on one
 * operator's queue, the only symptom was a digest disagreement, and establishing
 * the cause took an intervention experiment in both directions. The format field
 * says it outright: on a GPU run an operator recording 11 has a CPU worker in it.
 * Both values are valid squashes of the same input -- they simply differ -- so a
 * fleet split across them cannot reach quorum on the SNS digest while agreeing on
 * everything else, which is the confusing shape B-1 presented as.
 *
 * Absence is not agreement: a run where nothing has been squashed yet would pass
 * a "do they match" check trivially, so no rows anywhere is a failure.
 */
export async function assertCiphertext128Format(
  databaseUrls: readonly string[],
  backendClass: string,
  operators: readonly number[] = databaseUrls.map((_, index) => index),
): Promise<Map<number, number[]>> {
  const expected = backendClass.startsWith('gpu') ? CIPHERTEXT128_FORMAT_GPU : CIPHERTEXT128_FORMAT_CPU;
  const perOperator = new Map<number, number[]>();
  for (const operator of operators) {
    const formats = await withPool(databaseUrls[operator], async (pool) => {
      const rows = await pool.query<{ format: string | null }>(
        `SELECT DISTINCT ciphertext128_format::text AS format
           FROM ciphertext_digest
          WHERE ciphertext128 IS NOT NULL
          ORDER BY 1`,
      );
      return rows.rows
        .map((row) => (row.format === null ? null : Number.parseInt(row.format, 10)))
        .filter((value): value is number => value !== null);
    });
    perOperator.set(operator, formats);
  }

  const seen = [...perOperator.values()].flat();
  if (seen.length === 0)
    throw new InvalidRunError(
      'no operator holds any squashed ciphertext, so the ciphertext128_format gate had nothing to check; ' +
        'a run that squashed nothing cannot evidence a backend',
    );

  // The backend expectation is REPORTED, not enforced, and that is a deliberate
  // consequence of what this gate found. `sns-worker`'s executor picks the format
  // from compression alone --
  //     let format = if enable_compression { CompressedOnCpu } else { UncompressedOnCpu };
  // -- with no GPU branch, and the GPU variants are never assigned anywhere in
  // production code. So a `--features gpu` fleet actively computing on an H100
  // records 11, "compressed on CPU". Failing on that would block every GPU run
  // for a product gap the run cannot fix (Consensus Defect Log, D-4), and would
  // teach whoever hits it to delete the gate. Reporting it keeps the signal
  // where a reader will see it and flips to an assertion the moment D-4 is.
  for (const [operator, formats] of perOperator) {
    const unexpected = formats.filter((format) => format !== expected);
    if (unexpected.length > 0)
      console.warn(
        `[validity] operator ${operator} recorded ciphertext128_format ${unexpected.join(',')} on a ` +
          `${backendClass} run, which produces ${expected}. Expected while D-4 stands: the squash path ` +
          'hardcodes the CPU variants, so the field cannot evidence the backend or detect a ' +
          'mixed-backend fleet.',
      );
  }

  const distinct = new Set(seen);
  if (distinct.size > 1)
    throw new InvalidRunError(
      `operators disagree on ciphertext128_format (${[...distinct].join(', ')}); whatever the field ` +
        'records, the operators do not agree on it, so they did not squash the same way and cannot ' +
        'reach quorum on the SNS digest',
    );
  return perOperator;
}

export interface RunValidityOptions {
  databaseUrls: readonly string[];
  /** Host-chain RPC; when absent the liveness gate is *reported as skipped*, never silently dropped. */
  rpcUrl?: string;
  /**
   * Operators to gate, defaulting to all of them.
   *
   * A matrix cell that deliberately holds an operator down must exclude it
   * here, or the gate reports the injected fault as an invalid run and the cell
   * can never pass. The summary names the exclusions, so a green run cannot
   * quietly have gated nothing.
   */
  operators?: readonly number[];
  /** Set false only where a suite deliberately holds the scheduler still. */
  checkDeferred?: boolean;
}

/**
 * Runs every gate available in this process and returns a one-line summary for
 * the suite to log, so a green run states which gates it passed rather than
 * leaving a reader to assume.
 */
export async function assertRunValidity(options: RunValidityOptions): Promise<string> {
  const all = options.databaseUrls.map((_, index) => index);
  const operators = options.operators ?? all;
  if (operators.length === 0) {
    throw new InvalidRunError('no operators left to gate; a run that gates nothing is not validated');
  }
  const excluded = all.filter((operator) => !operators.includes(operator));
  const passed: string[] = [];

  const keys = await assertKeyMaterial(options.databaseUrls, operators);
  passed.push(`key material on ${keys.length} operator(s), key ${keys[0].keyIdGw?.slice(0, 16) ?? 'none'}`);

  if (options.checkDeferred !== false) {
    await assertNoDeferredTransactions(operators);
    passed.push('deferred transactions at zero');
  }

  if (options.rpcUrl) {
    const head = await assertChainAdvances(options.rpcUrl);
    passed.push(`chain advancing (head ${head})`);
  } else {
    passed.push('chain liveness NOT CHECKED (no rpcUrl given)');
  }

  if (excluded.length > 0) passed.push(`operators ${excluded.join(',')} held out by the caller`);
  return passed.join('; ');
}

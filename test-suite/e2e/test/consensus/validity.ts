/**
 * Run-validity gates: the difference between a result and a number.
 *
 * The coverage inventory sets this as a harness contract rather than a test:
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
 * The fourth gate the inventory names -- a journal grep for
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

/**
 * Fails a promise that outlives its own deadline.
 *
 * Every read in this file is bounded. A gate that declares a 60s deadline and
 * then blocks forever on a stalled endpoint is worse than one that fails: the
 * run neither passes nor reports, and the suite's own timeout eventually kills
 * it with no indication of which gate was stuck.
 */
export async function withDeadline<T>(promise: Promise<T>, ms: number, what: string): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      promise,
      new Promise<never>((_, reject) => {
        timer = setTimeout(() => reject(new InvalidRunError(`${what} did not answer within ${ms}ms`)), ms);
      }),
    ]);
  } finally {
    if (timer) clearTimeout(timer);
  }
}

/**
 * A pool whose statements cannot outlive the gate.
 *
 * `statement_timeout` is set on the connection rather than relying on a
 * wrapper: a query already in flight when a JS timer fires would otherwise keep
 * holding the connection, and the gate's "timed out" would be a lie about a
 * query that is still running.
 */
async function withPool<T>(databaseUrl: string, fn: (pool: Pool) => Promise<T>, statementTimeoutMs = 30_000): Promise<T> {
  const pool = new Pool({
    connectionString: databaseUrl,
    max: 1,
    connectionTimeoutMillis: 10_000,
    statement_timeout: statementTimeoutMs,
  } as never);
  try {
    return await withDeadline(fn(pool), statementTimeoutMs + 15_000, `query against ${redactUrl(databaseUrl)}`);
  } finally {
    await pool.end().catch(() => undefined);
  }
}

/** A database URL without its password, for messages that end up in artifacts. */
export function redactUrl(databaseUrl: string): string {
  return databaseUrl.replace(/(\w+):\/\/([^:@/]+):[^@]*@/, "$1://$2:***@");
}

/** Which key encodings a topology's workers can actually consume. */
export type KeyMaterialCapability = 'compressed-required' | 'compressed-or-legacy';

/**
 * The capability a backend class implies.
 *
 * `sns-worker`'s `keyset.rs` reads `compressed_xof_keyset` when it is present
 * and falls back to a plain ServerKey in the `sns_pk` large object when it is
 * not -- except under `--features gpu`, where the legacy encoding is refused
 * outright ("GPU coprocessor cannot read a legacy ServerKey-format key"). So
 * the same key row is usable material on a CPU topology and unusable on a GPU
 * one, and a gate that applies the compressed column to every topology reports
 * a legacy CLI stack as unprovisioned.
 */
export function keyMaterialCapabilityFor(backendClass: string | undefined): KeyMaterialCapability {
  return (backendClass ?? '').startsWith('gpu') ? 'compressed-required' : 'compressed-or-legacy';
}

export interface KeyMaterialReport {
  operator: number;
  keyRows: number;
  /** Rows carrying a CompressedXofKeySet. */
  compressedRows: number;
  /** Rows carrying only the legacy decompressed ServerKey large object. */
  legacyRows: number;
  crsRows: number;
  /** The key the workers will actually load: highest sequence_number. */
  activeKeyIdGw: string | null;
  activeKeySequence: number | null;
  activeKeyEncoding: 'compressed' | 'legacy' | 'none';
}

/**
 * Gate 1: every required operator holds usable key material, and they all hold
 * the same ACTIVE key.
 *
 * Two things this gate used to get wrong.
 *
 * It took `MIN(encode(key_id_gw,'hex'))` as the operator's key. That is the
 * lexicographically smallest key id in the operator's history, not the key its
 * workers use: `db_keys.rs` and `keyset.rs` both select
 * `ORDER BY sequence_number DESC LIMIT 1`. A shared minimum across historical
 * rows says nothing about whether the three workers are computing under one
 * key, which is the whole point of the check -- every non-scalar operation
 * would diverge deterministically if they were not.
 *
 * And it required `compressed_xof_keyset` on every topology, which is a GPU
 * requirement rather than a universal one; see `keyMaterialCapabilityFor`.
 */
export async function assertKeyMaterial(
  databaseUrls: readonly string[],
  operators: readonly number[] = databaseUrls.map((_, index) => index),
  capability: KeyMaterialCapability = 'compressed-or-legacy',
): Promise<KeyMaterialReport[]> {
  const reports: KeyMaterialReport[] = [];
  for (const operator of operators) {
    const databaseUrl = databaseUrls[operator];
    let report: KeyMaterialReport;
    try {
      report = await withPool(databaseUrl, async (pool) => {
        const totals = await pool.query<{ total: string; compressed: string; legacy: string }>(
          `SELECT COUNT(*)::text AS total,
                  COUNT(compressed_xof_keyset)::text AS compressed,
                  COUNT(sns_pk)::text AS legacy
             FROM keys`,
        );
        // The active key, read exactly the way the workers read it.
        const active = await pool.query<{
          key_id_gw: string;
          sequence_number: string;
          has_compressed: boolean;
          has_legacy: boolean;
        }>(
          `SELECT encode(key_id_gw, 'hex') AS key_id_gw,
                  sequence_number::text     AS sequence_number,
                  compressed_xof_keyset IS NOT NULL AS has_compressed,
                  sns_pk IS NOT NULL                AS has_legacy
             FROM keys
            ORDER BY sequence_number DESC
            LIMIT 1`,
        );
        const crs = await pool.query<{ total: string }>(
          'SELECT COUNT(*)::text AS total FROM kms_crs_activation_events',
        );
        const activeRow = active.rows[0];
        return {
          operator,
          keyRows: Number.parseInt(totals.rows[0].total, 10),
          compressedRows: Number.parseInt(totals.rows[0].compressed, 10),
          legacyRows: Number.parseInt(totals.rows[0].legacy, 10),
          crsRows: Number.parseInt(crs.rows[0].total, 10),
          activeKeyIdGw: activeRow?.key_id_gw ?? null,
          activeKeySequence: activeRow ? Number.parseInt(activeRow.sequence_number, 10) : null,
          activeKeyEncoding: activeRow
            ? activeRow.has_compressed
              ? 'compressed'
              : activeRow.has_legacy
                ? 'legacy'
                : 'none'
            : 'none',
        };
      });
    } catch (error) {
      throw new InvalidRunError(
        `operator ${operator}'s key material could not be read from ${redactUrl(databaseUrl)}: ` +
          `${error instanceof Error ? error.message : String(error)}`,
      );
    }
    if (report.keyRows === 0) {
      throw new InvalidRunError(`operator ${operator} holds no key rows; nothing it computes is meaningful`);
    }
    if (report.activeKeyEncoding === 'none') {
      throw new InvalidRunError(
        `operator ${operator}'s active key (sequence ${report.activeKeySequence}) carries neither ` +
          'compressed_xof_keyset nor sns_pk, so no worker can load it',
      );
    }
    if (capability === 'compressed-required' && report.activeKeyEncoding !== 'compressed') {
      throw new InvalidRunError(
        `operator ${operator}'s active key is legacy-encoded (sns_pk only) and this run is on a GPU ` +
          'backend, which refuses that encoding outright; the material is not usable here',
      );
    }
    if (report.crsRows === 0) {
      throw new InvalidRunError(`operator ${operator} holds no CRS activation rows`);
    }
    reports.push(report);
  }

  const activeKeys = new Set(reports.map((report) => report.activeKeyIdGw ?? 'none'));
  if (activeKeys.size > 1) {
    throw new InvalidRunError(
      `operators would load different ACTIVE keys (${reports
        .map((report) => `${report.operator}:${(report.activeKeyIdGw ?? 'none').slice(0, 16)}`)
        .join(', ')}); every non-scalar operation would diverge deterministically and a byte-consensus ` +
        'result would blame the protocol for a provisioning fault',
    );
  }
  const encodings = new Set(reports.map((report) => report.activeKeyEncoding));
  if (encodings.size > 1) {
    throw new InvalidRunError(
      `operators hold the same active key in different encodings (${reports
        .map((report) => `${report.operator}:${report.activeKeyEncoding}`)
        .join(', ')}); they would decompress it by different paths`,
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
        // Bounded: a metrics endpoint that accepts the connection and never
        // answers would otherwise hold this gate past its own deadline, and the
        // suite timeout would then blame whatever ran next.
        const response = await withDeadline(fetch(url), 10_000, `metrics fetch ${url}`);
        if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
        exposition = await withDeadline(response.text(), 10_000, `metrics body ${url}`);
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
    const response = await withDeadline(
      fetch(rpcUrl, {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'eth_blockNumber', params: [] }),
      }),
      10_000,
      `eth_blockNumber at ${rpcUrl}`,
    );
    if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
    const payload = (await withDeadline(response.json(), 10_000, `eth_blockNumber body`)) as { result?: string };
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

export interface Ciphertext128FormatRow {
  handle: string;
  format: number | null;
}

/** Require complete format evidence for this workload on every participating operator. */
export function assertCiphertext128FormatEvidence(
  perOperator: ReadonlyMap<number, readonly Ciphertext128FormatRow[]>,
  handles: readonly string[],
  backendClass: string,
): Map<number, number[]> {
  const expected = backendClass.startsWith('gpu') ? CIPHERTEXT128_FORMAT_GPU : CIPHERTEXT128_FORMAT_CPU;
  const targets = new Set(handles.map((handle) => handle.toLowerCase().replace(/^0x/, '')));
  if (targets.size === 0 || perOperator.size === 0)
    throw new InvalidRunError('ciphertext128_format requires workload handles and participating operators');
  const formats = new Map<number, number[]>();
  for (const [operator, rows] of perOperator) {
    for (const handle of targets) {
      const evidence = rows.filter((row) => row.handle.toLowerCase().replace(/^0x/, '') === handle);
      if (evidence.length !== 1)
        throw new InvalidRunError(`operator ${operator} has ${evidence.length} squashed rows for 0x${handle}; expected one`);
      if (evidence[0].format !== expected)
        throw new InvalidRunError(
          `operator ${operator} recorded ciphertext128_format ${evidence[0].format} for 0x${handle}; ` +
            `${backendClass} requires ${expected}`,
        );
    }
    formats.set(operator, [expected]);
  }
  return formats;
}

/** Historical ciphertexts may belong to an earlier backend; inspect only this run's outputs. */
export async function assertCiphertext128Format(
  databaseUrls: readonly string[],
  backendClass: string,
  handles: readonly string[],
  operators: readonly number[] = databaseUrls.map((_, index) => index),
): Promise<Map<number, number[]>> {
  if (handles.some((handle) => !/^0x[0-9a-f]{64}$/i.test(handle)))
    throw new InvalidRunError('ciphertext128_format requires bytes32 workload handles');
  const perOperator = new Map<number, Ciphertext128FormatRow[]>();
  for (const operator of operators) {
    const rows = await withPool(databaseUrls[operator], async (pool) =>
      pool.query<Ciphertext128FormatRow>(
        `SELECT encode(handle, 'hex') AS handle, ciphertext128_format AS format
           FROM ciphertext_digest
          WHERE handle = ANY($1::bytea[]) AND ciphertext128 IS NOT NULL`,
        [handles.map((handle) => Buffer.from(handle.slice(2), 'hex'))],
      ),
    );
    perOperator.set(operator, rows.rows);
  }
  return assertCiphertext128FormatEvidence(perOperator, handles, backendClass);
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
  /**
   * Backend class of the run, which decides which key encodings count as usable
   * material. Defaults to the environment's `CONSENSUS_BACKEND_CLASS`.
   */
  backendClass?: string;
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

  const capability = keyMaterialCapabilityFor(options.backendClass ?? process.env.CONSENSUS_BACKEND_CLASS);
  const keys = await assertKeyMaterial(options.databaseUrls, operators, capability);
  passed.push(
    `active key ${keys[0].activeKeyIdGw?.slice(0, 16) ?? 'none'} (${keys[0].activeKeyEncoding}, ` +
      `${capability}) on ${keys.length} operator(s)`,
  );

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

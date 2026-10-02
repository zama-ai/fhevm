/** Scheduling evidence comes from persisted batch sizes, never timer-poll counts. */
import { expect } from 'chai';
import { parseGpuExecutionCounters, type GpuExecutionCounters } from './gpuExecutionEvidence';

import { parseSchedulingClasses } from './helpers';
import { InvalidRunError, looksLikeCoprocessorWorker, parseGauge, tfheWorkerMetricsUrl, withDeadline } from './validity';

/** Counters that describe executed scheduling, read from a worker's exposition. */
export interface SchedulingCounters {
  operator: number;
  gpuExecution?: GpuExecutionCounters[];
  /** Persisted batches (histogram observation count). */
  batches: number;
  /** Transactions in persisted batches (histogram sum). */
  transactions: number;
  /** Work items successfully processed. */
  itemsProcessed: number;
  /** Dependence chains acquired. */
  chainsAcquired: number;
}

const COUNTERS = {
  batches: 'coprocessor_work_batch_transactions_count',
  transactions: 'coprocessor_work_batch_transactions_sum',
  itemsProcessed: 'coprocessor_work_items_processed',
  chainsAcquired: 'coprocessor_tfhe_worker_dcid_counter',
} as const;

/** Reads one operator's scheduling counters. */
export async function readSchedulingCounters(
  operator: number,
  metricsPort?: number,
): Promise<SchedulingCounters> {
  const url = tfheWorkerMetricsUrl(operator, metricsPort);
  let exposition: string;
  try {
    const response = await withDeadline(fetch(url), 10_000, `metrics fetch ${url}`);
    if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
    exposition = await withDeadline(response.text(), 10_000, `metrics body ${url}`);
  } catch (error) {
    throw new InvalidRunError(
      `operator ${operator}'s worker metrics could not be read at ${url} ` +
        `(${error instanceof Error ? error.message : String(error)}); a scheduling claim cannot be ` +
        'made about a worker whose executed behaviour was never observed',
    );
  }
  if (!looksLikeCoprocessorWorker(exposition)) {
    throw new InvalidRunError(`operator ${operator} answers at ${url} but exposes no coprocessor_ metrics`);
  }
  // A counter that has never been touched is absent from a lazy registry, and
  // zero is the right reading for it. A worker that has polled nothing at all,
  // though, is not a worker this gate can say anything about, and the caller
  // checks that against the delta.
  return {
    operator,
    gpuExecution: parseGpuExecutionCounters(exposition),
    batches: parseGauge(exposition, COUNTERS.batches) ?? 0,
    transactions: parseGauge(exposition, COUNTERS.transactions) ?? 0,
    itemsProcessed: parseGauge(exposition, COUNTERS.itemsProcessed) ?? 0,
    chainsAcquired: parseGauge(exposition, COUNTERS.chainsAcquired) ?? 0,
  };
}

export async function readAllSchedulingCounters(
  operators: readonly number[],
  metricsPort?: number,
): Promise<Map<number, SchedulingCounters>> {
  const entries = await Promise.all(
    operators.map(async (operator) => [operator, await readSchedulingCounters(operator, metricsPort)] as const),
  );
  return new Map(entries);
}

export interface ExecutedScheduling {
  operator: number;
  batches: number;
  transactions: number;
  chainsAcquired: number;
  /** Work transactions per batch: the batch the worker actually executed. */
  transactionsPerBatch: number;
  /** Chains per batch, against `--dependence-chains-per-batch`. */
  chainsPerBatch: number;
  /** The configured window this operator was given, when it is recorded. */
  configuredWindow?: number;
  /**
   * False when the operator runs with `FHEVM_DCID_BATCH_EXECUTION=false`, which
   * makes it execute one dependence chain at a time however wide its window is.
   * Its window therefore predicts nothing about the batch it commits.
   */
  batchesIndependentChains?: boolean;
}

/** The delta between two readings, which is what describes the workload. */
export function executedScheduling(
  before: Map<number, SchedulingCounters>,
  after: Map<number, SchedulingCounters>,
  schedulingClasses?: string,
): ExecutedScheduling[] {
  const configured = new Map<number, number>();
  const batching = new Map<number, boolean>();
  if (schedulingClasses) {
    for (const [operator, description] of parseSchedulingClasses(schedulingClasses)) {
      const match = /window:(\d+)/.exec(description);
      if (match) configured.set(operator, Number.parseInt(match[1], 10));
      const batch = /batch:([^,;]+)/.exec(description);
      if (batch) batching.set(operator, batch[1].trim() !== 'false');
    }
  }
  const result: ExecutedScheduling[] = [];
  for (const [operator, start] of before) {
    const end = after.get(operator);
    if (!end) throw new InvalidRunError(`operator ${operator} was read before the workload but not after`);
    const batches = end.batches - start.batches;
    const transactions = end.transactions - start.transactions;
    const chainsAcquired = end.chainsAcquired - start.chainsAcquired;
    if (batches < 0 || transactions < 0 || chainsAcquired < 0) {
      throw new InvalidRunError(`operator ${operator}'s scheduling counters reset during the workload`);
    }
    result.push({
      operator,
      batches,
      transactions,
      chainsAcquired,
      transactionsPerBatch: batches > 0 ? transactions / batches : 0,
      chainsPerBatch: batches > 0 ? chainsAcquired / batches : 0,
      configuredWindow: configured.get(operator),
      batchesIndependentChains: batching.get(operator),
    });
  }
  return result.sort((a, b) => a.operator - b.operator);
}

/**
 * Requires the workload to have exercised materially different batch behaviour
 * across the fleet.
 *
 * Two things are checked, and both are needed. Every operator must have
 * actually done work during the window -- a zero-batch operator makes any ratio
 * comparison meaningless -- and the operator with the smallest configured
 * window must obey its capacity while the widest averages at least one full
 * transaction beyond that capacity. The second is the assertion the previous version was missing: the
 * configured values were compared for distinctness, which a mistyped override
 * fails but an unexercised workload does not.
 */
export function assertExecutedSchedulingDiffers(executed: readonly ExecutedScheduling[], backlogTransactions: number): string {
  if (executed.length < 2) {
    throw new InvalidRunError('an executed-scheduling comparison needs at least two operators');
  }
  const idle = executed.filter((entry) => entry.batches === 0);
  if (idle.length > 0) {
    throw new InvalidRunError(
      `operator(s) ${idle.map((entry) => entry.operator).join(', ')} committed no batches during the ` +
        'workload, so nothing can be concluded about the batch they executed',
    );
  }
  const withConfigured = executed.filter((entry) => entry.configuredWindow !== undefined);
  if (withConfigured.length < 2) {
    throw new InvalidRunError(
      'fewer than two operators have a recorded configured window, so an executed comparison cannot be ' +
        'directed; the launcher must record the resolved configuration',
    );
  }
  // An operator running with batch execution disabled commits one dependence
  // chain per batch whatever its window says, so its window cannot be read as a
  // batching capacity. Comparing against it asks the fleet to prove something
  // its own configuration forbids -- the scenario gives exactly that operator
  // the widest window on purpose. It still has to agree on bytes and still has
  // to have done work; it just cannot direct a window-versus-batch comparison.
  const comparable = withConfigured.filter((entry) => entry.batchesIndependentChains !== false);
  if (comparable.length < 2) {
    throw new InvalidRunError(
      'fewer than two operators both have a recorded window and batch independent chains, so no ' +
        'window-versus-executed-batch comparison can be directed; operator(s) ' +
        `${withConfigured
          .filter((entry) => entry.batchesIndependentChains === false)
          .map((entry) => entry.operator)
          .join(', ')} run with batch execution disabled`,
    );
  }
  const narrowest = comparable.reduce((a, b) => (a.configuredWindow! <= b.configuredWindow! ? a : b));
  const widest = comparable.reduce((a, b) => (a.configuredWindow! >= b.configuredWindow! ? a : b));
  expect(
    narrowest.configuredWindow,
    'the fleet must have at least two DIFFERENT configured windows for this comparison to mean anything',
  ).to.not.eq(widest.configuredWindow);

  // A meaningful distinction crosses the narrow configuration's integer
  // capacity by a full transaction per average batch. A microscopic ratio
  // difference (one merged pair among thousands of singleton batches) does
  // not demonstrate that the workload exercised the wide batching strategy.
  const distinctBatchSize = narrowest.configuredWindow! + 1;
  if (!Number.isSafeInteger(backlogTransactions) || backlogTransactions < 2 * distinctBatchSize) {
    throw new InvalidRunError(`the identified backlog needs at least ${2 * distinctBatchSize} transactions to exercise two batches beyond narrow window ${narrowest.configuredWindow}`);
  }
  if (executed.some(entry => entry.transactions < backlogTransactions)) {
    throw new InvalidRunError('scheduling counters do not cover the complete identified backlog on every operator');
  }

  const summary = executed
    .map(
      (entry) =>
        `${entry.operator}: ${entry.transactions} transaction(s) over ${entry.batches} batch(es) = ` +
        `${entry.transactionsPerBatch.toFixed(2)}/batch, ${entry.chainsPerBatch.toFixed(2)} chain(s)/batch` +
        `${entry.configuredWindow !== undefined ? ` (window ${entry.configuredWindow})` : ''}`,
    )
    .join('; ');

  // Equal batches and inverted batches are NOT the same finding, and recording
  // both as a failure is what made this case say the fleet was broken when the
  // truth was that the workload never drove it.
  //
  // Equal means the comparison was never directed: with nothing queued beyond
  // what the narrow window already takes, a wide window has nothing to batch,
  // and the run reports agreement across strategies it did not exercise. That
  // is an invalid run, not a violated property.
  //
  // Inverted -- the narrowly configured operator taking MORE per batch than the
  // wide one -- is a real violation: the configuration is not in force in the
  // direction it claims.
  if (narrowest.transactionsPerBatch > widest.transactionsPerBatch) {
    expect.fail(
      `operator ${narrowest.operator} is configured with a window of ${narrowest.configuredWindow} and ` +
        `operator ${widest.operator} with ${widest.configuredWindow}, but the NARROWER window executed the ` +
        `larger average batch. The configuration is not in force in the direction it claims. ` +
        `Observed: ${summary}`,
    );
  }
  if (narrowest.transactionsPerBatch === widest.transactionsPerBatch) {
    throw new InvalidRunError(
      `operator ${narrowest.operator} is configured with a window of ${narrowest.configuredWindow} and ` +
        `operator ${widest.operator} with ${widest.configuredWindow}, but they executed the same average ` +
        `batch. The workload did not exercise the difference, so this run reports agreement across ` +
        `scheduling strategies it never drove. Observed: ${summary}`,
    );
  }

  if (widest.transactionsPerBatch < distinctBatchSize) {
    throw new InvalidRunError(
      `wide operator ${widest.operator} must average at least ${distinctBatchSize} transactions per batch ` +
      `to exceed the narrow configured capacity by a full transaction; the observed difference is insufficient. Observed: ${summary}`,
    );
  }

  // The narrow operator must genuinely be near its own bound, or "smaller" is
  // just noise: an operator pinned to one item per batch cannot average more.
  if (narrowest.configuredWindow !== undefined) {
    expect(
      narrowest.transactionsPerBatch,
      `operator ${narrowest.operator} averaged ${narrowest.transactionsPerBatch.toFixed(2)} transactions per batch against a ` +
        `configured window of ${narrowest.configuredWindow}; the flag was not in force. Observed: ${summary}`,
    ).to.be.at.most(narrowest.configuredWindow);
  }
  // Acquisition counts are independent from transaction batch sizes. With
  // this fresh independent-chain backlog, both must cover the work; a wide
  // transaction batch alone could come from a single long dependence chain.
  if (executed.some(entry => entry.chainsAcquired < backlogTransactions)) {
    throw new InvalidRunError('chain acquisition counters do not cover the independent scheduling backlog on every operator');
  }
  if (widest.chainsPerBatch < 2) {
    throw new InvalidRunError('the widest operator must acquire at least two independent chains per committed batch on average');
  }
  return summary;
}

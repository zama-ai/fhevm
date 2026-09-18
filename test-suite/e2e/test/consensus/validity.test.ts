/**
 * Unit tests for the run-validity gates' parsing, which is the part that can be
 * wrong without a stack. The gates themselves need a live fleet; the reader of a
 * Prometheus exposition does not.
 */
import { expect } from 'chai';

import {
  assertCiphertext128FormatEvidence,
  DEFERRED_TRANSACTIONS_METRIC,
  looksLikeCoprocessorWorker,
  parseGauge,
  tfheWorkerMetricsUrl,
} from './validity';

describe('parseGauge', () => {
  const exposition = [
    '# HELP coprocessor_worker_deferred_transactions_current transactions deferred in the most recent work window',
    '# TYPE coprocessor_worker_deferred_transactions_current gauge',
    'coprocessor_worker_deferred_transactions_current 0',
    'coprocessor_work_items_polls 1234',
  ].join('\n');

  it('reads a bare gauge', () => {
    expect(parseGauge(exposition, DEFERRED_TRANSACTIONS_METRIC)).to.eq(0);
  });

  it('reads a non-zero value, which is what a wedged scheduler looks like', () => {
    expect(parseGauge('coprocessor_worker_deferred_transactions_current 7', DEFERRED_TRANSACTIONS_METRIC)).to.eq(7);
  });

  it('reads a labeled gauge', () => {
    expect(parseGauge('some_metric{operator="1",shard="a"} 3', 'some_metric')).to.eq(3);
  });

  it('ignores comment lines that mention the metric name', () => {
    // The HELP and TYPE lines both contain the metric name; taking either as a
    // value would make the gate report nonsense instead of failing.
    const commentsOnly = exposition.split('\n').filter((line) => line.startsWith('#')).join('\n');
    expect(parseGauge(commentsOnly, DEFERRED_TRANSACTIONS_METRIC)).to.eq(undefined);
  });

  it('does not match a metric whose name merely starts the same', () => {
    // `_total` would otherwise be read as the gauge and the gate would pass on
    // an unrelated counter.
    expect(parseGauge('coprocessor_worker_deferred_transactions_current_total 9', DEFERRED_TRANSACTIONS_METRIC))
      .to.eq(undefined);
  });

  it('returns undefined for an absent metric, which the caller treats as a wrong build', () => {
    expect(parseGauge('other_metric 1', DEFERRED_TRANSACTIONS_METRIC)).to.eq(undefined);
  });

  it('tolerates scientific notation and negatives rather than reporting NaN', () => {
    expect(parseGauge('g 1e2', 'g')).to.eq(100);
    expect(parseGauge('g -3', 'g')).to.eq(-3);
  });
});

describe('tfheWorkerMetricsUrl', () => {
  it('names operator 0 without an index, matching the container naming', () => {
    expect(tfheWorkerMetricsUrl(0)).to.eq('http://coprocessor-tfhe-worker:9100/metrics');
  });

  it('names later operators with their index', () => {
    expect(tfheWorkerMetricsUrl(2)).to.eq('http://coprocessor2-tfhe-worker:9100/metrics');
  });

  it('honors a non-default port', () => {
    expect(tfheWorkerMetricsUrl(1, 9999)).to.eq('http://coprocessor1-tfhe-worker:9999/metrics');
  });
});

describe('looksLikeCoprocessorWorker', () => {
  it('accepts an exposition carrying any coprocessor_ series', () => {
    // Metrics register lazily, so a worker that has never deferred exposes no
    // deferred gauge -- only whatever has been touched. This is what the live
    // fleet actually returns on an idle stack.
    const idle = ['coprocessor_tfhe_worker_bridge_errors_counter 89', 'coprocessor_worker_errors 18'].join('\n');
    expect(looksLikeCoprocessorWorker(idle)).to.eq(true);
    expect(parseGauge(idle, DEFERRED_TRANSACTIONS_METRIC)).to.eq(undefined);
  });

  it('rejects an exposition from something that is not a coprocessor worker', () => {
    expect(looksLikeCoprocessorWorker('go_goroutines 12\nprocess_cpu_seconds_total 3')).to.eq(false);
  });

  it('rejects an empty body, which is how a wrong port answers', () => {
    expect(looksLikeCoprocessorWorker('')).to.eq(false);
  });
});


describe('ciphertext128 format evidence', () => {
  const handle = `0x${'ab'.repeat(32)}`;
  const historic = `0x${'cd'.repeat(32)}`;
  const row = { handle: handle.slice(2), format: 21 };

  it('accepts GPU evidence while excluding historical CPU outputs', () => {
    const rows = new Map([[0, [row, { handle: historic, format: 11 }]], [1, [row]]]);
    expect([...assertCiphertext128FormatEvidence(rows, [handle], 'gpu-cuda').keys()]).to.deep.eq([0, 1]);
  });

  it('rejects a missing participant output', () => {
    expect(() => assertCiphertext128FormatEvidence(new Map([[0, [row]], [1, []]]), [handle], 'gpu-cuda'))
      .to.throw('operator 1 has 0 squashed rows');
  });

  it('rejects a missing format value', () => {
    expect(() => assertCiphertext128FormatEvidence(new Map([[0, [{ ...row, format: null }]]]), [handle], 'gpu-cuda'))
      .to.throw('ciphertext128_format null');
  });

  it('rejects a CPU output even when every operator agrees on it during a GPU run', () => {
    const cpu = [{ ...row, format: 11 }];
    expect(() => assertCiphertext128FormatEvidence(new Map([[0, cpu], [1, cpu]]), [handle], 'gpu-cuda'))
      .to.throw('gpu-cuda requires 21');
  });

  it('accepts the CPU format for a CPU run', () => {
    expect([...assertCiphertext128FormatEvidence(new Map([[0, [{ ...row, format: 11 }]]]), [handle], 'cpu').values()])
      .to.deep.eq([[11]]);
  });

  it('rejects duplicate evidence for a workload handle', () => {
    expect(() => assertCiphertext128FormatEvidence(new Map([[0, [row, row]]]), [handle], 'gpu-cuda'))
      .to.throw('2 squashed rows');
  });

  it('rejects an empty workload or an empty fleet', () => {
    expect(() => assertCiphertext128FormatEvidence(new Map([[0, [row]]]), [], 'gpu-cuda')).to.throw('workload handles');
    expect(() => assertCiphertext128FormatEvidence(new Map(), [handle], 'gpu-cuda')).to.throw('participating operators');
  });
});

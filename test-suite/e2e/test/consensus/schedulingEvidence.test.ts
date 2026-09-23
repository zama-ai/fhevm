import { expect } from 'chai';
import { assertExecutedSchedulingDiffers, executedScheduling, type SchedulingCounters } from './schedulingEvidence';

const counters = (operator: number, batches: number, transactions: number): SchedulingCounters => ({
  operator, batches, transactions, chainsAcquired: transactions, itemsProcessed: transactions,
});
const before = new Map([0, 1, 2].map((operator) => [operator, counters(operator, 0, 0)]));
const classes = '0=window:100;1=window:1;2=window:200';

describe('Committed scheduling evidence', () => {
  it('counts batches that finish between scrapes and compares all operators', () => {
    const after = new Map([[0, counters(0, 3, 24)], [1, counters(1, 24, 24)], [2, counters(2, 2, 24)]]);
    const execution = executedScheduling(before, after, classes);
    expect(execution[2].transactionsPerBatch).to.eq(12);
    expect(assertExecutedSchedulingDiffers(execution, 24)).to.include('24 transaction(s) over 2 batch(es)');
  });
  it('rejects distinct flags that did not produce distinct batches', () => {
    const after = new Map([0, 1, 2].map((operator) => [operator, counters(operator, 24, 24)]));
    expect(() => assertExecutedSchedulingDiffers(executedScheduling(before, after, classes), 24)).to.throw('same average');
  });
  it('rejects a missing or idle operator', () => {
    expect(() => executedScheduling(before, new Map(), classes)).to.throw('not after');
    expect(() => assertExecutedSchedulingDiffers(executedScheduling(before, before, classes), 24)).to.throw('nothing can be concluded');
  });
  it('rejects a counter reset instead of averaging across process restarts', () => {
    const start = new Map([[0, counters(0, 8, 24)]]);
    expect(() => executedScheduling(start, new Map([[0, counters(0, 1, 2)]]), classes)).to.throw('reset');
  });
  it('rejects a one-transaction margin caused by one coincident batch', () => {
    const after = new Map([[0, counters(0, 24, 24)], [1, counters(1, 24, 24)], [2, counters(2, 23, 24)]]);
    expect(() => assertExecutedSchedulingDiffers(executedScheduling(before, after, classes), 24)).to.throw('difference is insufficient');
  });
  it('accepts a full transaction of batching beyond the narrow capacity', () => {
    const after = new Map([[0, counters(0, 12, 24)], [1, counters(1, 24, 24)], [2, counters(2, 12, 24)]]);
    expect(() => assertExecutedSchedulingDiffers(executedScheduling(before, after, classes), 24)).not.to.throw();
  });
  it('does not direct the comparison at an operator configured not to batch chains', () => {
    // The heterogeneous scenario gives the widest window to the operator that
    // runs with batch execution off, so it commits one chain per batch however
    // wide the window is. Reading that as "the wide window did not batch" asks
    // the fleet to disprove its own configuration.
    const scenario = '0=window:100,batch:default;1=window:1,adaptive:false,batch:default;2=window:200,batch:false';
    const after = new Map([[0, counters(0, 3, 24)], [1, counters(1, 24, 24)], [2, counters(2, 24, 24)]]);
    const executed = executedScheduling(before, after, scenario);
    expect(executed[2].batchesIndependentChains).to.eq(false);
    expect(assertExecutedSchedulingDiffers(executed, 24)).to.include('window 200');
  });
  it('refuses when excluding the non-batching operators leaves nothing to compare', () => {
    const scenario = '0=window:100,batch:false;1=window:1,batch:false;2=window:200,batch:false';
    const after = new Map([[0, counters(0, 3, 24)], [1, counters(1, 24, 24)], [2, counters(2, 24, 24)]]);
    expect(() => assertExecutedSchedulingDiffers(executedScheduling(before, after, scenario), 24))
      .to.throw('batch execution disabled');
  });
  it('requires enough identified work for the configured capacities and complete counter coverage', () => {
    const after = new Map([[0, counters(0, 3, 24)], [1, counters(1, 24, 24)], [2, counters(2, 2, 24)]]);
    const executed = executedScheduling(before, after, classes);
    expect(() => assertExecutedSchedulingDiffers(executed, 3)).to.throw('at least 4 transactions');
    expect(() => assertExecutedSchedulingDiffers(executed, 25)).to.throw('complete identified backlog');
  });
});

describe('Independent chain scheduling evidence', () => {
  it('rejects transaction batching without acquisition of the independent backlog', () => {
    const after = new Map([[0, counters(0, 3, 24)], [1, counters(1, 24, 24)], [2, counters(2, 2, 24)]]);
    const execution = executedScheduling(before, after, classes);
    execution[2].chainsAcquired = 1;
    execution[2].chainsPerBatch = 0.5;
    expect(() => assertExecutedSchedulingDiffers(execution, 24)).to.throw('independent scheduling backlog');
  });
  it('rejects acquired chains spread across singleton batches even if transaction batches differ', () => {
    const after = new Map([[0, counters(0, 3, 24)], [1, counters(1, 24, 24)], [2, { ...counters(2, 24, 96), chainsAcquired: 24 }]]);
    expect(() => assertExecutedSchedulingDiffers(executedScheduling(before, after, classes), 24)).to.throw('at least two independent chains');
  });
});

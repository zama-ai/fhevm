import { successfulForkReceipt, waitForForkSentinel } from './forkRecovery';
import { expect } from 'chai';
import { assertRepairDisabledControl, assertReplayHasEffects, assertReplayAttempts } from './forkRecovery';

describe('fork fault controls', () => {
  const stranded = { status: 'updated', dependencyCount: 1, owned: false, unprocessedProducers: 0 };
  it('rejects a child that completed through ordinary parent release', () => {
    expect(() => assertRepairDisabledControl({ ...stranded, status: 'processed', dependencyCount: 0 }, 1)).to.throw('genuinely stranded');
  });
  it('rejects a gate still waiting on a live producer', () => {
    expect(() => assertRepairDisabledControl({ ...stranded, unprocessedProducers: 1 }, 1)).to.throw('genuinely stranded');
  });
  it('requires the disabled repair to have actually been attempted', () => {
    expect(() => assertRepairDisabledControl(stranded, 0)).to.throw('never attempted');
    expect(() => assertRepairDisabledControl(stranded, 1)).not.to.throw();
  });
  it('rejects the empty-block replay that previously passed', () => {
    expect(() => assertReplayHasEffects({ computations: 0, allows: 0, chains: 0 })).to.throw('identified graph');
  });
  it('requires every claimed replay effect to be present', () => {
    for (const field of ['computations', 'allows', 'chains']) {
      expect(() => assertReplayHasEffects({ computations: 2, allows: 2, chains: 2, [field]: 0 })).to.throw();
    }
    expect(() => assertReplayHasEffects({ computations: 2, allows: 2, chains: 2 })).not.to.throw();
  });
  it('rejects cursor-only progress without committed attempts for every event', () => {
    for (const attempts of [[], [1], [0, 0], [1, 0]]) {
      expect(() => assertReplayAttempts(attempts, 2)).to.throw('every selected replay event');
    }
    expect(() => assertReplayAttempts([1, 1], 2)).not.to.throw();
  });
});

describe('Fork branch inclusion evidence', () => {
  it('waits for the exact receipt before permitting a branch state read', async () => {
    let release!: (value: { status: number; blockNumber: number }) => void;
    const pending = new Promise<{ status: number; blockNumber: number }>(resolve => { release = resolve; });
    let read = false;
    const check = successfulForkReceipt({ wait: () => pending }, 'fork').then(() => { read = true; });
    await Promise.resolve();
    expect(read).to.eq(false);
    release({ status: 1, blockNumber: 42 });
    await check;
    expect(read).to.eq(true);
  });
  it('rejects missing, reverted and unmined receipts', async () => {
    for (const receipt of [null, { status: 0, blockNumber: 42 }, { status: 1, blockNumber: 0 }]) {
      let error: unknown;
      try { await successfulForkReceipt({ wait: async () => receipt }, 'fork'); } catch (caught) { error = caught; }
      expect(String(error)).to.include('successful mined receipt');
    }
  });
});

describe('Fork operator recovery evidence', () => {
  it('rejects a stopped operator even when canonical decryption could succeed', async () => {
    let now = 0;
    let failure: unknown;
    try {
      await waitForForkSentinel(async () => ({ replacementSeen: false, total: 0, completed: 0, errors: 0 }),
        { now: () => now, timeoutMs: 2, pause: async () => { now++; } });
    } catch (error) { failure = error; }
    expect(String(failure)).to.include('did not ingest');
  });
  it('requires completion as well as replacement ingestion', async () => {
    let calls = 0;
    await waitForForkSentinel(async () => ({ replacementSeen: true, total: 1, completed: calls++ ? 1 : 0, errors: 0 }), { pause: async () => {} });
    expect(calls).to.eq(2);
  });
  it('rejects a caught-up listener whose worker never completes the sentinel', async () => {
    let failure: unknown;
    try {
      await waitForForkSentinel(async () => ({ replacementSeen: true, total: 1, completed: 0, errors: 0 }), { timeoutMs: 0 });
    } catch (error) { failure = error; }
    expect(String(failure)).to.include('complete its fresh sentinel');
  });
});

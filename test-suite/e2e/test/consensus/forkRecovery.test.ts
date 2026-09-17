import { successfulForkReceipt } from './forkRecovery';
import { expect } from 'chai';

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

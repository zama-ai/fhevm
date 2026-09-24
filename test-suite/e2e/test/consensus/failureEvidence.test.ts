import { requireQuorumConfiguration } from './helpers';
import { expect } from 'chai';
import { requireDriftBaseline, requireIngestionTarget, requireBacklogTargets } from './failureEvidence';

describe('Required failure evidence', () => {
  it('requires gateway quorum configuration for every non-smoke workload', () => {
    const address = `0x${'ab'.repeat(20)}`;
    for (const workload of ['compute-chain', 'ingestion', 'submission', 'storage', 'detector-drift', 'zkproof-input']) {
      expect(() => requireQuorumConfiguration(workload, 'http://gateway:8546', address, address)).not.to.throw();
      for (const args of [['', address, address], ['http://gateway:8546', '', address], ['http://gateway:8546', address, '']]) {
        expect(() => requireQuorumConfiguration(workload, ...args as [string, string, string])).to.throw('requires gateway');
      }
    }
    expect(() => requireQuorumConfiguration('smoke', '', '', '')).not.to.throw();
  });
  it('does not let a missing workload height collapse ingestion recovery to block zero', () => {
    expect(requireIngestionTarget({ blockNumber: 42, chainId: '12345' })).to.deep.eq({ blockNumber: 42, chainId: '12345' });
    for (const blockNumber of [undefined, null, 0, -1, NaN, Infinity, '42', 1.5]) {
      expect(() => requireIngestionTarget({ blockNumber, chainId: '12345' })).to.throw('recorded integer');
    }
    for (const chainId of [undefined, '', '0', 'undefined', 12345]) {
      expect(() => requireIngestionTarget({ blockNumber: 42, chainId })).to.throw('chain ID');
    }
  });
  it('requires complete, consistent drift baselines while accepting an actually observed zero', () => {
    expect(requireDriftBaseline({ signalsBefore: 0, signalsBeforeByOperator: [0, 0, 0] }, 1, 3)).to.deep.eq([0, 0, 0]);
    for (const detail of [undefined, {}, { signalsBeforeByOperator: [0, 0, 0] }, { signalsBefore: 0, signalsBeforeByOperator: [0, undefined, 0] }, { signalsBefore: 0, signalsBeforeByOperator: [0, 0] }, { signalsBefore: 1, signalsBeforeByOperator: [0, 0, 0] }]) {
      expect(() => requireDriftBaseline(detail, 1, 3)).to.throw();
    }
  });
});

describe('bounded backlog target contract', () => {
  const values = Array.from({length: 12}, (_, i) => '0x' + (i + 1).toString(16).padStart(64, '0'));
  const blocks = Array.from({length: 12}, (_, i) => i + 100);
  it('rejects a dropped target, aliased target, missing receipt or single-page backlog', () => {
    expect(() => requireBacklogTargets(values, values, blocks)).not.to.throw();
    for (const targets of [values.slice(1), [...values.slice(0, 11), values[0]]]) expect(() => requireBacklogTargets(targets, values, blocks)).to.throw('twelve');
    expect(() => requireBacklogTargets(values, values.slice(1), blocks)).to.throw('twelve');
    expect(() => requireBacklogTargets(values, values, blocks.map(() => 100))).to.throw('pages');
    expect(() => requireBacklogTargets(values, values, blocks.slice(1))).to.throw('pages');
  });
});

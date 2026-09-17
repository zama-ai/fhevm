import { expect } from 'chai';
import { assertFixtureTransactionShape } from './materializationProvenance';
import { FIXTURE_TRANSACTIONS } from './materializationFixtureModel';
import type { MaterializationFixtureRun } from './materializationFixture';
import type { ConsensusDatabaseReport } from './helpers';

function fixture() {
  const run = { handles: {}, transactionHashes: {}, hostChainId: 12345,
    sameBlockNumber: 100, terminalBlockNumber: 101 } as unknown as MaterializationFixtureRun;
  const report: ConsensusDatabaseReport = { databaseUrl: 'isolated fixture', outputs: [], transactions: [] };
  let handleIndex = 1;
  for (const [index, transaction] of FIXTURE_TRANSACTIONS.entries()) {
    const transactionId = Buffer.alloc(32, index + 1);
    (run.transactionHashes as Record<string, string>)[transaction.name] = `0x${transactionId.toString('hex')}`;
    const blockNumber = transaction.block === 'terminal' ? 101 : 100;
    for (const label of transaction.producedLabels) {
      const handle = Buffer.alloc(32, handleIndex++);
      (run.handles as Record<string, string>)[label] = `0x${handle.toString('hex')}`;
      report.outputs.push({ handle, transactionId, blockNumber, hostChainId: 12345 } as typeof report.outputs[number]);
    }
    report.transactions.push({ transactionId, blockNumber, hostChainId: 12345,
      totalCount: transaction.exactComputationCount, completedCount: transaction.exactComputationCount, errorCount: 0 });
  }
  return { run, report };
}
describe('materialization receipt provenance', () => {
  it('accepts complete graphs attributed to their actual receipt transactions and chain', () => {
    const { run, report } = fixture();
    expect(() => assertFixtureTransactionShape([report, report, report], run)).not.to.throw();
  });
  it('rejects unanimously fabricated transaction IDs despite internally consistent graph completion', () => {
    const { run, report } = fixture();
    for (const output of report.outputs) output.transactionId = Buffer.from(output.transactionId.map(byte => byte + 16));
    for (const row of report.transactions) row.transactionId = Buffer.from(row.transactionId.map(byte => byte + 16));
    expect(() => assertFixtureTransactionShape([report, report, report], run)).to.throw();
  });
  it('rejects swapping the two equal-size graphs in the same block', () => {
    const { run, report } = fixture();
    const a = run.transactionHashes['stage-input-a'], b = run.transactionHashes['run-independent'];
    for (const output of report.outputs) {
      const id = `0x${output.transactionId.toString('hex')}`;
      if (id === a || id === b) output.transactionId = Buffer.from((id === a ? b : a).slice(2), 'hex');
    }
    expect(() => assertFixtureTransactionShape([report, report, report], run)).to.throw();
  });
  it('rejects a common wrong host-chain attribution', () => {
    const { run, report } = fixture();
    for (const output of report.outputs) output.hostChainId = 67890;
    for (const row of report.transactions) row.hostChainId = 67890;
    expect(() => assertFixtureTransactionShape([report, report, report], run)).to.throw('selected host chain');
  });
});

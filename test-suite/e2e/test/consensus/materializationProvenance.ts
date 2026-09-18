import { expect } from 'chai';
import type { ConsensusDatabaseReport } from './helpers';
import type { MaterializationFixtureRun } from './materializationFixture';
import { FIXTURE_TRANSACTIONS } from './materializationFixtureModel';

/**
 * The transaction is the materialization boundary: every produced output of
 * one fixture transaction must carry that transaction's provenance, the four
 * fixture transactions must stay distinct, land on the expected L1 blocks,
 * and be completely executed (exact per-transaction row counts).
 */
export function assertFixtureTransactionShape(
  reports: readonly ConsensusDatabaseReport[],
  run: MaterializationFixtureRun,
): void {
  for (const report of reports) {
    const outputByHandle = new Map(report.outputs.map((output) => [`0x${output.handle.toString('hex')}`, output]));
    const completionById = new Map(
      report.transactions.map((transaction) => [`0x${transaction.transactionId.toString('hex')}`, transaction]),
    );
    const seenTransactionIds = new Set<string>();

    for (const transaction of FIXTURE_TRANSACTIONS) {
      const outputs = transaction.producedLabels.map((label) => {
        const output = outputByHandle.get(run.handles[label].toLowerCase());
        expect(output, `${report.databaseUrl} is missing ${label}`).to.not.be.undefined;
        return output!;
      });
      const transactionIdHex = run.transactionHashes[transaction.name].toLowerCase();
      const transactionId = Buffer.from(transactionIdHex.slice(2), 'hex');
      seenTransactionIds.add(transactionIdHex);

      const expectedBlockNumber = transaction.block === 'terminal' ? run.terminalBlockNumber : run.sameBlockNumber;
      for (const output of outputs) {
        expect(output.hostChainId, `${transaction.name} must belong to the selected host chain`).to.eq(run.hostChainId);
        expect(
          output.transactionId.equals(transactionId),
          `${report.databaseUrl} split ${transaction.name} across producing transactions`,
        ).to.eq(true);
        expect(
          output.blockNumber,
          `${report.databaseUrl} assigned ${transaction.name} output to the wrong canonical block height`,
        ).to.eq(expectedBlockNumber);
      }

      const completion = completionById.get(transactionIdHex);
      expect(completion, `${report.databaseUrl} must report completion for ${transaction.name}`).to.not.be.undefined;
      expect(completion!.hostChainId, `${transaction.name} completion has the wrong host chain`).to.eq(run.hostChainId);
      expect(completion!.totalCount, `${transaction.name} must persist its exact computation row count`).to.eq(
        transaction.exactComputationCount,
      );
      expect(completion!.completedCount, `${transaction.name} must be completely executed at quiescence`).to.eq(
        transaction.exactComputationCount,
      );
      expect(completion!.errorCount, `${transaction.name} must have no errored computations`).to.eq(0);
      expect(completion!.blockNumber, `${transaction.name} completion has the wrong block height`).to.eq(
        expectedBlockNumber,
      );
    }

    expect(
      seenTransactionIds.size,
      'the staged, derived, independent, and terminal graphs must stay distinct transactions',
    ).to.eq(FIXTURE_TRANSACTIONS.length);
    expect(report.transactions).to.have.length(FIXTURE_TRANSACTIONS.length);
  }
}

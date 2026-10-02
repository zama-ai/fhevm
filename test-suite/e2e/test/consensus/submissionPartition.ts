import { expect } from 'chai';
import { Pool } from 'pg';
import { emitAssertions } from './assertionEvidence';
import { publishHandshake, readHandshake } from './handshake';
import { assertGatewayTopology, getCoprocessorDbUrls, waitForDatabaseReadiness } from './helpers';
import { assertOperatorsAgree, assertQuorumOutcome, deployProbe, operatorSet, type ProbeContract } from './probe';
import { collectOperatorEvidence, queryStorageRowCount } from './comparator';
import { assertNoQuorumWithSurvivorSubmissions } from './quorumObservation';
import { assertRunValidity, withDeadline } from './validity';

const CASE = 'DEG-07-SUBMISSION-PARTITION';
type Target = { caseId: string; address: string; handle: string; transaction: string };
describe('Compute participation versus submission visibility', function () {
  this.timeout(20 * 60_000);
  it('retains work below threshold, reaches quorum after one rejoin and converges after full recovery', async function () {
    if (process.env.RUN_SUBMISSION_PARTITION !== '1') this.skip();
    const phase = process.env.SUBMISSION_PARTITION_PHASE;
    if (!['arm', 'majority', 'rejoin'].includes(phase ?? '')) throw new Error('explicit partition phase required');
    const databases = getCoprocessorDbUrls(3);
    await waitForDatabaseReadiness(databases);
    if (phase === 'arm') await assertRunValidity({ databaseUrls: databases, rpcUrl: process.env.RPC_URL });
    const membership = await assertGatewayTopology(process.env.GATEWAY_RPC_URL!, process.env.GATEWAY_CONFIG_ADDRESS!, 3, 2);
    const { initSigners, getSigners } = await import('../signers');
    await initSigners(2);
    const signers = await getSigners();
    let target: Target, contract: ProbeContract;
    if (phase === 'arm') {
      const deployed = await deployProbe(signers.alice);
      contract = deployed.contract;
      const receipt = await (await contract.combineFromStorage({ gasLimit: 10_000_000 })).wait() as { hash: string };
      expect(receipt.hash).to.match(/^0x[0-9a-f]{64}$/i);
      target = { caseId: CASE, address: deployed.address, handle: (await contract.combined()).toLowerCase(), transaction: receipt!.hash };
      await assertOperatorsAgree(databases, operatorSet(3), target.handle, { timeoutMs: 300_000 });
      const evidence = await collectOperatorEvidence(databases[0], 0, target.handle);
      await assertNoQuorumWithSurvivorSubmissions({
        gatewayRpcUrl: process.env.GATEWAY_RPC_URL!, ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!,
        handle: target.handle, authorizedSenders: membership.txSenders, survivorCount: 1,
        expected: { keyId: BigInt(`0x${evidence.keyId.toString('hex')}`).toString(),
          ciphertextDigest: `0x${evidence.computeDigest!.toString('hex')}`, snsCiphertextDigest: `0x${evidence.snsDigest!.toString('hex')}` },
      });
      publishHandshake('submission-partition', target);
      emitAssertions(CASE, ['precondition', 'bytes', 'safety'], 'All three operators computed the original output; advancing observations with one authorized submission showed no quorum while two senders were offline.');
    } else {
      target = readHandshake<Target>('submission-partition').payload;
      expect(target.caseId).to.eq(CASE);
      const { ethers } = await import('hardhat');
      contract = await ethers.getContractAt('AliasFixture', target.address, signers.alice) as unknown as ProbeContract;
      await assertQuorumOutcome({ mode: 'required', gatewayRpcUrl: process.env.GATEWAY_RPC_URL!,
        ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!, handle: target.handle,
        authorizedSenders: membership.txSenders, threshold: 2, label: CASE });
      if (phase === 'majority') emitAssertions(CASE, ['quorum'], 'The original handle reached the observed threshold of two after only one of the two stopped senders returned.');
      else {
        await assertOperatorsAgree(databases, operatorSet(3), target.handle, { timeoutMs: 300_000 });
        for (const database of databases) {
          expect(await queryStorageRowCount(database, target.handle)).to.eq(1);
          const pool = new Pool({ connectionString: database, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
          try {
            const rows = (await pool.query('SELECT is_completed,is_error FROM computations WHERE output_handle=$1 AND transaction_id=$2',
              [Buffer.from(target.handle.slice(2), 'hex'), Buffer.from(target.transaction.slice(2), 'hex')])).rows;
            expect(rows).to.deep.eq([{ is_completed: true, is_error: false }]);
          } finally { await pool.end(); }
        }
        const { createInstances } = await import('../instance');
        const instance = (await createInstances(signers)).alice;
        const decrypt = (handle: string) => withDeadline(instance.userDecryptSingleHandle({ handle, contractAddress: target.address, signer: signers.alice }), 300_000, 'partition recovery decryption');
        expect(String(await decrypt(target.handle))).to.eq('12');
        await (await contract.consumeCombined({ gasLimit: 10_000_000 })).wait();
        const fresh = (await contract.consumed()).toLowerCase();
        await assertOperatorsAgree(databases, operatorSet(3), fresh, { timeoutMs: 300_000 });
        await assertQuorumOutcome({ mode: 'required', gatewayRpcUrl: process.env.GATEWAY_RPC_URL!,
          ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!, handle: fresh,
          authorizedSenders: membership.txSenders, threshold: 2, label: CASE });
        expect(String(await decrypt(fresh))).to.eq('19');
        emitAssertions(CASE, ['liveness', 'correctness'], 'The original transaction remained unique and complete across all operators and decrypted to twelve; fresh work after full rejoin agreed and decrypted to nineteen.');
      }
    }
    console.info(`[submission-partition/${phase}] CASE COMPLETE`);
  });
});

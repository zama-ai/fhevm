import { expect } from 'chai';
import { readFileSync } from 'node:fs';
import { Pool } from 'pg';

import { emitAssertions } from './assertionEvidence';
import { queryStorageRowCount } from './comparator';
import { publishHandshake, readHandshake } from './handshake';
import { assertGatewayTopology, getCoprocessorDbUrls, waitForDatabaseReadiness } from './helpers';
import { type ProbeContract, assertOperatorsAgree, assertQuorumOutcome, deployProbe, operatorSet } from './probe';
import { type SenderFaultMode, type SenderRpcEvent, assertSenderFault } from './senderRpcEvidence';
import { assertRunValidity, withDeadline } from './validity';

type Target = { handle: string; transaction: string };
type Workload = { caseId: string; address: string; targets: Target[] };
type QueueRow = {
  handle: string;
  txn_is_sent: boolean;
  txn_limited_retries_count: number;
  txn_last_error: string | null;
};
const sleep = () => new Promise((resolve) => setTimeout(resolve, 500));

describe('Transaction sender RPC recovery', function () {
  this.timeout(20 * 60_000);
  it('recovers every original material submission after the actual RPC fault', async function () {
    if (process.env.RUN_SENDER_RPC_RECOVERY !== '1') this.skip();
    const mode = process.env.SENDER_RPC_MODE as SenderFaultMode;
    const phase = process.env.SENDER_RPC_PHASE;
    if (!['nonce-low', 'accepted-timeout'].includes(mode) || !['arm', 'fault', 'verify'].includes(phase ?? ''))
      throw new Error('explicit sender mode and phase required');
    const caseId = `FM-SENDER-RPC-${mode.toUpperCase()}`;
    const databases = getCoprocessorDbUrls(3);
    await waitForDatabaseReadiness(databases);
    await assertRunValidity({
      databaseUrls: databases,
      rpcUrl: process.env.RPC_URL,
      operators: operatorSet(3, phase === 'arm' ? [1] : []),
      checkDeferred: false,
    });
    const membership = await assertGatewayTopology(
      process.env.GATEWAY_RPC_URL!,
      process.env.GATEWAY_CONFIG_ADDRESS!,
      3,
      3,
    );
    const { initSigners, getSigners } = await import('../signers');
    await initSigners(2);
    const signers = await getSigners();
    const pool = new Pool({
      connectionString: databases[1],
      connectionTimeoutMillis: 10_000,
      statement_timeout: 10_000,
    });
    const queue = async (targets: Target[]): Promise<QueueRow[]> =>
      (
        await pool.query<QueueRow>(
          "SELECT '0x'||encode(handle,'hex') AS handle, txn_is_sent, txn_limited_retries_count, txn_last_error FROM ciphertext_digest WHERE handle = ANY($1::bytea[])",
          [targets.map((target) => Buffer.from(target.handle.slice(2), 'hex'))],
        )
      ).rows;
    try {
      if (phase === 'arm') {
        // The runner holds only the victim sender down. Host ingestion,
        // computation, object publication, and the other operators are real.
        const deployed = await deployProbe(signers.alice);
        const targets: Target[] = [];
        for (let index = 0; index < 12; index++) {
          const receipt = (await (await deployed.contract.combineFromStorage({ gasLimit: 10_000_000 })).wait()) as {
            hash: string;
          };
          targets.push({
            handle: (await deployed.contract.combined()).toLowerCase(),
            transaction: receipt.hash.toLowerCase(),
          });
        }
        expect(new Set(targets.map((target) => target.handle)).size).to.eq(12);
        for (const target of targets)
          await assertOperatorsAgree(databases, operatorSet(3), target.handle, { timeoutMs: 300_000 });
        const rows = await queue(targets);
        expect(rows).to.have.length(12);
        expect(rows.every((row) => !row.txn_is_sent && row.txn_limited_retries_count === 0)).to.eq(true);
        publishHandshake('sender-rpc-workload', { caseId, address: deployed.address, targets } satisfies Workload);
        emitAssertions(
          caseId,
          ['precondition', 'bytes'],
          'Twelve receipt-identified outputs have matching complete material on all operators, zero victim retry counts, and no victim submission.',
        );
      } else {
        const workload = readHandshake<Workload>('sender-rpc-workload').payload;
        expect(workload.caseId).to.eq(caseId);
        const events = (): SenderRpcEvent[] =>
          readFileSync(process.env.SENDER_RPC_EVENTS!, 'utf8')
            .split('\n')
            .filter(Boolean)
            .map((line) => JSON.parse(line));
        if (phase === 'fault') {
          const deadline = Date.now() + 180_000;
          for (;;) {
            try {
              assertSenderFault(
                events(),
                workload.targets.map((target) => target.handle),
                mode,
              );
              break;
            } catch (error) {
              if (Date.now() >= deadline) throw error;
            }
            await sleep();
          }
          // Retain old-code exhaustion diagnostics as evidence, rather than
          // accepting a reduced backlog after dropped/deleted work.
          publishHandshake('sender-rpc-fault', { caseId, mode, rows: await queue(workload.targets) });
          emitAssertions(
            caseId,
            ['fault'],
            `Every original handle saw ${mode === 'nonce-low' ? 'three nonce rejections' : 'a real successful receipt whose response was withheld until client cancellation'}.`,
          );
        } else {
          assertSenderFault(
            events(),
            workload.targets.map((target) => target.handle),
            mode,
          );
          const deadline = Date.now() + 180_000;
          let rows: QueueRow[];
          for (;;) {
            rows = await queue(workload.targets);
            if (rows.length === 12 && rows.every((row) => row.txn_is_sent)) break;
            if (Date.now() >= deadline)
              throw new Error(`original sender backlog did not drain after RPC recovery: ${JSON.stringify(rows)}`);
            await sleep();
          }
          expect(
            rows.every((row) => row.txn_limited_retries_count === 0),
            'recoverable RPC faults must preserve the finite budget',
          ).to.eq(true);
          const { ethers } = await import('hardhat');
          const gateway = new ethers.JsonRpcProvider(process.env.GATEWAY_RPC_URL!);
          for (const target of workload.targets) {
            await assertOperatorsAgree(databases, operatorSet(3), target.handle, { timeoutMs: 120_000 });
            for (const database of databases) expect(await queryStorageRowCount(database, target.handle)).to.eq(1);
            await assertQuorumOutcome({
              mode: 'required',
              gatewayRpcUrl: process.env.GATEWAY_RPC_URL!,
              ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!,
              handle: target.handle,
              authorizedSenders: membership.txSenders,
              threshold: 3,
              label: caseId,
            });
            if (mode === 'accepted-timeout') {
              const observed = events().find(
                (event) => event.handle === target.handle && event.mode === 'accepted-response-dropped',
              )!;
              const receipt = await gateway.getTransactionReceipt(observed.hash!);
              expect(receipt?.status, 'the withheld response belongs to a real mined transaction').to.eq(1);
              expect(membership.txSenders.map((sender) => sender.toLowerCase())).to.include(
                receipt?.from.toLowerCase(),
              );
              expect(
                receipt?.logs.some(
                  (log) =>
                    log.address.toLowerCase() === process.env.CIPHERTEXT_COMMITS_ADDRESS!.toLowerCase() &&
                    log.topics[1]?.toLowerCase() === target.handle,
                ),
                'the withheld receipt contains the original material handle',
              ).to.eq(true);
            }
          }
          const contract = (await ethers.getContractAt(
            'AliasFixture',
            workload.address,
            signers.alice,
          )) as unknown as ProbeContract;
          const { createInstances } = await import('../instance');
          const instance = (await createInstances(signers)).alice;
          const decrypt = (handle: string) =>
            withDeadline(
              instance.userDecryptSingleHandle({ handle, contractAddress: workload.address, signer: signers.alice }),
              300_000,
              'sender RPC recovery decryption',
            );
          for (const target of workload.targets) expect(String(await decrypt(target.handle))).to.eq('12');
          const receipt = (await (await contract.combineFromStorage({ gasLimit: 10_000_000 })).wait()) as {
            hash: string;
          };
          const fresh = { handle: (await contract.combined()).toLowerCase(), transaction: receipt.hash };
          expect(workload.targets.some((target) => target.handle === fresh.handle)).to.eq(false);
          await assertOperatorsAgree(databases, operatorSet(3), fresh.handle, { timeoutMs: 300_000 });
          await assertQuorumOutcome({
            mode: 'required',
            gatewayRpcUrl: process.env.GATEWAY_RPC_URL!,
            ciphertextCommitsAddress: process.env.CIPHERTEXT_COMMITS_ADDRESS!,
            handle: fresh.handle,
            authorizedSenders: membership.txSenders,
            threshold: 3,
            label: caseId,
          });
          expect((await queue([fresh]))[0]?.txn_is_sent).to.eq(true);
          expect(String(await decrypt(fresh.handle))).to.eq('12');
          emitAssertions(
            caseId,
            ['liveness', 'quorum', 'safety', 'correctness'],
            'All twelve original rows completed without spending the finite budget, remained unique across operators, reached three-sender quorum and decrypted to twelve; fresh work also completed and decrypted.',
          );
        }
      }
      console.info(`[sender-rpc/${caseId}/${phase}] CASE COMPLETE`);
    } finally {
      await pool.end();
    }
  });
});

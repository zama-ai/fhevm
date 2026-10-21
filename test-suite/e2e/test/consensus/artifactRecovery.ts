/** Full-environment Artifact recovery. The host runner owns RPC/service faults;
 * this suite uses deployed contracts, running services, Gateway and the SDK.
 * Only the explicit rewind phase writes DB state, and only the poller cursor. */
import { expect } from 'chai';
import { ethers } from 'hardhat';
import { randomBytes } from 'node:crypto';
import { Pool } from 'pg';

import { emitAssertions } from './assertionEvidence';
import { publishHandshake, readHandshake } from './handshake';
import { assertGatewayTopology, getCoprocessorDbUrls, requireQuorumConfiguration } from './helpers';
import { assertOperatorsAgree, assertQuorumOutcome } from './probe';
import { withDeadline } from './validity';

const ENABLE = process.env.RUN_ARTIFACT_RECOVERY === '1';
const PHASE = process.env.ARTIFACT_PHASE ?? '';
const CASE = process.env.ARTIFACT_CASE_ID ?? '';
const GAS = { gasLimit: 10_000_000 };
interface Workload {
  caseId: string;
  runId: string;
  chainId: string;
  contract: string;
  acl: string;
  transactionHash: string;
  blockHash: string;
  blockNumber: number;
  lastBlock: number;
  handles: string[];
  expected: string[];
  snapshot?: unknown[];
}
const bytes = (handle: string) => Buffer.from(handle.slice(2), 'hex');
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

async function until(check: () => Promise<boolean>, label: string, timeout = 180_000) {
  const deadline = Date.now() + timeout;
  do {
    if (await check()) return;
    await sleep(1_000);
  } while (Date.now() < deadline);
  throw new Error(`timed out: ${label}`);
}

(ENABLE ? describe : describe.skip)('Artifact ingestion recovery on deployed services', function () {
  this.timeout(900_000);
  it('recovers the original graph through Gateway and public decryption', async function () {
    expect(['seed', 'blocked', 'overlap', 'rewind', 'verify', 'duplicate']).to.include(PHASE);
    expect(CASE).to.match(/^INGEST-ACL-(OVERLAP|CURSOR)$/);
    const runId = process.env.CONSENSUS_RUN_ID;
    expect(runId).to.be.a('string').and.not.empty;
    const rpc = process.env.GATEWAY_RPC_URL ?? '';
    const config = process.env.GATEWAY_CONFIG_ADDRESS ?? '';
    const commits = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
    requireQuorumConfiguration(CASE, rpc, config, commits);
    // Unanimity makes the victim's Gateway participation mandatory.
    const membership = await assertGatewayTopology(rpc, config, 3, 3);
    const urls = getCoprocessorDbUrls(3);
    const pool = new Pool({
      connectionString: urls[1],
      max: 1,
      connectionTimeoutMillis: 10_000,
      statement_timeout: 15_000,
    });
    const chainId = String((await ethers.provider.getNetwork()).chainId);
    const { getSigners, initSigners } = await import('../signers');
    await initSigners(2);
    const signers = await getSigners();
    const load = () => {
      const record = readHandshake<Workload>('artifact-workload').payload;
      expect(record.caseId).to.eq(CASE);
      expect(record.runId).to.eq(runId);
      expect(record.chainId).to.eq(chainId);
      return record;
    };
    const flags = async (handle: string) => {
      const result = await pool.query(
        `SELECT c.is_allowed, c.is_completed, c.is_error,
        (SELECT count(*)::int FROM ciphertexts ct WHERE ct.handle=c.output_handle) AS ciphertexts
        FROM computations c WHERE c.output_handle=$1`,
        [bytes(handle)],
      );
      expect(result.rows.length, `one computation for ${handle}`).to.be.at.most(1);
      return result.rows[0];
    };
    const snapshot = async (record: Workload) =>
      (
        await pool.query(
          `SELECT encode(c.output_handle,'hex') AS handle,
      c.is_allowed,c.is_completed,c.is_error,md5(ct.ciphertext) AS ciphertext,
      ct.ciphertext_version FROM computations c JOIN ciphertexts ct ON ct.handle=c.output_handle
      WHERE c.output_handle=ANY($1::bytea[]) ORDER BY c.output_handle,ct.ciphertext_version`,
          [record.handles.map(bytes)],
        )
      ).rows;
    const quorum = (handle: string) =>
      assertQuorumOutcome({
        mode: 'required',
        gatewayRpcUrl: rpc,
        ciphertextCommitsAddress: commits,
        handle,
        authorizedSenders: membership.txSenders,
        threshold: 3,
        timeoutMs: 240_000,
      });
    const healthy = async () => {
      const result = await pool.query('SELECT count(*)::int AS n FROM chain_recovery_halts WHERE resolved_at IS NULL');
      expect(result.rows[0].n, 'test must not repair published history or trigger containment').to.eq(0);
    };
    try {
      await healthy();
      if (PHASE === 'seed') {
        const contract = (await (
          await ethers.getContractFactory('ArtifactRecoveryFixture', signers.alice)
        ).deploy()) as unknown as import('ethers').Contract;
        await contract.waitForDeployment();
        // A unique seed prevents a previously materialized trivial handle from
        // accidentally satisfying this run's deliberately missing producer.
        const seed = BigInt(`0x${randomBytes(6).toString('hex')}`);
        const receipt = await (await contract.seed(seed, GAS)).wait();
        expect(receipt?.status).to.eq(1);
        const acl = (process.env.ACL_CONTRACT_ADDRESS ?? '').toLowerCase();
        expect(acl).to.match(/^0x[0-9a-f]{40}$/);
        expect(
          receipt!.logs.filter((log: { address: string }) => log.address.toLowerCase() === acl).length,
        ).to.be.greaterThan(0);
        const root = String(await contract.root()).toLowerCase();
        expect(await flags(root), 'victim ingestion must be stopped before seeding').to.eq(undefined);
        publishHandshake<Workload>('artifact-workload', {
          caseId: CASE,
          runId: runId!,
          chainId,
          contract: await contract.getAddress(),
          acl,
          transactionHash: receipt!.hash.toLowerCase(),
          blockHash: receipt!.blockHash.toLowerCase(),
          blockNumber: receipt!.blockNumber,
          lastBlock: receipt!.blockNumber,
          handles: [root],
          expected: [String(seed)],
        });
      } else {
        const record = load();
        const contract = await ethers.getContractAt('ArtifactRecoveryFixture', record.contract, signers.alice);
        if (PHASE === 'blocked') {
          await until(async () => (await flags(record.handles[0])) !== undefined, 'partial producer ingestion');
          const missing = { is_allowed: false, is_completed: true, is_error: false, ciphertexts: 0 };
          expect(await flags(record.handles[0])).to.deep.eq(missing);
          const children = await (await contract.children(GAS)).wait();
          const terminal = await (await contract.grandchild(GAS)).wait();
          const canary = await (await contract.independent(BigInt(record.expected[0]) + 100n, GAS)).wait();
          expect(children!.blockNumber).to.be.greaterThan(record.blockNumber);
          expect(terminal!.blockNumber).to.be.greaterThan(children!.blockNumber);
          record.lastBlock = canary!.blockNumber;
          record.handles.push(
            ...(await Promise.all(
              ['left', 'right', 'terminal', 'canary'].map(async (key) => String(await contract[key]()).toLowerCase()),
            )),
          );
          expect(new Set(record.handles).size).to.eq(5);
          const root = BigInt(record.expected[0]);
          record.expected.push(String(root + 5n), String(root + 9n), String(root * 2n + 14n), String(root + 100n));
          publishHandshake('artifact-workload', record);
          await assertOperatorsAgree(urls, [0, 1, 2], record.handles[4], { timeoutMs: 240_000 });
          await quorum(record.handles[4]);
          // The independent canary completed all the way to Gateway, including
          // the victim. Observe the graph blocked over repeated live samples.
          for (let i = 0; i < 10; i++) {
            await healthy();
            expect(await flags(record.handles[0])).to.deep.eq(missing);
            for (const handle of record.handles.slice(1, 4)) {
              expect(await flags(handle)).to.deep.eq({
                is_allowed: true,
                is_completed: false,
                is_error: false,
                ciphertexts: 0,
              });
            }
            await sleep(1_000);
          }
          emitAssertions(
            CASE,
            ['precondition', 'fault'],
            'The original producer is unallowed without ciphertext; three allowed descendants remain pending while the independent canary reaches unanimous Gateway consensus.',
          );
        } else if (PHASE === 'overlap') {
          const result = await pool.query(
            'SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=$1',
            [chainId],
          );
          const cursor = Number(result.rows[0]?.last_caught_up_block);
          expect(Number.isSafeInteger(cursor)).to.eq(true);
          expect(cursor).to.be.at.least(record.lastBlock);
          expect(
            cursor - record.blockNumber,
            'original producer must be inside the configured 512-block startup overlap',
          ).to.be.lessThan(512);
        } else if (PHASE === 'rewind') {
          if (!record.snapshot && CASE === 'INGEST-ACL-CURSOR') {
            const cursor = await pool.query(
              'SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=$1',
              [chainId],
            );
            expect(
              Number(cursor.rows[0]?.last_caught_up_block),
              'one-block overlap alone must miss the producer',
            ).to.be.greaterThan(record.blockNumber);
          }
          // Host runner stops the poller before invoking this phase. This is
          // the supported operator replay action; no computation/ACL repair SQL.
          const result = await pool.query(
            `UPDATE host_listener_poller_state
            SET last_caught_up_block=LEAST(last_caught_up_block,$2)
            WHERE chain_id=$1 RETURNING last_caught_up_block`,
            [chainId, record.blockNumber - 1],
          );
          expect(result.rowCount).to.eq(1);
        } else if (PHASE === 'verify') {
          const { createInstances } = await import('../instance');
          const instance = (await createInstances(signers)).alice;
          for (let i = 0; i < record.handles.length; i++) {
            const handle = record.handles[i];
            await assertOperatorsAgree(urls, [0, 1, 2], handle, { timeoutMs: 240_000 });
            expect(await flags(handle)).to.deep.eq({
              is_allowed: true,
              is_completed: true,
              is_error: false,
              ciphertexts: 1,
            });
            await quorum(handle);
            const result = await withDeadline(
              instance.publicDecrypt([handle]),
              240_000,
              'original graph public decrypt',
            );
            expect(result.clearValues[handle as `0x${string}`]).to.eq(BigInt(record.expected[i]));
          }
          await healthy();
          record.snapshot = await snapshot(record);
          expect(record.snapshot.length).to.eq(5);
          publishHandshake('artifact-workload', record);
          emitAssertions(
            CASE,
            ['bytes', 'quorum', 'correctness', 'liveness'],
            'All five original handles agree across three operators, reach unanimous Gateway consensus, and decrypt through the relayer/KMS to their independently calculated values.',
          );
        } else {
          expect(record.snapshot, 'verify phase must precede duplicate replay').to.have.length(5);
          await until(async () => {
            const result = await pool.query(
              'SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=$1',
              [chainId],
            );
            return Number(result.rows[0]?.last_caught_up_block ?? -1) >= record.lastBlock;
          }, 'duplicate replay crosses the original graph');
          expect(await snapshot(record)).to.deep.eq(record.snapshot);
          await healthy();
          emitAssertions(
            CASE,
            ['safety'],
            'An explicit second replay crossed all original blocks without changing ciphertext bytes, completion flags or storage cardinality, and no containment halt appeared.',
          );
        }
      }
      console.info(`[artifact-recovery/${CASE}/${PHASE}] CASE COMPLETE`);
    } finally {
      await pool.end();
    }
  });
});

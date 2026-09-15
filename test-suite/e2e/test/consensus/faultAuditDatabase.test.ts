import { createDisposableOracleDatabase } from './disposableOracleDatabase';
import { expect } from 'chai';
import { Pool } from 'pg';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { recoverAbortedSuite, completeDetectorRecovery } from './abortRecovery';
import { INTERRUPTED_WORKER_LOCKS_SQL } from './faultEvidence';
import { INSTALL_PROOF_OUTCOME_AUDIT, assertOriginalProofSucceeded, recoveredVerifierOutcomes, type ProofOutcome } from './proofRecovery';

// Explicit admin URL; the fixture creates and owns a unique child database.
const adminDatabaseUrl = process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL;
(adminDatabaseUrl ? describe : describe.skip)('committed fault audits (isolated PostgreSQL)', function () {
  let pool: Pool;
  let databaseUrl: string;
  let disposable: Awaited<ReturnType<typeof createDisposableOracleDatabase>> | undefined;
  const handle = Buffer.alloc(32, 0xab);
  const worker = '11111111-1111-4111-8111-111111111111';
  before(async () => {
    disposable = await createDisposableOracleDatabase(adminDatabaseUrl!);
    pool = disposable.pool;
    databaseUrl = disposable.databaseUrl;
    await pool.query(`
      CREATE TABLE verify_proofs (zk_proof_id bigint PRIMARY KEY, verified boolean, verified_at timestamptz, handles bytea);
      CREATE TABLE ciphertexts (handle bytea PRIMARY KEY, ciphertext bytea);
      CREATE TABLE ciphertext_digest (handle bytea PRIMARY KEY, ciphertext bytea, txn_is_sent boolean);
      CREATE TABLE dependence_chain (dependence_chain_id bytea PRIMARY KEY, worker_id uuid, lock_expires_at timestamptz);
      ${INSTALL_PROOF_OUTCOME_AUDIT}`);
  });
  after(async () => {
    await disposable?.close();
  });
  beforeEach(async () => {
    await pool.query('TRUNCATE verify_proofs, ciphertexts, ciphertext_digest, consensus_test_proof_outcomes, dependence_chain');
  });
  const outcomes = async () => (await pool.query<ProofOutcome>(`
    SELECT verified, encode(handles, 'hex') AS handles, host(client_address) AS "clientAddress",
           observed_at::text AS "observedAt", operation FROM consensus_test_proof_outcomes`)).rows;
  const insertPendingReplay = async () => {
    await pool.query('INSERT INTO verify_proofs VALUES (71, NULL, NULL, NULL)');
    // The gateway listener converts the interrupted original to local replay.
    // This UPDATE must not masquerade as local cryptographic verification.
    await pool.query('UPDATE verify_proofs SET verified=NULL, verified_at=NOW(), handles=$1 WHERE zk_proof_id=71', [handle]);
    expect(await outcomes()).to.have.length(0);
  };

  it('records successful replay DELETE only with exact ciphertext materialization in the committed transaction', async () => {
    await insertPendingReplay();
    const fault = (await pool.query('SELECT clock_timestamp()::text AS at')).rows[0].at;
    await pool.query('BEGIN');
    await pool.query('INSERT INTO ciphertexts VALUES ($1, $2)', [handle, Buffer.from('ciphertext')]);
    await pool.query('DELETE FROM verify_proofs WHERE zk_proof_id=71');
    await pool.query('COMMIT');
    const rows = await outcomes();
    expect(rows).to.have.length(1);
    expect(rows[0].operation).to.eq('replay-delete');
    expect(() => assertOriginalProofSucceeded(recoveredVerifierOutcomes(rows, rows[0].clientAddress, fault), [`0x${handle.toString('hex')}`])).not.to.throw();
    expect(() => assertOriginalProofSucceeded(recoveredVerifierOutcomes(rows, '192.0.2.1', fault), [`0x${handle.toString('hex')}`])).to.throw('no committed success');
  });
  it('does not accept deletion with missing or different materialization', async () => {
    await insertPendingReplay();
    await pool.query('INSERT INTO ciphertexts VALUES ($1, $2)', [Buffer.alloc(32, 0xcd), Buffer.from('other')]);
    await pool.query('DELETE FROM verify_proofs WHERE zk_proof_id=71');
    expect(await outcomes()).to.have.length(0);
  });
  it('does not accept a ciphertext placeholder without materialized bytes', async () => {
    await insertPendingReplay();
    await pool.query('INSERT INTO ciphertexts VALUES ($1, NULL)', [handle]);
    await pool.query('DELETE FROM verify_proofs WHERE zk_proof_id=71');
    expect(await outcomes()).to.have.length(0);
  });
  it('does not preserve an outcome when replay materialization and deletion roll back', async () => {
    await insertPendingReplay();
    await pool.query('BEGIN');
    await pool.query('INSERT INTO ciphertexts VALUES ($1, $2)', [handle, Buffer.from('ciphertext')]);
    await pool.query('DELETE FROM verify_proofs WHERE zk_proof_id=71');
    await pool.query('ROLLBACK');
    expect(await outcomes()).to.have.length(0);
  });
  it('preserves ordinary committed acceptance and rejection after sender cleanup', async () => {
    await pool.query('INSERT INTO verify_proofs VALUES (71, NULL, NULL, NULL), (72, NULL, NULL, NULL)');
    await pool.query('UPDATE verify_proofs SET verified=true, handles=$1 WHERE zk_proof_id=71', [handle]);
    await pool.query('UPDATE verify_proofs SET verified=false WHERE zk_proof_id=72');
    await pool.query('DELETE FROM verify_proofs');
    expect((await outcomes()).map(row => row.verified)).to.deep.eq([true, false]);
  });
  it('finds recently expired and unexpired leases of the interrupted worker without blaming another worker', async () => {
    await pool.query(`INSERT INTO dependence_chain VALUES
      ('\\x01', $1, NOW() - INTERVAL '1 second'),
      ('\\x02', $1, NOW() + INTERVAL '1 minute'),
      ('\\x03', '22222222-2222-4222-8222-222222222222', NOW() - INTERVAL '1 second')`, [worker]);
    expect((await pool.query(INTERRUPTED_WORKER_LOCKS_SQL, [worker])).rows.map(row => row.chain)).to.deep.eq(['01', '02']);
    await pool.query('UPDATE dependence_chain SET worker_id=NULL WHERE worker_id=$1', [worker]);
    expect((await pool.query(INTERRUPTED_WORKER_LOCKS_SQL, [worker])).rows).to.have.length(0);
  });
  it('restores an unsubmitted detector digest after its arming process is killed', async function () {
    this.timeout(15_000);
    const directory = mkdtempSync(path.join(tmpdir(), 'detector-abort-pg-'));
    const previous = process.env.CONSENSUS_RECOVERY_DIR;
    process.env.CONSENSUS_RECOVERY_DIR = directory;
    const original = Buffer.alloc(32, 0xcd);
    try {
      await pool.query('INSERT INTO ciphertext_digest VALUES ($1,$2,false)', [handle, original]);
      const child = spawnSync(process.execPath, ['-r', require.resolve('ts-node/register/transpile-only'), '-e',
        `require(${JSON.stringify(require.resolve('./canary'))}).tamperUnsubmittedDigest(process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL, '0x${handle.toString('hex')}').then(() => process.kill(process.pid, 'SIGKILL')).catch(error => { console.error(error); process.exit(1); })`],
        { env: { ...process.env, CONSENSUS_ORACLE_TEST_DATABASE_URL: databaseUrl }, timeout: 10_000, encoding: 'utf8' });
      expect(child.signal, child.stderr).to.eq('SIGKILL');
      expect(readdirSync(directory).filter(name => name.endsWith('.json'))).to.have.length(1);
      const poisoned = (await pool.query('SELECT ciphertext, txn_is_sent FROM ciphertext_digest')).rows[0];
      expect(poisoned.ciphertext.equals(original)).to.eq(false);
      expect(poisoned.txn_is_sent).to.eq(false);
      await recoverAbortedSuite();
      const restored = (await pool.query('SELECT ciphertext, txn_is_sent FROM ciphertext_digest')).rows[0];
      expect(restored.ciphertext.equals(original)).to.eq(true);
      expect(restored.txn_is_sent).to.eq(false);
      expect(readdirSync(directory)).to.have.length(0);
      await recoverAbortedSuite();
    } finally {
      if (previous === undefined) delete process.env.CONSENSUS_RECOVERY_DIR;
      else process.env.CONSENSUS_RECOVERY_DIR = previous;
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it('retains contamination after possible detector publication until explicit verified recovery', async function () {
    this.timeout(15_000);
    const directory = mkdtempSync(path.join(tmpdir(), 'detector-published-abort-pg-'));
    const previous = process.env.CONSENSUS_RECOVERY_DIR;
    process.env.CONSENSUS_RECOVERY_DIR = directory;
    const original = Buffer.alloc(32, 0xcd);
    const handleHex = `0x${handle.toString('hex')}`;
    try {
      await pool.query('INSERT INTO ciphertext_digest VALUES ($1,$2,false)', [handle, original]);
      const child = spawnSync(process.execPath, ['-r', require.resolve('ts-node/register/transpile-only'), '-e',
        `const recovery=require(${JSON.stringify(require.resolve('./abortRecovery'))}); require(${JSON.stringify(require.resolve('./canary'))}).tamperUnsubmittedDigest(process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL, '${handleHex}').then(() => { recovery.markDetectorPublicationStarted(process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL, '${handleHex}'); process.kill(process.pid, 'SIGKILL'); }).catch(error => { console.error(error); process.exit(1); })`],
        { env: { ...process.env, CONSENSUS_ORACLE_TEST_DATABASE_URL: databaseUrl }, timeout: 10_000, encoding: 'utf8' });
      expect(child.signal, child.stderr).to.eq('SIGKILL');
      let caught: unknown;
      try { await recoverAbortedSuite(); } catch (error) { caught = error; }
      expect(String(caught)).to.include('retain the private recovery journal');
      expect(readdirSync(directory).filter(name => name.endsWith('.json'))).to.have.length(1);
      expect((await pool.query('SELECT ciphertext FROM ciphertext_digest')).rows[0].ciphertext.equals(original)).to.eq(false);
      // A later verified detector completion is a different authority from
      // generic abort cleanup. Its caller must prove the signal reached done.
      await pool.query('UPDATE ciphertext_digest SET ciphertext=$1, txn_is_sent=true', [original]);
      await completeDetectorRecovery(databaseUrl!, handleHex, original);
      expect(readdirSync(directory)).to.have.length(0);
      await recoverAbortedSuite();
    } finally {
      if (previous === undefined) delete process.env.CONSENSUS_RECOVERY_DIR;
      else process.env.CONSENSUS_RECOVERY_DIR = previous;
      rmSync(directory, { recursive: true, force: true });
    }
  });

});

import { expect } from 'chai';
import { createDisposableOracleDatabase } from './disposableOracleDatabase';
import { syntheticTrackReadinessSql, syntheticEvidenceAuditSql, SYNTHETIC_EVIDENCE_CLEANUP_SQL, DRY_RUN_EVIDENCE_INSTALL_SQL, DRY_RUN_EVIDENCE_CLEANUP_SQL, dryRunEvidenceReadinessSql } from './upgradeEvidenceSql';

const url = process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL;
(url ? describe : describe.skip)('quiet upgrade evidence (isolated PostgreSQL)', () => {
  let database: Awaited<ReturnType<typeof createDisposableOracleDatabase>>;
  beforeEach(async () => {
    database = await createDisposableOracleDatabase(url!);
    await database.pool.query(`CREATE TABLE upgrade_state(stack_role text,host_chain_id bigint,version text,synthetic_txn_hashes bytea);
      CREATE SCHEMA "gcs-v0.15";
      CREATE TABLE "gcs-v0.15".computations(transaction_id bytea,host_chain_id bigint,is_completed boolean,is_error boolean);
      INSERT INTO upgrade_state VALUES ('GCS',12345,'v0.15',decode(repeat('ab',32),'hex')),('GCS',67890,'v0.15',decode(repeat('cd',32),'hex'));
      INSERT INTO "gcs-v0.15".computations SELECT synthetic_txn_hashes,host_chain_id,true,false FROM upgrade_state CROSS JOIN generate_series(1,3);`);
    await database.pool.query(syntheticEvidenceAuditSql('v0.15'));
  });
  afterEach(async () => { await database?.close(); });
  const read = async () => Number((await database.pool.query(syntheticTrackReadinessSql('v0.15', ['12345', '67890']))).rows[0].count);
  it('retains both complete host tracks after cutover removes Green and clears markers', async () => {
    expect(await read(), 'no cutover evidence yet').to.eq(0);
    await database.pool.query(`BEGIN; DELETE FROM "gcs-v0.15".computations;
      UPDATE upgrade_state SET synthetic_txn_hashes=''::bytea;
      DROP SCHEMA "gcs-v0.15" CASCADE; COMMIT;`);
    expect(await read()).to.eq(2);
    const receipt = await database.pool.query(`SELECT encode(markers,'hex') AS markers, jsonb_array_length(computations) AS computations FROM consensus_test_synthetic_evidence`);
    expect(receipt.rows).to.have.length(2);
    for (const row of receipt.rows) { expect(row.markers).to.have.length(64); expect(row.computations).to.eq(3); }
    await database.pool.query(SYNTHETIC_EVIDENCE_CLEANUP_SQL);
    expect((await database.pool.query(`SELECT to_regclass('consensus_test_synthetic_evidence') AS audit, to_regprocedure('consensus_test_capture_synthetic()') AS fn`)).rows[0]).to.deep.eq({ audit: null, fn: null });
  });
  it('rejects incomplete computations observed at deletion', async () => {
    await database.pool.query(`UPDATE "gcs-v0.15".computations SET is_completed=false WHERE ctid IN (SELECT ctid FROM "gcs-v0.15".computations WHERE host_chain_id=67890 LIMIT 1)`);
    await database.pool.query(`DELETE FROM "gcs-v0.15".computations`);
    expect(await read()).to.eq(1);
  });
  it('rejects errors and an entirely missing marked transaction', async () => {
    await database.pool.query(`UPDATE "gcs-v0.15".computations SET is_error=true WHERE host_chain_id=67890;
      UPDATE upgrade_state SET synthetic_txn_hashes=synthetic_txn_hashes || decode(repeat('ef',32),'hex') WHERE host_chain_id=12345;
      DELETE FROM "gcs-v0.15".computations;`);
    expect(await read()).to.eq(0);
  });
  it('does not turn an earlier single-row deletion into complete work', async () => {
    await database.pool.query(`DELETE FROM "gcs-v0.15".computations WHERE ctid IN (SELECT ctid FROM "gcs-v0.15".computations WHERE host_chain_id=67890 LIMIT 1);
      DELETE FROM "gcs-v0.15".computations;`);
    expect(await read()).to.eq(1);
  });
  it('requires a recorded marker and ignores unrelated application deletions', async () => {
    await database.pool.query(`INSERT INTO "gcs-v0.15".computations VALUES (decode(repeat('ee',32),'hex'),12345,true,false);
      DELETE FROM "gcs-v0.15".computations WHERE transaction_id=decode(repeat('ee',32),'hex');`);
    expect(await read()).to.eq(0);
    await database.pool.query(`UPDATE upgrade_state SET synthetic_txn_hashes=''::bytea WHERE host_chain_id=12345`);
    await database.pool.query(`DELETE FROM "gcs-v0.15".computations`);
    expect(await read()).to.eq(1);
  });
  it('rolls observations back with cutover and cleans up before schema removal too', async () => {
    await database.pool.query(`BEGIN; DELETE FROM "gcs-v0.15".computations; ROLLBACK;`);
    expect(await read()).to.eq(0);
    await database.pool.query(SYNTHETIC_EVIDENCE_CLEANUP_SQL);
    await database.pool.query(`DELETE FROM "gcs-v0.15".computations`);
    expect((await database.pool.query(`SELECT count(*) FROM "gcs-v0.15".computations`)).rows[0].count).to.eq('0');
  });
});

describe('synthetic evidence query contract', () => {
  it('refuses unsafe or incomplete version/chain identities', () => {
    for (const [version, chains] of [['v0.15"', ['1']], ['v0.15', []], ['v0.15', ['1', '1']], ['v0.15', ['1;DROP']]] as const) {
      expect(() => syntheticTrackReadinessSql(version, [...chains])).to.throw('invalid');
    }
  });
});

(url ? describe : describe.skip)('dry-run transition evidence (isolated PostgreSQL)', () => {
  let database: Awaited<ReturnType<typeof createDisposableOracleDatabase>>;
  beforeEach(async () => {
    database = await createDisposableOracleDatabase(url!);
    await database.pool.query(`CREATE TABLE upgrade_state (
      stack_role text, host_chain_id bigint, version text, proposal_id bytea,
      proposal_block bigint, state text);
      INSERT INTO upgrade_state SELECT 'GCS', chain, 'v0.15',
        decode(lpad('2',64,'0'),'hex'), 100, 'UpgradeActivated'
        FROM unnest(ARRAY[12345,67890]) chain;`);
    await database.pool.query(DRY_RUN_EVIDENCE_INSTALL_SQL);
  });
  afterEach(async () => { await database?.close(); });
  const read = async () => (await database.pool.query(dryRunEvidenceReadinessSql('v0.15', ['12345', '67890'], 2))).rows[0].case;
  it('observes both committed transitions even when promotion happens between polls', async () => {
    expect(await read()).to.eq('waiting');
    await database.pool.query(`UPDATE upgrade_state SET state='DryRunStarted';
      UPDATE upgrade_state SET state='LIVE';`);
    expect(await read()).to.eq('ready');
    await database.pool.query(DRY_RUN_EVIDENCE_CLEANUP_SQL);
    expect((await database.pool.query(`SELECT to_regclass('consensus_test_dry_run_evidence') AS audit,
      to_regprocedure('consensus_test_capture_dry_run()') AS fn`)).rows[0]).to.deep.eq({ audit: null, fn: null });
    await database.pool.query(`UPDATE upgrade_state SET state='DryRunStarted'`);
  });
  it('does not infer the transition from LIVE or accept a missing host', async () => {
    await database.pool.query(`UPDATE upgrade_state SET state='LIVE';`);
    expect(await read()).to.eq('waiting');
    await database.pool.query(`UPDATE upgrade_state SET state='DryRunStarted' WHERE host_chain_id=12345`);
    expect(await read()).to.eq('waiting');
  });
  it('rejects old proposals, mismatched blocks, and rolled-back observations', async () => {
    await database.pool.query(`BEGIN; UPDATE upgrade_state SET state='DryRunStarted'; ROLLBACK;`);
    expect(await read()).to.eq('waiting');
    await database.pool.query(`UPDATE upgrade_state SET proposal_id=decode(lpad('1',64,'0'),'hex'),state='DryRunStarted';`);
    expect(await read()).to.eq('waiting');
    await database.pool.query(`UPDATE upgrade_state SET proposal_id=decode(lpad('2',64,'0'),'hex'),state='UpgradeActivated';
      UPDATE upgrade_state SET proposal_block=101 WHERE host_chain_id=67890;
      UPDATE upgrade_state SET state='DryRunStarted';`);
    expect(await read()).to.eq('waiting');
  });
  it('does not reuse committed evidence after a proposal block changes', async () => {
    await database.pool.query(`UPDATE upgrade_state SET state='DryRunStarted';`);
    expect(await read()).to.eq('ready');
    await database.pool.query(`UPDATE upgrade_state SET state='UpgradeActivated',proposal_block=101;`);
    expect(await read()).to.eq('waiting');
  });
});

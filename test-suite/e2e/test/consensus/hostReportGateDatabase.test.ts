import { expect } from 'chai';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { createDisposableOracleDatabase } from './disposableOracleDatabase';

const admin = process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL;
(admin ? describe : describe.skip)('host report gate (isolated PostgreSQL)', function () {
  it('withholds only the selected active proposal window and expires without releasing false evidence', async function () {
    const database = await createDisposableOracleDatabase(admin!);
    const pool = database.pool;
    // Execute the exact query compiled into the opt-in Rust uploader.
    const sql = readFileSync(path.resolve(__dirname, '../../../../coprocessor/fhevm-engine/consensus-detector/src/test_host_report.sql'), 'utf8');
    try {
      await pool.query(`
        CREATE TABLE upgrade_state(stack_role text,state text,version text,proposal_id bytea,host_chain_id bigint,start_block bigint,end_block bigint);
        CREATE TABLE consensus_test_host_report_fault(chain_id bigint PRIMARY KEY,version text,proposal_id bytea,expires_at timestamptz,observed_at timestamptz,observed_block bigint,mode text NOT NULL,reports jsonb NOT NULL DEFAULT '{}');
        INSERT INTO consensus_test_host_report_fault(chain_id,version,proposal_id,expires_at,observed_at,observed_block,mode) VALUES(12345,'v0.15',decode('01','hex'),now()+interval '10 minutes',NULL,NULL,'withhold');
        INSERT INTO upgrade_state VALUES('GCS','DryRunStarted','v0.15',decode('02','hex'),12345,100,200);
      `);
      const held = async (chain = 12345, block = 150) => (await pool.query(sql, [chain, block])).rows[0]?.mode ?? null;
      expect(await held()).to.eq(null); // A fresh/stale proposal cannot borrow a gate.
      await pool.query("UPDATE upgrade_state SET proposal_id=decode('01','hex')");
      expect(await held(67890)).to.eq(null);
      expect(await held(12345, 99)).to.eq(null);
      expect(await held(12345, 201)).to.eq(null);
      expect((await pool.query('SELECT observed_at FROM consensus_test_host_report_fault')).rows[0].observed_at).to.eq(null);
      expect(await held()).to.eq('withhold');
      expect(await held(12345, 151)).to.eq('withhold');
      expect((await pool.query('SELECT observed_block FROM consensus_test_host_report_fault')).rows[0].observed_block).to.eq('150');
      await pool.query("UPDATE consensus_test_host_report_fault SET mode='diverge'");
      expect(await held()).to.eq('diverge');
      const journal = readFileSync(path.resolve(__dirname, '../../../../coprocessor/fhevm-engine/consensus-detector/src/test_host_report_journal.sql'), 'utf8');
      await pool.query(journal, [12345, '150', '42'.repeat(32), '0x' + 'ab'.repeat(32), 'coproc-1']);
      await pool.query(journal, [12345, '151', '43'.repeat(32), '0x' + 'cd'.repeat(32), 'coproc-1']);
      const reports = (await pool.query('SELECT reports FROM consensus_test_host_report_fault')).rows[0].reports;
      expect(Object.keys(reports)).to.deep.eq(['150', '151']);
      expect(reports['150']).to.deep.eq({ chain: '12345', block: '150', original: '42'.repeat(32), blockHash: '0x' + 'ab'.repeat(32), bucket: 'coproc-1' });

      await pool.query("UPDATE upgrade_state SET state='PAUSED'");
      expect(await held()).to.eq(null);
      await pool.query("UPDATE upgrade_state SET state='DryRunStarted'; UPDATE consensus_test_host_report_fault SET expires_at=now()-interval '1 second'");
      expect(await held()).to.eq(null);
      await pool.query('DELETE FROM consensus_test_host_report_fault');
      expect(await held()).to.eq(null);
    } finally { await database.close(); }
  });
});

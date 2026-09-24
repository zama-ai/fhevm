import { expect } from 'chai';
import { Pool } from 'pg';
import { createDisposableOracleDatabase } from './disposableOracleDatabase';
import { keyApplicationGateSql, KEY_GATE_WAITERS, DROP_KEY_GATE } from './keyApplicationGate';
const url = process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL;
(url ? describe : describe.skip)('migration application boundary (isolated PostgreSQL)', function () {
  this.timeout(15_000);
  let database: Awaited<ReturnType<typeof createDisposableOracleDatabase>>;
  const key = 'ab'.repeat(32);
  before(async () => {
    database = await createDisposableOracleDatabase(url!);
    await database.pool.query(`CREATE TABLE keys(key_id bytea PRIMARY KEY,compressed_xof_keyset bytea);
      INSERT INTO keys VALUES (decode('${key}','hex'),NULL),(decode(repeat('cd',32),'hex'),NULL);`);
  });
  after(async () => { await database?.close(); });
  it('holds only the selected real update, rolls it back on owner death and permits normal replay', async () => {
    const connections = new Pool({ connectionString: database.databaseUrl, max: 2, connectionTimeoutMillis: 5_000, statement_timeout: 10_000 });
    const gate = await connections.connect(), worker = await connections.connect();
    let outcome: Promise<unknown> | undefined;
    try {
      await gate.query('SELECT pg_advisory_lock(721029,1)');
      await database.pool.query(keyApplicationGateSql(key));
      await database.pool.query("UPDATE keys SET compressed_xof_keyset='\\x01' WHERE key_id=decode(repeat('cd',32),'hex')");
      const pid = Number((await worker.query('SELECT pg_backend_pid() AS pid')).rows[0].pid);
      worker.on('error', () => undefined);
      outcome = worker.query("UPDATE keys SET compressed_xof_keyset='\\x02' WHERE key_id=$1", [Buffer.from(key, 'hex')]).catch(error => error);
      const deadline = Date.now() + 5_000;
      while (Number((await database.pool.query(KEY_GATE_WAITERS)).rows[0].count) !== 1) {
        if (Date.now() >= deadline) throw new Error('selected transaction never blocked');
        await new Promise(resolve => setTimeout(resolve, 10));
      }
      expect((await database.pool.query('SELECT compressed_xof_keyset FROM keys WHERE key_id=$1', [Buffer.from(key, 'hex')])).rows[0].compressed_xof_keyset).to.eq(null);
      await database.pool.query('SELECT pg_terminate_backend($1)', [pid]);
      expect(await outcome).to.be.instanceOf(Error);
      await gate.query('SELECT pg_advisory_unlock(721029,1)');
      await database.pool.query(DROP_KEY_GATE);
      expect((await database.pool.query('SELECT compressed_xof_keyset FROM keys WHERE key_id=$1', [Buffer.from(key, 'hex')])).rows[0].compressed_xof_keyset).to.eq(null);
      await database.pool.query("UPDATE keys SET compressed_xof_keyset='\\x02' WHERE key_id=$1", [Buffer.from(key, 'hex')]);
      expect((await database.pool.query('SELECT encode(compressed_xof_keyset,\'hex\') AS bytes FROM keys WHERE key_id=$1', [Buffer.from(key, 'hex')])).rows[0].bytes).to.eq('02');
    } finally {
      await gate.query('SELECT pg_advisory_unlock_all()');
      await outcome;
      worker.release(true); gate.release();
      await connections.end();
      await database.pool.query(DROP_KEY_GATE);
    }
  });
});

describe('migration gate identity', () => {
  it('refuses missing, malformed and SQL-shaped key identifiers', () => {
    for (const key of ['', 'ab', "';DROP TABLE keys;--"]) expect(() => keyApplicationGateSql(key)).to.throw('invalid');
  });
});

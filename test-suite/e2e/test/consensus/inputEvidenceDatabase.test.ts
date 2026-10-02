import { expect } from 'chai';
import { createDisposableOracleDatabase } from './disposableOracleDatabase';
import { compareInputRows, inputHandle, readInputRows, type InputIdentity } from './inputEvidence';

const url = process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL;
(url ? describe : describe.skip)('input storage adapter (isolated PostgreSQL)', () => {
  let database: Awaited<ReturnType<typeof createDisposableOracleDatabase>>;
  const identity: InputIdentity = { blobHash: `0x${'11'.repeat(32)}`, chainId: 12345n, acl: `0x${'22'.repeat(20)}`, types: [5, 5], handles: [] };
  identity.handles = identity.types.map((type, index) => inputHandle(identity, index, type));
  before(async () => {
    database = await createDisposableOracleDatabase(url!);
    await database.pool.query(`CREATE TABLE ciphertexts(handle bytea,ciphertext bytea,ciphertext_type smallint,
      ciphertext_version smallint,input_blob_index integer,input_blob_hash bytea,is_input boolean)`);
    for (let index = 0; index < 2; index++) await database.pool.query('INSERT INTO ciphertexts VALUES($1,$2,5,0,$3,$4,true)',
      [Buffer.from(identity.handles[index].slice(2), 'hex'), Buffer.from([1, index]), index, Buffer.from(identity.blobHash.slice(2), 'hex')]);
  });
  after(async () => { await database?.close(); });
  it('reads only the exact blob and detects duplicated canonical input rows', async () => {
    const rows = await readInputRows(database.databaseUrl, identity.blobHash);
    compareInputRows(identity, [rows, structuredClone(rows)]);
    expect(await readInputRows(database.databaseUrl, `0x${'33'.repeat(32)}`)).to.have.length(0);
    await database.pool.query('INSERT INTO ciphertexts SELECT * FROM ciphertexts WHERE input_blob_index=1');
    const duplicated = await readInputRows(database.databaseUrl, identity.blobHash);
    expect(() => compareInputRows(identity, [rows, duplicated])).to.throw('storage-row-uniqueness');
  });
});

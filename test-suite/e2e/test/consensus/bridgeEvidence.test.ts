import { expect } from 'chai';
import { bridgeHandle, compareBridgeRows, readBridgeRows, type BridgeIdentity } from './bridgeEvidence';
import { createDisposableOracleDatabase } from './disposableOracleDatabase';

const hex = (byte: string, length = 32) => `0x${byte.repeat(length)}`;
const healthy = { source: '0x1234', destination: '0x1234', sourceType: 5, destinationType: 5, sourceVersion: 0, destinationVersion: 0 };
describe('bridge association oracle', () => {
  it('rejects empty, missing, duplicated, corrupt and wrong-type association evidence', () => {
    compareBridgeRows(hex('11'), [[healthy], [{ ...healthy }]]);
    for (const rows of [[], [healthy, healthy], [{ ...healthy, destination: '0x1235' }], [{ ...healthy, destinationType: 4 }]]) {
      expect(() => compareBridgeRows(hex('11'), [[healthy], rows])).to.throw('consensus mismatch');
    }
    expect(() => compareBridgeRows(hex('11'), [[healthy], [{ ...healthy, source: '0xab', destination: '0xab' }]])).to.throw('raw-bytes');
  });
  it('binds every bridge identity coordinate and preserves destination metadata', () => {
    const source = `${hex('11').slice(0, -4)}0500`;
    const baseline = bridgeHandle(source, hex('22', 20), 67890n, hex('33'), 100n);
    expect(baseline.slice(44)).to.eq('ff00000000000109320500');
    for (const changed of [
      bridgeHandle(hex('44'), hex('22', 20), 67890n, hex('33'), 100n),
      bridgeHandle(source, hex('44', 20), 67890n, hex('33'), 100n),
      bridgeHandle(source, hex('22', 20), 67891n, hex('33'), 100n),
      bridgeHandle(source, hex('22', 20), 67890n, hex('44'), 100n),
      bridgeHandle(source, hex('22', 20), 67890n, hex('33'), 101n),
    ]) expect(changed).to.not.eq(baseline);
  });
});

const url = process.env.CONSENSUS_ORACLE_TEST_DATABASE_URL;
(url ? describe : describe.skip)('bridge adapter (isolated PostgreSQL)', () => {
  let database: Awaited<ReturnType<typeof createDisposableOracleDatabase>>;
  const identity: BridgeIdentity = { source: hex('11'), destination: hex('22'), sourceChain: 12345, destinationChain: 67890,
    sender: hex('33', 20), receiver: hex('44', 20), guid: hex('55'),
    send: { hash: hex('66'), blockHash: hex('77'), blockNumber: 4 }, receive: { hash: hex('88'), blockHash: hex('99'), blockNumber: 7 } };
  before(async () => {
    database = await createDisposableOracleDatabase(url!);
    await database.pool.query(`CREATE TABLE ciphertexts(handle bytea,ciphertext bytea,ciphertext_type smallint,ciphertext_version smallint);
      CREATE TABLE bridge_handle_events(src_handle bytea, dst_chain_id bigint,src_chain_id bigint,sender_dapp bytea,guid bytea,block_number bigint,block_hash bytea,transaction_id bytea);
      CREATE TABLE handle_bridged_events(src_handle bytea,dst_handle bytea,dst_chain_id bigint,receiver_dapp bytea,guid bytea,block_number bigint,block_hash bytea,transaction_id bytea,is_associated boolean);
      CREATE TABLE host_chain_blocks_valid(chain_id bigint,block_hash bytea,block_status text);
      INSERT INTO ciphertexts VALUES(decode(repeat('11',32),'hex'),'\\x1234',5,0),(decode(repeat('22',32),'hex'),'\\x1234',5,0);
      INSERT INTO bridge_handle_events VALUES(decode(repeat('11',32),'hex'),67890,12345,decode(repeat('33',20),'hex'),decode(repeat('55',32),'hex'),4,decode(repeat('77',32),'hex'),decode(repeat('66',32),'hex'));
      INSERT INTO handle_bridged_events VALUES(decode(repeat('11',32),'hex'),decode(repeat('22',32),'hex'),67890,decode(repeat('44',20),'hex'),decode(repeat('55',32),'hex'),7,decode(repeat('99',32),'hex'),decode(repeat('88',32),'hex'),true);
      INSERT INTO host_chain_blocks_valid VALUES(12345,decode(repeat('77',32),'hex'),'finalized'),(67890,decode(repeat('99',32),'hex'),'finalized');`);
  });
  after(async () => { await database?.close(); });
  it('requires the exact canonical approval, receipt and associated ciphertext', async () => {
    const rows = await readBridgeRows(database.databaseUrl, identity);
    compareBridgeRows(identity.destination, [rows, structuredClone(rows)]);
    for (const changed of [{ ...identity, guid: hex('aa') }, { ...identity, send: { ...identity.send, hash: hex('aa') } }, { ...identity, destinationChain: 12345 }]) {
      expect(await readBridgeRows(database.databaseUrl, changed)).to.have.length(0);
    }
    await database.pool.query("UPDATE handle_bridged_events SET is_associated=false");
    expect(await readBridgeRows(database.databaseUrl, identity)).to.have.length(0);
    await database.pool.query("UPDATE handle_bridged_events SET is_associated=true; UPDATE host_chain_blocks_valid SET block_status='orphaned' WHERE chain_id=12345");
    expect(await readBridgeRows(database.databaseUrl, identity)).to.have.length(0);
  });
});

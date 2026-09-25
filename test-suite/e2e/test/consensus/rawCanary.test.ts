import { expect } from 'chai';
import { keccak256 } from 'ethers';
import { Pool } from 'pg';
import { mkdtempSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { assertRawByteCanaryFiresWith, tamperCanonicalCiphertext } from './canary';
import { recoverAbortedSuite } from './abortRecovery';
import { ComparisonMismatch } from './mismatch';

const handle = `0x${'ab'.repeat(32)}`;
const hash = (bytes: Buffer) => Buffer.from(keccak256(bytes).slice(2), 'hex');
describe('raw-byte canary recovery', () => {
  const connect = Pool.prototype.connect;
  let directory: string;
  let previous: string | undefined;
  const original = Buffer.from('canonical ciphertext');
  let ciphertext: Buffer;
  let digest: Buffer;
  let loseCommit: boolean;
  let published: boolean;
  beforeEach(() => {
    previous = process.env.CONSENSUS_RECOVERY_DIR;
    directory = mkdtempSync(path.join(tmpdir(), 'raw-canary-'));
    process.env.CONSENSUS_RECOVERY_DIR = directory;
    ciphertext = Buffer.from(original); digest = hash(original); loseCommit = false; published = true;
    // Every DB call is intercepted: these are never live-database tests.
    Pool.prototype.connect = (async () => {
      let transaction: { ciphertext: Buffer; digest: Buffer } | undefined;
      return { release() {}, async query(sql: string, args?: unknown[]) {
        if (sql === 'BEGIN') { transaction = { ciphertext: Buffer.from(ciphertext), digest: Buffer.from(digest) }; return {}; }
        if (sql === 'ROLLBACK') { if (transaction) { ciphertext = transaction.ciphertext; digest = transaction.digest; } transaction = undefined; return {}; }
        if (sql === 'COMMIT') { transaction = undefined; if (loseCommit) { loseCommit = false; throw new Error('lost commit reply'); } return {}; }
        if (sql.startsWith('SELECT')) return { rowCount: 1, rows: [{ ciphertext, digest, ciphertext_version: 0, txn_is_sent: published }] };
        if (sql.startsWith('UPDATE')) {
          expect(readdirSync(directory).some(name => name.endsWith('.json')), 'journal precedes either write').to.eq(true);
          expect(transaction, 'both writes must be transactional').not.to.eq(undefined);
          if (sql.startsWith('UPDATE ciphertexts ')) ciphertext = Buffer.from(args![1] as Buffer);
          else digest = Buffer.from(args![1] as Buffer);
          return { rowCount: 1 };
        }
        throw new Error(`unexpected SQL: ${sql}`);
      } };
    }) as never;
  });
  afterEach(() => {
    Pool.prototype.connect = connect;
    rmSync(directory, { recursive: true, force: true });
    if (previous === undefined) delete process.env.CONSENSUS_RECOVERY_DIR;
    else process.env.CONSENSUS_RECOVERY_DIR = previous;
  });
  it('requires cross-operator rejection while the local digest stays valid', async () => {
    await assertRawByteCanaryFiresWith('mock-only', handle, 'raw', async phase => {
      expect(digest.equals(hash(ciphertext))).to.eq(true);
      if (phase === 'poisoned') {
        expect(ciphertext.equals(original)).to.eq(false);
        throw new ComparisonMismatch('raw-bytes', handle, [0, 1], 'fixture cross-operator difference');
      }
      expect(ciphertext.equals(original)).to.eq(true);
    });
    expect(readdirSync(directory)).to.have.length(0);
  });
  it('does not accept a local digest failure or a disabled comparator', async () => {
    for (const kind of ['compute-digest', 'none']) {
      let failure: unknown;
      try {
        await assertRawByteCanaryFiresWith('mock-only', handle, 'raw', async phase => {
          if (phase === 'poisoned' && kind !== 'none') throw new ComparisonMismatch('compute-digest', handle, [1], 'wrong check');
        });
      } catch (error) { failure = error; }
      expect(String(failure)).to.include('not rejected as a cross-operator');
      expect(ciphertext.equals(original)).to.eq(true);
      expect(digest.equals(hash(original))).to.eq(true);
    }
  });
  it('restores both values after a lost mutation commit reply', async () => {
    loseCommit = true;
    let failure: unknown;
    try { await tamperCanonicalCiphertext('mock-only', handle); } catch (error) { failure = error; }
    expect(String(failure)).to.include('lost commit reply');
    expect(ciphertext.equals(original)).to.eq(true);
    expect(digest.equals(hash(original))).to.eq(true);
    expect(readdirSync(directory)).to.have.length(0);
  });
  it('replays the durable raw-byte journal after interrupted comparison', async () => {
    await tamperCanonicalCiphertext('mock-only', handle);
    expect(ciphertext.equals(original)).to.eq(false);
    await recoverAbortedSuite();
    expect(ciphertext.equals(original)).to.eq(true);
    expect(digest.equals(hash(original))).to.eq(true);
    expect(readdirSync(directory)).to.have.length(0);
  });
  it('refuses an unpublished operator without changing either value', async () => {
    published = false;
    let failure: unknown;
    try { await tamperCanonicalCiphertext('mock-only', handle, 0); } catch (error) { failure = error; }
    expect(String(failure)).to.include('has not published');
    expect(ciphertext.equals(original)).to.eq(true);
    expect(readdirSync(directory)).to.have.length(0);
  });
});

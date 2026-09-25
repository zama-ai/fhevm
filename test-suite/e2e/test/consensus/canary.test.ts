import { expect } from 'chai';
import { Pool } from 'pg';
import { mkdtempSync, rmSync, readdirSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

import { tamperDigest, tamperUnsubmittedDigest } from './canary';

const handle = `0x${'ab'.repeat(32)}`;

describe('canary publication safety', () => {
  const connect = Pool.prototype.connect;
  let recoveryDir: string;
  let previousDir: string | undefined;
  beforeEach(() => {
    previousDir = process.env.CONSENSUS_RECOVERY_DIR;
    recoveryDir = mkdtempSync(path.join(tmpdir(), 'canary-journal-'));
    process.env.CONSENSUS_RECOVERY_DIR = recoveryDir;
  });
  afterEach(() => {
    Pool.prototype.connect = connect;
    rmSync(recoveryDir, { recursive: true, force: true });
    if (previousDir === undefined) delete process.env.CONSENSUS_RECOVERY_DIR;
    else process.env.CONSENSUS_RECOVERY_DIR = previousDir;
  });

  function mockPublication(publications: boolean[], failUpdate = false, failCommit = false, expectedPublished = true) {
    const original = Buffer.alloc(32, 1);
    const calls: string[] = [];
    let selected = false;
    let locked = false;
    let released = false;
    let writes = 0;
    Pool.prototype.connect = (async () => ({
      async query(sql: string, params?: unknown[]) {
        calls.push(sql);
        if (sql === 'BEGIN') { locked = true; return {}; }
        if (sql === 'COMMIT' || sql === 'ROLLBACK') {
          locked = false;
          if (sql === 'COMMIT' && failCommit) throw new Error('lost commit reply');
          return {};
        }
        if (sql.startsWith('SELECT')) {
          expect(sql).to.include('FOR UPDATE');
          selected = publications.length > 1 ? publications.shift()! : publications[0];
          return { rowCount: 1, rows: [{ ciphertext: original, txn_is_sent: selected }] };
        }
        if (sql.startsWith('UPDATE')) {
          if ((params![1] as Buffer).equals(original)) {
            expect(released, 'uncertain commit must be repaired on a fresh connection').to.eq(true);
            return { rowCount: 1 };
          }
          expect(locked, 'publication check and write must share the lock').to.eq(true);
          expect(selected, 'mutation must enforce its publication stage').to.eq(expectedPublished);
          const records = readdirSync(recoveryDir).filter(name => name.endsWith('.json'));
          expect(records, 'original must be durable before the mutation').to.have.length(1);
          expect(JSON.parse(readFileSync(path.join(recoveryDir, records[0]), 'utf8')).original).to.eq(original.toString('hex'));
          writes += 1;
          if (failUpdate) throw new Error('injected write failure');
          const poison = params![1] as Buffer;
          expect(poison.equals(original)).to.eq(false);
          return { rowCount: 1 };
        }
        throw new Error(`unexpected query: ${sql}`);
      },
      release() { expect(locked).to.eq(false); released = true; },
    })) as never;
    return { original, calls, writes: () => writes, released: () => released };
  }

  it('waits for the selected operator to publish, releasing the lock between polls', async () => {
    const fixture = mockPublication([false, true]);
    const original = await tamperDigest('mock-only', handle, { timeoutMs: 1000, pollIntervalMs: 0 });
    expect(original.equals(fixture.original)).to.eq(true);
    expect(fixture.calls.indexOf('ROLLBACK')).to.be.lessThan(fixture.calls.findIndex((sql) => sql.startsWith('UPDATE')));
    expect(fixture.writes()).to.eq(1);
    expect(fixture.released()).to.eq(true);
  });

  it('refuses to poison an unpublished victim even when other operators could have quorum', async () => {
    const fixture = mockPublication([false]);
    let error: unknown;
    try { await tamperDigest('mock-only', handle, { timeoutMs: 0 }); } catch (caught) { error = caught; }
    expect(String(error)).to.include('refusing to expose canary poison');
    expect(fixture.writes()).to.eq(0);
    expect(fixture.released()).to.eq(true);
  });

  it('rolls back and releases the lock on a failed poison write', async () => {
    const fixture = mockPublication([true], true);
    let error: unknown;
    try { await tamperDigest('mock-only', handle); } catch (caught) { error = caught; }
    expect(String(error)).to.include('injected write failure');
    expect(fixture.calls).to.include('ROLLBACK');
    expect(fixture.calls.at(-1)).to.match(/^SELECT/);
    expect(fixture.released()).to.eq(true);
  });

  it('restores with a fresh connection when the COMMIT result is uncertain', async () => {
    const fixture = mockPublication([true], false, true);
    let error: unknown;
    try { await tamperDigest('mock-only', handle); } catch (caught) { error = caught; }
    expect(String(error)).to.include('lost commit reply');
    expect(fixture.calls.filter(sql => sql.startsWith('UPDATE'))).to.have.length(2);
    expect(fixture.calls.at(-1)).to.match(/^SELECT/);
  });

  it('journals a detector original under its row lock before poisoning the pending submission', async () => {
    const fixture = mockPublication([false], false, false, false);
    expect((await tamperUnsubmittedDigest('mock-only', handle)).equals(fixture.original)).to.eq(true);
    expect(fixture.writes()).to.eq(1);
    expect(readdirSync(recoveryDir).filter(name => name.endsWith('.json'))).to.have.length(1);
  });

  it('refuses a detector mutation when the sender already published', async () => {
    const fixture = mockPublication([true]);
    let caught: unknown;
    try { await tamperUnsubmittedDigest('mock-only', handle); } catch (error) { caught = error; }
    expect(String(caught)).to.include('detector fault must precede submission');
    expect(fixture.writes()).to.eq(0);
    expect(readdirSync(recoveryDir)).to.have.length(0);
  });

});

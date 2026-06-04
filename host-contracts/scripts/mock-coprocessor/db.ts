/**
 * SQLite wrapper for the mock coprocessor.
 *
 *   `ciphertexts(handle, clear_text)`
 *     The mock's whole purpose: maps each FHE handle (globally unique, see
 *     RFC 008 §Handle uniqueness) to its decoded plaintext as a decimal string.
 *     INSERT OR IGNORE so re-processing the same event is a no-op; INSERT OR
 *     REPLACE for non-deterministic ops (FheRand/FheRandBounded) where each
 *     event is the single source of truth for a fresh random.
 *
 * sqlite3 is async — we wrap every call in a Promise. A single connection is
 * shared across all chain workers; sqlite serialises writes internally and
 * concurrent reads are fine.
 *
 * The daemon is live-event-only: it does NOT persist a per-chain block cursor
 * and does NOT catch up missed events on restart. Each daemon start scans
 * from the current chain head. The ciphertext table IS persisted across runs
 * so `pnpm mock:query` (a separate process) can read it — wipe with
 * `pnpm mock:reset` between sessions if accumulated entries get in the way.
 */
import sqlite3 from 'sqlite3';

export class MockDb {
  private constructor(private readonly db: sqlite3.Database) {}

  static async open(path: string): Promise {
    const db = await new Promise<sqlite3.Database>((resolve, reject) => {
      const conn = new sqlite3.Database(path, (err) => (err ? reject(err) : resolve(conn)));
    });
    const wrapper = new MockDb(db);
    await wrapper.init();
    return wrapper;
  }

  private async init(): Promise {
    await this.run(
      `CREATE TABLE IF NOT EXISTS ciphertexts (
         handle      TEXT PRIMARY KEY,
         clear_text  TEXT NOT NULL,
         updated_at  INTEGER NOT NULL DEFAULT (strftime('%s','now'))
       )`
    );
  }

  async insertCiphertext(handle: string, clearText: bigint | string, replace = false): Promise {
    const sql = replace
      ? 'INSERT OR REPLACE INTO ciphertexts (handle, clear_text, updated_at) VALUES (?, ?, strftime("%s","now"))'
      : 'INSERT OR IGNORE INTO ciphertexts  (handle, clear_text, updated_at) VALUES (?, ?, strftime("%s","now"))';
    await this.run(sql, [handle.toLowerCase(), clearText.toString()]);
  }

  async getClearText(handle: string): Promise {
    const row = await this.get<{ clear_text: string }>('SELECT clear_text FROM ciphertexts WHERE handle = ?', [
      handle.toLowerCase(),
    ]);
    return row ? BigInt(row.clear_text) : null;
  }

  async close(): Promise {
    await new Promise<void>((resolve, reject) => {
      this.db.close((err) => (err ? reject(err) : resolve()));
    });
  }

  // ---- internal sqlite3 promise wrappers ----

  private run(sql: string, params: unknown[] = []): Promise {
    return new Promise((resolve, reject) => {
      this.db.run(sql, params, (err) => (err ? reject(err) : resolve()));
    });
  }

  private get<T>(sql: string, params: unknown[] = []): Promise {
    return new Promise((resolve, reject) => {
      this.db.get(sql, params, (err, row) => (err ? reject(err) : resolve(row as T | undefined)));
    });
  }
}

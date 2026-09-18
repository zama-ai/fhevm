import { randomUUID } from 'node:crypto';
import { Pool, type PoolConfig } from 'pg';

/** The supplied URL is administrative only; no fixture table DDL reaches it. */
export async function createDisposableOracleDatabase(
  adminUrl: string,
  factory: (config: PoolConfig) => Pool = config => new Pool(config),
): Promise<{ pool: Pool; databaseUrl: string; close(): Promise<void> }> {
  const name = `consensus_oracle_${randomUUID().replaceAll('-', '')}`;
  const admin = factory({ connectionString: adminUrl, max: 1, connectionTimeoutMillis: 10_000 });
  let child: Pool | undefined;
  let owned = false;
  let closed = false;
  const close = async () => {
    if (closed) return;
    closed = true;
    try {
      try { if (child) await child.end(); }
      finally { if (owned) await admin.query(`DROP DATABASE "${name}" WITH (FORCE)`); }
    } finally { await admin.end(); }
  };
  try {
    await admin.query(`CREATE DATABASE "${name}"`);
    owned = true;
    const url = new URL(adminUrl);
    url.pathname = `/${name}`;
    // These query options can override the URL path in PostgreSQL parsers.
    url.searchParams.delete('database');
    url.searchParams.delete('dbname');
    child = factory({ connectionString: url.toString(), max: 1, connectionTimeoutMillis: 10_000 });
    const actual = await child.query<{ name: string }>('SELECT current_database() AS name');
    if (actual.rows[0]?.name !== name) throw new Error('disposable oracle connection reached a different database');
    return { pool: child, databaseUrl: url.toString(), close };
  } catch (error) {
    try { await close(); }
    catch (cleanup) { throw new AggregateError([error, cleanup], 'disposable database setup and cleanup failed'); }
    throw error;
  }
}

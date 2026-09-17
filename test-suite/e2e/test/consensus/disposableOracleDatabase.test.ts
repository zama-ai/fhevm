import { expect } from 'chai';
import type { Pool, PoolConfig } from 'pg';
import { createDisposableOracleDatabase } from './disposableOracleDatabase';

function fixture(fail: 'create' | 'connection' | 'setup' | 'close' | undefined) {
  const calls: string[] = [];
  let created = '';
  let count = 0;
  const factory = (config: PoolConfig) => {
    const admin = count++ === 0;
    return {
      query: async (sql: string) => {
        calls.push(`${admin ? 'admin' : 'child'}:${sql}`);
        if (admin) {
          if (sql.startsWith('CREATE DATABASE')) {
            if (fail === 'create') throw new Error('cannot create');
            created = sql.match(/"([^"]+)"/)![1];
          }
          expect(sql).to.match(/^(CREATE|DROP) DATABASE "consensus_oracle_[0-9a-f]+"/);
          return { rows: [] };
        }
        expect(new URL(config.connectionString!).pathname).to.eq(`/${created}`);
        expect(new URL(config.connectionString!).searchParams.has('database')).to.eq(false);
        if (sql.startsWith('SELECT current_database')) return { rows: [{ name: fail === 'connection' ? 'postgres' : created }] };
        if (fail === 'setup') throw new Error('fixture setup failed');
        return { rows: [] };
      },
      end: async () => { calls.push(`${admin ? 'admin' : 'child'}:end`); if (!admin && fail === 'close') throw new Error('child close failed'); },
    } as unknown as Pool;
  };
  return { calls, factory };
}

describe('Disposable SQL fixture ownership', () => {
  it('never drops anything when CREATE DATABASE fails', async () => {
    const { calls, factory } = fixture('create');
    let error: unknown;
    try { await createDisposableOracleDatabase('postgres://localhost/postgres', factory); } catch (caught) { error = caught; }
    expect(String(error)).to.include('cannot create');
    expect(calls.some(call => call.includes('DROP'))).to.eq(false);
    expect(calls.at(-1)).to.eq('admin:end');
  });
  it('cleans only its created child after fixture setup failure, closing both pools', async () => {
    const { calls, factory } = fixture('setup');
    const owned = await createDisposableOracleDatabase('postgres://localhost/postgres?database=postgres', factory);
    try { await owned.pool.query('CREATE TABLE verify_proofs (id int)'); } catch { /* simulate a failing before hook */ }
    await owned.close();
    await owned.close();
    expect(calls.filter(call => call.startsWith('admin:DROP DATABASE'))).to.have.length(1);
    expect(calls.some(call => call.startsWith('admin:') && call.includes('TABLE'))).to.eq(false);
    expect(calls.filter(call => call.endsWith(':end'))).to.deep.eq(['child:end', 'admin:end']);
  });
  it('rejects a misdirected connection before permitting any fixture DDL', async () => {
    const { calls, factory } = fixture('connection');
    let error: unknown;
    try { await createDisposableOracleDatabase('postgres://localhost/postgres', factory); } catch (caught) { error = caught; }
    expect(String(error)).to.include('different database');
    expect(calls.some(call => call.includes('TABLE'))).to.eq(false);
    expect(calls.filter(call => call.startsWith('admin:DROP DATABASE'))).to.have.length(1);
    expect(calls.at(-1)).to.eq('admin:end');
  });
  it('still drops the owned child and closes the admin pool if child pool shutdown fails', async () => {
    const { calls, factory } = fixture('close');
    const owned = await createDisposableOracleDatabase('postgres://localhost/postgres', factory);
    try { await owned.close(); } catch { /* shutdown error remains visible */ }
    expect(calls.filter(call => call.startsWith('admin:DROP DATABASE'))).to.have.length(1);
    expect(calls.at(-1)).to.eq('admin:end');
  });
});

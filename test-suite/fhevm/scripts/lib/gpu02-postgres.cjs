#!/usr/bin/env node
// The migration client can be gone while its PostgreSQL backend still runs.
// Recover the target from the retained container; never print its credentials.
const fs = require('node:fs');
const path = require('node:path');
const {spawnSync} = require('node:child_process');

function terminateMigrationBackends(docker, name, runtime) {
  if (!/^gpu02-revert-[0-9]+-[0-9]+$/.test(name)) throw new Error('invalid migration identity');
  const call = (args, milliseconds = 15000) => spawnSync(docker, args, {
    encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], timeout: milliseconds, killSignal: 'SIGKILL',
  });
  const inspected = call(['inspect', name]);
  if (inspected.status !== 0) throw new Error('cannot inspect retained migration database identity');
  let records;
  try { records = JSON.parse(inspected.stdout); } catch { throw new Error('invalid retained migration metadata'); }
  const container = records[0];
  if (records.length !== 1 || container?.Config?.Labels?.['fhevm.gpu02-owner'] !== runtime) {
    throw new Error('migration recovery owner does not match');
  }
  const env = Object.fromEntries((container.Config.Env || []).map(value => {
    const equal = value.indexOf('='); return [value.slice(0, equal), value.slice(equal + 1)];
  }));
  let connection;
  try { connection = new URL(env.DATABASE_URL); } catch { throw new Error('invalid retained migration database identity'); }
  if (!['postgres:', 'postgresql:'].includes(connection.protocol) || env.PGAPPNAME !== name || connection.searchParams.get('application_name') !== name) {
    throw new Error('migration has no durable attributed PostgreSQL application name');
  }
  const network = container.HostConfig?.NetworkMode;
  if (!network || !container.Image) throw new Error('missing retained migration network/image identity');
  // The cleanup client must never match the migration's application_name.
  connection.searchParams.set('application_name', `${name}-cleanup`);
  const directory = fs.mkdtempSync(path.join(runtime, 'gpu02-pg-'));
  const envFile = path.join(directory, 'client.env');
  fs.writeFileSync(envFile, `DATABASE_URL=${connection.toString()}\nPGCONNECT_TIMEOUT=5\nPGOPTIONS=-c statement_timeout=5000 -c lock_timeout=5000\n`, {mode: 0o600});
  const query = (sql) => {
    const result = call(['run', '--rm', '--network', network, '--env-file', envFile,
      '--entrypoint', '/bin/sh', container.Image, '-c',
      'exec psql "$DATABASE_URL" -X -A -t -v ON_ERROR_STOP=1 -c "$1"', '--', sql]);
    if (result.status !== 0) throw new Error('attributed PostgreSQL cancellation query failed');
    return result.stdout.trim();
  };
  try {
    const predicate = `application_name = '${name}' AND pid <> pg_backend_pid()`;
    if (query(`SELECT COALESCE(bool_and(pg_terminate_backend(pid)), true) FROM pg_stat_activity WHERE ${predicate}`) !== 't') {
      throw new Error('PostgreSQL refused migration backend termination');
    }
    // Acknowledgement only means the signal was sent. Independently observe
    // absence before either the CLI finally or the parent may restore writers.
    const deadline = Date.now() + 10000;
    do {
      const count = query(`SELECT count(*) FROM pg_stat_activity WHERE ${predicate}`);
      if (count === '0') return;
      if (!/^[0-9]+$/.test(count)) throw new Error('invalid migration backend observation');
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 50);
    } while (Date.now() < deadline);
    throw new Error('migration PostgreSQL backend remains active');
  } finally { fs.rmSync(directory, {recursive: true, force: true}); }
}
module.exports = {terminateMigrationBackends};
if (require.main === module) {
  try { terminateMigrationBackends(process.env.GPU02_REAL_DOCKER, process.argv[2], process.env.SP_RUNTIME_DIR); }
  catch (error) { console.error(`GPU02 PostgreSQL recovery failed: ${error.message}`); process.exitCode = 1; }
}

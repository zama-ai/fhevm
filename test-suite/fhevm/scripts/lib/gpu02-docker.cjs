#!/usr/bin/env node
// Scoped GPU02 transport. Remote work belongs to the parent recovery owner,
// even when Docker's attached client disappears or the CLI is killed.
const fs = require('node:fs');
const {spawnSync} = require('node:child_process');
const {terminateMigrationBackends} = require('./gpu02-postgres.cjs');
const argv = process.argv.slice(2);
const env = process.env;
const docker = env.GPU02_REAL_DOCKER;
if (!docker) throw new Error('missing GPU02 Docker transport identity');
const call = (args, capture = false, seconds = 120) => spawnSync(docker, args, {
  stdio: capture ? ['ignore', 'pipe', 'pipe'] : 'inherit', encoding: 'utf8',
  timeout: seconds * 1000, killSignal: 'SIGKILL',
});
const remaining = () => Math.max(1, Number(env.CASE_DEADLINE_EPOCH) - Math.floor(Date.now()/1000));
const phase = fs.readFileSync(env.GPU02_PHASE_HELPER, 'utf8');
const fenced = () => fs.existsSync(`${env.SP_RUNTIME_DIR}/cancelling`);
const unsafe = (message) => {
  try {
    fs.mkdirSync(require('node:path').dirname(env.SP_CONTAMINATION), {recursive: true});
    fs.writeFileSync(env.SP_CONTAMINATION, `phase_registry=${env.SP_PHASE_REGISTRY}\nrestore_log=${env.SC_RESTORE_LOG || ''}\ngpu02_remote_unverified=true\n`);
  } catch (error) { console.error('Cannot persist GPU02 recovery fence:', error.message); }
  console.error(`GPU02 remote cancellation unverified: ${message}; retaining ownership`);
  // Do not return into withQuiescedWriters.finally while remote SQL may run.
  try { process.kill(process.ppid, 'SIGKILL'); } catch {}
  process.exit(125);
};
const failIfClosed = () => { if (fenced()) process.exit(143); };
const stoppedMigration = (name) => {
  const state = call(['inspect', '-f', '{{.State.Status}} {{.State.Running}} {{.State.Pid}}', name], true, 10);
  return state.status === 0 && /^(exited|dead) false 0$/.test(state.stdout.trim());
};
const stopMigration = (name) => {
  const owner = call(['inspect', '-f', '{{index .Config.Labels \"fhevm.gpu02-owner\"}}', name], true, 10);
  if (owner.status !== 0 || owner.stdout.trim() !== env.SP_RUNTIME_DIR) return false;
  if (stoppedMigration(name)) return true;
  call(['stop', '-t', '2', name], true, 15);
  return stoppedMigration(name);
};
if (argv[0] === 'run' && argv.includes('/revert_coprocessor_db_state.sh')) {
  failIfClosed();
  const name = `gpu02-revert-${process.pid}-${Date.now()}`;
  // Keep the name even if create's response is lost. Absence is then ambiguous,
  // and parent cleanup must retain its fence rather than infer no work exists.
  fs.appendFileSync(env.GPU02_MIGRATION_REGISTRY, `${name}\n`);
  failIfClosed();
  // Persist attribution in the container before a start request can reach the
  // daemon. URL application_name wins over libpq environment defaults.
  const createArgs = argv.slice(1).filter(arg => arg !== '--rm');
  const connectionAt = createArgs.findIndex(arg => arg.startsWith('DATABASE_URL='));
  if (connectionAt < 0) unsafe('migration database identity is missing');
  let connection;
  try { connection = new URL(createArgs[connectionAt].slice('DATABASE_URL='.length)); }
  catch { unsafe('migration database identity is invalid'); }
  if (!['postgres:', 'postgresql:'].includes(connection.protocol)) unsafe('migration database scheme is invalid');
  connection.searchParams.set('application_name', name);
  createArgs[connectionAt] = `DATABASE_URL=${connection.toString()}`;
  const create = call(['create', '--name', name, '--label', `fhevm.gpu02-owner=${env.SP_RUNTIME_DIR}`,
    '-e', `PGAPPNAME=${name}`, ...createArgs], false, remaining());
  if (create.status !== 0) unsafe('migration creation was not acknowledged');
  failIfClosed();
  fs.writeFileSync(`${env.GPU02_MIGRATION_REGISTRY}.${name}.start-requested`, 'requested');
  failIfClosed();
  const attached = call(['start', '-a', name], false, remaining());
  if (!stopMigration(name)) unsafe('migration is still running or cannot be inspected');
  try { terminateMigrationBackends(docker, name, env.SP_RUNTIME_DIR); }
  catch { unsafe('migration PostgreSQL backend quiescence could not be proven'); }
  const exit = call(['inspect', '-f', '{{.State.ExitCode}}', name], true, 10);
  if (exit.status !== 0 || !/^\d+\s*$/.test(exit.stdout)) unsafe('migration exit cannot be inspected');
  process.exit(attached.status === 0 ? Number(exit.stdout.trim()) : (attached.status || 1));
}
if (argv[0] === 'exec' && argv.includes('./run-tests.sh')) {
  failIfClosed();
  const commandAt = argv.indexOf('./run-tests.sh');
  const container = argv[commandAt - 1];
  if (!/^[a-zA-Z0-9_.-]+$/.test(container)) throw new Error('invalid GPU02 test container');
  const token = `gpu02_${process.pid}_${Date.now()}`;
  fs.appendFileSync(env.SP_PHASE_REGISTRY, `${container}|${token}\n`);
  failIfClosed();
  const result = call([...argv.slice(0, commandAt), 'node', '-e', phase, 'run', token,
    String(Number(env.CASE_DEADLINE_EPOCH) * 1000), ...argv.slice(commandAt)], false, remaining());
  // Also covers an attached-client failure while its remote phase remains live.
  const cancelled = call(['exec', container, 'node', '-e', phase, 'cancel', token], true, 20);
  if (cancelled.status !== 0) unsafe('E2E phase did not acknowledge cancellation');
  process.exit(result.status ?? 1);
}
const result = call(argv, false, remaining());
process.exit(result.status ?? 1);

import { expect } from 'chai';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

const cwd = path.join(__dirname, '../..');
const original = '01'.repeat(32);
const handle = `0x${'ab'.repeat(32)}`;
// Storage survives the child process, like the database and Anvil do. The
// transport is fake; ordering, durable journal and SIGKILL are real.
const transport = `
const assert=require('node:assert/strict');
const fs=require('node:fs');
const state=()=>JSON.parse(fs.readFileSync(process.env.STATE_FILE,'utf8'));
const put=value=>fs.writeFileSync(process.env.STATE_FILE,JSON.stringify(value));
const {Pool}=require('pg');
Pool.prototype.connect=async function(){
 const url=this.options.connectionString;
 if(process.env.FAIL_RESTORE===url) throw new Error('injected connection failure');
 return {release(){},async query(sql,params){
  if(sql==='BEGIN'||sql==='COMMIT'||sql==='ROLLBACK') return {};
  const current=state();
  if(sql.startsWith('UPDATE')) {
   assert(fs.readdirSync(process.env.CONSENSUS_RECOVERY_DIR).some(n=>n.endsWith('.json')),'journal must precede any mutation');
   if(process.env.IGNORE_RESTORE!==url) {current[url]=params[1].toString('hex');put(current);}
   return {rowCount:1};
  }
  if(sql.startsWith('SELECT')) return {rowCount:1,rows:[{ciphertext:Buffer.from(current[url],'hex'),txn_is_sent:true}]};
  throw new Error('unexpected SQL');
 }};
};
const {JsonRpcProvider}=require('ethers');
JsonRpcProvider.prototype.send=async function(method,params){
 const current=state();
 if(method==='anvil_getAutomine') return current.automine;
 if(method==='anvil_getIntervalMining') return current.interval===0?null:current.interval;
 assert(fs.readdirSync(process.env.CONSENSUS_RECOVERY_DIR).some(n=>n.endsWith('.json')),'journal must precede mining mutation');
 if(method==='evm_setAutomine') current.automine=params[0];
 else if(method==='evm_setIntervalMining') current.interval=params[0];
 else throw new Error('unexpected RPC');
 put(current);return true;
};
`;

describe('durable suite abort recovery', function () {
  this.timeout(15_000);
  let dir: string;
  beforeEach(() => { dir = mkdtempSync(path.join(tmpdir(), 'suite-abort-')); });
  afterEach(() => { rmSync(dir, { recursive: true, force: true }); });
  function run(body: string, extraEnv: Record<string, string> = {}) {
    return spawnSync(process.execPath, ['-r', require.resolve('ts-node/register/transpile-only'), '-e', `${transport}
      (async()=>{${body}})().catch(e=>{console.error(e);process.exitCode=1});`], {
      cwd, encoding: 'utf8', timeout: 10_000,
      env: { ...process.env, CONSENSUS_RECOVERY_DIR: path.join(dir, 'private'), STATE_FILE: path.join(dir, 'state.json'), ...extraEnv },
    });
  }
  function state(): Record<string, unknown> { return JSON.parse(readFileSync(path.join(dir, 'state.json'), 'utf8')); }
  function records(): string[] { return readdirSync(path.join(dir, 'private')).filter(name => name.endsWith('.json')); }

  it('recovers committed canary poison after SIGKILL without the original JS finally', () => {
    writeFileSync(path.join(dir, 'state.json'), JSON.stringify({ victim: original }));
    const killed = run(`await require('./test/consensus/canary').tamperDigest('victim','${handle}'); process.kill(process.pid,'SIGKILL');`);
    expect(killed.signal, killed.stderr).to.eq('SIGKILL');
    expect(state().victim).not.to.eq(original);
    expect(records()).to.have.length(1);
    expect(statSync(path.join(dir, 'private', records()[0])).mode & 0o777).to.eq(0o600);
    expect(statSync(path.join(dir, 'private')).mode & 0o777).to.eq(0o700);
    const restored = run(`await require('./test/consensus/abortRecovery').recoverAbortedSuite();`);
    expect(restored.status, restored.stderr).to.eq(0);
    expect(state().victim).to.eq(original);
    expect(records()).to.have.length(0);
  });

  it('retains failed restoration for retry while restoring every other record', () => {
    writeFileSync(path.join(dir, 'state.json'), JSON.stringify({ victim: original, peer: original }));
    const armed = run(`const {tamperDigest}=require('./test/consensus/canary');
      await tamperDigest('victim','${handle}');await tamperDigest('peer','${handle}');`);
    expect(armed.status, armed.stderr).to.eq(0);
    const failed = run(`await require('./test/consensus/abortRecovery').recoverAbortedSuite();`, { FAIL_RESTORE: 'victim' });
    expect(failed.status).to.eq(1);
    expect(state().peer).to.eq(original);
    expect(state().victim).not.to.eq(original);
    expect(records()).to.have.length(1);
    const retried = run(`await require('./test/consensus/abortRecovery').recoverAbortedSuite();`);
    expect(retried.status, retried.stderr).to.eq(0);
    expect(state().victim).to.eq(original);
    expect(records()).to.have.length(0);
  });

  it('retains the journal if UPDATE reports success without restoring the saved value', () => {
    writeFileSync(path.join(dir, 'state.json'), JSON.stringify({ victim: original }));
    const armed = run(`await require('./test/consensus/canary').tamperDigest('victim','${handle}');`);
    expect(armed.status, armed.stderr).to.eq(0);
    const failed = run(`await require('./test/consensus/abortRecovery').recoverAbortedSuite();`, { IGNORE_RESTORE: 'victim' });
    expect(failed.status).to.eq(1);
    expect(state().victim).not.to.eq(original);
    expect(records()).to.have.length(1);
  });

  it('restores the exact non-default mining pair after SIGKILL and nested ownership', () => {
    writeFileSync(path.join(dir, 'state.json'), JSON.stringify({ automine: true, interval: 7 }));
    const killed = run(`const {rememberMiningState}=require('./test/consensus/abortRecovery');
      const rpc=new JsonRpcProvider('http://unused.invalid');
      await rememberMiningState(rpc,'http://unused.invalid');
      await rpc.send('evm_setIntervalMining',[0]);await rpc.send('evm_setAutomine',[false]);
      await rememberMiningState(rpc,'http://unused.invalid');
      process.kill(process.pid,'SIGKILL');`);
    expect(killed.signal, killed.stderr).to.eq('SIGKILL');
    expect(state()).to.deep.eq({ automine: false, interval: 0 });
    const restored = run(`await require('./test/consensus/abortRecovery').recoverAbortedSuite();`);
    expect(restored.status, restored.stderr).to.eq(0);
    expect(state()).to.deep.eq({ automine: true, interval: 7 });
    expect(records()).to.have.length(0);
  });

  it('preserves an initially disabled interval represented by Anvil as null', () => {
    writeFileSync(path.join(dir, 'state.json'), JSON.stringify({ automine: false, interval: 0 }));
    const changed = run(`const {rememberMiningState}=require('./test/consensus/abortRecovery');
      const rpc=new JsonRpcProvider('http://unused.invalid');
      await rememberMiningState(rpc,'http://unused.invalid');
      await rpc.send('evm_setIntervalMining',[2]);`);
    expect(changed.status, changed.stderr).to.eq(0);
    const restored = run(`await require('./test/consensus/abortRecovery').recoverAbortedSuite();`);
    expect(restored.status, restored.stderr).to.eq(0);
    expect(state()).to.deep.eq({ automine: false, interval: 0 });
    expect(records()).to.have.length(0);
  });
});

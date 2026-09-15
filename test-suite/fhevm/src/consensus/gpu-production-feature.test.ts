import { expect, test } from 'bun:test';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';

const scripts = path.resolve(import.meta.dir, '../../scripts');
const source = readFileSync(path.join(scripts, 'gpu-consensus-workers.sh'), 'utf8');

test('GPU builds default to production features and record the explicit fault feature set', () => {
  for (const hooks of ['0', '1']) {
    const dir = mkdtempSync(path.join(tmpdir(), 'gpu-features-'));
    try {
      mkdirSync(path.join(dir, 'bin'));
      for (const binary of ['nvcc', 'tfhe_worker', 'sns_worker', 'zkproof_worker']) {
        writeFileSync(path.join(dir, 'bin', binary), '#!/bin/sh\nexit 0\n', { mode: 0o755 });
      }
      const harness = source.slice(0, source.lastIndexOf('case "${1:-}" in'))
        .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${scripts}'`)
        .replace(/^readonly BIN_DIR=.*$/m, `readonly BIN_DIR='${dir}/bin'`) + `
require(){ :; }; require_three_operator_topology(){ :; }; require_clean_source(){ :; }; validate_tuning(){ :; }
gpu_count(){ echo 1; }; gpu_name(){ echo H100; }; gpu_uuid(){ echo uuid; }
cargo(){ printf '%s\\n' "$@" > '${dir}/cargo-args'; }
build
verify_build_manifest
`;
      const result = Bun.spawnSync(['bash', '-c', harness], { env: { ...process.env,
        FHEVM_STATE_DIR: dir, CUDA_PATH: dir, GPU_CONSENSUS_TEST_FAILPOINTS: hooks, GPU_CONSENSUS_DEVICE: '0' } });
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      expect(readFileSync(path.join(dir, 'cargo-args'), 'utf8').trim().split('\n').at(-1))
        .toBe(hooks === '1' ? 'gpu,tfhe-worker/test-failpoints' : 'gpu');
      const manifest = readFileSync(path.join(dir, 'runtime/gpu-consensus-workers/build-manifest.env'), 'utf8');
      expect(manifest).toContain(hooks === '1' ? 'test_features=tfhe-worker/test-failpoints' : "test_features=''");
      const mismatched = Bun.spawnSync(['bash', '-c', harness.slice(0, harness.lastIndexOf('\nbuild')) + '\nverify_build_manifest'],
        { env: { ...process.env, FHEVM_STATE_DIR: dir, GPU_CONSENSUS_TEST_FAILPOINTS: hooks === '0' ? '1' : '0', GPU_CONSENSUS_DEVICE: '0' } });
      expect(mismatched.exitCode).toBe(1);
      expect(mismatched.stderr.toString()).toContain('feature set differs');
    } finally { rmSync(dir, { recursive: true, force: true }); }
  }
});

test('GPU handover refuses missing compressed keys before stopping any writer', () => {
  const dir = mkdtempSync(path.join(tmpdir(), 'gpu-keys-handover-'));
  try {
    const harness = source.slice(0, source.lastIndexOf('case "${1:-}" in'))
      .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${scripts}'`) + `
require(){ :; }; require_three_operator_topology(){ :; }; require_clean_source(){ :; }; validate_tuning(){ :; }
verify_build_manifest(){ :; }; ensure_no_active_gpu_session(){ :; }
bun(){ [[ "$1" == */gpu-key-readiness.ts ]] && echo checked > '${dir}/checked'; return 1; }
record_running_docker_workers(){ echo mutated > '${dir}/mutated'; }
stop_host_workers(){ echo mutated > '${dir}/mutated'; }
start
`;
    const result = Bun.spawnSync(['bash', '-c', harness], { env: { ...process.env, FHEVM_STATE_DIR: dir } });
    expect(result.exitCode).toBe(1);
    expect(readFileSync(path.join(dir, 'checked'), 'utf8').trim()).toBe('checked');
    expect(() => readFileSync(path.join(dir, 'mutated'))).toThrow();
    expect(result.stderr.toString()).toContain('compressed key material');
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

test('test-env cannot attribute a CPU fleet to a stale GPU manifest', () => {
  const dir = mkdtempSync(path.join(tmpdir(), 'gpu-stale-attribution-'));
  try {
    const runtime = path.join(dir, 'runtime/gpu-consensus-workers');
    mkdirSync(runtime, { recursive: true });
    writeFileSync(path.join(runtime, 'node-config.env'), 'homogeneous=true\n');
    writeFileSync(path.join(runtime, 'build-manifest.env'), 'software_revision=revision\n');
    const harness = source.slice(0, source.lastIndexOf('case "${1:-}" in'))
      .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${scripts}'`) + `
require_clean_source(){ :; }; verify_build_manifest(){ :; }; instance_indexes(){ echo 0; }
docker(){ echo running; }; systemctl(){ echo 0; }; hardware_class(){ echo gpu-H100; }
device_set(){ echo 0; }; binary_sha(){ echo hash; }; scheduling_classes(){ echo recorded; }
test_env
`;
    const result = Bun.spawnSync(['bash', '-c', harness], { env: { ...process.env, FHEVM_STATE_DIR: dir } });
    expect(result.exitCode).toBe(1);
    expect(result.stderr.toString()).toContain('stale session marker');
    expect(result.stdout.toString()).not.toContain('CONSENSUS_BACKEND_CLASS');
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

test('GPU unit journal description excludes signer arguments from the actual invocation', () => {
  const dir = mkdtempSync(path.join(tmpdir(), 'gpu-safe-description-'));
  try {
    writeFileSync(path.join(dir, 'worker.env'), 'DATABASE_URL=postgres://localhost/fixture\nBUCKET_NAME=fixture\nTX_SENDER_PRIVATE_KEY=synthetic-fixture-key\n');
    const harness = source.slice(0, source.lastIndexOf('case "${1:-}" in'))
      .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${scripts}'`) + `
stop_transient_unit(){ :; }
UNIT_DEVICE=0; UNIT_STREAMS=16; UNIT_WORK_ITEMS=100; UNIT_CHAINS=20
UNIT_FHE_THREADS=8; UNIT_TOKIO_THREADS=4; UNIT_ADAPTIVE=; UNIT_BATCH=
systemd-run(){
 local description='' arg signer_present=0
 for arg in "$@"; do
  case "$arg" in --description=*) description="\${arg#--description=}";; --private-key=*) signer_present=1;; esac
 done
 # systemd-run's omitted Description defaults to the full command, which
 # systemd subsequently logs when starting/stopping the transient unit.
 [[ -n "$description" ]] || description="$*"
 printf '%s' "$description" > '${dir}/description'
 [[ "$signer_present" == 1 && "$description" != *synthetic-fixture-key* ]]
}
start_unit sns 2 '${dir}/worker.env'
`;
    const result = Bun.spawnSync(['bash', '-c', harness], { env: { ...process.env, FHEVM_STATE_DIR: dir } });
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(readFileSync(path.join(dir, 'description'), 'utf8')).toBe('FHEVM GPU sns worker operator 2');
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

for (const boundary of ['before', 'after']) {
  test(`GPU handover rejects global owner conflicts ${boundary} mutation and rolls back partial ownership`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), 'gpu-owner-boundary-'));
    try {
      const harness = source.slice(0, source.lastIndexOf('case "${1:-}" in'))
        .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${scripts}'`) + `
require(){ :; }; require_three_operator_topology(){ :; }; require_clean_source(){ :; }; validate_tuning(){ :; }
verify_build_manifest(){ :; }; ensure_no_active_gpu_session(){ :; }
bun(){
 [[ "$1" == */queue-ownership.ts ]] || return 0
 printf 'check %s\\n' "$*" >> '${dir}/trace'
 if [[ '${boundary}' == before || "$*" != *--allow-missing* ]]; then echo 'rogue worker owns this queue' >&2; return 1; fi
}
require_scenario_tuning_carried(){ :; }
record_running_docker_workers(){ echo record >> '${dir}/trace'; touch "$DOCKER_WORKER_STATE_FILE"; }
stop_host_workers(){ echo stop >> '${dir}/trace'; }; write_host_env(){ :; }
instance_indexes(){ echo 0; }; env_file_for(){ echo fixture; }
stop_recorded_docker_workers(){ echo displaced >> '${dir}/trace'; }
resolve_unit_tuning(){ :; }; record_unit_tuning(){ :; }; load_unit_tuning(){ :; }
start_unit(){ echo start >> '${dir}/trace'; }; record_unit_invocation(){ :; }; wait_for_units(){ :; }
write_node_config(){ touch "$NODE_CONFIG"; }
restore_docker_session(){ echo rollback >> '${dir}/trace'; }
start
`;
      const result=Bun.spawnSync(['bash','-c',harness],{env:{...process.env,FHEVM_STATE_DIR:dir}});
      expect(result.exitCode).toBe(1);
      const trace=readFileSync(path.join(dir,'trace'),'utf8');
      expect(trace).toContain('queue-ownership.ts 3 --allow-missing');
      if(boundary==='before') {
        expect(trace).not.toContain('record\n'); expect(trace).not.toContain('start\n');
      } else {
        expect(trace).toContain('start\n'); expect(trace.trim().endsWith('rollback')).toBe(true);
        expect(result.stdout.toString()).not.toContain('all 3 operators run');
      }
    } finally {rmSync(dir,{recursive:true,force:true});}
  });
}

test('GPU handover rejects a blue-green scenario before querying or changing services', () => {
  const dir=mkdtempSync(path.join(tmpdir(),'gpu-blue-green-'));
  try {
    mkdirSync(path.join(dir,'state'));
    writeFileSync(path.join(dir,'state/state.json'),JSON.stringify({scenario:{kind:'blue-green',topology:{count:3,threshold:3}}}));
    const result=Bun.spawnSync([process.execPath,path.join(scripts,'queue-ownership.ts'),'3','--allow-missing','--no-blue-green'],{env:{...process.env,FHEVM_STATE_DIR:dir}});
    expect(result.exitCode).toBe(1);
    expect(result.stderr.toString()).toContain('does not support blue/green');
  } finally {rmSync(dir,{recursive:true,force:true});}
});

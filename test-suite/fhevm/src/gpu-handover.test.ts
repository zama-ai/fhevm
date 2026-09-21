import { expect, test } from "bun:test";
import { existsSync, mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

/** Exercise the actual handover functions with isolated supervisor/Docker state.
 * Source only the definitions to avoid requiring a real systemd user bus. */
const fixture = (check: (invoke: (env?: Record<string,string>) => {code:number; error:string}, record: string, marker: string, calls: string) => void) => {
  const dir = mkdtempSync(path.join(tmpdir(), "gpu-handover-"));
  try {
    const bin = path.join(dir,"bin"); mkdirSync(bin);
    const gpu = path.join(dir,"runtime/gpu-consensus-workers"); mkdirSync(gpu,{recursive:true});
    const env = path.join(dir,"runtime/env"); mkdirSync(env);
    for (const name of ['coprocessor.env','coprocessor.1.env','coprocessor.2.env']) writeFileSync(path.join(env,name),'');
    const record = path.join(gpu,'docker-workers-to-restore'); const marker = path.join(gpu,'node-config.env');
    writeFileSync(record,'coprocessor-tfhe-worker\n'); writeFileSync(marker,'homogeneous=true\n');
    writeFileSync(path.join(dir,'active'),'active'); const calls = path.join(dir,'calls'); writeFileSync(calls,'');
    writeFileSync(path.join(bin,'systemctl'),`#!/bin/sh
case "$*" in
 *' stop '*)
  if [ "$FAIL_STOP" = 1 ]; then exit 1; fi
  if [ "$DELAY_STOP" = 1 ]; then echo deactivating > "$FAKE_ROOT/active"; else echo inactive > "$FAKE_ROOT/active"; fi;;
 *' show '*)
  state=$(/bin/cat "$FAKE_ROOT/active")
  case "$*" in
   *ActiveState*--value*)
    if [ "$state" = deactivating ]; then
     if [ -f "$FAKE_ROOT/stop-polled" ]; then state=inactive; echo inactive > "$FAKE_ROOT/active"; else touch "$FAKE_ROOT/stop-polled"; fi
    fi
    echo "$state"; exit 0;;
   *InvocationID*) if [ -f "$FAKE_ROOT/restarted" ]; then echo replacement; else echo original; fi; exit 0;;
   *ExecStart*) echo '--work-items-batch-size=100 --dependence-chains-per-batch=20'; exit 0;;
   *LoadState*--value*) if [ "$state" = inactive ]; then echo not-found; else echo loaded; fi; exit 0;;
   *MainPID*--value*) if [ "$state" = active ]; then echo 123; else echo 0; fi; exit 0;;
  esac
  case "$*" in *--value*) echo "$state";; *)
   echo LoadState=loaded
   echo ActiveState="$state"
   if [ "$state" = active ]; then echo MainPID=123; else echo MainPID=0; fi;; esac;;
 *) exit 2;;
esac
`,{mode:0o755});
    writeFileSync(path.join(bin,'docker'),`#!/bin/sh
printf '%s\\n' "$1" >> "$FAKE_ROOT/calls"
case "$1" in
 inspect)
  if [ "$GPU03_FAIL_FIRST_INSPECT" = 1 ] && [ ! -f "$FAKE_ROOT/inspect-failed" ]; then
    touch "$FAKE_ROOT/inspect-failed"; echo 'temporary Docker inspect failure' >&2; exit 1
  fi
  if [ "$INSPECT_ERROR" = 1 ]; then echo 'Cannot connect to the Docker daemon' >&2; exit 1; fi
  if [ "$MISSING" = 1 ]; then echo 'No such container: coprocessor-tfhe-worker' >&2; exit 1; fi
  case "$*" in *State.Status*) if [ -f "$FAKE_ROOT/started" ]; then echo running; else echo exited; fi;; *) echo '{}';; esac;;
 start) touch "$FAKE_ROOT/started";;
 *) exit 2;;
esac
`,{mode:0o755});
    writeFileSync(path.join(bin,'systemd-run'),`#!/bin/sh
[ "$(cat "$FAKE_ROOT/active")" != deactivating ] || { echo 'Unit already exists' >&2; exit 1; }
echo active > "$FAKE_ROOT/active"
touch "$FAKE_ROOT/restarted"
`,{mode:0o755});
    const launcher = readFileSync(path.join(import.meta.dir,'../scripts/gpu-consensus-workers.sh'),'utf8');
    const harness = path.join(dir,'handover.sh');
    writeFileSync(harness,launcher.slice(0,launcher.lastIndexOf('case "${1:-}" in')).replace(/^readonly SCRIPT_DIR=.*$/m,
      `readonly SCRIPT_DIR='${path.resolve(import.meta.dir, '../scripts')}'`) + `
# This fixture tests supervisor restoration; shared discovery has independent
# real-CLI fixtures and is an explicit boundary here, not live host inspection.
bun() { [[ "$1" == */queue-ownership.ts ]] && return "\${OWNERSHIP_FAILURE:-0}"; command bun "$@"; }
if [[ "\${GPU03_TEST:-0}" == 1 ]]; then
  # The fixture has one fake unit. The readiness boundary is also fake; the
  # actual stop/restart/EXIT cleanup functions run unchanged.
  binary_sha() { echo "\${FAKE_BINARY_HASH:-recorded}"; }
  wait_for_units() { [[ "$(cat "$FAKE_ROOT/active")" == active ]]; }
fi
"\${HANDOVER_ACTION:-restore_docker_session}" tfhe 0
`);
    const invoke = (extra:Record<string,string>={}) => {
      const result = Bun.spawnSync(['bash',harness],{env:{...process.env,...extra,FAKE_ROOT:dir,FHEVM_STATE_DIR:dir,PATH:`${bin}:${process.env.PATH}`}});
      return {code:result.exitCode,error:result.stderr.toString()};
    };
    check(invoke,record,marker,calls);
  } finally {rmSync(dir,{recursive:true,force:true});}
};

test('failed GPU stop never starts Docker and retains ownership for a successful retry', () => fixture((invoke,record,marker,calls) => {
  expect(invoke({FAIL_STOP:'1'}).code).toBe(1);
  expect(readFileSync(calls,'utf8')).not.toContain('start');
  expect(existsSync(record)).toBe(true); expect(existsSync(marker)).toBe(true);
  expect(invoke().code).toBe(0);
  expect(readFileSync(calls,'utf8')).toContain('start');
  expect(existsSync(record)).toBe(false); expect(existsSync(marker)).toBe(false);
  expect(invoke({HANDOVER_ACTION:'restart_unit'}).error).toContain('no GPU session is active');
}));
test('Docker inspection errors preserve handover state and fail instead of claiming container removal', () => fixture((invoke,record,marker) => {
  expect(invoke({INSPECT_ERROR:'1'}).code).toBe(1);
  expect(existsSync(record)).toBe(true); expect(existsSync(marker)).toBe(true);
  expect(invoke().code).toBe(0);
}));
test('explicitly removed containers permit completing a handover after stack teardown', () => fixture((invoke,record,marker,calls) => {
  expect(invoke({MISSING:'1'}).code).toBe(0);
  expect(readFileSync(calls,'utf8')).not.toContain('start');
  expect(existsSync(record)).toBe(false); expect(existsSync(marker)).toBe(false);
}));

test('a global owner conflict retains handover records after canonical Docker restoration', () => fixture((invoke,record,marker) => {
  expect(invoke({OWNERSHIP_FAILURE:'1'}).code).toBe(1);
  expect(existsSync(record)).toBe(true); expect(existsSync(marker)).toBe(true);
  expect(invoke().code).toBe(0);
  expect(existsSync(record)).toBe(false); expect(existsSync(marker)).toBe(false);
}));

test('GPU restart verification restores its original owner after a transient restart failure', () => fixture((invoke,record,marker) => {
  const runtime = path.dirname(record);
  mkdirSync(path.join(runtime,'invocations'),{recursive:true});
  writeFileSync(path.join(runtime,'invocations/fhevm-gpu-consensus-tfhe-0.config'),
    'worker_sha256=recorded\nbuild_test_features=\ndevice=0\nstreams=16\nwork_items=100\nchains=20\nfhe_threads=8\ntokio_threads=4\nadaptive=\nbatch=\n');
  writeFileSync(path.join(runtime,'coprocessor.0.env'),'DATABASE_URL=postgres://localhost/coprocessor\n');
  const result=invoke({HANDOVER_ACTION:'verify_restore',GPU03_TEST:'1',GPU03_FAIL_FIRST_INSPECT:'1'});
  expect(result.code).not.toBe(0); // Recovery does not erase the failed test.
  expect(result.error).toContain('restoring fhevm-gpu-consensus-tfhe-0 after aborted verification');
  expect(readFileSync(path.join(runtime,'../../active'),'utf8').trim()).toBe('active');
  expect(existsSync(path.join(runtime,'../../restarted'))).toBe(true);
  expect(existsSync(record)).toBe(true);
  expect(existsSync(marker)).toBe(true);
}));

test('an unrecoverable GPU verification reports failed cleanup and retains ownership', () => fixture((invoke,record,marker) => {
  const runtime = path.dirname(record);
  mkdirSync(path.join(runtime,'invocations'),{recursive:true});
  writeFileSync(path.join(runtime,'invocations/fhevm-gpu-consensus-tfhe-0.config'),
    'worker_sha256=recorded\nbuild_test_features=\ndevice=0\nstreams=16\nwork_items=100\nchains=20\nfhe_threads=8\ntokio_threads=4\nadaptive=\nbatch=\n');
  const result=invoke({HANDOVER_ACTION:'verify_restore',GPU03_TEST:'1',INSPECT_ERROR:'1'});
  expect(result.code).not.toBe(0);
  expect(result.error).toContain('cleanup failed for fhevm-gpu-consensus-tfhe-0; ownership retained');
  expect(existsSync(record)).toBe(true);
  expect(existsSync(marker)).toBe(true);
}));

test('GPU unit restart waits through deactivating until the transient name is released', () => fixture((invoke,record) => {
  const runtime = path.dirname(record);
  mkdirSync(path.join(runtime,'invocations'),{recursive:true});
  writeFileSync(path.join(runtime,'invocations/fhevm-gpu-consensus-tfhe-0.config'),
    'worker_sha256=recorded\nbuild_test_features=\ndevice=0\nstreams=16\nwork_items=100\nchains=20\nfhe_threads=8\ntokio_threads=4\nadaptive=\nbatch=\n');
  writeFileSync(path.join(runtime,'coprocessor.0.env'),'DATABASE_URL=postgres://localhost/coprocessor\n');
  const result=invoke({HANDOVER_ACTION:'verify_restore',GPU03_TEST:'1',DELAY_STOP:'1'});
  expect(result.code, result.error).toBe(0);
  expect(existsSync(path.join(runtime,'../../stop-polled'))).toBe(true);
}));

test('GPU fault recovery refuses a binary rebuilt with another feature set', () => fixture((invoke,record) => {
  const runtime = path.dirname(record);
  mkdirSync(path.join(runtime,'invocations'),{recursive:true});
  writeFileSync(path.join(runtime,'invocations/fhevm-gpu-consensus-tfhe-0.config'),
    'worker_sha256=recorded\nbuild_test_features=tfhe-worker/test-failpoints\n');
  const result=invoke({HANDOVER_ACTION:'restart_unit',GPU03_TEST:'1',FAKE_BINARY_HASH:'production-rebuild'});
  expect(result.code).toBe(1);
  expect(result.error).toContain('binary changed since this invocation');
  expect(existsSync(path.join(runtime,'../../restarted'))).toBe(false);
}));

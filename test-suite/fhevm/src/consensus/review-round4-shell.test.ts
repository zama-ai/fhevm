import {expect,test} from 'bun:test';
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,rmSync,symlinkSync,existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';
const scripts=path.resolve(import.meta.dir,'../../scripts');
function section(file:string,start:string,end:string){const s=readFileSync(path.join(scripts,file),'utf8');return s.slice(s.indexOf(start),s.indexOf(end,s.indexOf(start)+start.length));}
for(const code of [0,37]) test(`identity capture refuses partial output when its producer exits ${code}`,()=>{
 const d=mkdtempSync(path.join(tmpdir(),'identity-status-'));
 try{symlinkSync(path.join(scripts,'lib'),path.join(d,'lib'));writeFileSync(path.join(d,'record-run-identity.sh'),`#!/bin/bash\necho artifact=image=sha256:fixture\nexit ${code}\n`,{mode:0o755});
 const r=Bun.spawnSync(['bash','-c',`SCRIPT_DIR='${d}'; source "$SCRIPT_DIR/lib/case-result.sh"; values=(); cr_read_run_identity values; status=$?; echo "$status:\${#values[@]}"`]);
 expect(r.stdout.toString().trim()).toBe(code===0?'0:1':'1:0');
 }finally{rmSync(d,{recursive:true,force:true});}
});
test('unknown topology is refused before it can skip every selected case green',()=>{
 const r=Bun.spawnSync(['bash','-c',`SCRIPT_DIR='${scripts}'; source "$SCRIPT_DIR/lib/case-result.sh"; sr_revision(){ echo fixture; }; REPO_ROOT=/unused; CONSENSUS_SCENARIO=unknown; cr_init unknown`]);
 expect(r.exitCode).toBe(2);expect(r.stderr.toString()).toContain('must identify');
});
test('DEG06 unrecorded setup error produces its own INVALID and increments once',()=>{
 const body=section('run-degraded-consensus.sh','case_gw() {','case_gw_body() {');
 const d=mkdtempSync(path.join(tmpdir(),'gw-early-verdict-'));
 try{const r=Bun.spawnSync(['bash','-c',`set -uo pipefail; SP_RUNTIME_DIR='${d}'; FAILURES=0
${body}
case_gw_body(){ return 1; }
cr_record(){ echo "$1:$2"; printf '%s\\n' "$1" >> "$CR_TERMINAL_FILE"; }
case_gw; echo status=$? failures=$FAILURES
`]);expect(r.stdout.toString()).toBe('DEG-06-GW-LISTENER-INFLIGHT:INVALID\nstatus=1 failures=1\n');
 }finally{rmSync(d,{recursive:true,force:true});}
});
test('a selected stopped sensitivity target is recorded before fleet exclusivity can mask it',()=>{
 const body=section('run-failure-matrix.sh','precheck_selected_target() {','main() {');
 const r=Bun.spawnSync(['bash','-c',`set -uo pipefail
${body}
ONLY=FM-ZKPROOF-CRASH; MATRIX=('FM-ZKPROOF-CRASH|crash|worker|kill|proof')
service_for(){ echo coprocessor1-zkproof-worker; }
sc_state(){ echo stopped; }
cr_record(){ printf '%s\\n' "$@"; }
precheck_selected_target
`]);expect(r.exitCode).toBe(1);expect(r.stdout.toString()).toContain('INVALID');expect(r.stdout.toString()).toContain('coprocessor1-zkproof-worker was not live before the fault (state=stopped)');
});
for(const failCopy of [false,true]) test(`harness recreation carries durable abort journals and refuses a failed restore copy (${failCopy})`,()=>{
 const d=mkdtempSync(path.join(tmpdir(),'recreate-journal-'));
 try{
 const r=Bun.spawnSync(['bash','-c',`set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${d}'; SP_RUNTIME_DIR='${d}/runtime'; FHEVM_STATE_DIR='${d}'
source "$SCRIPT_DIR/lib/suite-process.sh"
sp_init
mkdir -p '${d}/remote/tmp/handshake' '${d}/remote/private-journal'
echo original-work > '${d}/remote/tmp/handshake/work.json'
echo private-recovery > '${d}/remote/private-journal/recovery.json'
chmod 600 '${d}/remote/private-journal/recovery.json'
docker() {
 local op="$1"; shift
 case "$op" in
  exec)
   shift
   case "$1" in
    sh) printf /private-journal;;
    mkdir) shift 2; local p; for p in "$@"; do mkdir -p '${d}/remote'"$p"; done;;
    chmod) local mode="$2"; shift 2; local p; for p in "$@"; do chmod "$mode" '${d}/remote'"$p"; done;;
    *) return 90;;
   esac;;
  stop) echo stopped > '${d}/state';;
  inspect) echo 'false 0';;
  cp)
   [[ "$1" != -a ]] || shift
   local src="$1" dest="$2"
   if [[ "$src" == fixture:* ]]; then
     src='${d}/remote'"\${src#fixture:}"
     tar -C "$src" -cf - .
   else
     [[ ! -f '${d}/fail-copy' ]] || return 37
     dest='${d}/remote'"\${dest#fixture:}"
     tar -C "$dest" -xpf -
   fi;;
  *) return 90;;
 esac
}
sp_snapshot_recreate fixture /tmp/handshake || exit 2
[[ -f "$SP_RUNTIME_DIR/recreate-fixture/ready" ]] || exit 3
# Emulate Compose replacing the old filesystem, not an in-place restart.
rm -rf '${d}/remote'; mkdir -p '${d}/remote'
${failCopy?'touch '+JSON.stringify(d+'/fail-copy'):':'}
sp_restore_recreated fixture; copied=$?
[[ "$copied" == ${failCopy?1:0} ]] || exit 4
[[ -s "$SP_RUNTIME_DIR/recreate-fixture/journal.tar" ]] || exit 5
[[ "$(stat -c %a "$SP_RUNTIME_DIR/recreate-fixture/journal.tar")" == 600 ]] || exit 11
${failCopy?`[[ ! -e "$SP_RUNTIME_DIR/recreate-fixture/restored" ]] || exit 6
rm '${d}/fail-copy'
sp_restore_recreated fixture || exit 7`:':'}
[[ "$(cat '${d}/remote/private-journal/recovery.json')" == private-recovery ]] || exit 8
[[ "$(cat '${d}/remote/tmp/handshake/work.json')" == original-work ]] || exit 9
[[ "$(stat -c %a '${d}/remote/private-journal/recovery.json')" == 600 ]] || exit 10
`],{timeout:10000});
 expect(r.exitCode,r.stderr.toString()).toBe(0);
 }finally{rmSync(d,{recursive:true,force:true});}
});
test('deferred device split fails before launching a workload or a Docker operation',()=>{
 const d=mkdtempSync(path.join(tmpdir(),'deferred-device-'));
 try{writeFileSync(path.join(d,'docker'),`#!/bin/bash\ntouch '${d}/unexpected'\nexit 90\n`,{mode:0o755});
 const r=Bun.spawnSync(['bash',path.join(scripts,'run-materialization-consensus.sh'),'--device-split'],{env:{...process.env,PATH:`${d}:${process.env.PATH}`,FHEVM_STATE_DIR:d},timeout:5000});
 expect(r.exitCode).toBe(2);expect(r.stderr.toString()).toContain('selected-work GPU execution attribution is not implemented');
 expect(existsSync(path.join(d,'unexpected'))).toBe(false);
 }finally{rmSync(d,{recursive:true,force:true});}
});
for(const mode of ['valid','stale','wrong-case','wrong-workload','no-signal','multiple-signals','unrecovered','flat','not-ready']) test(`detector host requires exact factual receipt (${mode})`,()=>{
 const receipt={runId:mode==='stale'?'earlier':'this-run',caseId:mode==='wrong-case'?'FM-TFHE-STALL':'FM-CONSENSUS-DETECTOR',workload:mode==='wrong-workload'?'compute-chain':'detector-drift',driftDetected:true,driftRecovered:mode!=='unrecovered',signalsBefore:7,signalsAfter:mode==='no-signal'?7:mode==='multiple-signals'?9:8};
 const body=section('run-failure-matrix.sh','verify_detector_receipt() {','# --------------------------------------------------------------------------\n# One cell.');
 const r=Bun.spawnSync(['bash','-c',`set -uo pipefail
${body}
TEST_CONTAINER=fixture; HANDSHAKE_DIR=/unused; CR_RUN_ID=this-run
docker(){ printf '%s' "$FIXTURE_RECEIPT"; }
verify_detector_receipt
`],{env:{...process.env,FIXTURE_RECEIPT:JSON.stringify(mode==='flat'?receipt:{name:'failure-verification',ready:mode!=='not-ready',payload:receipt})}});
 expect(r.exitCode).toBe(mode==='valid'?0:1);
});
test('DEG06 early INVALID is represented without rewriting an independently passed DEG01 after clean EXIT recovery',()=>{
 const body=section('run-degraded-consensus.sh','case_gw() {','case_gw_body() {');
 const d=mkdtempSync(path.join(tmpdir(),'gw-sibling-verdict-'));
 try{const r=Bun.spawnSync(['bash','-c',`set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${d}'; SP_RUNTIME_DIR='${d}/runtime'
mkdir -p "$SP_RUNTIME_DIR"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/result-staging.sh"
CR_RUN_ID=gateway-fixture; CR_REVISION=fixture; CR_SCENARIO=three-of-three; CR_OPERATORS=3; CR_THRESHOLD=3
CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=fixture; FAILURES=0
CONSENSUS_RESULTS_DIR='${d}/published'; export CONSENSUS_RESULTS_DIR
rs_stage_results
cr_record DEG-01-AGREEMENT-QUORUM PASS cleanup=ok assert=bytes=pass:fixture assert=quorum=pass:fixture || exit 2
${body}
case_gw_body(){ return 1; }
case_gw; [[ "$?" == 1 ]] || exit 3
rs_finalize_results 1 ok; [[ "$?" == 1 ]] || exit 4
cat '${d}/published/gateway-fixture.jsonl'
`],{timeout:5000});expect(r.exitCode,r.stderr.toString()).toBe(0);
 const rows=r.stdout.toString().split('\n').filter(x=>x.startsWith('{')).map(x=>JSON.parse(x));
 expect(rows.map(x=>[x.caseId,x.state,x.cleanup.state])).toEqual([['DEG-01-AGREEMENT-QUORUM','PASS','ok'],['DEG-06-GW-LISTENER-INFLIGHT','INVALID','ok']]);
 }finally{rmSync(d,{recursive:true,force:true});}
});
test('detector host accepts the actual E2E publishHandshake envelope',()=>{
 const d=mkdtempSync(path.join(tmpdir(),'detector-envelope-'));
 try {
  const payload={runId:'this-run',caseId:'FM-CONSENSUS-DETECTOR',workload:'detector-drift',driftDetected:true,driftRecovered:true,signalsBefore:7,signalsAfter:8};
  const published=Bun.spawnSync(['bun','-e',`import {publishHandshake} from ${JSON.stringify(path.resolve(scripts,'../../e2e/test/consensus/handshake.ts'))}; publishHandshake('failure-verification',${JSON.stringify(payload)});`],{env:{...process.env,CONSENSUS_HANDSHAKE_DIR:d}});
  expect(published.exitCode,published.stderr.toString()).toBe(0);
  const body=section('run-failure-matrix.sh','verify_detector_receipt() {','# --------------------------------------------------------------------------\n# One cell.');
  const r=Bun.spawnSync(['bash','-c',`set -uo pipefail
${body}
TEST_CONTAINER=fixture; HANDSHAKE_DIR='${d}'; CR_RUN_ID=this-run
docker(){ shift 2; "$@"; }
verify_detector_receipt
`]);expect(r.exitCode,r.stderr.toString()).toBe(0);
 } finally {rmSync(d,{recursive:true,force:true});}
});
for (const missing of ['GATEWAY_URL', 'GATEWAY_CONFIG_ADDRESS', 'CIPHERTEXT_COMMITS_ADDRESS']) {
 test(`crash host rejects missing ${missing} before worker inspection, pause or SQL faults`,()=>{
  const d=mkdtempSync(path.join(tmpdir(),'crash-quorum-preflight-'));
  try {
   const envDir=path.join(d,'env'); mkdirSync(envDir);
   writeFileSync(path.join(envDir,'coprocessor.env'), ['GATEWAY_URL','GATEWAY_CONFIG_ADDRESS','CIPHERTEXT_COMMITS_ADDRESS'].filter(x=>x!==missing).map(x=>`${x}=fixture`).join('\n')+'\n');
   const preamble=section('run-crash-retry-consensus.sh','die() {','cleanup_crash() {');
   // Execute the actual main prefix through fault setup. A future regression
   // that moves the gate after inspection/SQL/pause trips the strict stubs.
   const main=section('run-crash-retry-consensus.sh','main() {','  # Clear stale handshake files,');
   const r=Bun.spawnSync(['bash','-c',`set -uo pipefail
ENV_DIR='${envDir}'; SP_RUNTIME_DIR='${d}'; CASE_ID=CR-01-INTERRUPT-BEFORE-COMMIT; VICTIM=1; TEST_CONTAINER=fixture
${preamble}
${main}
}
cr_init(){ :; }
cr_results_file(){ echo '${d}/results.jsonl'; }
cr_now(){ echo fixture-time; }
operator_count(){ echo 3; }
suite_identity_assert(){ :; }
cr_skip_wrong_scenario(){ return 1; }
env_value(){ sed -n "s/^$1=//p" "$2" | tail -1; }
sp_case_start(){ :; }
victim_container(){ echo worker; }
docker(){ echo unexpected-docker >&2; exit 90; }
sc_state(){ echo running; }
sc_restart_budget(){ echo available; }
require_observability(){ echo unexpected-observability >&2; exit 91; }
psql_victim(){ echo unexpected-SQL >&2; exit 92; }
sc_pause(){ echo unexpected-pause >&2; exit 93; }
cr_record(){ printf '%s\\n' "$@"; }
trap 'crash_record_abort ok' EXIT
main
`],{timeout:5000});
   expect(r.exitCode,r.stderr.toString()).toBe(1);
   expect(r.stdout.toString()).toContain('INVALID');
   expect(r.stdout.toString()).toContain(`required quorum configuration ${missing} is missing`);
   expect(r.stderr.toString()).not.toContain('unexpected-');
  } finally {rmSync(d,{recursive:true,force:true});}
 });
}

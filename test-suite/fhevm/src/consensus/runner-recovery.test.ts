import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, existsSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { finalizeCaseResults } from "../../scripts/finalize-case-results";
import { RESULT_SCHEMA_VERSION, type CaseResult } from "./results";

const scripts = path.resolve(import.meta.dir, "../../scripts");
const source = (name: string) => readFileSync(path.join(scripts, name), "utf8");
const section = (text: string, start: string, end: string) => {
  const a = text.indexOf(start), b = text.indexOf(end, a + start.length);
  if (a < 0 || b < 0) throw new Error(`missing fixture boundary ${start}/${end}`);
  return text.slice(a, b);
};
function shell(body: string) {
  const dir = mkdtempSync(path.join(tmpdir(), "runner-recovery-"));
  try {
    return Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'
REPO_ROOT='${dir}'
FHEVM_STATE_DIR='${dir}'
SP_RUNTIME_DIR='${dir}/processes'
SC_RESTORE_LOG='${dir}/restores'
mkdir -p "$SP_RUNTIME_DIR"; touch "$SC_RESTORE_LOG"
source "$SCRIPT_DIR/lib/host-command.sh"
${body}`], { timeout: 10000 });
  } finally { rmSync(dir, { recursive: true, force: true }); }
}

for (const cleanupFails of [false, true]) {
  test(`failed phase admits a later real phase only after complete recovery (cleanup fails=${cleanupFails})`, () => {
    const run = shell(`export CR_RUN_ID=runner-recovery-fixture CONSENSUS_RUN_ID=runner-recovery-fixture
source "$SCRIPT_DIR/lib/suite-process.sh"
sp_init
SP_OWNS_RUNTIME=1
bun() { echo 'timeout: 60s'; }
cat > "$REPO_ROOT/docker" <<'MOCK'
#!/bin/bash
[[ "$1" == exec ]] || exit 90
shift
while [[ "$1" == -e || "$1" == --env ]]; do
  [[ "$#" -ge 2 ]] || exit 91
  export "$2" || exit 91
  shift 2
done
[[ "$#" -ge 2 ]] || exit 91
shift
exec "$@"
MOCK
chmod +x "$REPO_ROOT/docker"
export PATH="$REPO_ROOT:$PATH"
sp_recover_suite_state() { :; }
sp_case_cleanup() { echo special-recovery; }
sc_run_restores() { echo service-recovery; return ${cleanupFails ? 1 : 0}; }
sp_case_start ONE || exit 2
sp_exec target node -e 'if(process.env.CONSENSUS_RUN_ID!=="runner-recovery-fixture")process.exit(91);require("fs").appendFileSync(process.argv[1],"first\\n");process.exit(1)' "$REPO_ROOT/launches"; first=$?
[[ "$first" == 1 && -f "$SP_RUNTIME_DIR/cancelling" ]] || exit 3
sp_case_start TWO; boundary=$?
sp_exec target node -e 'require("fs").appendFileSync(process.argv[1],"second\\n")' "$REPO_ROOT/launches"; second=$?
cat "$REPO_ROOT/launches"
[[ "$boundary" == ${cleanupFails ? 1 : 0} && "$second" == ${cleanupFails ? 143 : 0} ]]`);
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    expect(run.stdout.toString()).toBe(`special-recovery\nservice-recovery\nfirst\n${cleanupFails ? "" : "second\n"}`);
  });
}

test("fork SQL cleanup failure restores unrelated services and retains the gated worker's ownership", () => {
  const fork = source("run-fork-consensus.sh");
  const run = shell(`${section(fork, "sp_case_cleanup() {", "cleanup_on_exit() {")}
SP_CONTAMINATION="$REPO_ROOT/blocked"; SP_PHASE_REGISTRY="$SP_RUNTIME_DIR/phases"
FORK_OPERATOR=2; FORK_GATE_ARMED=1; FORK_REPLAY_ARMED=0
printf 'coprocessor2-tfhe-worker|resume\nunrelated-listener|start\n' > "$SC_RESTORE_LOG"
fork_prefix() { echo coprocessor2; }
timeout() { return 1; }
sc_resume() { echo "resumed $1"; }
sc_start() { echo "started $1"; }
sp_case_cleanup; result=$?
[[ "$result" == 1 && -f "$SP_RUNTIME_DIR/fork-recovery" ]] || exit 2
grep -F 'coprocessor2-tfhe-worker|resume' "$SC_RESTORE_LOG" >/dev/null`);
  expect(run.exitCode, run.stderr.toString()).toBe(0);
  expect(run.stdout.toString()).toBe("started unrelated-listener\n");
});

test("empty or unfamiliar phase errors always produce a nonempty failure reason", () => {
  const run = shell(`source "$SCRIPT_DIR/lib/case-result.sh"
cr_failure_reason ''
cr_failure_reason 'signal terminated'`);
  expect(run.exitCode, run.stderr.toString()).toBe(0);
  expect(run.stdout.toString()).toBe("suite exited unsuccessfully without diagnostic output\nsignal terminated\n");
});

const pass = (caseId: string): CaseResult => ({
  schemaVersion: RESULT_SCHEMA_VERSION, runId: "run", caseId, state: "PASS", revision: "abc123",
  executionClass: { software: "abc123", backend: "cpu", hardware: "cpu" },
  topology: { scenario: "three-of-three", operators: 3, threshold: 3 },
  startedAt: "2026-09-13T00:00:00Z", endedAt: "2026-09-13T00:00:01Z",
  cleanup: { state: "ok" }, assertions: [{ name: "bytes", outcome: "pass" }],
});

test("parent restore failure publishes one failed verdict per staged case, including delegated siblings", () => {
  const records = [pass("FM-TFHE-CRASH"), pass("CR-01-INTERRUPT-BEFORE-COMMIT"), pass("REG-03-SUPERVISED-DAEMON-RECOVERY")];
  const final = finalizeCaseResults(records, { state: "FAIL", cleanup: "failed", detail: "parent restore timed out" });
  expect(final.map(({ state, cleanup }) => [state, cleanup.state])).toEqual([["FAIL", "failed"], ["FAIL", "failed"], ["FAIL", "failed"]]);
  expect(final.map(({ caseId }) => caseId)).toEqual(records.map(({ caseId }) => caseId));
  expect(records.every(({ state }) => state === "PASS")).toBe(true);
  expect(finalizeCaseResults(records)).toEqual(records);
});

test("child contradictions are refused when parent recovery cannot explain a final failure", () => {
  const record = pass("FM-TFHE-CRASH");
  expect(() => finalizeCaseResults([record, { ...record, state: "FAIL", detail: "failed" }])).toThrow("conflicting child verdicts");
});

test("parent finalization retains an earlier INVALID reason instead of reducing it to a generic child exit", () => {
  const record: CaseResult = { ...pass("FM-TFHE-CRASH"), state: "INVALID", detail: "never observed the pending chain" };
  const [final] = finalizeCaseResults([record], { state: "FAIL", cleanup: "ok", detail: "case runner exited 1" });
  expect(final!.state).toBe("INVALID");
  expect(final!.detail).toBe("never observed the pending chain; case runner exited 1");
});

test("a bare failed phase can be recorded with the actual CLI and a nonempty diagnostic", () => {
  const run = shell(`source "$SCRIPT_DIR/lib/case-result.sh"
CR_RUN_ID=empty-failure; CR_REVISION=abc123; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=cpu
CR_SCENARIO=three-of-three; CR_OPERATORS=3; CR_THRESHOLD=3
CONSENSUS_RESULTS_DIR="$REPO_ROOT/results"
export CONSENSUS_RESULTS_DIR
cr_record DEG-01-AGREEMENT-QUORUM FAIL cleanup=not_required detail='' || exit 2
cat "$(cr_results_file)"`);
  expect(run.exitCode, run.stderr.toString()).toBe(0);
  const record = JSON.parse(run.stdout.toString());
  expect(record.state).toBe("FAIL");
  expect(record.detail.length).toBeGreaterThan(0);
});

test("failed early squash freeze prevents matrix arming and records INVALID", () => {
  const matrix = source("run-failure-matrix.sh");
  const fragment = section(matrix, "  local armed_early=0", "  local -a extra_down=()");
  const run = shell(`run() {
 local case_id=FM-OBJECT-STORAGE-OUTAGE service=minio workload=materialization started=now
 arm_depends_on() { return 0; }; hold_back_kind() { echo sns; }; operator_count() { echo 3; }
 operator_prefix() { echo "coprocessor$1"; }; docker() { return 0; }
 sc_pause() { [[ "$1" != coprocessor1-sns-worker ]]; }
 restore_case_resources() { echo restored; }
 cr_record() { echo "$1 $2"; }
 run_phase() { echo 'UNSAFE ARM'; return 0; }
${fragment}
}
run; [[ "$?" == 1 ]]`);
  expect(run.exitCode, run.stderr.toString()).toBe(0);
  expect(run.stdout.toString()).toBe("restored\nFM-OBJECT-STORAGE-OUTAGE INVALID\n");
});

test("request restart budget is renewed before pausing downstream or launching a request", () => {
  const request = source("run-request-recovery.sh");
  const fragment = section(request, "# Renew the supervisor", "# KMS is the durable");
  const run = shell(`TARGET=target
sc_restart_budget() { echo exhausted; }
sc_register_restore() { echo registered; }
sc_reset_restart_budget() { echo reset; }
fail() { exit 1; }
${fragment}
echo arm`);
  expect(run.exitCode, run.stderr.toString()).toBe(0);
  expect(run.stdout.toString()).toBe("registered\nreset\narm\n");
});

test("crash restart budget is renewed before failpoint creation or workload arming", () => {
  const crash = source("run-crash-retry-consensus.sh");
  const fragment = section(crash, '  if [[ "$(sc_restart_budget "$container")" == exhausted ]]', "  require_observability");
  const run = shell(`container=target
sc_restart_budget() { echo exhausted; }
sc_register_restore() { echo registered; }
sc_reset_restart_budget() { echo reset; }
die() { exit 1; }
${fragment}
echo arm`);
  expect(run.exitCode, run.stderr.toString()).toBe(0);
  expect(run.stdout.toString()).toBe("registered\nreset\narm\n");
});

for (const setting of ["true", "false"]) {
  test(`core degraded coverage verifies live auto-revert=${setting} before allowing its oracle`, () => {
    const degraded = source("run-degraded-consensus.sh");
    const fragment = section(degraded, "core_require_auto_revert() {", "degraded_record_pass() {");
    const run = shell(`${fragment}
FAILURES=0
operator_count() { echo 3; }
timeout() { shift 2; "$@"; }
docker() { echo '[{"State":{"Status":"running"},"Config":{"Env":["DRIFT_AUTO_REVERT_ENABLED=${setting}"],"Cmd":[]}}]'; }
cr_skip_wrong_scenario() { return 1; }
cr_record() { echo "$1 $2"; }
core_require_auto_revert DEG-01-AGREEMENT-QUORUM; result=$?
[[ "$result" == ${setting === "true" ? 0 : 1} ]]`);
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    expect(run.stdout.toString()).toBe(setting === "true" ? "" : "DEG-01-AGREEMENT-QUORUM INVALID\n");
  });
}

test("crash precondition failures acquire a structured INVALID after cleanup", () => {
  const crash = source("run-crash-retry-consensus.sh");
  const run = shell(`${section(crash, "crash_record_abort() {", "retain_crash_cleanup() {")}
CR_TERMINAL_FILE="$SP_RUNTIME_DIR/recorded"; touch "$CR_TERMINAL_FILE"
CASE_ID=CR-01-INTERRUPT-BEFORE-COMMIT; CRASH_CONTROLS_ARMED=0; SP_FORCED_STOP=0
CRASH_PENDING_FAILURE='never observed operator 1 holding a pending target chain within the acquisition window'
sp_cancel_all() { return 0; }; sp_recover_suite_state() { return 0; }; sc_run_restores() { return 0; }; sp_dispose() { return 0; }
cr_now() { echo now; }; cr_record() { printf '%s\\n' "$*"; }
trap cleanup_crash EXIT
exit 1`);
  expect(run.exitCode, run.stderr.toString()).toBe(1);
  expect(run.stdout.toString()).toContain("CR-01-INTERRUPT-BEFORE-COMMIT INVALID");
  expect(run.stdout.toString()).toContain("never observed operator 1");
});

const verifierState = (paused: boolean) => ({
  Status: paused ? "paused" : "running", Running: true, Paused: paused,
  Restarting: false, Pid: 1234,
});
const verifierNetworks = {shared: {NetworkID: "database-network", IPAddress: "192.0.2.10"}};

function proofAcknowledgement(state: ReturnType<typeof verifierState>, recovered: boolean,
  networks: Record<string, {NetworkID: string; IPAddress: string}> = verifierNetworks) {
  const inspected = [
    {State: state, NetworkSettings: {Networks: networks}},
    {NetworkSettings: {Networks: {shared: {NetworkID: "database-network"}}}},
  ];
  return shell(`${section(source("run-failure-matrix.sh"), "worker_database_ip() {", "# The container whose network namespace")}
DB_CONTAINER=database; TEST_CONTAINER=target; HANDSHAKE_DIR="$REPO_ROOT/proof-ack"
hc_run() { [[ "$1" == timeout ]] || return 2; shift 3; "$@"; }
docker() {
  if [[ "$1" == inspect ]]; then
    cat <<'INSPECTED'
${JSON.stringify(inspected)}
INSPECTED
  else
    [[ "$1" == exec && "$2" == target ]] || return 2
    shift 2; "$@"
  fi
}
publish_fault_ack FM-ZKPROOF-CRASH verifier fault-before identity-before ${recovered ? "identity-after recovered-at" : ""}
status=$?
if [[ "$status" == 0 ]]; then
  cat "$HANDSHAKE_DIR/failure-fault.json"
else
  [[ ! -e "$HANDSHAKE_DIR/failure-fault.json" ]] || exit 99
fi
exit "$status"`);
}

for (const recovered of [false, true]) {
  const phase = recovered ? "recovery" : "held workload arm";
  test(`proof ${phase} acknowledgement accepts only its observed verifier state`, () => {
    const run = proofAcknowledgement(verifierState(!recovered), recovered);
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    const payload = JSON.parse(run.stdout.toString()).payload;
    expect(payload.recoveredWorkerAddress).toBe("192.0.2.10");
    expect(payload.recoveryObservedAt).toBe(recovered ? "recovered-at" : "");
  });

  for (const [label, state] of [
    ["opposite pause state", verifierState(recovered)],
    ["not running", {...verifierState(!recovered), Running: false}],
    ["inconsistent pause flag", {...verifierState(!recovered), Paused: recovered}],
    ["restarting", {...verifierState(!recovered), Restarting: true}],
    ["exited", {...verifierState(!recovered), Status: "exited", Running: false, Paused: false, Pid: 0}],
    ["missing live PID", {...verifierState(!recovered), Pid: 0}],
  ] as const) {
    test(`proof ${phase} acknowledgement rejects ${label} before publishing evidence`, () => {
      const run = proofAcknowledgement(state, recovered);
      expect(run.exitCode).toBe(1);
      expect(run.stderr.toString()).toContain(`verifier is not ${recovered ? "running" : "paused"}`);
      expect(run.stdout.toString()).toBe("");
    });
  }
}

for (const [label, networks] of [
  ["no shared network", {other: {NetworkID: "unrelated", IPAddress: "192.0.2.10"}}],
  ["ambiguous shared addresses", {...verifierNetworks, second: {NetworkID: "database-network", IPAddress: "192.0.2.11"}}],
  ["invalid client address", {shared: {NetworkID: "database-network", IPAddress: "not-an-address"}}],
] as const) {
  test(`proof verifier attribution rejects ${label}`, () => {
    const run = proofAcknowledgement(verifierState(false), true, networks);
    expect(run.exitCode).toBe(1);
    expect(run.stderr.toString()).toContain("cannot identify verifier database-client IP");
    expect(run.stdout.toString()).toBe("");
  });
}

const busybox = Bun.which("busybox");
const timeoutPrograms = [
  {name: "BusyBox", argv: busybox ? [busybox, "timeout"] : undefined},
  {name: "GNU", argv: Bun.which("timeout") ? [Bun.which("timeout")!] : undefined},
];
for (const runner of ["run-degraded-consensus.sh", "run-failure-matrix.sh"]) {
  for (const program of timeoutPrograms) for (const hangs of [false, true]) test.skipIf(!program.argv)(`${runner} DNS probe works with real ${program.name} and stops remote hangs (${hangs})`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "busybox-dns-probe-"));
    try {
      mkdirSync(path.join(dir, "bin"));
      const remotePid = path.join(dir, "remote-pid");
      // A Docker client and its exec process have separate lifetimes. Isolate
      // the remote group so the host timeout cannot accidentally satisfy this test.
      writeFileSync(path.join(dir, "bin/docker"), `#!/usr/bin/python3
import os,subprocess,sys
assert sys.argv[1:3] == ['exec','target']
args=sys.argv[3:]
if args[0]=='timeout': args=${JSON.stringify(program.argv)}+args[1:]
remote=subprocess.Popen(args,start_new_session=True)
sys.exit(remote.wait())
`, {mode: 0o755});
      writeFileSync(path.join(dir, "bin/getent"), `#!/usr/bin/python3
import os,signal,sys,time
if ${hangs ? "False" : "True"}:
 print('127.0.0.1 db'); sys.exit(0)
if sys.argv[-1] != 'db': sys.exit(1)
open(${JSON.stringify(remotePid)},'w').write(str(os.getpid()))
signal.signal(signal.SIGTERM,signal.SIG_IGN)
time.sleep(30)
`, {mode: 0o755});
      const body = section(source(runner), "container_resolves_something() {", "ensure_test_container_resolves() {");
      const started = Date.now();
      const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'
source "$SCRIPT_DIR/lib/host-command.sh"
TEST_CONTAINER=target
${body}
container_resolves_something
`], {env: {...process.env, PATH: `${dir}/bin:${process.env.PATH}`}, timeout: 8000});
      expect(run.exitCode, run.stderr.toString()).toBe(hangs ? 1 : 0);
      expect(Date.now() - started).toBeLessThan(6000);
      if (hangs) {
        expect(existsSync(remotePid)).toBe(true); // Reject a parse error before getent ran.
        const pid = Number(readFileSync(remotePid, "utf8"));
        let state = "gone";
        try { state = readFileSync(`/proc/${pid}/stat`, "utf8").split(") ")[1]!.split(" ")[0]!; } catch {}
        expect(["gone", "Z", "X"]).toContain(state);
      }
    } finally {
      // Cleanup a regression's escaped mock process without touching the stack.
      const file = path.join(dir, "remote-pid");
      if (existsSync(file)) {
        const pid = Number(readFileSync(file, "utf8"));
        try {
          if (readFileSync(`/proc/${pid}/cmdline`, "utf8").includes(path.join(dir, "bin/getent"))) process.kill(pid, "SIGKILL");
        } catch {}
      }
      rmSync(dir, {recursive: true, force: true});
    }
  }, 10000);
}

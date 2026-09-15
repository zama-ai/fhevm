import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const cliDir = path.resolve(import.meta.dir, "../..");

/** Run the real case and IP parser; Docker loses its IP exactly when stop runs. */
function exercise(mode: string) {
  const dir = mkdtempSync(path.join(tmpdir(), "fork-replay-ip-"));
  try {
    const body = `
trap 'rm -f "$SC_RESTORE_LOG"' EXIT
CR_RUN_ID=fork-ip-fixture
state_file='${dir}/state'
capture_file='${dir}/capture'
echo before > "$state_file"
log() { :; }
cr_now() { echo 2026-09-13T00:00:00Z; }
cr_record() { echo "RECORD $1 $2"; }
cr_failure_reason() { echo failure; }
sc_run_restores() { :; }
fork_restore_case() { :; }
sc_stop() { echo stopped > "$state_file"; }
sc_start() { echo after > "$state_file"; }
sc_identity() {
  if [[ "$(cat "$state_file")" == before || "$MODE" == same-process ]]; then
    echo 'container=poller pid=10 started=before restarts=0'
  else
    echo 'container=poller pid=20 started=after restarts=0'
  fi
}
docker() {
  if [[ "$1" == inspect ]]; then
    [[ "$#" == 3 ]] || { echo '{}'; return; }
    local phase status=running pid=10 address=172.20.0.7
    phase="$(cat "$state_file")"
    if [[ "$phase" == stopped ]]; then status=exited; pid=0; address='invalid IP'; fi
    if [[ "$phase" == after ]]; then
      pid=20
      [[ "$MODE" != changed-ip ]] || address=172.20.0.8
      [[ "$MODE" != missing-ip ]] || address='invalid IP'
    fi
    [[ "$MODE" != invalid-before ]] || address='invalid IP'
    printf '[{"State":{"Status":"%s","Pid":%s},"NetworkSettings":{"Networks":{"stack":{"NetworkID":"net","IPAddress":"%s"}}}},{"NetworkSettings":{"Networks":{"stack":{"NetworkID":"net","IPAddress":"172.20.0.2"}}}}]\n' "$status" "$pid" "$address"
  elif [[ "$1" == exec && "$3" == node ]]; then
    printf '%s' "\${@: -2:1}" > "$capture_file"
  elif [[ "$1" == exec && "$3" == cat ]]; then
    echo '{"rewoundTo":10,"watermarkBefore":12}'
  fi
}
run_phase() {
  local -n output="$1"
  output="$3"
  if [[ "$2" == f5-arm ]]; then
    [[ "$(cat "$state_file")" == stopped && "$(cat "$capture_file")" == 172.20.0.7 ]] || return 1
  elif [[ "$2" == f5-verify ]]; then echo VERIFIED; fi
}
case_f5
exit $?
`;
    const source = readFileSync(path.join(cliDir, "scripts/run-fork-consensus.sh"), "utf8")
      .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cliDir}/scripts'`)
      .replace(/^main\s*$/m, () => body);
    const script = path.join(dir, "runner.sh");
    writeFileSync(script, source);
    return Bun.spawnSync(["bash", script], { cwd: cliDir, env: { ...process.env, MODE: mode } });
  } finally { rmSync(dir, { recursive: true, force: true }); }
}

test("F5 captures the live IP before stop discards it and verifies it after replacement", () => {
  const result = exercise("success");
  expect(result.exitCode, result.stderr.toString()).toBe(0);
  expect(result.stdout.toString()).toContain("VERIFIED");
  expect(result.stdout.toString()).toContain("RECORD FORK-05-REPLAY PASS");
});

for (const mode of ["same-process", "changed-ip", "missing-ip", "invalid-before"]) {
  test(`F5 rejects ${mode} instead of claiming attributed replay`, () => {
    const result = exercise(mode);
    expect(result.exitCode).toBe(1);
    expect(result.stdout.toString()).toContain("RECORD FORK-05-REPLAY INVALID");
    expect(result.stdout.toString()).not.toContain("VERIFIED");
    expect(result.stdout.toString()).not.toContain("RECORD FORK-05-REPLAY PASS");
  });
}

import { expect, test } from "bun:test";
import { randomUUID } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const cli = path.resolve(import.meta.dir, "../..");
const supervisor = readFileSync(path.join(cli, "scripts/lib/container-phase.cjs"), "utf8");
const live = (pid: number) => {
  try {
    const stat = readFileSync(`/proc/${pid}/stat`, "utf8");
    return !["Z", "X"].includes(stat.slice(stat.lastIndexOf(")") + 2).split(" ")[0]);
  } catch { return false; }
};
const waitFor = async (predicate: () => boolean) => {
  const deadline = Date.now() + 5000;
  while (!predicate()) {
    if (Date.now() > deadline) throw new Error("fixture failed to become ready");
    await Bun.sleep(20);
  }
};
const run = (token: string, milliseconds: number, program: string) => Bun.spawn([
  "node", "-e", supervisor, "run", token, String(Date.now() + milliseconds), "node", "-e", program,
], { stdout: "pipe", stderr: "pipe" });
const cancel = (token: string) => Bun.spawn(["node", "-e", supervisor, "cancel", token], { stdout: "pipe", stderr: "pipe" });
const cleanupToken = (token: string) => {
  for (const suffix of ["json", "cancelled"]) rmSync(`/tmp/fhevm-consensus-phases/${token}.${suffix}`, {force: true});
};

test.skipIf(process.platform !== "linux")("a phase deadline kills TERM-resistant children and grandchildren", async () => {
  const directory = mkdtempSync(path.join(tmpdir(), "phase-deadline-"));
  const token = `test_${randomUUID()}`;
  const file = path.join(directory, "pids");
  const child = run(token, 1200, `
    const fs = require('fs'), {spawn} = require('child_process');
    process.on('SIGTERM', () => {});
    const child = spawn(process.execPath, ['-e', 'process.on("SIGTERM", () => {}); setInterval(() => {}, 1000)'], {stdio:'ignore'});
    fs.writeFileSync(${JSON.stringify(file)}, JSON.stringify([process.pid, child.pid]));
    setInterval(() => {}, 1000);
  `);
  try {
    await waitFor(() => existsSync(file));
    const pids: number[] = JSON.parse(readFileSync(file, "utf8"));
    expect(pids.every(live)).toBe(true);
    expect(await child.exited).toBe(124);
    expect(pids.some(live)).toBe(false);
  } finally {
    await cancel(token).exited;
    cleanupToken(token);
    rmSync(directory, {recursive: true, force: true});
  }
}, 10000);

test.skipIf(process.platform !== "linux")("explicit cancellation joins the actual test, and pre-launch cancellation prevents new work", async () => {
  const directory = mkdtempSync(path.join(tmpdir(), "phase-cancel-"));
  const token = `test_${randomUUID()}`;
  const file = path.join(directory, "pid");
  const child = run(token, 30000, `require('fs').writeFileSync(${JSON.stringify(file)}, String(process.pid)); setInterval(() => {}, 1000);`);
  try {
    await waitFor(() => existsSync(file));
    const pid = Number(readFileSync(file, "utf8"));
    expect(await cancel(token).exited).toBe(0);
    expect(await child.exited).toBe(143);
    expect(live(pid)).toBe(false);
    rmSync(file);
    const prevented = run(token, 30000, `require('fs').writeFileSync(${JSON.stringify(file)}, 'unwanted work');`);
    expect(await prevented.exited).toBe(143);
    expect(existsSync(file)).toBe(false);
  } finally {
    await cancel(token).exited;
    cleanupToken(token);
    rmSync(directory, {recursive: true, force: true});
  }
}, 10000);

test("arm and verify use the same remaining case budget", () => {
  const result = Bun.spawnSync(["bash", "-c", `
    set -eu
    source scripts/lib/case-deadline.sh
    now=100
    date() { echo "$now"; }
    case_deadline_start 30
    now=120
    case_phase_deadline_ms 900
    now=129
    case_seconds_left
    now=130
    case_seconds_left
  `], {cwd: cli});
  expect(result.stdout.toString()).toBe("130000\n1\n");
  expect(result.exitCode).toBe(124);
});

test.skipIf(process.platform !== "linux")("cancellation cleans owned descendants after their supervisor is killed", async () => {
  const directory = mkdtempSync(path.join(tmpdir(), "phase-orphan-"));
  const token = `test_${randomUUID()}`;
  const file = path.join(directory, "pid");
  const child = run(token, 30000, `require('fs').writeFileSync(${JSON.stringify(file)}, String(process.pid)); process.on('SIGTERM', () => {}); setInterval(() => {}, 1000);`);
  try {
    await waitFor(() => existsSync(file));
    const pid = Number(readFileSync(file, "utf8"));
    child.kill("SIGKILL");
    await Bun.sleep(50);
    expect(live(pid)).toBe(true);
    expect(await cancel(token).exited).toBe(0);
    expect(await child.exited).toBe(137);
    expect(live(pid)).toBe(false);
  } finally {
    await cancel(token).exited;
    cleanupToken(token);
    rmSync(directory, {recursive: true, force: true});
  }
}, 10000);

test.skipIf(process.platform !== "linux")("matrix timeout and runner abort cancel the test before restoring faults", async () => {
  for (const abort of [false, true]) {
    const directory = mkdtempSync(path.join(tmpdir(), "matrix-cancellation-"));
    const pidFile = path.join(directory, "pid");
    const restored = path.join(directory, "restored");
    const fixture = path.join(directory, "runner.sh");
    // Docker's client behavior is deliberately replaced only at its command
    // transport boundary. The actual shell traps and process supervisor run.
    writeFileSync(path.join(directory, "docker"), `#!/usr/bin/env bash
[[ "$1" == exec ]] || exit 1
shift
while [[ "$1" == -e ]]; do export "$2"; shift 2; done
shift
exec "$@"
`, {mode: 0o755});
    const program = `require('fs').writeFileSync(${JSON.stringify(pidFile)}, String(process.pid)); setInterval(() => {}, 1000);`;
    const replacement = `
bun() {
  if [[ "$1" == *consensus-inventory.ts && "$2" == show ]]; then echo 'timeout: ${abort ? 30 : 2}s';
  else command bun "$@"; fi
}
CR_RUN_ID=cancellation-fixture
cr_record() { echo "record $*"; }
sp_recover_suite_state() { :; }
sc_run_restores() {
  if [[ -f '${pidFile}' ]]; then
    if kill -0 "$(cat '${pidFile}')" 2>/dev/null; then echo 'restored while test alive' >&2; return 1; fi
    echo restored >> '${restored}'
  fi
}
run_cell_body() {
  sp_exec "$TEST_CONTAINER" node -e '${program.replaceAll("'", "'\\''")}'

}
run_cell FM-TFHE-STALL stall worker pause compute-chain
exit $?
`;
    const source = readFileSync(path.join(cli, "scripts/run-failure-matrix.sh"), "utf8")
      .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cli}/scripts'`)
      .replace(/^main\s*$/m, () => replacement);
    writeFileSync(fixture, source);
    const child = Bun.spawn(["bash", fixture], {cwd: cli, stdout: "pipe", stderr: "pipe", env: {
      ...process.env, PATH: `${directory}:${process.env.PATH}`, FHEVM_STATE_DIR: directory,
      SC_RESTORE_LOG: path.join(directory, "restore-log"),
    }});
    try {
      await waitFor(() => existsSync(pidFile));
      if (abort) child.kill("SIGTERM");
      const status = await child.exited;
      const errors = await new Response(child.stderr).text();
      expect(status, errors).toBe(abort ? 143 : 124);
      expect(live(Number(readFileSync(pidFile, "utf8")))).toBe(false);
      expect(existsSync(restored), errors).toBe(true);
      expect(errors).not.toContain("restored while test alive");
    } finally {
      child.kill("SIGTERM");
      rmSync(directory, {recursive: true, force: true});
    }
  }
}, 15000);

test("running image provenance survives tag reassignment", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "image-provenance-"));
  try {
    writeFileSync(path.join(directory, "docker"), `#!/usr/bin/env bash
case "$*" in
  'ps -a --format {{.Names}}') echo coprocessor-worker ;;
  'inspect -f {{.Config.Image}} coprocessor-worker') echo mutable:tag ;;
  'inspect -f {{.Image}} coprocessor-worker') echo sha256:running-original ;;
  *'image inspect'*'mutable:tag') echo sha256:retagged-new ;;
  *'image inspect'*'sha256:running-original') echo repository@sha256:original-digest ;;
  *) exit 1 ;;
esac
`, {mode: 0o755});
    const result = Bun.spawnSync(["bash", "scripts/record-run-identity.sh"], {
      cwd: cli, env: {...process.env, PATH: `${directory}:${process.env.PATH}`, FHEVM_STATE_DIR: directory},
    });
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(result.stdout.toString()).toContain("image_coprocessor-worker=sha256:running-original repository@sha256:original-digest (mutable:tag)");
    expect(result.stdout.toString()).not.toContain("retagged-new");
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

test("the host deadline kills blocked commands and the parent retains failed baseline recovery", async () => {
  const directory = mkdtempSync(path.join(tmpdir(), "matrix-host-deadline-"));
  const trace = path.join(directory, "trace");
  const fixture = path.join(directory, "runner.sh");
  const replacement = `
bun() {
  if [[ "$1" == *consensus-inventory.ts && "$2" == show ]]; then echo 'timeout: 2s';
  else command bun "$@"; fi
}
CR_RUN_ID=deadline-fixture
cr_record() { :; }
sp_recover_suite_state() { :; }
sc_run_restores() { :; }
sc_restore_running() {
  [[ -f "$1" ]] || { echo missing-baseline >> '${trace}'; return 1; }
  cat "$1" >> '${trace}'
  if [[ ! -f '${directory}/failed-once' ]]; then touch '${directory}/failed-once'; return 1; fi
}
run_cell_body() {
  echo original-writer > "$SC_CASE_BASELINE"
  sleep 30
}
run_cell FM-TFHE-STALL stall worker pause compute-chain
[[ -f "$SC_CASE_BASELINE" ]] || exit 9
restore_case || exit 10
dispose_case || exit 11
exit 0
`;
  writeFileSync(fixture, readFileSync(path.join(cli, "scripts/run-failure-matrix.sh"), "utf8")
    .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cli}/scripts'`)
    .replace(/^main\s*$/m, () => replacement));
  const started = Date.now();
  const child = Bun.spawn(["bash", fixture], {cwd: cli, stdout: "pipe", stderr: "pipe", env: {
    ...process.env, FHEVM_STATE_DIR: directory, SC_RESTORE_LOG: path.join(directory, "restore-log"),
  }});
  try {
    expect(await child.exited, await new Response(child.stderr).text()).toBe(0);
    expect(Date.now() - started).toBeLessThan(7000);
    expect(readFileSync(trace, "utf8")).toBe("original-writer\noriginal-writer\n");
    expect(existsSync(path.join(directory, "runtime/failure-matrix/uncancelled-phase"))).toBe(false);
  } finally { child.kill("SIGTERM"); rmSync(directory, {recursive: true, force: true}); }
}, 10000);

test("normal child cleanup leaves its host supervisor alive to publish the verdict", async () => {
  const directory = mkdtempSync(path.join(tmpdir(), "matrix-child-cleanup-"));
  const fixture = path.join(directory, "runner.sh");
  const replacement = `
bun() {
  if [[ "$1" == *consensus-inventory.ts && "$2" == show ]]; then echo 'timeout: 30s';
  else command bun "$@"; fi
}
CR_RUN_ID=normal-child; CR_REVISION=abc123; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=cpu
CR_SCENARIO=three-of-three; CR_OPERATORS=3; CR_THRESHOLD=3
sp_recover_suite_state() { :; }
sc_run_restores() { :; }
sc_restore_running() { :; }
run_cell_body() {
  restore_case_resources || return 1
  cr_record_checked_pass FM-TFHE-STALL assert=fault=pass:fixture assert=safety=pass:fixture assert=liveness=pass:fixture assert=bytes=pass:fixture assert=scope=pass:fixture cleanup=ok || return 1
  echo CASE_VERDICT_PUBLISHED
}
run_cell FM-TFHE-STALL stall worker pause compute-chain
exit $?
`;
  writeFileSync(fixture, readFileSync(path.join(cli, "scripts/run-failure-matrix.sh"), "utf8")
    .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cli}/scripts'`)
    .replace(/^main\s*$/m, () => replacement));
  const child = Bun.spawn(["bash", fixture], {cwd: cli, stdout: "pipe", stderr: "pipe", env: {
    ...process.env, FHEVM_STATE_DIR: directory, SC_RESTORE_LOG: path.join(directory, "restore-log"),
    CONSENSUS_RESULTS_DIR: path.join(directory, "results"),
  }});
  try {
    expect(await child.exited, await new Response(child.stderr).text()).toBe(0);
    expect(await new Response(child.stdout).text()).toContain("CASE_VERDICT_PUBLISHED");
    expect(JSON.parse(readFileSync(path.join(directory, "results/normal-child.jsonl"), "utf8").trim()).state).toBe("PASS");
  } finally { child.kill("SIGTERM"); rmSync(directory, {recursive: true, force: true}); }
}, 10000);

test("failed phase cancellation requires a verified container stop before faults heal", () => {
  for (const verified of [false, true]) {
    const directory = mkdtempSync(path.join(tmpdir(), "phase-cancel-fallback-"));
    try {
      const trace = path.join(directory, "trace");
      writeFileSync(path.join(directory, "docker"), `#!/usr/bin/env bash
echo "$1" >> '${trace}'
case "$1" in
  stop) exit 0 ;;
  inspect) echo '${verified ? "false 0" : "true 456"}' ;;
  *) exit 1 ;;
esac
`, {mode: 0o755});
      const source = readFileSync(path.join(cli, "scripts/run-failure-matrix.sh"), "utf8")
        .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cli}/scripts'`)
        .replace(/^main\s*$/m, () => `
SP_RUNTIME_DIR='${directory}/suite-process'
sp_init
FM_PHASE_REGISTRY="$SP_PHASE_REGISTRY"
printf '%s|%s\\n' "$TEST_CONTAINER" test_token > "$SP_PHASE_REGISTRY"
sp_recover_suite_state() { :; }
sc_run_restores() { echo heal >> '${trace}'; }
ensure_test_container_resolves() { echo recreate >> '${trace}'; }
restore_case
exit $?
`);
      const file = path.join(directory, "runner.sh");
      writeFileSync(file, source);
      const result = Bun.spawnSync(["bash", file], {cwd: cli, env: {
        ...process.env, PATH: `${directory}:${process.env.PATH}`, FHEVM_STATE_DIR: directory,
        SC_RESTORE_LOG: path.join(directory, "restore-log"),
      }});
      expect(result.exitCode).toBe(1);
      const events = readFileSync(trace, "utf8").trim().split("\n");
      expect(events.slice(0, 3)).toEqual(["exec", "stop", "inspect"]);
      expect(events.includes("heal")).toBe(verified);
      expect(events.includes("recreate")).toBe(verified);
      expect(existsSync(path.join(directory, "runtime/failure-matrix/uncancelled-phase"))).toBe(!verified);
    } finally { rmSync(directory, {recursive: true, force: true}); }
  }
});


test("delegated crash and request suites receive a captured parent-owned baseline", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "matrix-delegated-baseline-"));
  const source = readFileSync(path.join(cli, "scripts/run-failure-matrix.sh"), "utf8");
  const body = source.slice(source.indexOf("run_timed_cell() {"), source.indexOf("run_cell_body() {"));
  try {
    for (const script of ["run-crash-retry-consensus.sh", "run-request-recovery.sh"]) {
      writeFileSync(path.join(directory, script), '#!/bin/bash\ncat "$SC_CASE_BASELINE"\n', {mode: 0o755});
    }
    for (const id of ["FM-TFHE-CRASH", "FM-RELAYER-CRASH", "FM-KMS-CONNECTOR-CRASH"]) {
      const result = Bun.spawnSync(["bash", "-c", `
        set -eu
        SCRIPT_DIR="$1"; export SC_CASE_BASELINE="$1/baseline"; VICTIM=1
        sc_snapshot_running() { echo original-writer; }
        cr_record() { return 1; }
        ${body}
        run_timed_cell "$2"
      `, "--", directory, id]);
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      expect(result.stdout.toString()).toBe("original-writer\n");
    }
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

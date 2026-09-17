import { expect, test } from "bun:test";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

test("a Docker command that applies the fault then fails retains recovery ownership", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "consensus-restore-"));
  try {
    for (const action of ["pause", "stop"]) {
      const result = Bun.spawnSync(["bash", "-c", `
        set -uo pipefail
        source "$SCRIPT_DIR/lib/service-control.sh"
        sc_init
        trap 'rm -f "$SC_RESTORE_LOG"' EXIT
        echo running > "$FHEVM_STATE_DIR/state"
        sc_state() { cat "$FHEVM_STATE_DIR/state"; }
        sc_kind() { echo container; }
        docker() {
          case "$1" in
            pause) echo paused > "$FHEVM_STATE_DIR/state"; return 1;;
            stop) echo stopped > "$FHEVM_STATE_DIR/state"; return 1;;
            unpause|start) echo running > "$FHEVM_STATE_DIR/state";;
            *) return 1;;
          esac
        }
        status=0
        "sc_$ACTION" victim >/dev/null 2>&1 || status=$?
        echo "fault=$status records=$(wc -l < "$SC_RESTORE_LOG")"
        sc_run_restores >/dev/null 2>&1
        echo "healed=$? state=$(sc_state victim) records=$(wc -l < "$SC_RESTORE_LOG")"
      `], {env: {...process.env, ACTION: action, FHEVM_STATE_DIR: directory,
        SCRIPT_DIR: path.resolve(import.meta.dir, "../../scripts"), REPO_ROOT: path.resolve(import.meta.dir, "../../../..")}});
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      expect(result.stdout.toString()).toBe("fault=1 records=1\nhealed=0 state=running records=0\n");
    }
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

for (const action of ["pause", "stop", "kill", "reset_restart_budget"]) {
  test(`failed ledger IO blocks ${action} before any service mutation`, () => {
    const directory = mkdtempSync(path.join(tmpdir(), "consensus-ledger-"));
    try {
      const result = Bun.spawnSync(["bash", "-c", `
        set -uo pipefail
        source "$SCRIPT_DIR/lib/service-control.sh"
        # A directory cannot receive a ledger append even when tests run as root.
        SC_RESTORE_LOG="$FHEVM_STATE_DIR"
        sc_state() { echo running; }; sc_kind() { echo container; }
        sc_identity() { echo before; }; sc_main_pid() { echo 123456; }
        docker() { echo MUTATION; }; kill() { echo MUTATION; }
        "sc_$ACTION" victim; status=$?
        [[ "$status" != 0 ]]
      `], {env: {...process.env, ACTION: action, FHEVM_STATE_DIR: directory,
        SCRIPT_DIR: path.resolve(import.meta.dir, "../../scripts"), REPO_ROOT: directory}});
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      expect(result.stdout.toString()).not.toContain("MUTATION");
    } finally { rmSync(directory, {recursive: true, force: true}); }
  });
}

test("unwritable ledger blocks mutation and failed atomic rewrite preserves all owners", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "consensus-ledger-permissions-"));
  try {
    const result = Bun.spawnSync(["bash", "-c", `
      set -uo pipefail
      source "$SCRIPT_DIR/lib/service-control.sh"
      mkdir "$FHEVM_STATE_DIR/ledger-dir"
      SC_RESTORE_LOG="$FHEVM_STATE_DIR/ledger-dir/owners"
      printf 'a|resume\\nb|start\\n' > "$SC_RESTORE_LOG"
      sc_state() { echo running; }; sc_kind() { echo container; }
      docker() { echo MUTATION; }
      if [[ $(id -u) != 0 ]]; then
        chmod 400 "$SC_RESTORE_LOG"
        sc_pause victim; [[ "$?" != 0 ]] || exit 2
        chmod 600 "$SC_RESTORE_LOG"
        chmod 500 "$(dirname "$SC_RESTORE_LOG")"
        sc_clear_restore a resume; [[ "$?" != 0 ]] || exit 3
        chmod 700 "$(dirname "$SC_RESTORE_LOG")"
        chmod 000 "$SC_RESTORE_LOG"
        sc_clear_restore a resume; [[ "$?" != 0 ]] || exit 4
        sc_run_restores; [[ "$?" != 0 ]] || exit 5
        chmod 600 "$SC_RESTORE_LOG"
      fi
      # Deterministic source read failure also runs for privileged test hosts.
      grep() { return 2; }
      sc_clear_restore a resume; [[ "$?" != 0 ]] || exit 6
      unset -f grep
      [[ "$(cat "$SC_RESTORE_LOG")" == $'a|resume\\nb|start' ]] || exit 7
      echo owners-preserved
    `], {env: {...process.env, FHEVM_STATE_DIR: directory,
      SCRIPT_DIR: path.resolve(import.meta.dir, "../../scripts"), REPO_ROOT: directory}});
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(result.stdout.toString()).toBe("owners-preserved\n");
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

test("an unreadable retained-fault ledger never permits baseline healing", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "retained-ledger-"));
  try {
    const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
source "$SCRIPT_DIR/lib/service-control.sh"
printf 'victim\\n' > "$FHEVM_STATE_DIR/baseline"
SC_RESTORE_LOG="$FHEVM_STATE_DIR"
sc_state() { echo paused; }; sc_resume() { echo UNSAFE_HEAL; }
sc_restore_running "$FHEVM_STATE_DIR/baseline" 1; [[ "$?" != 0 ]]
`], {env: {...process.env, FHEVM_STATE_DIR: directory, REPO_ROOT: directory, SCRIPT_DIR: path.resolve(import.meta.dir, "../../scripts")}});
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    expect(run.stdout.toString()).not.toContain("UNSAFE_HEAL");
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

test("an unreadable baseline is a failed restoration, never an empty successful fleet", () => {
  const scripts = path.resolve(import.meta.dir, "../../scripts");
  const result = Bun.spawnSync(["bash", "-c", `SCRIPT_DIR='${scripts}'; REPO_ROOT=/tmp
source "$SCRIPT_DIR/lib/service-control.sh"
sc_restore_running /proc/self/mem; [[ "$?" != 0 ]]
`]);
  expect(result.exitCode, result.stderr.toString()).toBe(0);
});

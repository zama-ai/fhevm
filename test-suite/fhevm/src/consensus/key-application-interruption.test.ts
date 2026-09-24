import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
for (const mode of ["success", "early-release", "orphaned-backend", "partial-publication", "parent-exit", "replacement-failed", "restore-failed"] as const) {
  test(`key application supervisor owns the private gate and recovery: ${mode}`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "key-application-owner-"));
    try {
      mkdirSync(path.join(dir, "scripts/lib"), { recursive: true });
      writeFileSync(path.join(dir, "scripts/interrupt.sh"), readFileSync(path.resolve(import.meta.dir, "../../scripts/interrupt-key-application.sh")));
      writeFileSync(path.join(dir, "scripts/lib/service-control.sh"), `
sc_init(){ touch "$SC_RESTORE_LOG"; }
hc_cleanup_signals(){ trap - EXIT; trap ':' INT TERM; }
hc_begin_cleanup(){ :; }
sc_state(){ echo running; }
sc_register_restore(){ echo "$1" >> "$SC_RESTORE_LOG"; }
sc_kill(){ echo "$1" >> "$TRACE.killed"; echo old; }
sc_wait_replaced(){ [[ "$MODE" != replacement-failed ]]; }
sc_stop(){ touch "$TRACE.stopped"; }
sc_run_restores(){ cp "$SC_RESTORE_LOG" "$TRACE.restored"; [[ "$MODE" != restore-failed ]]; }
bun(){ case "$2" in install) echo INSTALL_GATE;; waiters) echo WAITERS;; drop) echo DROP_GATE;; esac; }
docker(){
  if [[ "$*" == *' -c '* ]]; then
    local sql=""
    for sql in "$@"; do :; done
    case "$sql" in
      *to_regprocedure*) echo t;;
      *'SELECT count(*) FROM pg_locks'*) echo 0;;
      *WAITERS*)
        if [[ "$MODE" == orphaned-backend && -e "$TRACE.stopped" && ! -e "$TRACE.gate-released" ]]; then echo 1;
        elif [[ "$MODE" == parent-exit || -e "$TRACE.stopped" ]]; then echo 0;
        elif [[ "$MODE" == early-release && ! -e "$TRACE.polled" ]]; then touch "$TRACE.polled"; echo 0;
        else echo 1; fi;;
      *compressed_xof_keyset*) if [[ "$MODE" == partial-publication && -e "$TRACE.gate-released" ]]; then echo 0; else echo 1; fi;;
      *INSTALL_GATE*) echo installed >> "$TRACE";;
      *DROP_GATE*) echo removed >> "$TRACE";;
    esac
  else
    while IFS= read -r line; do
      [[ "$line" != *KEY_GATE_READY* ]] || echo KEY_GATE_READY
      [[ "$line" != *pg_advisory_unlock* ]] || echo released >> "$TRACE"
      [[ "$line" != *pg_advisory_unlock* ]] || touch "$TRACE.gate-released"
      [[ "$line" != '\\q' ]] || break
    done
  fi
}
`);
      const result = Bun.spawnSync(["bash", path.join(dir, "scripts/interrupt.sh"), "coprocessor1-gcs-host-listener", "coprocessor1-gcs-host-listener-poller", "coprocessor1-gcs-host-listener-consumer"], {
        env: { ...process.env, MODE: mode, TRACE: path.join(dir, "trace"), SC_RESTORE_LOG: path.join(dir, "restore.log"), MIGRATION_DATABASE: "coprocessor_1", MIGRATION_KEY_HEX: "ab".repeat(32) }, stdin: Buffer.from(mode === "early-release" ? "release\n" : ""), timeout: 5_000,
      });
      const succeeds = mode === "success" || mode === "early-release" || mode === "orphaned-backend";
      expect(result.exitCode, result.stderr.toString()).toBe(succeeds ? 0 : 1);
      const trace = readFileSync(path.join(dir, "trace"), "utf8");
      expect(trace).toContain("installed");
      expect(trace).toContain("released");
      expect(trace).toContain("removed");
      if (succeeds) {
        expect(readFileSync(path.join(dir, "trace.killed"), "utf8").trim().split("\n")).toHaveLength(3);
        expect(result.stdout.toString()).toContain("rollback=observed");
      } else expect(result.stdout.toString()).not.toContain("rollback=observed");
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}

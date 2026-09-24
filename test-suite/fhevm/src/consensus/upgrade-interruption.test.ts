import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

for (const mode of ["dry-run-started", "before-cutover-commit", "after-cutover-commit", "wrong-side", "parent-exit", "replacement-failed", "cleanup-failed"] as const) {
  test(`upgrade boundary supervisor is fail-closed: ${mode}`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "upgrade-interrupt-"));
    try {
      mkdirSync(path.join(dir, "scripts/lib"), { recursive: true });
      writeFileSync(path.join(dir, "scripts/interrupt.sh"), readFileSync(path.resolve(import.meta.dir, "../../scripts/interrupt-upgrade-controller.sh")));
      writeFileSync(path.join(dir, "scripts/lib/service-control.sh"), `sc_init(){ :; }
hc_cleanup_signals(){ trap - EXIT; trap ':' INT TERM; }
hc_begin_cleanup(){ :; }
sc_restart_budget(){ echo available; }
sc_kill(){ echo kill >> "$SC_RESTORE_LOG"; echo old; }
sc_identity(){ echo new; }
sc_wait_replaced(){ [[ "$MODE" != replacement-failed ]]; }
sc_run_restores(){ echo restored >> "$SC_RESTORE_LOG"; [[ "$MODE" != cleanup-failed ]]; }
docker(){
  local query="\u0024{*: -1}"
  echo "$query" >> "$SC_RESTORE_LOG.sql"
  case "$query" in
    *to_regclass*) echo t ;;
    *'SELECT reached'*) [[ "$MODE" != parent-exit ]] && echo t || echo f ;;
    *'SELECT stack_version'*)
      if [[ "$MODE" == wrong-side ]]; then echo wrong;
      elif [[ "$MODE" == after-cutover-commit ]]; then echo v0.15;
      else echo v0.14; fi ;;
  esac
  return 0
}
`);
      const result = Bun.spawnSync(["bash", path.join(dir, "scripts/interrupt.sh")], {
        env: { ...process.env, SC_RESTORE_LOG: path.join(dir, "restore.log"), MODE: mode,
          UPGRADE_FAULT_DATABASE: "coprocessor_1", UPGRADE_FAULT_VERSION: "v0.15", UPGRADE_FAULT_OLD_VERSION: "v0.14",
          BLUE_GREEN_INTERRUPT: mode.includes("commit") || mode === "dry-run-started" ? mode : "before-cutover-commit" },
        stdin: Buffer.from(""), timeout: 5_000,
      });
      expect(result.exitCode, result.stderr.toString()).toBe(["dry-run-started", "before-cutover-commit", "after-cutover-commit"].includes(mode) ? 0 : 1);
      const effects = readFileSync(path.join(dir, "restore.log"), "utf8");
      expect(effects).toContain("restored");
      expect(effects.includes("kill")).toBe(!["wrong-side", "parent-exit"].includes(mode));
      // Cleanup drops the control table so the next boundary can run on this stack.
      expect(readFileSync(path.join(dir, "restore.log.sql"), "utf8")).toContain("DROP TABLE IF EXISTS public.consensus_test_upgrade_fault");
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}

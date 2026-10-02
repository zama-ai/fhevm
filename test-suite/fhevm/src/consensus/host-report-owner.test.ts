import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

for (const mode of ["success", "unobserved", "mismatch", "parent-exit", "cleanup-failed", "interrupt", "replacement-failed", "diverge", "false-divergence", "restore-failed"]) {
  test(`host-report ownership restores after ${mode}`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "host-report-owner-"));
    try {
      mkdirSync(path.join(dir, "scripts/lib"), { recursive: true });
      writeFileSync(path.join(dir, "scripts/hold.sh"), readFileSync(path.resolve(import.meta.dir, "../../scripts/hold-host-report.sh")));
      writeFileSync(path.join(dir, "scripts/lib/service-control.sh"), [
        "sc_init(){ :; }",
        'hc_run(){ if [[ "$1" == bun ]]; then [[ "$CASE_MODE" != restore-failed ]]; else "$@"; fi; }',
        'sc_stop(){ echo stopped >> "$SC_RESTORE_LOG"; }',
        "sc_run_restores(){ :; }",
        "sc_restart_budget(){ echo available; }",
        'sc_kill(){ echo killed >> "$SC_RESTORE_LOG"; echo old; }',
        'sc_wait_replaced(){ [[ "$CASE_MODE" != replacement-failed ]]; }',
        "sc_identity(){ echo replacement; }",
        "hc_cleanup_signals(){ trap - EXIT; trap '' INT TERM; }",
        "hc_begin_cleanup(){ :; }",
        "docker(){",
        ' if [[ "$*" == *hostReportObservation.cjs* ]]; then [[ "$CASE_MODE" != false-divergence ]]; return; fi',
        " local query= db= next=0 arg",
        ' for arg in "$@"; do [[ "$next" != 1 ]] || { db="$arg"; next=0; }; [[ "$arg" != -d ]] || next=1; query="$arg"; done',
        ' case "$db" in coprocessor|coprocessor_1|coprocessor_2) ;; *) echo "unknown operator database: $db" >&2; return 1 ;; esac',
        ' echo "$db" >> "$SC_RESTORE_LOG.databases"',
        ' echo "$query" >> "$SC_RESTORE_LOG"',
        ' case "$query" in',
        ' *to_regclass*) echo t ;;',
        ' *"DROP TABLE"*) [[ "$CASE_MODE" != cleanup-failed ]] ;;',
        ' *"SELECT observed_block"*) [[ "$CASE_MODE" == unobserved ]] || echo 150 ;;',
        ' *"SELECT block_number"*) echo 160 ;;',
        ' *"SELECT s3_uploaded_at"*) if [[ "$db" == coprocessor_1 && "$CASE_MODE" != diverge && "$CASE_MODE" != false-divergence && "$CASE_MODE" != restore-failed ]]; then echo f; else echo t; fi ;;',
        ' *"SELECT state_hash"*) if [[ "$CASE_MODE" == mismatch && "$db" == coprocessor_2 ]]; then echo different; else printf "%064d\\n" 1; fi ;;',
        ' *"row_to_json"*) echo "{}" ;;',
        " esac",
        "}",
      ].join("\n"));
      const result = Bun.spawnSync(["bash", path.join(dir, "scripts/hold.sh")], {
        env: { ...process.env, CASE_MODE: mode, HOST_REPORT_MODE: ["interrupt", "replacement-failed"].includes(mode) ? "interrupt" : ["diverge", "false-divergence", "restore-failed"].includes(mode) ? "diverge" : "withhold", SC_RESTORE_LOG: path.join(dir, "restore.log"),
          UPGRADE_FAULT_VERSION: "v0.15", UPGRADE_FAULT_CHAIN: "12345", UPGRADE_OTHER_CHAIN: "67890" },
        stdin: Buffer.from(mode === "parent-exit" ? "" : "release\n"), timeout: 5_000,
      });
      expect(result.exitCode, result.stderr.toString()).toBe(["success", "interrupt", "diverge"].includes(mode) ? 0 : 1);
      if (["success", "interrupt", "diverge"].includes(mode)) {
        expect([...new Set(readFileSync(path.join(dir, "restore.log.databases"), "utf8").trim().split("\n"))].sort())
          .toEqual(["coprocessor", "coprocessor_1", "coprocessor_2"]);
      }
      expect(readFileSync(path.join(dir, "restore.log"), "utf8").includes("DROP TABLE public.consensus_test_host_report_fault")).toBe(mode !== "restore-failed");
      if (mode === "success") expect(result.stdout.toString()).toContain("HOST_REPORT_OBSERVED mode=withhold");
      if (mode === "interrupt") expect(result.stdout.toString()).toContain("DETECTOR_INTERRUPT_OBSERVED");
      expect(readFileSync(path.join(dir, "restore.log"), "utf8").includes("killed")).toBe(["interrupt", "replacement-failed"].includes(mode));
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}

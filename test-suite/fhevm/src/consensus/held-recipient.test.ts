import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

for (const mode of ["release", "parent-exit", "partial-stop-failure", "restore-failure"] as const) {
  test(`rollout recipient ownership survives ${mode}`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "held-recipient-"));
    try {
      mkdirSync(path.join(dir, "scripts/lib"), { recursive: true });
      writeFileSync(path.join(dir, "scripts/hold.sh"), readFileSync(path.resolve(import.meta.dir, "../../scripts/hold-rollout-services.sh")));
      writeFileSync(path.join(dir, "scripts/lib/service-control.sh"), `sc_init(){ touch "$SC_RESTORE_LOG"; }
hc_cleanup_signals(){ trap - EXIT; trap ':' INT TERM; }
hc_begin_cleanup(){ :; }
sc_stop(){ echo "$1" >> "$SC_RESTORE_LOG"; [[ "$MODE" != partial-stop-failure || "$1" != *consumer ]]; }
sc_run_restores(){ cp "$SC_RESTORE_LOG" "$SC_RESTORE_LOG.restored"; [[ "$MODE" != restore-failure ]]; }
`);
      const result = Bun.spawnSync(["bash", path.join(dir, "scripts/hold.sh"), "coprocessor1-gcs-host-listener", "coprocessor1-gcs-host-listener-consumer"], {
        env: { ...process.env, SC_RESTORE_LOG: path.join(dir, "restore.log"), MODE: mode },
        stdin: Buffer.from(mode === "parent-exit" ? "" : "release\n"), timeout: 5_000,
      });
      expect(result.exitCode).toBe(mode === "release" ? 0 : 1);
      expect(readFileSync(path.join(dir, "restore.log.restored"), "utf8")).toContain("coprocessor1-gcs-host-listener\n");
      if (mode === "partial-stop-failure") expect(result.stdout.toString()).not.toContain("ROLLOUT_HOLD_READY");
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}

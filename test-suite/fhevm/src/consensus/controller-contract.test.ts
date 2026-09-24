import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import path from "node:path";

const source = readFileSync(path.resolve(import.meta.dir, "../../scripts/run-stackless-cases.sh"), "utf8");
const counters = source.slice(source.indexOf("cargo_test_count()"), source.indexOf("# record_case <case-id>"));
for (const outcome of ["ok", "ignored", "FAILED", "absent"]) {
  test(`controller crash evidence cannot be replaced by unrelated passes (${outcome})`, () => {
    const log = `${outcome === "absent" ? "" : `test tests::cutover_recovers_after_process_kill ... ${outcome}\n`}test result: ok. 45 passed; 0 failed; 1 ignored; 0 measured; 1 filtered out;`;
    const child = Bun.spawnSync(["bash", "-c", `${counters}\ncontroller_crash_test_count "$LOG"`], { env: { ...process.env, LOG: log } });
    expect(child.exitCode).toBe(0);
    expect(child.stdout.toString().trim()).toBe(outcome === "ok" ? "45" : "0");
  });
}

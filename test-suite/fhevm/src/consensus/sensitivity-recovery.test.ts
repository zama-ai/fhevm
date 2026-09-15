import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, symlinkSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
const scripts = path.resolve(import.meta.dir, "../../scripts");
for (const abort of [false, true]) test(`sensitivity owns failed restoration through EXIT (abort=${abort})`, async () => {
  const dir = mkdtempSync(path.join(tmpdir(), "sensitivity-recovery-"));
  try {
    mkdirSync(path.join(dir, "bin"));
    symlinkSync(path.join(scripts, "lib"), path.join(dir, "lib"));
    writeFileSync(path.join(dir, "state"), "running\n");
    writeFileSync(path.join(dir, "bin/docker"), `#!/bin/bash
case "$1" in
 stop) echo stopped > "$FIXTURE/state";;
 start) echo attempt >> "$FIXTURE/restarts"; exit 1;;
 inspect) if [[ "$3" == '{{.State.Status}}' ]]; then cat "$FIXTURE/state"; else echo 0; fi;;
 *) exit 1;;
esac
`, {mode: 0o755});
    writeFileSync(path.join(dir, "run-failure-matrix.sh"), `#!/bin/bash
trap 'exit 143' TERM
touch "$FIXTURE/child-started"
${abort ? 'sleep 30 & wait' : `mkdir -p "$CONSENSUS_RESULTS_DIR"
echo '{"caseId":"FM-ZKPROOF-CRASH","state":"INVALID","detail":"target stopped"}' > "$CONSENSUS_RESULTS_DIR/control.jsonl"
exit 1`}
`, {mode: 0o755});
    const source = readFileSync(path.join(scripts, "consensus-sensitivity.sh"), "utf8").replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${dir}'`);
    writeFileSync(path.join(dir, "runner.sh"), source);
    const child = Bun.spawn(["bash", path.join(dir, "runner.sh"), "--control", "no-live-target"], {env: {...process.env, FIXTURE: dir, FHEVM_STATE_DIR: dir, SC_RESTORE_LOG: path.join(dir, "owners"), PATH: `${dir}/bin:${process.env.PATH}`}, stdout: "pipe", stderr: "pipe"});
    if (abort) {
      for (let i=0; i<100; i++) {
        try { readFileSync(path.join(dir, "child-started")); break; } catch { await Bun.sleep(10); }
      }
      child.kill("SIGTERM");
    }
    const status = await child.exited;
    const output = await new Response(child.stdout).text();
    expect(status).not.toBe(0);
    expect(output).not.toContain("every control behaved as required");
    expect(readFileSync(path.join(dir, "owners"), "utf8")).toContain("coprocessor1-zkproof-worker|start");
    expect(readFileSync(path.join(dir, "restarts"), "utf8").trim().split("\n").length).toBeGreaterThanOrEqual(abort ? 1 : 2);
  } finally { rmSync(dir, {recursive: true, force: true}); }
}, 10000);

import { expect, test } from "bun:test";
import path from "node:path";

const cliDir = path.resolve(import.meta.dir, "../..");
const run = (body: string) => Bun.spawnSync(["bash", "-c", `
set -eu
SCRIPT_DIR="$PWD/scripts"
REPO_ROOT="$(cd ../.. && pwd)"
source scripts/lib/service-control.sh
sc_init
snapshot="$(mktemp)"
trap 'rm -f "$snapshot" "$SC_RESTORE_LOG"' EXIT
${body}
`], { cwd: cliDir });

test("recovery includes logical GPU owners and excludes previously stopped services", () => {
  const r = run(`
docker() { printf '%s\\n' coprocessor-tfhe-worker coprocessor-sns-worker coprocessor-host-listener; }
sc_state() { case "$1" in coprocessor-host-listener) echo stopped;; *) echo running;; esac; }
sc_snapshot_running > "$snapshot"
sc_state() { echo stopped; }
sc_start() { printf 'restore %s\\n' "$1"; }
sc_restore_running "$snapshot"
`);
  expect(r.exitCode).toBe(0);
  expect(r.stdout.toString().trim().split("\n")).toEqual(["restore coprocessor-tfhe-worker", "restore coprocessor-sns-worker"]);
});

test("collateral recovery leaves deliberate faults in place until final cleanup", () => {
  const r = run(`
printf '%s\\n' coprocessor-host-listener coprocessor-tfhe-worker > "$snapshot"
sc_register_restore coprocessor-host-listener start
sc_state() { echo stopped; }
sc_start() { echo "$1"; }
sc_restore_running "$snapshot" 1
`);
  expect(r.exitCode).toBe(0);
  expect(r.stdout.toString().trim()).toBe("coprocessor-tfhe-worker");
});

test("a failed collateral restart fails recovery", () => {
  const r = run(`
echo coprocessor-tfhe-worker > "$snapshot"
sc_state() { echo stopped; }
sc_start() { return 1; }
sc_restore_running "$snapshot"
`);
  expect(r.exitCode).toBe(1);
});

test("unreadable fleet enumeration cannot create an empty recovery snapshot", () => {
  const r = run(`docker() { return 1; }; sc_snapshot_running > "$snapshot"`);
  expect(r.exitCode).toBe(1);
});

test("CI planning emits a CPU matrix without GPU or stackless jobs", () => {
  const r = Bun.spawnSync([process.execPath, "scripts/consensus-inventory.ts", "plan", "full", "--format", "github"], { cwd: cliDir });
  expect(r.exitCode).toBe(0);
  const values = new Map(r.stdout.toString().trim().split("\n").map((line) => {
    const split = line.indexOf("="); return [line.slice(0, split), line.slice(split + 1)];
  }));
  const jobs = JSON.parse(values.get("cpu-matrix")!).include;
  expect(jobs.length).toBeGreaterThan(0);
  expect(jobs.every((job: {timeoutMinutes: number}) => job.timeoutMinutes <= 360)).toBe(true);
  const shards = jobs.filter((job: {leg: string; scenario: string}) => job.leg === 'failure-matrix' && job.scenario === 'three-of-three');
  expect(new Set(shards.map((job: {shard: string}) => job.shard)).size).toBe(3);
  expect(jobs.every((job: { leg: string; scenario: string }) => job.leg !== "gpu" && job.scenario !== "none")).toBe(true);
  expect(values.get("needs-two-gpus")).toBe("false");
});

test("replacement evidence rejects transient PID zero and waits for a stable process", () => {
  const r = run(`
sc_identity() {
  local n; n="$(cat "$snapshot")"; echo $((n + 1)) > "$snapshot"
  case "$n" in
    0) echo 'container=worker pid=0 started=new restarts=1';;
    1) echo 'container=worker pid=11 started=new restarts=1';;
    *) echo 'container=worker pid=12 started=stable restarts=2';;
  esac
}
echo 0 > "$snapshot"
sc_state() { echo running; }
sleep() { :; }
sc_wait_replaced worker 'container=worker pid=10 started=old restarts=0' 5
`);
  expect(r.exitCode).toBe(0);
  expect(r.stdout.toString()).toBe("container=worker pid=12 started=stable restarts=2");
});


test("operator outages include every host chain without selecting other operators or setup jobs", () => {
  const r = run(`
docker() { printf '%s\\n' coprocessor2-host-listener coprocessor2-host-listener-chain-b coprocessor2-host-listener-poller-chain-b coprocessor2-host-listener-consumer coprocessor2-tfhe-worker coprocessor-host-listener-chain-b coprocessor2-db-migration; }
sc_operator_services 2
`);
  expect(r.exitCode).toBe(0);
  expect(r.stdout.toString().trim().split("\n")).toEqual([
    "coprocessor2-host-listener", "coprocessor2-host-listener-chain-b",
    "coprocessor2-host-listener-consumer", "coprocessor2-host-listener-poller-chain-b", "coprocessor2-tfhe-worker",
  ]);
});


test("a dependency reaction requires a failure level on the matching log line", () => {
  const r = run(`
if sc_logs_show_failure '{"level":"INFO","message":"database connection ready"}' 'database|pool'; then exit 1; fi
if sc_logs_show_failure $'{"level":"INFO","message":"database ready"}\\n{"level":"ERROR","message":"telemetry unavailable"}' 'database|pool'; then exit 1; fi
sc_logs_show_failure '{"level":"ERROR","message":"database connection refused"}' 'database|pool'
sc_logs_show_failure '2026-09-12 WARN pool acquisition timed out' 'database|pool'
`);
  expect(r.exitCode).toBe(0);
});


test("automatic operator recovery cannot be supplied by collateral cleanup", () => {
  const r = run(`
printf '%s\\n' coprocessor-tfhe-worker coprocessor1-host-listener kms-connector-gw-listener > "$snapshot"
sc_state() { echo stopped; }
sc_start() { echo "$1"; }
sc_restore_running "$snapshot" 1 1
`);
  expect(r.exitCode).toBe(0);
  expect(r.stdout.toString().trim()).toBe("kms-connector-gw-listener");
});

test("degraded and fork abort paths restore their registered faults and preserve failure", async () => {
  const { mkdtempSync, readFileSync, writeFileSync, rmSync } = await import("node:fs");
  const { tmpdir } = await import("node:os");
  const dir = mkdtempSync(path.join(tmpdir(), "consensus-abort-"));
  try {
    for (const name of ["run-degraded-consensus.sh", "run-fork-consensus.sh"]) {
      const source = readFileSync(path.join(cliDir, "scripts", name), "utf8")
        .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cliDir}/scripts'`)
        .replace(/^main\s*$/m, () => `sc_resume() { echo "restored $1"; }\nsc_register_restore victim resume\nkill -TERM $$`);
      const file = path.join(dir, name);
      writeFileSync(file, source);
      const result = Bun.spawnSync(["bash", file], {cwd: cliDir});
      expect(result.exitCode, result.stderr.toString()).toBe(143);
      expect(result.stdout.toString()).toContain("restored victim");
    }
  } finally { rmSync(dir, {recursive:true, force:true}); }
});

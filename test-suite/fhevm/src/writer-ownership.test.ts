import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

// Run against a fake Docker executable in a separate process so module mocks and
// state-directory overrides cannot affect the rest of the CLI suite.
const exercise = (state: string, script: string, env: Record<string, string> = {}) => {
  const dir = mkdtempSync(path.join(tmpdir(), "writer-ownership-"));
  try {
    const bin = path.join(dir, "bin");
    mkdirSync(bin);
    writeFileSync(path.join(dir, "state"), state);
    writeFileSync(path.join(dir, "calls"), "");
    writeFileSync(path.join(bin, "docker"), `#!/bin/sh
if [ "$DETAILED_CALLS" = 1 ]; then printf '%s\\n' "$*"; else printf '%s\\n' "$1"; fi >> "$FAKE_ROOT/calls"
case "$1" in
  inspect)
    if [ "$INSPECT_ERROR" = 1 ]; then echo 'daemon unavailable' >&2; exit 1; fi
    if [ -n "$INSPECT_ERROR_FOR" ]; then case "$*" in *"$INSPECT_ERROR_FOR"*) echo 'temporary inspect failure' >&2; exit 1;; esac; fi
    cat "$FAKE_ROOT/state" ;;
  stop)
    if [ "$STOP_NOOP" != 1 ]; then echo exited > "$FAKE_ROOT/state"; fi ;;
  start)
    if [ "$START_NOOP" != 1 ]; then echo running > "$FAKE_ROOT/state"; fi ;;
  *) exit 2 ;;
esac
`, { mode: 0o755 });
    if (env.GPU_SESSION === "1") {
      mkdirSync(path.join(dir, "runtime/gpu-consensus-workers"), {recursive: true});
      writeFileSync(path.join(dir, "runtime/gpu-consensus-workers/node-config.env"), "");
      writeFileSync(path.join(bin, "systemctl"), `#!/bin/sh
if [ "$GPU_PAUSED" = 1 ]; then state=active; pid=500; else state=inactive; pid=0; fi
case "$*" in *--value*) echo "$state";; *) printf 'ActiveState=%s\\nMainPID=%s\\n' "$state" "$pid";; esac
`, {mode: 0o755});
      writeFileSync(path.join(bin, "cat"), `#!/bin/sh
if [ "$1" = /proc/500/stat ]; then echo '500 (tfhe_worker) T 1'; else exec /bin/cat "$@"; fi
`, {mode: 0o755});
    }
    const module = path.join(import.meta.dir, "flow", "writer-ownership.ts");
    const result = Bun.spawnSync([process.execPath, "-e", `
      import assert from 'node:assert/strict';
      import { recordWriterOwnership, quiesceWriters, restoreWriters } from ${JSON.stringify(module)};
      ${script}
    `], {
      env: { ...process.env, ...env, FAKE_ROOT: dir, FHEVM_STATE_DIR: dir, PATH: `${bin}:${process.env.PATH}` },
    });
    expect(result.stderr.toString()).toBe("");
    expect(result.exitCode).toBe(0);
    return readFileSync(path.join(dir, "calls"), "utf8").trim().split("\n");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
};

test("unreadable Docker state cannot be treated as a stopped writer", () => {
  const calls = exercise("running", `
    await assert.rejects(recordWriterOwnership(['coprocessor-tfhe-worker']), /Cannot determine container/);
  `, { INSPECT_ERROR: "1" });
  expect(calls).toEqual(["inspect"]);
});

test("a successful stop command must actually quiesce the writer", () => {
  exercise("running", `
    const owners = await recordWriterOwnership(['coprocessor-tfhe-worker']);
    await assert.rejects(quiesceWriters(owners), /still live/);
  `, { STOP_NOOP: "1" });
});

test("writers that were stopped remain stopped through restoration", () => {
  const calls = exercise("exited", `
    const owners = await recordWriterOwnership(['coprocessor-tfhe-worker']);
    await quiesceWriters(owners);
    await restoreWriters(owners);
  `);
  expect(calls).not.toContain("start");
  expect(calls).not.toContain("stop");
});

test("a successful start command must actually restore the writer", () => {
  exercise("running", `
    const owners = await recordWriterOwnership(['coprocessor-tfhe-worker']);
    await quiesceWriters(owners);
    await assert.rejects(restoreWriters(owners), /Could not restore/);
  `, { START_NOOP: "1" });
});


test("GPU ownership refuses a running displaced container before SQL", () => {
  const calls = exercise("running", `
    await assert.rejects(recordWriterOwnership(['coprocessor-tfhe-worker']), /live Docker writer/);
  `, {GPU_SESSION: "1"});
  expect(calls).toEqual(["inspect"]);
});

test("a deliberately SIGSTOP-paused GPU owner is refused before any service mutation", () => {
  const calls = exercise("exited", `
    await assert.rejects(recordWriterOwnership(['coprocessor-tfhe-worker']), /while it is paused/);
  `, {GPU_SESSION: "1", GPU_PAUSED: "1"});
  expect(calls).toEqual(["inspect"]);
});

test("an inactive GPU owner stays absent in the recovery snapshot", () => {
  exercise("exited", `
    const owners = await recordWriterOwnership(['coprocessor-tfhe-worker']);
    assert.equal(owners[0].wasRunning, false);
  `, {GPU_SESSION: "1"});
});

test("quiescence independently rechecks displaced Docker owners after recording ownership", () => {
  exercise("running", `
    await assert.rejects(quiesceWriters([{container:'coprocessor-tfhe-worker', wasRunning:false,
      gpu:{kind:'tfhe',index:'0',unit:'fhevm-gpu-consensus-tfhe-0'}}]), /still live/);
  `, {GPU_SESSION: "1"});
});

test("restoration attempts later writers after an earlier inspection fails, but fails overall", () => {
  const calls = exercise("running", `
    await assert.rejects(restoreWriters([
      {container:'writer-a', wasRunning:true}, {container:'writer-b', wasRunning:true},
    ]), /writer-a.*temporary inspect failure/);
  `, {INSPECT_ERROR_FOR:"writer-a", DETAILED_CALLS:"1"});
  expect(calls).toContain("start writer-b");
  expect(calls).toContain("inspect -f {{.State.Status}} writer-b");
});

test("ownership verification checks later GPU queues after an earlier inspection fails", () => {
  const calls = exercise("exited", `
    await assert.rejects(restoreWriters([0,1].map(index => ({
      container:'coprocessor'+index+'-tfhe-worker', wasRunning:false,
      gpu:{kind:'tfhe', index:String(index), unit:'fhevm-gpu-consensus-tfhe-'+index},
    }))), /coprocessor0-tfhe-worker ownership verification.*temporary inspect failure/);
  `, {INSPECT_ERROR_FOR:"coprocessor0-", DETAILED_CALLS:"1"});
  expect(calls).toContain("inspect -f {{.State.Status}} coprocessor1-tfhe-worker");
});

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

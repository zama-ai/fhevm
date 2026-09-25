import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import os from "node:os";
import path from "node:path";
const cli = path.resolve(import.meta.dir, "../..");
test("source identity excludes only the exact generated file and fails closed on Git errors", () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "source-revision-"));
  const run = (...args: string[]) => Bun.spawnSync(args, { cwd: root });
  try {
    mkdirSync(path.join(root, "test-suite/e2e/contracts"), { recursive: true });
    const generated = path.join(root, "test-suite/e2e/contracts/E2ECoprocessorConfigLocal.sol");
    writeFileSync(generated, "original"); writeFileSync(path.join(root, "source.ts"), "original");
    expect(run("git", "init", "-q").exitCode).toBe(0);
    expect(run("git", "add", ".").exitCode).toBe(0);
    expect(run("git", "-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid", "commit", "-qm", "fixture").exitCode).toBe(0);
    const revision = () => Bun.spawnSync(["bash", "-c", 'source "$1"; sr_revision "$2"', "fixture", path.join(cli, "scripts/lib/source-revision.sh"), root]);
    const clean = revision().stdout.toString().trim(); expect(clean).toMatch(/^[a-f0-9]{40}$/);
    writeFileSync(generated, "regenerated"); expect(revision().stdout.toString().trim()).toBe(clean);
    writeFileSync(`${generated}.backup`, "untracked source"); expect(revision().stdout.toString().trim()).toBe(`${clean}-dirty`);
    rmSync(`${generated}.backup`); writeFileSync(path.join(root, "source.ts"), "modified");
    expect(revision().stdout.toString().trim()).toBe(`${clean}-dirty`);
    writeFileSync(path.join(root, "source.ts"), "original"); expect(revision().stdout.toString().trim()).toBe(clean);
    rmSync(path.join(root, ".git"), { recursive: true }); expect(revision().exitCode).not.toBe(0);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

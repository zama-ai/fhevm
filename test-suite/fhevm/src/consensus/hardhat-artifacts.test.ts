import { expect, test } from "bun:test";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { REPO_ROOT } from "../layout";

const e2e = path.join(REPO_ROOT, "test-suite/e2e");
// Drives the real Hardhat CLI, so it needs the e2e package's dependencies
// (`npm ci` in test-suite/e2e). A CLI-only checkout skips rather than fails.
const hardhatInstalled = existsSync(path.join(e2e, "node_modules/hardhat/internal/cli/cli.js"));

test.skipIf(!hardhatInstalled)("Hardhat --no-compile preserves prebuilt artifacts during concurrent test invocations", async () => {
  const root = mkdtempSync(path.join(tmpdir(), "consensus-hardhat-artifacts-"));
  try {
    symlinkSync(path.join(e2e, "node_modules"), path.join(root, "node_modules"), "dir");
    symlinkSync(path.join(e2e, "test"), path.join(root, "test"), "dir");
    for (const file of ["hardhat.config.ts", "tsconfig.json"]) copyFileSync(path.join(e2e, file), path.join(root, file));
    writeFileSync(path.join(root, "package.json"), JSON.stringify({ private: true }));
    const artifactDir = path.join(root, "artifacts/contracts/Fixture.sol");
    mkdirSync(artifactDir, { recursive: true });
    const artifact = JSON.stringify({
      _format: "hh-sol-artifact-1", contractName: "Fixture", sourceName: "contracts/Fixture.sol",
      abi: [], bytecode: "0x", deployedBytecode: "0x", linkReferences: {}, deployedLinkReferences: {},
    });
    const artifactPath = path.join(artifactDir, "Fixture.json");
    writeFileSync(artifactPath, artifact);
    writeFileSync(path.join(root, "probe.js"), `
      const assert = require('node:assert/strict');
      const { artifacts } = require('hardhat');
      describe('precompiled fixture', () => {
        it('remains available to the test', async () => {
          assert.equal((await artifacts.readArtifact('Fixture')).contractName, 'Fixture');
        });
      });
    `);
    const supported = Bun.spawnSync(["node", "-p", "process.allowedNodeEnvironmentFlags.has('--no-experimental-strip-types')"]);
    expect(supported.exitCode).toBe(0);
    const env = { ...process.env, CONSENSUS_WATCHDOG_DISABLED: "1", TS_NODE_TRANSPILE_ONLY: "true",
      NODE_OPTIONS: [process.env.NODE_OPTIONS ?? "", supported.stdout.toString().trim() === "true" ? "--no-experimental-strip-types" : ""].filter(Boolean).join(" ") };
    const results = await Promise.all([0, 1].map(async () => {
      const child = Bun.spawn(["node", path.join(e2e, "node_modules/hardhat/internal/cli/cli.js"),
        "test", "probe.js", "--no-compile", "--network", "staging"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
      const [code, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
      return { code, output: stdout + stderr };
    }));
    for (const result of results) {
      expect(result.output).toContain("1 passing");
      expect(result.code).toBe(0);
      expect(result.output).not.toContain("Nothing to compile");
    }
    expect(readFileSync(artifactPath, "utf8")).toBe(artifact);
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 60_000);

#!/usr/bin/env bun
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { REPO_ROOT } from "../src/layout";
import { verifyComparatorContracts } from "../src/consensus/comparator-contracts";

const directory = mkdtempSync(path.join(tmpdir(), "comparator-contracts-"));
try {
  const output = path.join(directory, "mocha.json");
  const cwd = path.join(REPO_ROOT, "test-suite/e2e");
  const files = [...new Set([
    "test/consensus/comparator.test.ts", "test/consensus/canary.test.ts", "test/consensus/rawCanary.test.ts", "test/consensus/helpers.test.ts",
    ...new Bun.Glob("test/consensus/*.test.ts").scanSync({ cwd }),
  ])];
  // Recent Node 22 releases can load .ts as native ESM before ts-node's
  // CommonJS hook handles it. Keep tsconfig/ts-node in charge of these tests.
  // Older Node versions have no stripping flag and need no opt-out.
  const supportsOptOut = Bun.spawnSync(["node", "-p",
    "process.allowedNodeEnvironmentFlags.has('--no-experimental-strip-types')"], { stdout: "pipe", stderr: "pipe" });
  if (supportsOptOut.exitCode !== 0) throw new Error("cannot inspect the Mocha Node runtime");
  const nodeOptions = [process.env.NODE_OPTIONS ?? "",
    supportsOptOut.stdout.toString().trim() === "true" ? "--no-experimental-strip-types" : ""].filter(Boolean).join(" ");
  const child = Bun.spawn(["npx", "--no-install", "mocha", "--require", "ts-node/register",
    "--reporter", "json", "--reporter-option", `output=${output}`,
    ...files], {
    cwd, env: { ...process.env, NODE_OPTIONS: nodeOptions, TS_NODE_TRANSPILE_ONLY: "true" },
    stdout: "inherit", stderr: "inherit", timeout: 180_000,
  });
  if (await child.exited !== 0) throw new Error("comparator contract suite failed");
  const count = verifyComparatorContracts(JSON.parse(readFileSync(output, "utf8")));
  console.log(`${count} passing (required named comparator and canary contracts verified)`);
} finally { rmSync(directory, { recursive: true, force: true }); }

import { describe, expect, test } from "bun:test";
import { existsSync } from "node:fs";
import path from "node:path";

import { createRolloutContext } from "./commands/rollout-run";
import { UPGRADEABLE_GROUPS } from "./flow/repair";

const ROOT = path.resolve(import.meta.dir, "..");
const DOCS_PATH = path.join(ROOT, "rollouts", "README.md");

describe("rollout documentation", () => {
  test("references only current rollout context operations", async () => {
    const docs = await Bun.file(DOCS_PATH).text();
    const documented = [...new Set([...docs.matchAll(/\bctx\.([A-Za-z][A-Za-z0-9]*)/g)].map((match) => match[1]))];
    const available = Object.keys(createRolloutContext()).sort();

    expect(documented.sort()).toEqual(available);
  });

  test("keeps checked-in runbook links valid", async () => {
    const docs = await Bun.file(DOCS_PATH).text();
    const runbooks = [...docs.matchAll(/\]\(\.\/([^\s)]+\/run\.ts)\)/g)].map((match) => match[1]);

    expect(runbooks.length).toBeGreaterThan(0);
    for (const runbook of runbooks) {
      expect(existsSync(path.join(ROOT, "rollouts", runbook))).toBe(true);
    }
  });

  test("keeps runtime groups and per-node limitations current", async () => {
    const docs = await Bun.file(DOCS_PATH).text();
    const groupSentence = docs.match(/Supported runtime groups are ([^.]+)\./)?.[1];
    const documentedGroups = [...(groupSentence ?? "").matchAll(/`([^`]+)`/g)].map((match) => match[1]);

    expect(documentedGroups.sort()).toEqual([...UPGRADEABLE_GROUPS].sort());

    const perNodeLimitation = "The current API upgrades whole runtime groups; it does not upgrade individual KMS nodes.";
    expect(docs.includes(perNodeLimitation)).toBe(!("upgradeKmsNodes" in createRolloutContext()));
  });
});

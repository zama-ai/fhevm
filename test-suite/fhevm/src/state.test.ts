import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { describe, expect, test } from "bun:test";

import { presetBundle } from "./resolve/target";
import { DEFAULT_KMS_TOPOLOGY } from "./scenario/resolve";
import { stackSpecForState } from "./stack-spec/stack-spec";
import { loadState } from "./state/state";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";

const writeLegacyState = (): string => {
  // A state.json written before the `kms` block existed: scenario has no `kms` field.
  const scenario = testDefaultScenario();
  delete (scenario as { kms?: unknown }).kms;
  const legacy = {
    target: "latest-main",
    requiresGitHub: true,
    versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
    overrides: [],
    scenario,
    completedSteps: [],
    updatedAt: "2026-01-01T00:00:00.000Z",
  };
  const file = path.join(fs.mkdtempSync(path.join(os.tmpdir(), "fhevm-state-")), "state.json");
  fs.writeFileSync(file, JSON.stringify(legacy));
  return file;
};

describe("loadState backward compatibility", () => {
  test("back-fills a missing scenario.kms with the centralized default", async () => {
    const loaded = (await loadState(writeLegacyState())) as State;
    expect(loaded.scenario.kms).toEqual(DEFAULT_KMS_TOPOLOGY);
  });

  test("resume/teardown can rebuild a StackSpec from a pre-kms state without crashing", () => {
    return loadState(writeLegacyState()).then((loaded) => {
      const spec = stackSpecForState(loaded as State);
      expect(spec.kms.mode).toBe("centralized");
      expect(spec.kms.parties).toBe(1);
    });
  });

  test("folds a legacy discovery.kmsSigner into the kmsSigners array", async () => {
    const scenario = testDefaultScenario();
    const legacy = {
      target: "latest-main",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides: [],
      scenario,
      // Discovery shape from before kmsSigners replaced the single kmsSigner.
      discovery: { gateway: {}, hosts: {}, kmsSigner: "0xabc", endpoints: {} },
      completedSteps: [],
      updatedAt: "2026-01-01T00:00:00.000Z",
    };
    const file = path.join(fs.mkdtempSync(path.join(os.tmpdir(), "fhevm-state-")), "state.json");
    fs.writeFileSync(file, JSON.stringify(legacy));
    const loaded = (await loadState(file)) as State;
    expect(loaded.discovery?.kmsSigners).toEqual(["0xabc"]);
  });
});

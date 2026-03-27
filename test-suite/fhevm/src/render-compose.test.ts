import path from "node:path";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { describe, expect, test } from "bun:test";
import YAML from "yaml";

import { COMPOSE_OUT_DIR, STATE_DIR, composePath, envPath } from "./layout";
import { generateComposeOverrides } from "./generate/compose";
import { presetBundle } from "./resolve/target";
import { parseCoprocessorScenario, resolveScenarioFile } from "./scenario/resolve";
import { stackSpecForState } from "./stack-spec/stack-spec";
import type { State } from "./types";

const scenario = resolveScenarioFile(
  path.join("/tmp", "two-of-two-local.yaml"),
  parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
instances:
  - index: 1
    source:
      mode: local
    localServices:
      - host-listener
`),
);

const state: State = {
  target: "latest-main",
  lockPath: "/tmp/latest-main.json",
  requiresGitHub: true,
  versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
  overrides: [],
  scenario,
  completedSteps: [],
  updatedAt: "2026-03-19T00:00:00.000Z",
};

const inheritedScenarioState: State = {
  ...state,
  overrides: [{ group: "coprocessor" }],
  scenario: resolveScenarioFile(
    path.join("/tmp", "two-of-two-inherit.yaml"),
    parseCoprocessorScenario(`
version: 1
kind: coprocessor-consensus
topology:
  count: 2
  threshold: 2
`),
  ),
};

describe("render-compose", () => {
  test("renders multi-instance coprocessor overrides with local poller siblings", async () => {
    await rm(STATE_DIR, { recursive: true, force: true });
    await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
    await writeFile(envPath("coprocessor"), "\n");
    await writeFile(envPath("coprocessor.1"), "\n");
    try {
      await generateComposeOverrides(state, stackSpecForState(state));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { image?: string }>;
      };
      expect(Object.keys(doc.services)).toContain("coprocessor1-host-listener");
      expect(Object.keys(doc.services)).toContain("coprocessor1-host-listener-poller");
      expect(doc.services["coprocessor1-host-listener"]?.image).toContain(":fhevm-local-i1");
      expect(doc.services["coprocessor1-host-listener-poller"]?.image).toContain(":fhevm-local-i1");
    } finally {
      await rm(COMPOSE_OUT_DIR, { recursive: true, force: true });
      await rm(STATE_DIR, { recursive: true, force: true });
    }
  });

  test("renders inherited two-of-two instances with local build tags when coprocessor build is active", async () => {
    await rm(STATE_DIR, { recursive: true, force: true });
    await mkdir(path.dirname(envPath("coprocessor")), { recursive: true });
    await writeFile(envPath("coprocessor"), "\n");
    await writeFile(envPath("coprocessor.1"), "\n");
    try {
      await generateComposeOverrides(inheritedScenarioState, stackSpecForState(inheritedScenarioState));
      const doc = YAML.parse(await readFile(composePath("coprocessor"), "utf8")) as {
        services: Record<string, { image?: string; build?: unknown }>;
      };
      expect(doc.services["coprocessor-host-listener"]?.image).toContain(":fhevm-local-i0");
      expect(doc.services["coprocessor1-host-listener"]?.image).toContain(":fhevm-local-i1");
      expect(doc.services["coprocessor-host-listener"]?.build).toBeTruthy();
      expect(doc.services["coprocessor1-host-listener"]?.build).toBeTruthy();
    } finally {
      await rm(COMPOSE_OUT_DIR, { recursive: true, force: true });
      await rm(STATE_DIR, { recursive: true, force: true });
    }
  });
});

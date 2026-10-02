import path from "node:path";
import { mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import os from "node:os";
import { expect, test } from "bun:test";
import YAML from "yaml";

import { prepareDegradedScenario, serializeDegradedScenario } from "../../scripts/prepare-degraded-scenario";
import { renderEnvMaps } from "../generate/env";
import { COMPONENTS, TEMPLATE_ENV_DIR } from "../layout";
import { presetBundle } from "../resolve/target";
import { loadCoprocessorScenario, parseCoprocessorScenario, resolveScenarioFile } from "../scenario/resolve";
import { stackSpecForState } from "../stack-spec/stack-spec";
import type { CoprocessorScenario, State } from "../types";
import { readEnvFile } from "../utils/fs";

const cliRoot = path.resolve(import.meta.dir, "../..");
const authoredScenario = `
version: 1
kind: coprocessor-consensus
name: Custom degraded sources
description: Preserve authored source selections and scheduling overrides.
topology: { count: 3, threshold: 3 }
hostChains:
  - { key: host, chainId: "12345", rpcPort: 8545 }
kms: { mode: centralized, fheParams: Test }
instances:
  - index: 0
    source: { mode: registry, tag: abcdef0 }
    args:
      tfhe-worker: [--work-items-batch-size=100]
    env: { UNRELATED: kept, DRIFT_AUTO_REVERT_ENABLED: "true" }
  - index: 1
    source: { mode: local }
    localServices: [tfhe-worker, host-listener]
    args:
      tfhe-worker: [--work-items-batch-size=1]
    env: { FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION: "false" }
  - index: 2
    source: { mode: inherit }
    env: { DRIFT_AUTO_REVERT_ENABLED: "false" }
`;
const deriveWallet = async (_mnemonic: string, index: number) => ({
  address: `0x${String(index + 1).padStart(40, "1")}`,
  privateKey: `0x${String(index + 1).padStart(64, "2")}`,
});

const renderedDriftSettings = async (scenario: CoprocessorScenario) => {
  const templates = Object.fromEntries(await Promise.all(COMPONENTS.map(async (component) => [
    component, await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
  ])));
  const state: State = {
    target: "latest-main", lockPath: "/tmp/latest-main.json", requiresGitHub: true,
    versions: presetBundle("latest-main", "abcdef0", "latest-main.json"), overrides: [],
    scenario: resolveScenarioFile("/tmp/consensus-degraded.yaml", scenario),
    completedSteps: [], updatedAt: "2026-09-13T00:00:00Z",
  };
  const maps = await renderEnvMaps({}, stackSpecForState(state), templates, deriveWallet);
  return [maps.componentEnvs.coprocessor, maps.instanceEnvs["coprocessor.1"], maps.instanceEnvs["coprocessor.2"]]
    .map((env) => env.DRIFT_AUTO_REVERT_ENABLED);
};

for (const name of ["two-of-three", "three-of-three"]) {
  test(`${name} disables auto-revert for DEG06 on every generated instance while preserving ordinary defaults`, async () => {
    const input = await loadCoprocessorScenario(name);
    expect(await renderedDriftSettings(input)).toEqual(["true", "true", "true"]);
    expect(await renderedDriftSettings(prepareDegradedScenario(input))).toEqual(["false", "false", "false"]);
    expect(await renderedDriftSettings(input)).toEqual(["true", "true", "true"]);
  });
}

test("degraded YAML roundtrip preserves authored sources, local services, arguments and unrelated env", () => {
  const input = parseCoprocessorScenario(authoredScenario);
  const before = structuredClone(input);
  const prepared = prepareDegradedScenario(input);
  const serialized = serializeDegradedScenario(input);
  const reloaded = parseCoprocessorScenario(serialized);
  expect(input).toEqual(before);
  expect(reloaded).toEqual(prepared);
  expect(YAML.parse(serialized).instances[1].localServices).toEqual([
    "tfhe-worker", "host-listener", "host-listener-poller",
  ]);
  expect(reloaded.instances?.[1]?.localServices).toEqual([
    "coprocessor-tfhe-worker", "coprocessor-host-listener", "coprocessor-host-listener-poller",
  ]);
  expect({ ...prepared, instances: undefined }).toEqual({ ...input, instances: undefined });
  for (const instance of input.instances ?? []) {
    expect(prepared.instances?.find((item) => item.index === instance.index)).toEqual({
      ...instance, env: { ...instance.env, DRIFT_AUTO_REVERT_ENABLED: "false" },
    });
  }
});

for (const localServices of [undefined, []]) {
  test(`degraded YAML roundtrip preserves ${localServices === undefined ? "all" : "empty"} local service selection and fills sparse instances`, () => {
    const input = parseCoprocessorScenario(YAML.stringify({
      version: 1, kind: "coprocessor-consensus", topology: { count: 3, threshold: 3 },
      instances: [{ index: 1, source: { mode: "local" }, localServices }],
    }));
    const reloaded = parseCoprocessorScenario(serializeDegradedScenario(input));
    expect(reloaded.instances?.[1]?.localServices).toEqual(localServices);
    expect(resolveScenarioFile("/tmp/degraded.yaml", reloaded)).toEqual(
      resolveScenarioFile("/tmp/degraded.yaml", prepareDegradedScenario(input)),
    );
    expect(reloaded.instances?.map((instance) => instance.env?.DRIFT_AUTO_REVERT_ENABLED))
      .toEqual(["false", "false", "false"]);
    expect(input.instances).toHaveLength(1);
  });
}

test("the real CI boot step applies the persisted scenario override only to degraded jobs", () => {
  const workflow = YAML.parse(readFileSync(path.join(cliRoot, "../../.github/workflows/test-suite-consensus.yml"), "utf8"));
  const job = Object.values(workflow.jobs).find((candidate: any) => candidate.steps?.some((step: any) => step.name === "Boot the stack")) as any;
  const boot = job.steps.find((step: any) => step.name === "Boot the stack").run;
  const temp = mkdtempSync(path.join(os.tmpdir(), "degraded-boot-"));
  try {
    symlinkSync(path.join(cliRoot, "scripts"), path.join(temp, "scripts"));
    writeFileSync(path.join(temp, "fhevm-cli"), '#!/bin/bash\nset -eu\n[[ "$1" == up && "$2" == --scenario ]]\nprintf "%s" "$3" > "$BOOT_CAPTURE"\n', { mode: 0o755 });
    const customPath = path.join(temp, "custom scenario.yaml");
    writeFileSync(customPath, authoredScenario);
    for (const [leg, source, cases] of [
      ["degraded", "three-of-three", "DEG-06-GW-LISTENER-INFLIGHT"], ["degraded", customPath, "DEG-06-GW-LISTENER-INFLIGHT"],
      ["degraded", "three-of-three", "DEG-01-AGREEMENT-QUORUM"],
      ["byte-agreement", "three-of-three", "MAT-01-BOUNDARY-FANOUT"], ["failure-matrix", "three-of-three", "FM-SNS-CRASH"],
    ]) {
      const capture = path.join(temp, `${leg}.path`);
      const result = Bun.spawnSync(["bash", "-e", "-c", `bun() { if [[ "$1" == scripts/checkout-build-receipt.ts ]]; then return 0; fi; command bun "$@"; }; ${boot}`], {
        cwd: temp,
        env: { ...process.env, LEG: leg, SCENARIO: source, CASES: cases, BUILD: "false", LOCK_FILE: "/tmp/lock.json", BOOT_CAPTURE: capture, GITHUB_ENV: path.join(temp, "github.env"), GITHUB_SHA: "branch-sha" },
      });
      expect(result.exitCode).toBe(0);
      const scenarioPath = readFileSync(capture, "utf8");
      if (cases === "DEG-06-GW-LISTENER-INFLIGHT") {
        try {
          const scenario = parseCoprocessorScenario(readFileSync(scenarioPath, "utf8"), scenarioPath);
          expect(scenario.instances?.map((instance) => instance.env?.DRIFT_AUTO_REVERT_ENABLED)).toEqual(["false", "false", "false"]);
          if (source === customPath) {
            expect(scenario).toEqual(prepareDegradedScenario(parseCoprocessorScenario(authoredScenario)));
          }
        } finally { rmSync(scenarioPath, { force: true }); }
      } else expect(scenarioPath).toBe(source);
    }
  } finally { rmSync(temp, { recursive: true, force: true }); }
});

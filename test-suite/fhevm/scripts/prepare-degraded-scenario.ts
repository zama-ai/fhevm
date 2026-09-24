import YAML from "yaml";

import { loadCoprocessorScenario } from "../src/scenario/resolve";
import type { CoprocessorScenario } from "../src/types";

/** DEG06 observes a conflicting event without allowing it to revert the fleet. */
export const prepareDegradedScenario = (input: CoprocessorScenario): CoprocessorScenario => {
  const scenario = structuredClone(input);
  scenario.instances = Array.from({ length: scenario.topology.count }, (_, index) => {
    const instance = scenario.instances?.find((candidate) => candidate.index === index);
    return {
      ...instance,
      index,
      env: { ...instance?.env, DRIFT_AUTO_REVERT_ENABLED: "false" },
    };
  });
  return scenario;
};

/** Convert normalized service names back to the authored YAML suffixes expected by up. */
export const serializeDegradedScenario = (input: CoprocessorScenario): string => {
  const scenario = prepareDegradedScenario(input);
  for (const instance of scenario.instances ?? []) {
    if (instance.localServices !== undefined) {
      instance.localServices = instance.localServices.map((service) => service.replace(/^coprocessor-/, ""));
    }
  }
  return YAML.stringify(scenario);
};

if (import.meta.main) {
  const [scenario, ...extra] = process.argv.slice(2);
  if (!scenario || extra.length) throw new Error("usage: prepare-degraded-scenario.ts <scenario>");
  process.stdout.write(serializeDegradedScenario(await loadCoprocessorScenario(scenario)));
}

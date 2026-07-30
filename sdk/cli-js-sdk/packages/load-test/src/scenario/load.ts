import { readFile } from "node:fs/promises";

import {
  createBuiltinScenario,
} from "./builtin";
import { applyScenarioOverrides, type ScenarioOverrides } from "./overrides";
import { scenarioSchema, type Scenario } from "./schema";

/**
 * Resolves a scenario reference: a built-in name (`baseline`, `open-steady`, ...)
 * or a path to a JSON scenario file (anything containing a path separator or
 * ending in `.json`).
 */
export const loadScenario = async (
  ref: string,
  params: ScenarioOverrides = {},
): Promise<Scenario> => {
  if (ref.endsWith(".json") || ref.includes("/")) {
    const text = await readFile(ref, "utf8");
    return applyScenarioOverrides(scenarioSchema.parse(JSON.parse(text)), params);
  }
  return createBuiltinScenario(ref, params);
};

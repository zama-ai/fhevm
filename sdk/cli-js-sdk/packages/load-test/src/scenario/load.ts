import { readFile } from "node:fs/promises";

import {
  createBuiltinScenario,
  type BuiltinParams,
} from "./builtin";
import { scenarioSchema, type Scenario } from "./schema";

/**
 * Resolves a scenario reference: a built-in name (`baseline`, `open-steady`, ...)
 * or a path to a JSON scenario file (anything containing a path separator or
 * ending in `.json`).
 */
export const loadScenario = async (
  ref: string,
  params: BuiltinParams = {},
): Promise<Scenario> => {
  if (ref.endsWith(".json") || ref.includes("/")) {
    const text = await readFile(ref, "utf8");
    return scenarioSchema.parse(JSON.parse(text));
  }
  return createBuiltinScenario(ref, params);
};

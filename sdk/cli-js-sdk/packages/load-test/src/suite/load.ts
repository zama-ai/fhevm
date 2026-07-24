import { readFile } from "node:fs/promises";

import { createBuiltinSuite } from "./builtin";
import { suiteSchema, type Suite } from "./schema";

/** Resolves a suite reference: built-in name or path to a suite JSON file. */
export const loadSuite = async (ref: string): Promise<Suite> => {
  if (ref.endsWith(".json") || ref.includes("/")) {
    const text = await readFile(ref, "utf8");
    return suiteSchema.parse(JSON.parse(text));
  }
  return createBuiltinSuite(ref);
};

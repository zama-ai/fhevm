import fs from "node:fs/promises";

import { STATE_FILE } from "../layout";
import type { State, StepName } from "../types";
import { exists, readJson, writeJson } from "../utils";

export const loadState = async (stateFile = STATE_FILE): Promise<State | undefined> =>
  (await exists(stateFile)) ? readJson<State>(stateFile) : undefined;

export const saveState = async (state: State, stateFile = STATE_FILE) => {
  await writeJson(stateFile, state);
};

export const markStep = async (state: State, step: StepName, stateFile = STATE_FILE) => {
  if (!state.completedSteps.includes(step)) {
    state.completedSteps.push(step);
  }
  state.updatedAt = new Date().toISOString();
  await saveState(state, stateFile);
};

export const clearState = async (stateFile = STATE_FILE) => {
  await fs.rm(stateFile, { force: true });
};

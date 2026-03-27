/**
 * Persists and mutates local CLI state across stack lifecycle operations.
 */
import fs from "node:fs/promises";

import { STATE_FILE } from "../layout";
import type { State, StepName } from "../types";
import { exists, readJson, writeJson } from "../utils/fs";

/** Loads persisted CLI state when it exists on disk. */
export const loadState = async (stateFile = STATE_FILE): Promise<State | undefined> =>
  (await exists(stateFile)) ? readJson<State>(stateFile) : undefined;

/** Persists the current CLI state to disk. */
export const saveState = async (state: State, stateFile = STATE_FILE) => {
  await writeJson(stateFile, state);
};

/** Marks a pipeline step as completed and writes the updated state. */
export const markStep = async (state: State, step: StepName, stateFile = STATE_FILE) => {
  if (!state.completedSteps.includes(step)) {
    state.completedSteps.push(step);
  }
  state.updatedAt = new Date().toISOString();
  await saveState(state, stateFile);
};

/** Removes the persisted state file without touching runtime artifacts. */
export const clearState = async (stateFile = STATE_FILE) => {
  await fs.rm(stateFile, { force: true });
};

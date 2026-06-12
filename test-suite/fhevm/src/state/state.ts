/**
 * Persists and mutates local CLI state across stack lifecycle operations.
 */
import fs from "node:fs/promises";

import { STATE_FILE } from "../layout";
import { DEFAULT_KMS_TOPOLOGY } from "../scenario/resolve";
import type { Discovery, State, StepName } from "../types";
import { exists, readJson, writeJson } from "../utils/fs";

/** Back-fills fields absent from states written by older CLI versions, so resume/teardown
 * paths can rebuild a StackSpec without crashing. */
const normalizePersistedState = (state: State): State => {
  // Pre-threshold states have no `scenario.kms`; an absent block means today's centralized node.
  if (state.scenario && !state.scenario.kms) {
    state.scenario.kms = { ...DEFAULT_KMS_TOPOLOGY };
  }
  // Discovery once held a single `kmsSigner`; fold it into the `kmsSigners` array it replaced so a
  // resumed centralized stack keeps its registered signer.
  const legacy = state.discovery as (Discovery & { kmsSigner?: string }) | undefined;
  if (legacy && !legacy.kmsSigners) {
    legacy.kmsSigners = legacy.kmsSigner ? [legacy.kmsSigner] : [];
  }
  return state;
};

/** Loads persisted CLI state when it exists on disk. */
export const loadState = async (stateFile = STATE_FILE): Promise<State | undefined> =>
  (await exists(stateFile)) ? normalizePersistedState(await readJson<State>(stateFile)) : undefined;

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

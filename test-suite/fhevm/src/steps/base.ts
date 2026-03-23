/**
 * Exposes the base-infra pipeline step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runBaseStep = (state: State) => runStep(state, "base");

/**
 * Exposes the resolve pipeline step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runResolveStep = (state: State) => runStep(state, "resolve");

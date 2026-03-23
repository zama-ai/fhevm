/**
 * Exposes the bootstrap-material publication step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runBootstrapStep = (state: State) => runStep(state, "bootstrap");

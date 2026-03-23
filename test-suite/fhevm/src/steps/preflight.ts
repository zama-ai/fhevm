/**
 * Exposes the preflight pipeline step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runPreflightStep = (state: State) => runStep(state, "preflight");

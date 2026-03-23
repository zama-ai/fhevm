/**
 * Exposes the coprocessor startup step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runCoprocessorStep = (state: State) => runStep(state, "coprocessor");

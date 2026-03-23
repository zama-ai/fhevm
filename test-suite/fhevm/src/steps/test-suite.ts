/**
 * Exposes the e2e test-suite startup step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runTestSuiteStep = (state: State) => runStep(state, "test-suite");

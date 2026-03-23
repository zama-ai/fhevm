/**
 * Exposes the kms-connector startup step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runKmsConnectorStep = (state: State) => runStep(state, "kms-connector");

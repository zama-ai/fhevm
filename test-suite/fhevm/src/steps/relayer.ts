/**
 * Exposes the relayer startup step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runRelayerStep = (state: State) => runStep(state, "relayer");

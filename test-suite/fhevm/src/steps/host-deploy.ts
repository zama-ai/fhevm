/**
 * Exposes the host contract deployment step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runHostDeployStep = (state: State) => runStep(state, "host-deploy");

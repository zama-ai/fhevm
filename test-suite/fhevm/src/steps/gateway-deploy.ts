/**
 * Exposes the gateway contract deployment step.
 */
import type { State } from "../types";
import { runStep } from "../flow/up-flow";

export const runGatewayDeployStep = (state: State) => runStep(state, "gateway-deploy");
